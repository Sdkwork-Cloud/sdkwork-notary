use sdkwork_notary_case_contract::{NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand};
use sdkwork_notary_case_repository_sqlx::{
    notary_foundation_migration_sql, SqliteNotaryCaseRepository,
};
use sdkwork_notary_case_service::{
    NotaryCaseAssignmentCommand, NotaryCaseEventListQuery, NotaryCaseListQuery,
    NotaryCaseUpdateCommand, NotaryDashboardStatisticsAggregate, NotaryDashboardStatisticsQuery,
    NotaryMonthlyCaseCountQuery, NotaryOrganizationProfileUpdateCommand, NotaryPartyListQuery,
    NotaryPartyUpdateCommand,
};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

const TEST_PII_VAULT_KEY: &str = "sdkwork-notary-test-pii-vault-key";

#[tokio::test]
async fn sqlite_repository_persists_profile_case_parties_and_events_without_dependency_tables() {
    std::env::set_var("NOTARY_PII_VAULT_KEY", TEST_PII_VAULT_KEY);
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "1");

    let profile = repository
        .upsert_organization_profile("200001", "drive-space-1", "notary")
        .await
        .unwrap();
    assert_eq!(profile.organization_id, "200001");
    assert_eq!(profile.drive_space_id, "drive-space-1");
    assert_eq!(profile.drive_space_type, "notary");

    let suspended_profile = repository
        .update_organization_profile(NotaryOrganizationProfileUpdateCommand {
            organization_id: "200001".to_string(),
            status: Some("suspended".to_string()),
            settings: Some(serde_json::json!({"reviewMode": "manual"})),
        })
        .await
        .unwrap();
    assert_eq!(suspended_profile.status, "suspended");

    repository
        .upsert_organization_profile("200002", "drive-space-2", "notary")
        .await
        .unwrap();
    let profiles = repository
        .list_organization_profiles(None, 10, None)
        .await
        .unwrap();
    assert_eq!(profiles.items.len(), 2);
    assert_eq!(profiles.items[0].organization_id, "200002");
    assert_eq!(profiles.items[1].organization_id, "200001");
    let org_1_profiles = repository
        .list_organization_profiles(Some("200001"), 10, None)
        .await
        .unwrap();
    assert_eq!(org_1_profiles.items.len(), 1);
    assert_eq!(org_1_profiles.items[0].organization_id, "200001");

    let inserted = repository.insert_case(case_record()).await.unwrap();
    assert_eq!(inserted.order_item_id, "order-item-1");
    assert_eq!(inserted.sku_id, "sku-notary-contract");
    assert_eq!(
        inserted.primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(
        inserted.primary_notary_user_id,
        Some("user-notary-1".to_string())
    );
    assert_eq!(inserted.drive_space_type, "notary");
    assert_eq!(inserted.drive_folder_node_id, "folder-case-1");

    repository
        .insert_party(
            "case-1",
            &NotaryPartyCommand {
                name: "Zhang San".to_string(),
                party_role: "applicant".to_string(),
                identity_no: "110105199001011234".to_string(),
                phone: Some("13800138000".to_string()),
            },
            "order-1",
            "order-item-1",
            "sku-notary-contract",
        )
        .await
        .unwrap();
    repository
        .append_event("case-1", "notary.case.submitted")
        .await
        .unwrap();

    let loaded = repository.get_case("case-1").await.unwrap().unwrap();
    assert_eq!(loaded.case_id, "case-1");
    assert_eq!(loaded.status, NotaryCaseStatus::PendingReview);
    assert_eq!(
        loaded.primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(
        loaded.primary_notary_user_id,
        Some("user-notary-1".to_string())
    );
    assert_eq!(loaded.drive_space_id, "drive-space-1");
    assert_eq!(loaded.drive_space_type, "notary");

    let updated = repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: "case-1".to_string(),
            expected_version: 1,
            title: Some("Updated contract preservation".to_string()),
            remarks: Some("accepted".to_string()),
            status: Some(NotaryCaseStatus::Processing),
            chain_hash: None,
            reject_reason: None,
            event_type: "notary.case.updated".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated contract preservation");
    assert_eq!(updated.status, NotaryCaseStatus::Processing);
    assert_eq!(updated.version, 2);

    let listed = repository
        .list_cases(NotaryCaseListQuery {
            organization_id: "200001".to_string(),
            status: Some("processing".to_string()),
            sku_id: Some("sku-notary-contract".to_string()),
            search_term: Some("contract".to_string()),
            page_size: 10,
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(listed.items.len(), 1);
    assert_eq!(listed.total_items, 1);
    assert_eq!(listed.items[0].case_id, "case-1");
    assert_eq!(listed.items[0].sku_id, "sku-notary-contract");
    assert_eq!(
        listed.items[0].primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(listed.items[0].drive_space_type, "notary");

    let parties = repository
        .list_parties(NotaryPartyListQuery {
            case_id: "case-1".to_string(),
            page_size: 20,
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(parties.items.len(), 1);
    assert_eq!(parties.items[0].name, "Zhang San");
    assert_eq!(parties.items[0].order_id, "order-1");
    assert_eq!(parties.items[0].sku_id, "sku-notary-contract");
    assert_eq!(parties.items[0].identity_no_last4, "1234");

    let updated_party = repository
        .update_party(NotaryPartyUpdateCommand {
            case_id: "case-1".to_string(),
            party_id: parties.items[0].party_id.clone(),
            name: Some("Li Si".to_string()),
            party_role: Some("counterparty".to_string()),
            identity_no: Some("110105199202021235".to_string()),
            phone: Some("13900139000".to_string()),
            signature_node_id: Some("signature-node-1".to_string()),
        })
        .await
        .unwrap();
    assert_eq!(updated_party.name, "Li Si");
    assert_eq!(updated_party.party_role, "counterparty");
    assert_eq!(updated_party.identity_no_last4, "1235");
    assert_eq!(updated_party.phone_masked, Some("139****9000".to_string()));
    assert_eq!(
        updated_party.signature_node_id,
        Some("signature-node-1".to_string())
    );

    let signed_parties = repository
        .list_parties(NotaryPartyListQuery {
            case_id: "case-1".to_string(),
            page_size: 20,
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(
        signed_parties.items[0].signature_node_id,
        Some("signature-node-1".to_string())
    );

    repository
        .remove_party("case-1", &updated_party.party_id)
        .await
        .unwrap();
    let active_parties = repository
        .list_parties(NotaryPartyListQuery {
            case_id: "case-1".to_string(),
            page_size: 20,
            cursor: None,
        })
        .await
        .unwrap();
    assert!(active_parties.items.is_empty());

    let assignment = repository
        .insert_assignment(NotaryCaseAssignmentCommand {
            case_id: "case-1".to_string(),
            organization_id: "200001".to_string(),
            organization_membership_id: "member-notary-1".to_string(),
            user_id: "user-notary-1".to_string(),
            assignment_role: "primary_notary".to_string(),
            assigned_by_membership_id: Some("member-admin-1".to_string()),
        })
        .await
        .unwrap();
    assert_eq!(assignment.case_id, "case-1");
    assert_eq!(assignment.organization_membership_id, "member-notary-1");
    assert_eq!(assignment.assignment_role, "primary_notary");
    assert_eq!(assignment.status, "active");

    repository
        .release_assignment("case-1", &assignment.assignment_id)
        .await
        .unwrap();

    let events = repository
        .list_events(NotaryCaseEventListQuery {
            case_id: "case-1".to_string(),
            page_size: 10,
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(events.items.len(), 2);
    assert_eq!(events.items[0].event_type, "notary.case.submitted");
    assert_eq!(events.items[1].event_type, "notary.case.updated");

    let dependency_table_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(1)
        FROM sqlite_master
        WHERE type = 'table'
          AND (
            name LIKE 'commerce_%'
            OR name LIKE 'dr_drive_%'
            OR name LIKE 'iam_%'
          )
        "#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(dependency_table_count, 0);
}

#[tokio::test]
async fn sqlite_case_update_uses_compare_and_swap_and_persists_the_event_atomically() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "actor-1");
    repository.insert_case(case_record()).await.unwrap();

    let updated = repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: "case-1".to_string(),
            expected_version: 1,
            title: Some("Accepted contract preservation".to_string()),
            remarks: Some("reviewed".to_string()),
            status: Some(NotaryCaseStatus::Rejected),
            chain_hash: None,
            reject_reason: Some("identity document expired".to_string()),
            event_type: "notary.case.rejected".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(updated.version, 2);
    assert_eq!(updated.status, NotaryCaseStatus::Rejected);
    assert_eq!(updated.title, "Accepted contract preservation");

    let persisted: (String, i64, Option<String>) = sqlx::query_as(
        "SELECT status, version, reject_reason FROM notary_case WHERE tenant_id = ?1 AND id = ?2",
    )
    .bind("100001")
    .bind("case-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(persisted.0, "rejected");
    assert_eq!(persisted.1, 2);
    assert_eq!(persisted.2.as_deref(), Some("identity document expired"));

    let event: (String, Option<String>) = sqlx::query_as(
        "SELECT event_type, actor_user_id FROM notary_case_event WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind("100001")
    .bind("case-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(event.0, "notary.case.rejected");
    assert_eq!(event.1.as_deref(), Some("actor-1"));
}

#[tokio::test]
async fn sqlite_case_update_rejects_a_stale_version_without_changing_state_or_events() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "actor-1");
    repository.insert_case(case_record()).await.unwrap();

    let first = repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: "case-1".to_string(),
            expected_version: 1,
            title: Some("First accepted title".to_string()),
            remarks: None,
            status: Some(NotaryCaseStatus::Processing),
            chain_hash: None,
            reject_reason: None,
            event_type: "notary.case.accepted".to_string(),
        })
        .await
        .unwrap();
    assert_eq!(first.version, 2);

    let error = repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: "case-1".to_string(),
            expected_version: 1,
            title: Some("Stale title".to_string()),
            remarks: Some("must not persist".to_string()),
            status: Some(NotaryCaseStatus::Rejected),
            chain_hash: None,
            reject_reason: Some("must not persist".to_string()),
            event_type: "notary.case.rejected".to_string(),
        })
        .await
        .expect_err("stale compare-and-swap update");

    assert_eq!(error.code(), "conflict");
    let persisted = repository.get_case("case-1").await.unwrap().unwrap();
    assert_eq!(persisted.version, 2);
    assert_eq!(persisted.status, NotaryCaseStatus::Processing);
    assert_eq!(persisted.title, "First accepted title");
    assert_eq!(persisted.remarks.as_deref(), Some("priority"));

    let event_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_case_event WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind("100001")
    .bind("case-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(event_count, 1);
}

#[tokio::test]
async fn sqlite_case_update_rolls_back_when_the_event_insert_fails() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "actor-1");
    repository.insert_case(case_record()).await.unwrap();
    sqlx::raw_sql(
        r#"
        CREATE TRIGGER fail_notary_case_event_insert
        BEFORE INSERT ON notary_case_event
        WHEN NEW.event_type = 'notary.case.rollback'
        BEGIN
          SELECT RAISE(ABORT, 'forced notary event failure');
        END;
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let error = repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: "case-1".to_string(),
            expected_version: 1,
            title: Some("Rolled back title".to_string()),
            remarks: Some("rolled back".to_string()),
            status: Some(NotaryCaseStatus::Processing),
            chain_hash: None,
            reject_reason: None,
            event_type: "notary.case.rollback".to_string(),
        })
        .await
        .expect_err("event insertion must fail the transaction");

    assert_eq!(error.code(), "storage");
    let persisted = repository.get_case("case-1").await.unwrap().unwrap();
    assert_eq!(persisted.version, 1);
    assert_eq!(persisted.status, NotaryCaseStatus::PendingReview);
    assert_eq!(persisted.title, "Electronic contract preservation");
    assert_eq!(persisted.remarks.as_deref(), Some("priority"));

    let event_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_case_event WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind("100001")
    .bind("case-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(event_count, 0);
}

#[tokio::test]
async fn concurrent_sqlite_case_updates_with_the_same_version_allow_only_one_success() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "actor-1");
    repository.insert_case(case_record()).await.unwrap();

    let accept = repository.update_case(NotaryCaseUpdateCommand {
        case_id: "case-1".to_string(),
        expected_version: 1,
        title: None,
        remarks: Some("accepted".to_string()),
        status: Some(NotaryCaseStatus::Processing),
        chain_hash: None,
        reject_reason: None,
        event_type: "notary.case.accepted".to_string(),
    });
    let reject = repository.update_case(NotaryCaseUpdateCommand {
        case_id: "case-1".to_string(),
        expected_version: 1,
        title: None,
        remarks: None,
        status: Some(NotaryCaseStatus::Rejected),
        chain_hash: None,
        reject_reason: Some("rejected concurrently".to_string()),
        event_type: "notary.case.rejected".to_string(),
    });
    let (accept_result, reject_result) = tokio::join!(accept, reject);

    assert_eq!(
        usize::from(accept_result.is_ok()) + usize::from(reject_result.is_ok()),
        1
    );
    let conflict = accept_result
        .err()
        .or_else(|| reject_result.err())
        .expect("one compare-and-swap loser");
    assert_eq!(conflict.code(), "conflict");

    let persisted = repository.get_case("case-1").await.unwrap().unwrap();
    assert_eq!(persisted.version, 2);
    assert!(matches!(
        persisted.status,
        NotaryCaseStatus::Processing | NotaryCaseStatus::Rejected
    ));
    let event_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_case_event WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind("100001")
    .bind("case-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(event_count, 1);
}

#[tokio::test]
async fn sqlite_dashboard_statistics_uses_utc_database_dates_and_scope_predicates() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "1");

    for record in [
        dashboard_case(
            "case-dashboard-pending",
            "200001",
            NotaryCaseStatus::PendingReview,
            None,
        ),
        dashboard_case(
            "case-dashboard-today",
            "200001",
            NotaryCaseStatus::Completed,
            Some("chain-today"),
        ),
        dashboard_case(
            "case-dashboard-yesterday",
            "200001",
            NotaryCaseStatus::Completed,
            None,
        ),
        dashboard_case(
            "case-dashboard-anomaly",
            "200001",
            NotaryCaseStatus::Rejected,
            None,
        ),
        dashboard_case(
            "case-dashboard-old-month",
            "200001",
            NotaryCaseStatus::Processing,
            None,
        ),
        dashboard_case(
            "case-dashboard-other-organization",
            "200002",
            NotaryCaseStatus::PendingReview,
            None,
        ),
    ] {
        repository.insert_case(record).await.unwrap();
    }

    let other_tenant_repository =
        SqliteNotaryCaseRepository::new(pool.clone(), "100002", "other-user");
    other_tenant_repository
        .insert_case(dashboard_case(
            "case-dashboard-other-tenant",
            "200001",
            NotaryCaseStatus::PendingReview,
            None,
        ))
        .await
        .unwrap();

    sqlx::query(
        r#"
        UPDATE notary_case
        SET
            created_at = CASE
                WHEN id = 'case-dashboard-old-month'
                    THEN strftime('%Y-%m-%dT12:00:00Z', 'now', 'start of month', '-1 day')
                ELSE strftime('%Y-%m-%dT12:00:00Z', 'now', 'start of month')
            END,
            completed_at = CASE
                WHEN id = 'case-dashboard-today'
                    THEN strftime('%Y-%m-%dT12:00:00Z', 'now')
                WHEN id = 'case-dashboard-yesterday'
                    THEN strftime('%Y-%m-%dT12:00:00Z', 'now', '-1 day')
                ELSE completed_at
            END
        WHERE id LIKE 'case-dashboard-%'
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let statistics = repository
        .get_dashboard_statistics(NotaryDashboardStatisticsQuery {
            organization_id: "200001".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(
        statistics,
        NotaryDashboardStatisticsAggregate {
            pending_review_count: 1,
            today_completed_count: 1,
            yesterday_completed_count: 1,
            monthly_case_count: 4,
            anomaly_intercepted_count: 1,
            unsynced_completed_count: 1,
        }
    );
}

#[tokio::test]
async fn sqlite_monthly_case_count_uses_utc_range_and_scope_predicates() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "100001", "1");

    let mut month_start = dashboard_case(
        "case-month-start",
        "200001",
        NotaryCaseStatus::PendingReview,
        None,
    );
    month_start.created_at = "2026-06-01T00:00:00Z".to_string();
    let mut utc_month_end_with_offset = dashboard_case(
        "case-month-end-offset",
        "200001",
        NotaryCaseStatus::Completed,
        Some("chain-month-end"),
    );
    utc_month_end_with_offset.created_at = "2026-07-01T07:59:59+08:00".to_string();
    let mut next_month = dashboard_case(
        "case-next-month",
        "200001",
        NotaryCaseStatus::Processing,
        None,
    );
    next_month.created_at = "2026-07-01T00:00:00Z".to_string();
    let mut other_organization = dashboard_case(
        "case-month-other-organization",
        "200002",
        NotaryCaseStatus::PendingReview,
        None,
    );
    other_organization.created_at = "2026-06-15T00:00:00Z".to_string();

    for record in [
        month_start,
        utc_month_end_with_offset,
        next_month,
        other_organization,
    ] {
        repository.insert_case(record).await.unwrap();
    }

    let other_tenant_repository =
        SqliteNotaryCaseRepository::new(pool.clone(), "100002", "other-user");
    let mut other_tenant = dashboard_case(
        "case-month-other-tenant",
        "200001",
        NotaryCaseStatus::PendingReview,
        None,
    );
    other_tenant.created_at = "2026-06-20T00:00:00Z".to_string();
    other_tenant_repository
        .insert_case(other_tenant)
        .await
        .unwrap();

    let count = repository
        .count_cases_for_month(NotaryMonthlyCaseCountQuery::new("200001", "2026-06").unwrap())
        .await
        .unwrap();

    assert_eq!(count.count, 2);
    for invalid in ["2026-6", "2026-00", "2026-13", "0000-01", "year-01"] {
        assert!(NotaryMonthlyCaseCountQuery::new("200001", invalid).is_err());
    }
}

async fn migrated_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(notary_foundation_migration_sql())
        .execute(&pool)
        .await
        .unwrap();
    pool
}

fn case_record() -> NotaryCaseRecord {
    NotaryCaseRecord {
        case_id: "case-1".to_string(),
        case_no: "NT-20260610-000001".to_string(),
        organization_id: "200001".to_string(),
        title: "Electronic contract preservation".to_string(),
        applicant_name: "Zhang San Network".to_string(),
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        primary_notary_user_id: Some("user-notary-1".to_string()),
        primary_notary_name: Some("Li Ming".to_string()),
        status: NotaryCaseStatus::PendingReview,
        order_id: "order-1".to_string(),
        order_item_id: "order-item-1".to_string(),
        sku_id: "sku-notary-contract".to_string(),
        matter_title: "Electronic contract preservation".to_string(),
        fee_amount: "500.00".to_string(),
        currency_code: "CNY".to_string(),
        drive_space_id: "drive-space-1".to_string(),
        drive_space_type: "notary".to_string(),
        drive_folder_node_id: "folder-case-1".to_string(),
        chain_hash: None,
        remarks: Some("priority".to_string()),
        request_no: "REQ-20260610-000001".to_string(),
        idempotency_key: "idem-case-1".to_string(),
        version: 1,
        created_at: "2026-06-10T10:00:00Z".to_string(),
        updated_at: "2026-06-10T10:00:00Z".to_string(),
    }
}

fn dashboard_case(
    case_id: &str,
    organization_id: &str,
    status: NotaryCaseStatus,
    chain_hash: Option<&str>,
) -> NotaryCaseRecord {
    let mut record = case_record();
    record.case_id = case_id.to_string();
    record.case_no = format!("NT-{case_id}");
    record.organization_id = organization_id.to_string();
    record.status = status;
    record.order_id = format!("order-{case_id}");
    record.order_item_id = format!("order-item-{case_id}");
    record.request_no = format!("request-{case_id}");
    record.idempotency_key = format!("idempotency-{case_id}");
    record.chain_hash = chain_hash.map(str::to_owned);
    record
}

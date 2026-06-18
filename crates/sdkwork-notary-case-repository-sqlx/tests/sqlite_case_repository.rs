use sdkwork_notary_case_contract::{NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand};
use sdkwork_notary_case_repository_sqlx::{
    notary_foundation_migration_sql, SqliteNotaryCaseRepository,
};
use sdkwork_notary_case_service::{
    NotaryCaseAssignmentCommand, NotaryCaseEventListQuery, NotaryCaseListQuery,
    NotaryCaseUpdateCommand, NotaryOrganizationProfileUpdateCommand, NotaryPartyUpdateCommand,
};
use sqlx::SqlitePool;

#[tokio::test]
async fn sqlite_repository_persists_profile_case_parties_and_events_without_dependency_tables() {
    let pool = migrated_pool().await;
    let repository = SqliteNotaryCaseRepository::new(pool.clone(), "tenant-1", "user-1");

    let profile = repository
        .upsert_organization_profile("org-1", "drive-space-1", "notary")
        .await
        .unwrap();
    assert_eq!(profile.organization_id, "org-1");
    assert_eq!(profile.drive_space_id, "drive-space-1");
    assert_eq!(profile.drive_space_type, "notary");

    let suspended_profile = repository
        .update_organization_profile(NotaryOrganizationProfileUpdateCommand {
            organization_id: "org-1".to_string(),
            status: Some("suspended".to_string()),
            settings: Some(serde_json::json!({"reviewMode": "manual"})),
        })
        .await
        .unwrap();
    assert_eq!(suspended_profile.status, "suspended");

    repository
        .upsert_organization_profile("org-2", "drive-space-2", "notary")
        .await
        .unwrap();
    let profiles = repository
        .list_organization_profiles(None, 10)
        .await
        .unwrap();
    assert_eq!(profiles.len(), 2);
    assert_eq!(profiles[0].organization_id, "org-2");
    assert_eq!(profiles[1].organization_id, "org-1");
    let org_1_profiles = repository
        .list_organization_profiles(Some("org-1"), 10)
        .await
        .unwrap();
    assert_eq!(org_1_profiles.len(), 1);
    assert_eq!(org_1_profiles[0].organization_id, "org-1");

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
            title: Some("Updated contract preservation".to_string()),
            remarks: Some("accepted".to_string()),
            status: Some(NotaryCaseStatus::Processing),
            chain_hash: None,
        })
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated contract preservation");
    assert_eq!(updated.status, NotaryCaseStatus::Processing);

    let listed = repository
        .list_cases(NotaryCaseListQuery {
            organization_id: "org-1".to_string(),
            status: Some("processing".to_string()),
            sku_id: Some("sku-notary-contract".to_string()),
            search_term: Some("contract".to_string()),
            page_size: 10,
            cursor: None,
        })
        .await
        .unwrap();
    assert_eq!(listed.items.len(), 1);
    assert_eq!(listed.items[0].case_id, "case-1");
    assert_eq!(listed.items[0].sku_id, "sku-notary-contract");
    assert_eq!(
        listed.items[0].primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(listed.items[0].drive_space_type, "notary");

    let parties = repository.list_parties("case-1").await.unwrap();
    assert_eq!(parties.len(), 1);
    assert_eq!(parties[0].name, "Zhang San");
    assert_eq!(parties[0].order_id, "order-1");
    assert_eq!(parties[0].sku_id, "sku-notary-contract");
    assert_eq!(parties[0].identity_no_last4, "1234");

    let updated_party = repository
        .update_party(NotaryPartyUpdateCommand {
            case_id: "case-1".to_string(),
            party_id: parties[0].party_id.clone(),
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

    let signed_parties = repository.list_parties("case-1").await.unwrap();
    assert_eq!(
        signed_parties[0].signature_node_id,
        Some("signature-node-1".to_string())
    );

    repository
        .remove_party("case-1", &updated_party.party_id)
        .await
        .unwrap();
    let active_parties = repository.list_parties("case-1").await.unwrap();
    assert!(active_parties.is_empty());

    let assignment = repository
        .insert_assignment(NotaryCaseAssignmentCommand {
            case_id: "case-1".to_string(),
            organization_id: "org-1".to_string(),
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
        .release_assignment(&assignment.assignment_id)
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
    assert_eq!(events.items.len(), 1);
    assert_eq!(events.items[0].event_type, "notary.case.submitted");

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

async fn migrated_pool() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
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
        organization_id: "org-1".to_string(),
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
        created_at: "2026-06-10T10:00:00Z".to_string(),
        updated_at: "2026-06-10T10:00:00Z".to_string(),
    }
}

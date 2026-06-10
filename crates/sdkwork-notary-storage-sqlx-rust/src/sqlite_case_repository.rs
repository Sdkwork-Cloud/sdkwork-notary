use async_trait::async_trait;
use sdkwork_notary_core::{
    NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand, NotaryServiceError,
};
use sdkwork_notary_runtime::{
    NotaryCaseAssignmentCommand, NotaryCaseAssignmentRecord, NotaryCaseEventListQuery,
    NotaryCaseListQuery, NotaryCaseRepositoryPort, NotaryCaseUpdateCommand,
    NotaryOrganizationProfile, NotaryOrganizationProfileUpdateCommand, NotaryPartyUpdateCommand,
};
pub use sdkwork_notary_runtime::{NotaryCaseEventRecord, NotaryPartyRecord};
use sqlx::{Row, SqlitePool};

const DEFAULT_NOW: &str = "2026-06-10T00:00:00Z";

pub fn notary_foundation_migration_sql() -> &'static str {
    include_str!("../migrations/0001_notary_foundation.sql")
}

#[derive(Clone, Debug)]
pub struct SqliteNotaryCaseRepository {
    pool: SqlitePool,
    tenant_id: String,
    actor_user_id: String,
}

impl SqliteNotaryCaseRepository {
    pub fn new(
        pool: SqlitePool,
        tenant_id: impl Into<String>,
        actor_user_id: impl Into<String>,
    ) -> Self {
        Self {
            pool,
            tenant_id: tenant_id.into(),
            actor_user_id: actor_user_id.into(),
        }
    }

    pub async fn upsert_organization_profile(
        &self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        if drive_space_type != "notary" {
            return Err(NotaryServiceError::validation(
                "notary organization profile requires drive_space_type notary",
            ));
        }

        sqlx::query(
            r#"
            INSERT INTO notary_organization_profile (
                id,
                tenant_id,
                organization_id,
                status,
                drive_space_id,
                drive_space_type,
                opened_by_membership_id,
                opened_at,
                created_at,
                updated_at,
                version
            )
            VALUES (?1, ?2, ?3, 'active', ?4, ?5, NULL, ?6, ?6, ?6, 1)
            ON CONFLICT (tenant_id, organization_id)
            DO UPDATE SET
                status = 'active',
                drive_space_id = excluded.drive_space_id,
                drive_space_type = excluded.drive_space_type,
                updated_at = excluded.updated_at,
                version = notary_organization_profile.version + 1
            "#,
        )
        .bind(format!("notary-profile-{organization_id}"))
        .bind(&self.tenant_id)
        .bind(organization_id)
        .bind(drive_space_id)
        .bind(drive_space_type)
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to upsert notary organization profile"))?;

        Ok(NotaryOrganizationProfile {
            organization_id: organization_id.to_string(),
            drive_space_id: drive_space_id.to_string(),
            drive_space_type: drive_space_type.to_string(),
            status: "active".to_string(),
        })
    }

    pub async fn get_organization_profile(
        &self,
        organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError> {
        let row = sqlx::query(
            r#"
            SELECT organization_id, drive_space_id, drive_space_type, status
            FROM notary_organization_profile
            WHERE tenant_id = ?1
              AND organization_id = ?2
            LIMIT 1
            "#,
        )
        .bind(&self.tenant_id)
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(store_error("failed to get notary organization profile"))?;

        Ok(row.as_ref().map(profile_from_row))
    }

    pub async fn list_organization_profiles(
        &self,
        organization_id: Option<&str>,
        page_size: i64,
    ) -> Result<Vec<NotaryOrganizationProfile>, NotaryServiceError> {
        let rows = sqlx::query(
            r#"
            SELECT organization_id, drive_space_id, drive_space_type, status
            FROM notary_organization_profile
            WHERE tenant_id = ?1
              AND (?2 IS NULL OR organization_id = ?2)
            ORDER BY updated_at DESC, id DESC
            LIMIT ?3
            "#,
        )
        .bind(&self.tenant_id)
        .bind(organization_id)
        .bind(page_size)
        .fetch_all(&self.pool)
        .await
        .map_err(store_error("failed to list notary organization profiles"))?;

        Ok(rows.iter().map(profile_from_row).collect())
    }

    pub async fn update_organization_profile(
        &self,
        command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        if let Some(status) = command.status.as_deref() {
            validate_profile_status(status)?;
        }
        let settings_json = command
            .settings
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|error| {
                NotaryServiceError::validation(format!(
                    "failed to serialize notary organization profile settings: {error}"
                ))
            })?;

        let result = sqlx::query(
            r#"
            UPDATE notary_organization_profile
            SET
                status = COALESCE(?3, status),
                settings_json = COALESCE(?4, settings_json),
                suspended_at = CASE WHEN ?3 = 'suspended' THEN ?5 ELSE suspended_at END,
                closed_at = CASE WHEN ?3 = 'closed' THEN ?5 ELSE closed_at END,
                updated_at = ?5,
                version = version + 1
            WHERE tenant_id = ?1
              AND organization_id = ?2
            "#,
        )
        .bind(&self.tenant_id)
        .bind(&command.organization_id)
        .bind(command.status.as_deref())
        .bind(settings_json.as_deref())
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to update notary organization profile"))?;

        if result.rows_affected() == 0 {
            return Err(NotaryServiceError::not_found(
                "notary organization profile not found",
            ));
        }

        self.get_organization_profile(&command.organization_id)
            .await?
            .ok_or_else(|| NotaryServiceError::not_found("notary organization profile not found"))
    }

    pub async fn insert_case(
        &self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        if record.drive_space_type != "notary" {
            return Err(NotaryServiceError::validation(
                "notary case requires drive_space_type notary",
            ));
        }

        sqlx::query(
            r#"
            INSERT INTO notary_case (
                id,
                tenant_id,
                organization_id,
                case_no,
                title,
                remarks,
                status,
                applicant_user_id,
                applicant_name_snapshot,
                primary_notary_membership_id,
                primary_notary_user_id,
                primary_notary_name_snapshot,
                order_id,
                order_item_id,
                sku_id,
                matter_title_snapshot,
                fee_amount_snapshot,
                currency_code,
                drive_space_id,
                drive_space_type,
                drive_folder_node_id,
                chain_hash,
                submitted_at,
                request_no,
                idempotency_key,
                created_at,
                updated_at,
                version
            )
            VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, 1
            )
            "#,
        )
        .bind(&record.case_id)
        .bind(&self.tenant_id)
        .bind(&record.organization_id)
        .bind(&record.case_no)
        .bind(&record.title)
        .bind(record.remarks.as_deref())
        .bind(record.status.as_storage_value())
        .bind(&self.actor_user_id)
        .bind(&record.applicant_name)
        .bind(record.primary_notary_membership_id.as_deref())
        .bind(record.primary_notary_user_id.as_deref())
        .bind(record.primary_notary_name.as_deref())
        .bind(&record.order_id)
        .bind(&record.order_item_id)
        .bind(&record.sku_id)
        .bind(&record.matter_title)
        .bind(&record.fee_amount)
        .bind(&record.currency_code)
        .bind(&record.drive_space_id)
        .bind(&record.drive_space_type)
        .bind(&record.drive_folder_node_id)
        .bind(record.chain_hash.as_deref())
        .bind(&record.created_at)
        .bind(&record.request_no)
        .bind(&record.idempotency_key)
        .bind(&record.created_at)
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to insert notary case"))?;

        Ok(record)
    }

    pub async fn insert_party(
        &self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let case = self
            .get_case(case_id)
            .await?
            .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
        let next_order = next_party_sort_order(&self.pool, &self.tenant_id, case_id).await?;
        let identity_hash = stable_hash(&party.identity_no);
        let identity_no_last4 = last4(&party.identity_no);

        sqlx::query(
            r#"
            INSERT INTO notary_party (
                id,
                tenant_id,
                organization_id,
                case_id,
                order_id,
                order_item_id,
                sku_id,
                party_role,
                name,
                identity_no_hash,
                identity_no_encrypted,
                identity_no_last4,
                phone_encrypted,
                phone_masked,
                sort_order,
                created_at,
                updated_at
            )
            VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?16
            )
            "#,
        )
        .bind(format!("party-{case_id}-{next_order}"))
        .bind(&self.tenant_id)
        .bind(&case.organization_id)
        .bind(case_id)
        .bind(order_id)
        .bind(order_item_id)
        .bind(sku_id)
        .bind(&party.party_role)
        .bind(&party.name)
        .bind(&identity_hash)
        .bind(format!("notary-local-vault:v1:{identity_hash}"))
        .bind(&identity_no_last4)
        .bind(
            party
                .phone
                .as_ref()
                .map(|phone| format!("notary-local-vault:v1:{}", stable_hash(phone))),
        )
        .bind(party.phone.as_deref().map(mask_phone))
        .bind(next_order)
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to insert notary party"))?;

        Ok(())
    }

    pub async fn append_event(
        &self,
        case_id: &str,
        event_type: &str,
    ) -> Result<(), NotaryServiceError> {
        let case = self
            .get_case(case_id)
            .await?
            .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
        let next_order = next_event_order(&self.pool, &self.tenant_id, case_id).await?;

        sqlx::query(
            r#"
            INSERT INTO notary_case_event (
                id,
                tenant_id,
                organization_id,
                case_id,
                event_type,
                event_title,
                actor_user_id,
                occurred_at,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
            "#,
        )
        .bind(format!("event-{case_id}-{next_order}"))
        .bind(&self.tenant_id)
        .bind(&case.organization_id)
        .bind(case_id)
        .bind(event_type)
        .bind(event_title(event_type))
        .bind(&self.actor_user_id)
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to append notary case event"))?;

        Ok(())
    }

    pub async fn get_case(
        &self,
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                case_no,
                organization_id,
                title,
                remarks,
                status,
                applicant_name_snapshot,
                primary_notary_membership_id,
                primary_notary_user_id,
                primary_notary_name_snapshot,
                order_id,
                order_item_id,
                sku_id,
                matter_title_snapshot,
                fee_amount_snapshot,
                currency_code,
                drive_space_id,
                drive_space_type,
                drive_folder_node_id,
                chain_hash,
                request_no,
                idempotency_key,
                created_at,
                updated_at
            FROM notary_case
            WHERE tenant_id = ?1
              AND id = ?2
            LIMIT 1
            "#,
        )
        .bind(&self.tenant_id)
        .bind(case_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(store_error("failed to get notary case"))?;

        row.as_ref().map(case_from_row).transpose()
    }

    pub async fn update_case(
        &self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        let mut record = self
            .get_case(&command.case_id)
            .await?
            .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;

        if let Some(title) = command.title {
            record.title = title;
        }
        if let Some(remarks) = command.remarks {
            record.remarks = Some(remarks);
        }
        if let Some(status) = command.status {
            record.status = status;
        }
        if let Some(chain_hash) = command.chain_hash {
            record.chain_hash = Some(chain_hash);
        }
        record.updated_at = DEFAULT_NOW.to_string();

        sqlx::query(
            r#"
            UPDATE notary_case
            SET
                title = ?3,
                remarks = ?4,
                status = ?5,
                chain_hash = ?6,
                accepted_at = CASE WHEN ?5 = 'processing' THEN ?7 ELSE accepted_at END,
                completed_at = CASE WHEN ?5 = 'completed' THEN ?7 ELSE completed_at END,
                rejected_at = CASE WHEN ?5 = 'rejected' THEN ?7 ELSE rejected_at END,
                updated_at = ?7,
                version = version + 1
            WHERE tenant_id = ?1
              AND id = ?2
            "#,
        )
        .bind(&self.tenant_id)
        .bind(&record.case_id)
        .bind(&record.title)
        .bind(record.remarks.as_deref())
        .bind(record.status.as_storage_value())
        .bind(record.chain_hash.as_deref())
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to update notary case"))?;

        Ok(record)
    }

    pub async fn get_party(
        &self,
        case_id: &str,
        party_id: &str,
    ) -> Result<Option<NotaryPartyRecord>, NotaryServiceError> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                case_id,
                order_id,
                order_item_id,
                sku_id,
                name,
                party_role,
                identity_no_last4,
                phone_masked,
                status,
                signature_node_id
            FROM notary_party
            WHERE tenant_id = ?1
              AND case_id = ?2
              AND id = ?3
            LIMIT 1
            "#,
        )
        .bind(&self.tenant_id)
        .bind(case_id)
        .bind(party_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(store_error("failed to get notary party"))?;

        Ok(row.as_ref().map(party_from_row))
    }

    pub async fn update_party(
        &self,
        command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        let identity_hash = command.identity_no.as_ref().map(|value| stable_hash(value));
        let identity_no_encrypted = identity_hash
            .as_ref()
            .map(|hash| format!("notary-local-vault:v1:{hash}"));
        let identity_no_last4 = command.identity_no.as_deref().map(last4);
        let phone_encrypted = command
            .phone
            .as_ref()
            .map(|phone| format!("notary-local-vault:v1:{}", stable_hash(phone)));
        let phone_masked = command.phone.as_deref().map(mask_phone);

        let result = sqlx::query(
            r#"
            UPDATE notary_party
            SET
                name = COALESCE(?4, name),
                party_role = COALESCE(?5, party_role),
                identity_no_hash = COALESCE(?6, identity_no_hash),
                identity_no_encrypted = COALESCE(?7, identity_no_encrypted),
                identity_no_last4 = COALESCE(?8, identity_no_last4),
                phone_encrypted = COALESCE(?9, phone_encrypted),
                phone_masked = COALESCE(?10, phone_masked),
                signature_node_id = COALESCE(?11, signature_node_id),
                updated_at = ?12,
                version = version + 1
            WHERE tenant_id = ?1
              AND case_id = ?2
              AND id = ?3
              AND status = 'active'
            "#,
        )
        .bind(&self.tenant_id)
        .bind(&command.case_id)
        .bind(&command.party_id)
        .bind(command.name.as_deref())
        .bind(command.party_role.as_deref())
        .bind(identity_hash.as_deref())
        .bind(identity_no_encrypted.as_deref())
        .bind(identity_no_last4.as_deref())
        .bind(phone_encrypted.as_deref())
        .bind(phone_masked.as_deref())
        .bind(command.signature_node_id.as_deref())
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to update notary party"))?;

        if result.rows_affected() == 0 {
            return Err(NotaryServiceError::not_found("notary party not found"));
        }

        self.get_party(&command.case_id, &command.party_id)
            .await?
            .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))
    }

    pub async fn remove_party(
        &self,
        case_id: &str,
        party_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let result = sqlx::query(
            r#"
            UPDATE notary_party
            SET
                status = 'removed',
                updated_at = ?4,
                version = version + 1
            WHERE tenant_id = ?1
              AND case_id = ?2
              AND id = ?3
              AND status = 'active'
            "#,
        )
        .bind(&self.tenant_id)
        .bind(case_id)
        .bind(party_id)
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to remove notary party"))?;

        if result.rows_affected() == 0 {
            return Err(NotaryServiceError::not_found("notary party not found"));
        }
        Ok(())
    }

    pub async fn insert_assignment(
        &self,
        command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
        validate_assignment_role(&command.assignment_role)?;
        let assignment_id = format!(
            "assignment-{}-{}-{}",
            command.case_id, command.organization_membership_id, command.assignment_role
        );

        sqlx::query(
            r#"
            INSERT INTO notary_case_assignment (
                id,
                tenant_id,
                organization_id,
                case_id,
                organization_membership_id,
                user_id,
                assignment_role,
                status,
                assigned_by_membership_id,
                assigned_at,
                released_at,
                created_at,
                updated_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'active', ?8, ?9, NULL, ?9, ?9)
            ON CONFLICT (tenant_id, case_id, organization_membership_id, assignment_role)
            DO UPDATE SET
                user_id = excluded.user_id,
                status = 'active',
                assigned_by_membership_id = excluded.assigned_by_membership_id,
                assigned_at = excluded.assigned_at,
                released_at = NULL,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&assignment_id)
        .bind(&self.tenant_id)
        .bind(&command.organization_id)
        .bind(&command.case_id)
        .bind(&command.organization_membership_id)
        .bind(&command.user_id)
        .bind(&command.assignment_role)
        .bind(command.assigned_by_membership_id.as_deref())
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to insert notary case assignment"))?;

        Ok(NotaryCaseAssignmentRecord {
            assignment_id,
            case_id: command.case_id,
            organization_membership_id: command.organization_membership_id,
            user_id: command.user_id,
            assignment_role: command.assignment_role,
            status: "active".to_string(),
            assigned_at: DEFAULT_NOW.to_string(),
        })
    }

    pub async fn release_assignment(&self, assignment_id: &str) -> Result<(), NotaryServiceError> {
        let result = sqlx::query(
            r#"
            UPDATE notary_case_assignment
            SET
                status = 'released',
                released_at = ?3,
                updated_at = ?3
            WHERE tenant_id = ?1
              AND id = ?2
              AND status = 'active'
            "#,
        )
        .bind(&self.tenant_id)
        .bind(assignment_id)
        .bind(DEFAULT_NOW)
        .execute(&self.pool)
        .await
        .map_err(store_error("failed to release notary case assignment"))?;

        if result.rows_affected() == 0 {
            return Err(NotaryServiceError::not_found(
                "notary case assignment not found",
            ));
        }
        Ok(())
    }

    pub async fn list_cases(
        &self,
        query: NotaryCaseListQuery,
    ) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError> {
        let search_pattern = query
            .search_term
            .as_ref()
            .map(|value| format!("%{}%", value.trim()));
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                case_no,
                organization_id,
                title,
                remarks,
                status,
                applicant_name_snapshot,
                primary_notary_membership_id,
                primary_notary_user_id,
                primary_notary_name_snapshot,
                order_id,
                order_item_id,
                sku_id,
                matter_title_snapshot,
                fee_amount_snapshot,
                currency_code,
                drive_space_id,
                drive_space_type,
                drive_folder_node_id,
                chain_hash,
                request_no,
                idempotency_key,
                created_at,
                updated_at
            FROM notary_case
            WHERE tenant_id = ?1
              AND organization_id = ?2
              AND (?3 IS NULL OR status = ?3)
              AND (
                ?4 IS NULL
                OR title LIKE ?4
                OR applicant_name_snapshot LIKE ?4
                OR matter_title_snapshot LIKE ?4
                OR case_no LIKE ?4
              )
              AND (?5 IS NULL OR sku_id = ?5)
              AND (?6 IS NULL OR id < ?6)
            ORDER BY updated_at DESC, id DESC
            LIMIT ?7
            "#,
        )
        .bind(&self.tenant_id)
        .bind(&query.organization_id)
        .bind(query.status.as_deref())
        .bind(search_pattern.as_deref())
        .bind(query.sku_id.as_deref())
        .bind(query.cursor.as_deref())
        .bind(query.page_size)
        .fetch_all(&self.pool)
        .await
        .map_err(store_error("failed to list notary cases"))?;

        rows.iter().map(case_from_row).collect()
    }

    pub async fn list_parties(
        &self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                case_id,
                order_id,
                order_item_id,
                sku_id,
                name,
                party_role,
                identity_no_last4,
                phone_masked,
                status,
                signature_node_id
            FROM notary_party
            WHERE tenant_id = ?1
              AND case_id = ?2
              AND status = 'active'
            ORDER BY sort_order ASC, id ASC
            "#,
        )
        .bind(&self.tenant_id)
        .bind(case_id)
        .fetch_all(&self.pool)
        .await
        .map_err(store_error("failed to list notary parties"))?;

        Ok(rows.iter().map(party_from_row).collect())
    }

    pub async fn list_events(
        &self,
        query: NotaryCaseEventListQuery,
    ) -> Result<Vec<NotaryCaseEventRecord>, NotaryServiceError> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                case_id,
                event_type,
                event_title,
                actor_user_id,
                occurred_at
            FROM notary_case_event
            WHERE tenant_id = ?1
              AND case_id = ?2
              AND (?3 IS NULL OR id > ?3)
            ORDER BY occurred_at ASC, id ASC
            LIMIT ?4
            "#,
        )
        .bind(&self.tenant_id)
        .bind(&query.case_id)
        .bind(query.cursor.as_deref())
        .bind(query.page_size)
        .fetch_all(&self.pool)
        .await
        .map_err(store_error("failed to list notary case events"))?;

        Ok(rows.iter().map(event_from_row).collect())
    }
}

#[async_trait]
impl NotaryCaseRepositoryPort for SqliteNotaryCaseRepository {
    async fn upsert_organization_profile(
        &mut self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        SqliteNotaryCaseRepository::upsert_organization_profile(
            self,
            organization_id,
            drive_space_id,
            drive_space_type,
        )
        .await
    }

    async fn get_organization_profile(
        &mut self,
        organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError> {
        SqliteNotaryCaseRepository::get_organization_profile(self, organization_id).await
    }

    async fn list_organization_profiles(
        &mut self,
        organization_id: Option<&str>,
        page_size: i64,
    ) -> Result<Vec<NotaryOrganizationProfile>, NotaryServiceError> {
        SqliteNotaryCaseRepository::list_organization_profiles(self, organization_id, page_size)
            .await
    }

    async fn update_organization_profile(
        &mut self,
        command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        SqliteNotaryCaseRepository::update_organization_profile(self, command).await
    }

    async fn insert_case(
        &mut self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        SqliteNotaryCaseRepository::insert_case(self, record).await
    }

    async fn insert_party(
        &mut self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError> {
        SqliteNotaryCaseRepository::insert_party(
            self,
            case_id,
            party,
            order_id,
            order_item_id,
            sku_id,
        )
        .await
    }

    async fn append_event(
        &mut self,
        case_id: &str,
        event_type: &str,
    ) -> Result<(), NotaryServiceError> {
        SqliteNotaryCaseRepository::append_event(self, case_id, event_type).await
    }

    async fn get_case(
        &mut self,
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        SqliteNotaryCaseRepository::get_case(self, case_id).await
    }

    async fn update_case(
        &mut self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        SqliteNotaryCaseRepository::update_case(self, command).await
    }

    async fn update_party(
        &mut self,
        command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        SqliteNotaryCaseRepository::update_party(self, command).await
    }

    async fn remove_party(
        &mut self,
        case_id: &str,
        party_id: &str,
    ) -> Result<(), NotaryServiceError> {
        SqliteNotaryCaseRepository::remove_party(self, case_id, party_id).await
    }

    async fn insert_assignment(
        &mut self,
        command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
        SqliteNotaryCaseRepository::insert_assignment(self, command).await
    }

    async fn release_assignment(&mut self, assignment_id: &str) -> Result<(), NotaryServiceError> {
        SqliteNotaryCaseRepository::release_assignment(self, assignment_id).await
    }

    async fn list_cases(
        &mut self,
        query: NotaryCaseListQuery,
    ) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError> {
        SqliteNotaryCaseRepository::list_cases(self, query).await
    }

    async fn list_parties(
        &mut self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError> {
        SqliteNotaryCaseRepository::list_parties(self, case_id).await
    }

    async fn list_events(
        &mut self,
        query: NotaryCaseEventListQuery,
    ) -> Result<Vec<NotaryCaseEventRecord>, NotaryServiceError> {
        SqliteNotaryCaseRepository::list_events(self, query).await
    }
}

fn profile_from_row(row: &sqlx::sqlite::SqliteRow) -> NotaryOrganizationProfile {
    NotaryOrganizationProfile {
        organization_id: string_cell(row, "organization_id"),
        drive_space_id: string_cell(row, "drive_space_id"),
        drive_space_type: string_cell(row, "drive_space_type"),
        status: string_cell(row, "status"),
    }
}

fn case_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<NotaryCaseRecord, NotaryServiceError> {
    let status =
        NotaryCaseStatus::from_storage_value(&string_cell(row, "status")).ok_or_else(|| {
            NotaryServiceError::storage("unsupported notary case status in database row")
        })?;

    Ok(NotaryCaseRecord {
        case_id: string_cell(row, "id"),
        case_no: string_cell(row, "case_no"),
        organization_id: string_cell(row, "organization_id"),
        title: string_cell(row, "title"),
        applicant_name: string_cell(row, "applicant_name_snapshot"),
        primary_notary_membership_id: optional_string_cell(row, "primary_notary_membership_id"),
        primary_notary_user_id: optional_string_cell(row, "primary_notary_user_id"),
        primary_notary_name: optional_string_cell(row, "primary_notary_name_snapshot"),
        status,
        order_id: string_cell(row, "order_id"),
        order_item_id: string_cell(row, "order_item_id"),
        sku_id: string_cell(row, "sku_id"),
        matter_title: string_cell(row, "matter_title_snapshot"),
        fee_amount: string_cell(row, "fee_amount_snapshot"),
        currency_code: string_cell(row, "currency_code"),
        drive_space_id: string_cell(row, "drive_space_id"),
        drive_space_type: string_cell(row, "drive_space_type"),
        drive_folder_node_id: string_cell(row, "drive_folder_node_id"),
        chain_hash: optional_string_cell(row, "chain_hash"),
        remarks: optional_string_cell(row, "remarks"),
        request_no: string_cell(row, "request_no"),
        idempotency_key: string_cell(row, "idempotency_key"),
        created_at: string_cell(row, "created_at"),
        updated_at: string_cell(row, "updated_at"),
    })
}

fn party_from_row(row: &sqlx::sqlite::SqliteRow) -> NotaryPartyRecord {
    NotaryPartyRecord {
        party_id: string_cell(row, "id"),
        case_id: string_cell(row, "case_id"),
        order_id: string_cell(row, "order_id"),
        order_item_id: string_cell(row, "order_item_id"),
        sku_id: string_cell(row, "sku_id"),
        name: string_cell(row, "name"),
        party_role: string_cell(row, "party_role"),
        identity_no_last4: string_cell(row, "identity_no_last4"),
        phone_masked: optional_string_cell(row, "phone_masked"),
        status: string_cell(row, "status"),
        signature_node_id: optional_string_cell(row, "signature_node_id"),
    }
}

fn event_from_row(row: &sqlx::sqlite::SqliteRow) -> NotaryCaseEventRecord {
    NotaryCaseEventRecord {
        event_id: string_cell(row, "id"),
        case_id: string_cell(row, "case_id"),
        event_type: string_cell(row, "event_type"),
        event_title: string_cell(row, "event_title"),
        actor_user_id: optional_string_cell(row, "actor_user_id"),
        occurred_at: string_cell(row, "occurred_at"),
    }
}

async fn next_party_sort_order(
    pool: &SqlitePool,
    tenant_id: &str,
    case_id: &str,
) -> Result<i64, NotaryServiceError> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_party WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind(tenant_id)
    .bind(case_id)
    .fetch_one(pool)
    .await
    .map_err(store_error("failed to count notary parties"))?;
    Ok(count + 1)
}

async fn next_event_order(
    pool: &SqlitePool,
    tenant_id: &str,
    case_id: &str,
) -> Result<i64, NotaryServiceError> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(1) FROM notary_case_event WHERE tenant_id = ?1 AND case_id = ?2",
    )
    .bind(tenant_id)
    .bind(case_id)
    .fetch_one(pool)
    .await
    .map_err(store_error("failed to count notary case events"))?;
    Ok(count + 1)
}

fn optional_string_cell(row: &sqlx::sqlite::SqliteRow, column: &str) -> Option<String> {
    row.try_get::<Option<String>, _>(column).ok().flatten()
}

fn string_cell(row: &sqlx::sqlite::SqliteRow, column: &str) -> String {
    optional_string_cell(row, column).unwrap_or_default()
}

fn last4(value: &str) -> String {
    let mut chars = value.chars().rev().take(4).collect::<Vec<_>>();
    chars.reverse();
    chars.into_iter().collect()
}

fn mask_phone(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= 7 {
        return "****".to_string();
    }
    let prefix = trimmed.chars().take(3).collect::<String>();
    let suffix = trimmed
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("{prefix}****{suffix}")
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("notary-fnv64:{hash:016x}")
}

fn event_title(event_type: &str) -> &'static str {
    match event_type {
        "notary.case.submitted" => "Case submitted",
        "notary.case.accepted" => "Case accepted",
        "notary.case.rejected" => "Case rejected",
        "notary.case.completed" => "Case completed",
        "notary.party.video_invite.created" => "Party video verification invite created",
        "notary.party.signature_invite.created" => "Party mobile signature invite created",
        _ => "Notary case event",
    }
}

fn validate_profile_status(status: &str) -> Result<(), NotaryServiceError> {
    if matches!(status, "active" | "suspended" | "closed") {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary organization profile status: {status}"
        )))
    }
}

fn validate_assignment_role(role: &str) -> Result<(), NotaryServiceError> {
    if matches!(
        role,
        "primary_notary" | "assistant" | "reviewer" | "approver"
    ) {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary case assignment role: {role}"
        )))
    }
}

fn store_error(context: &'static str) -> impl FnOnce(sqlx::Error) -> NotaryServiceError {
    move |error| NotaryServiceError::storage(format!("{context}: {error}"))
}

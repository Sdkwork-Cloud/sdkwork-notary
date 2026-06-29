use async_trait::async_trait;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{AppbaseOrganizationMember, AppbasePort};
use sqlx::Row;

pub struct IamSqlxAppbasePort {
    pool: DatabasePool,
    development_mode: bool,
}

impl IamSqlxAppbasePort {
    pub fn new(pool: DatabasePool, development_mode: bool) -> Self {
        Self {
            pool,
            development_mode,
        }
    }
}

#[async_trait]
impl AppbasePort for IamSqlxAppbasePort {
    async fn get_organization_member(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
        match &self.pool {
            DatabasePool::Sqlite(pool, _) => {
                load_member_sqlite(pool, organization_id, membership_id, self.development_mode)
                    .await
            }
            DatabasePool::Postgres(pool, _) => {
                load_member_postgres(pool, organization_id, membership_id, self.development_mode)
                    .await
            }
        }
    }

    async fn list_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
        match &self.pool {
            DatabasePool::Sqlite(pool, _) => {
                list_members_sqlite(pool, organization_id, self.development_mode).await
            }
            DatabasePool::Postgres(pool, _) => {
                list_members_postgres(pool, organization_id, self.development_mode).await
            }
        }
    }
}

async fn load_member_sqlite(
    pool: &sqlx::SqlitePool,
    organization_id: &str,
    membership_id: &str,
    development_mode: bool,
) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
    let row = sqlx::query(
        "SELECT m.id, m.tenant_id, m.organization_id, m.user_id, m.display_name, m.status, \
         COALESCE(o.verification_status, 'verified') AS verification_status \
         FROM iam_organization_membership m \
         LEFT JOIN iam_organization o \
           ON o.tenant_id = m.tenant_id AND o.id = m.organization_id \
         WHERE m.id = ?1 AND m.organization_id = ?2 AND m.status = 'active' \
         LIMIT 1",
    )
    .bind(membership_id)
    .bind(organization_id)
    .fetch_optional(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;

    let Some(row) = row else {
        return Ok(None);
    };
    let tenant_id: String = row.try_get("tenant_id").map_err(storage_error)?;
    let membership_id: String = row.try_get("id").map_err(storage_error)?;
    let user_id: String = row.try_get("user_id").map_err(storage_error)?;
    let organization_id: String = row.try_get("organization_id").map_err(storage_error)?;
    let display_name = row
        .try_get::<Option<String>, _>("display_name")
        .ok()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| user_id.clone());
    let verification_status: String = row.try_get("verification_status").map_err(storage_error)?;
    let roles = load_roles_sqlite(pool, &tenant_id, &membership_id).await?;
    let positions = load_positions_sqlite(pool, &tenant_id, &membership_id).await?;
    let departments = load_departments_sqlite(pool, &tenant_id, &membership_id).await?;
    Ok(Some(build_member(
        membership_id,
        user_id,
        organization_id,
        display_name,
        verification_status,
        roles,
        positions,
        departments,
        development_mode,
    )))
}

async fn list_members_sqlite(
    pool: &sqlx::SqlitePool,
    organization_id: &str,
    development_mode: bool,
) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT m.id, m.tenant_id, m.organization_id, m.user_id, m.display_name, m.status, \
         COALESCE(o.verification_status, 'verified') AS verification_status \
         FROM iam_organization_membership m \
         LEFT JOIN iam_organization o \
           ON o.tenant_id = m.tenant_id AND o.id = m.organization_id \
         WHERE m.organization_id = ?1 AND m.status = 'active' \
         ORDER BY m.is_primary DESC, m.id",
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;

    let mut members = Vec::with_capacity(rows.len());
    for row in rows {
        let tenant_id: String = row.try_get("tenant_id").map_err(storage_error)?;
        let membership_id: String = row.try_get("id").map_err(storage_error)?;
        let user_id: String = row.try_get("user_id").map_err(storage_error)?;
        let organization_id: String = row.try_get("organization_id").map_err(storage_error)?;
        let display_name = row
            .try_get::<Option<String>, _>("display_name")
            .ok()
            .flatten()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| user_id.clone());
        let verification_status: String =
            row.try_get("verification_status").map_err(storage_error)?;
        let roles = load_roles_sqlite(pool, &tenant_id, &membership_id).await?;
        let positions = load_positions_sqlite(pool, &tenant_id, &membership_id).await?;
        let departments = load_departments_sqlite(pool, &tenant_id, &membership_id).await?;
        members.push(build_member(
            membership_id,
            user_id,
            organization_id,
            display_name,
            verification_status,
            roles,
            positions,
            departments,
            development_mode,
        ));
    }
    Ok(members)
}

async fn load_member_postgres(
    pool: &sqlx::PgPool,
    organization_id: &str,
    membership_id: &str,
    development_mode: bool,
) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
    let row = sqlx::query(
        "SELECT m.id, m.tenant_id, m.organization_id, m.user_id, m.display_name, m.status, \
         COALESCE(o.verification_status, 'verified') AS verification_status \
         FROM iam_organization_membership m \
         LEFT JOIN iam_organization o \
           ON o.tenant_id = m.tenant_id AND o.id = m.organization_id \
         WHERE m.id = $1 AND m.organization_id = $2 AND m.status = 'active' \
         LIMIT 1",
    )
    .bind(membership_id)
    .bind(organization_id)
    .fetch_optional(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;

    let Some(row) = row else {
        return Ok(None);
    };
    let tenant_id: String = row.try_get("tenant_id").map_err(storage_error)?;
    let membership_id: String = row.try_get("id").map_err(storage_error)?;
    let user_id: String = row.try_get("user_id").map_err(storage_error)?;
    let organization_id: String = row.try_get("organization_id").map_err(storage_error)?;
    let display_name = row
        .try_get::<Option<String>, _>("display_name")
        .ok()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| user_id.clone());
    let verification_status: String = row.try_get("verification_status").map_err(storage_error)?;
    let roles = load_roles_postgres(pool, &tenant_id, &membership_id).await?;
    let positions = load_positions_postgres(pool, &tenant_id, &membership_id).await?;
    let departments = load_departments_postgres(pool, &tenant_id, &membership_id).await?;
    Ok(Some(build_member(
        membership_id,
        user_id,
        organization_id,
        display_name,
        verification_status,
        roles,
        positions,
        departments,
        development_mode,
    )))
}

async fn list_members_postgres(
    pool: &sqlx::PgPool,
    organization_id: &str,
    development_mode: bool,
) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT m.id, m.tenant_id, m.organization_id, m.user_id, m.display_name, m.status, \
         COALESCE(o.verification_status, 'verified') AS verification_status \
         FROM iam_organization_membership m \
         LEFT JOIN iam_organization o \
           ON o.tenant_id = m.tenant_id AND o.id = m.organization_id \
         WHERE m.organization_id = $1 AND m.status = 'active' \
         ORDER BY m.is_primary DESC, m.id",
    )
    .bind(organization_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;

    let mut members = Vec::with_capacity(rows.len());
    for row in rows {
        let tenant_id: String = row.try_get("tenant_id").map_err(storage_error)?;
        let membership_id: String = row.try_get("id").map_err(storage_error)?;
        let user_id: String = row.try_get("user_id").map_err(storage_error)?;
        let organization_id: String = row.try_get("organization_id").map_err(storage_error)?;
        let display_name = row
            .try_get::<Option<String>, _>("display_name")
            .ok()
            .flatten()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| user_id.clone());
        let verification_status: String =
            row.try_get("verification_status").map_err(storage_error)?;
        let roles = load_roles_postgres(pool, &tenant_id, &membership_id).await?;
        let positions = load_positions_postgres(pool, &tenant_id, &membership_id).await?;
        let departments = load_departments_postgres(pool, &tenant_id, &membership_id).await?;
        members.push(build_member(
            membership_id,
            user_id,
            organization_id,
            display_name,
            verification_status,
            roles,
            positions,
            departments,
            development_mode,
        ));
    }
    Ok(members)
}

fn build_member(
    membership_id: String,
    user_id: String,
    organization_id: String,
    display_name: String,
    verification_status: String,
    roles: Vec<String>,
    positions: Vec<String>,
    departments: Vec<String>,
    development_mode: bool,
) -> AppbaseOrganizationMember {
    let enterprise_verified =
        development_mode || verification_status.eq_ignore_ascii_case("verified");
    let notary_enabled = development_mode
        || enterprise_verified
            && (roles.iter().any(|role| {
                matches!(
                    role.as_str(),
                    "notary" | "notary_admin" | "assistant" | "reviewer" | "approver" | "owner"
                )
            }) || positions.iter().any(|position| {
                position.contains("公证") || position.eq_ignore_ascii_case("notary")
            }));
    AppbaseOrganizationMember {
        membership_id,
        user_id,
        organization_id,
        display_name,
        enterprise_verified,
        notary_enabled,
        roles,
        positions,
        departments,
    }
}

async fn load_roles_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT r.code FROM iam_role_binding b \
         JOIN iam_role r ON r.tenant_id = b.tenant_id AND r.id = b.role_id \
         WHERE b.tenant_id = ?1 AND b.principal_id = ?2 AND b.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("code").ok())
        .collect())
}

async fn load_roles_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT r.code FROM iam_role_binding b \
         JOIN iam_role r ON r.tenant_id = b.tenant_id AND r.id = b.role_id \
         WHERE b.tenant_id = $1 AND b.principal_id = $2 AND b.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("code").ok())
        .collect())
}

async fn load_positions_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT p.name FROM iam_position_assignment pa \
         JOIN iam_position p ON p.tenant_id = pa.tenant_id AND p.id = pa.position_id \
         WHERE pa.tenant_id = ?1 AND pa.organization_membership_id = ?2 AND pa.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect())
}

async fn load_positions_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT p.name FROM iam_position_assignment pa \
         JOIN iam_position p ON p.tenant_id = pa.tenant_id AND p.id = pa.position_id \
         WHERE pa.tenant_id = $1 AND pa.organization_membership_id = $2 AND pa.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect())
}

async fn load_departments_sqlite(
    pool: &sqlx::SqlitePool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT d.name FROM iam_department_assignment da \
         JOIN iam_department d ON d.tenant_id = da.tenant_id AND d.id = da.department_id \
         WHERE da.tenant_id = ?1 AND da.organization_membership_id = ?2 AND da.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect())
}

async fn load_departments_postgres(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    membership_id: &str,
) -> Result<Vec<String>, NotaryServiceError> {
    let rows = sqlx::query(
        "SELECT d.name FROM iam_department_assignment da \
         JOIN iam_department d ON d.tenant_id = da.tenant_id AND d.id = da.department_id \
         WHERE da.tenant_id = $1 AND da.organization_membership_id = $2 AND da.status = 'active'",
    )
    .bind(tenant_id)
    .bind(membership_id)
    .fetch_all(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
    Ok(rows
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect())
}

fn storage_error(error: sqlx::Error) -> NotaryServiceError {
    NotaryServiceError::storage(error.to_string())
}

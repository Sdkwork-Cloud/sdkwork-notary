use std::collections::HashMap;

use async_trait::async_trait;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_drive_workspace_service::application::node_service::{
    CreateNodeCommand, DriveNodeService,
};
use sdkwork_drive_workspace_service::application::space_service::{
    CreateSpaceCommand, SqlDriveSpaceService,
};
use sdkwork_drive_workspace_service::application::workspace_service::{
    DriveWorkspaceNodeKind, ListDriveWorkspaceChildrenCommand, SqlDriveWorkspaceService,
};
use sdkwork_drive_workspace_service::domain::node::DriveNodeType;
use sdkwork_drive_workspace_service::domain::space::DriveSpaceType;
use sdkwork_drive_workspace_service::infrastructure::sql::node_store::SqlNodeStore;
use sdkwork_drive_workspace_service::infrastructure::sql::postgres_pool_from_database_pool;
use sdkwork_drive_workspace_service::DriveServiceError;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    validated_list_page_size, DriveCreateFolderCommand, DriveCreateSpaceCommand,
    DriveFolderReference, DriveListNodesPage, DriveListNodesQuery, DriveNodeReference, DrivePort,
    DriveRegisterCaseFileCommand, NOTARY_FILE_CATEGORY_PROPERTY,
    NOTARY_FILE_MATERIAL_CODE_PROPERTY, NOTARY_FILE_PARTY_ID_PROPERTY,
    NOTARY_FILE_REVIEW_STATUS_PROPERTY,
};
use sdkwork_utils_rust::{base64_decode, base64_encode, format_bytes, sha256_hash};
use sqlx::PgPool;

const NOTARY_FILE_PROPERTY_VISIBILITY: &str = "app_public";
const NOTARY_DRIVE_CURSOR_PREFIX: &str = "ndf1:";

pub struct DriveWorkspacePort {
    tenant_id: String,
    operator_id: String,
    space_service: SqlDriveSpaceService,
    node_service: DriveNodeService<SqlNodeStore>,
    workspace_service: SqlDriveWorkspaceService,
    pool: PgPool,
}

impl DriveWorkspacePort {
    pub async fn new(
        pool: DatabasePool,
        tenant_id: impl Into<String>,
        operator_id: impl Into<String>,
    ) -> Result<Self, NotaryServiceError> {
        let postgres_pool =
            postgres_pool_from_database_pool(&pool).map_err(NotaryServiceError::storage)?;
        Ok(Self::from_postgres_pool(
            postgres_pool,
            tenant_id.into(),
            operator_id.into(),
        ))
    }

    fn from_postgres_pool(pool: PgPool, tenant_id: String, operator_id: String) -> Self {
        Self {
            tenant_id,
            operator_id,
            space_service: SqlDriveSpaceService::new(pool.clone()),
            node_service: DriveNodeService::new(SqlNodeStore::new(pool.clone())),
            workspace_service: SqlDriveWorkspaceService::new(pool.clone()),
            pool,
        }
    }
}

#[async_trait]
impl DrivePort for DriveWorkspacePort {
    async fn create_notary_space(
        &self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError> {
        let space_id = format!("space-notary-{}", slug_segment(&command.owner_subject_id));
        let created = self
            .space_service
            .create_space(CreateSpaceCommand {
                id: space_id.clone(),
                tenant_id: self.tenant_id.clone(),
                owner_subject_type: command.owner_subject_type,
                owner_subject_id: command.owner_subject_id,
                display_name: command.display_name,
                space_type: DriveSpaceType::Notary,
                presentation_icon: None,
                presentation_color: None,
                description: None,
                operator_id: self.operator_id.clone(),
            })
            .await
            .map_err(map_drive_error)?;
        Ok(created.id)
    }

    async fn create_case_folder(
        &self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError> {
        let folder_node_id = format!("folder-{}", slug_segment(&command.order_id));
        let _node = self
            .node_service
            .create_node(CreateNodeCommand {
                id: folder_node_id.clone(),
                tenant_id: self.tenant_id.clone(),
                space_id: command.space_id.clone(),
                parent_node_id: command.parent_node_id,
                node_type: DriveNodeType::Folder,
                node_name: command.folder_name,
                operator_id: self.operator_id.clone(),
            })
            .await
            .map_err(map_drive_error)?;
        Ok(DriveFolderReference {
            folder_node_id,
            space_id: command.space_id,
            space_type: command.space_type,
        })
    }

    async fn list_nodes(
        &self,
        query: DriveListNodesQuery,
    ) -> Result<DriveListNodesPage, NotaryServiceError> {
        let (mut scan_offset, mut skip_files) = parse_list_cursor(query.cursor.as_deref())?;
        let page_size = validated_list_page_size(query.page_size)?;
        let mut items = Vec::new();
        let mut next_cursor = None;
        let mut has_more = false;

        'paginate: loop {
            let page = self
                .workspace_service
                .list_children(ListDriveWorkspaceChildrenCommand {
                    tenant_id: self.tenant_id.clone(),
                    space_id: query.space_id.clone(),
                    parent_node_id: Some(query.parent_node_id.clone()),
                    offset: scan_offset,
                    page_size,
                })
                .await
                .map_err(map_drive_error)?;

            let file_nodes: Vec<_> = page
                .nodes
                .iter()
                .filter(|node| node.kind == DriveWorkspaceNodeKind::File)
                .collect();
            let node_ids: Vec<String> = file_nodes.iter().map(|node| node.id.clone()).collect();
            let metadata = load_file_metadata(&self.pool, &self.tenant_id, &node_ids).await?;
            let mut matched_files_seen = 0usize;

            for node in file_nodes {
                let file_meta = metadata.get(&node.id);
                let category = file_meta
                    .and_then(|meta| meta.category.clone())
                    .unwrap_or_else(|| "evidence".to_string());
                if query
                    .category
                    .as_ref()
                    .is_some_and(|filter| filter != &category)
                {
                    continue;
                }
                if matched_files_seen < skip_files {
                    matched_files_seen += 1;
                    continue;
                }

                let status = file_meta
                    .and_then(|meta| meta.review_status.clone())
                    .unwrap_or_else(|| "pending".to_string());
                let size_label = node
                    .content_length
                    .map(|length| format_bytes(length, 1))
                    .unwrap_or_default();
                items.push(DriveNodeReference {
                    node_id: node.id.clone(),
                    node_name: node.name.clone(),
                    category,
                    size_label,
                    status,
                    material_code: file_meta.and_then(|meta| meta.material_code.clone()),
                    party_id: file_meta.and_then(|meta| meta.party_id.clone()),
                });
                matched_files_seen += 1;

                if items.len() == page_size as usize {
                    let total_matched_on_page = page
                        .nodes
                        .iter()
                        .filter(|candidate| candidate.kind == DriveWorkspaceNodeKind::File)
                        .filter(|candidate| {
                            file_category_matches(
                                &metadata,
                                &candidate.id,
                                query.category.as_deref(),
                            )
                        })
                        .count();
                    (has_more, next_cursor) = resolve_list_continuation(
                        scan_offset,
                        matched_files_seen,
                        total_matched_on_page,
                        page.next_offset,
                    );
                    break 'paginate;
                }
            }

            skip_files = 0;
            match page.next_offset {
                Some(next_offset) => scan_offset = next_offset,
                None => break,
            }
        }

        Ok(DriveListNodesPage {
            items,
            has_more,
            next_cursor,
        })
    }

    async fn register_case_file(
        &self,
        command: DriveRegisterCaseFileCommand,
    ) -> Result<(), NotaryServiceError> {
        let node = self
            .workspace_service
            .get_node(sdkwork_drive_workspace_service::application::workspace_service::GetDriveWorkspaceNodeCommand {
                tenant_id: self.tenant_id.clone(),
                space_id: command.space_id,
                node_id: command.node_id.clone(),
            })
            .await
            .map_err(map_drive_error)?
            .ok_or_else(|| NotaryServiceError::not_found("drive node not found"))?;

        upsert_node_property(
            &self.pool,
            &self.tenant_id,
            &node.id,
            NOTARY_FILE_CATEGORY_PROPERTY,
            &command.category,
            &self.operator_id,
        )
        .await?;
        upsert_node_property(
            &self.pool,
            &self.tenant_id,
            &node.id,
            NOTARY_FILE_REVIEW_STATUS_PROPERTY,
            &command.review_status,
            &self.operator_id,
        )
        .await?;
        if let Some(material_code) = command.material_code.as_deref() {
            upsert_node_property(
                &self.pool,
                &self.tenant_id,
                &node.id,
                NOTARY_FILE_MATERIAL_CODE_PROPERTY,
                material_code,
                &self.operator_id,
            )
            .await?;
        }
        if let Some(party_id) = command.party_id.as_deref() {
            upsert_node_property(
                &self.pool,
                &self.tenant_id,
                &node.id,
                NOTARY_FILE_PARTY_ID_PROPERTY,
                party_id,
                &self.operator_id,
            )
            .await?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct FileNodeMetadata {
    category: Option<String>,
    review_status: Option<String>,
    material_code: Option<String>,
    party_id: Option<String>,
}

async fn load_file_metadata(
    pool: &PgPool,
    tenant_id: &str,
    node_ids: &[String],
) -> Result<HashMap<String, FileNodeMetadata>, NotaryServiceError> {
    let mut metadata_by_node = HashMap::new();
    if node_ids.is_empty() {
        return Ok(metadata_by_node);
    }

    let placeholders = node_ids
        .iter()
        .enumerate()
        .map(|(index, _)| format!("${}", index + 3))
        .collect::<Vec<_>>()
        .join(", ");
    let query = format!(
        "SELECT node_id, property_key, property_value
         FROM dr_drive_node_property
         WHERE tenant_id=$1
           AND visibility=$2
           AND lifecycle_status='active'
           AND property_key IN (
             '{NOTARY_FILE_CATEGORY_PROPERTY}',
             '{NOTARY_FILE_REVIEW_STATUS_PROPERTY}',
             '{NOTARY_FILE_MATERIAL_CODE_PROPERTY}',
             '{NOTARY_FILE_PARTY_ID_PROPERTY}'
           )
           AND node_id IN ({placeholders})"
    );

    let mut sql_query = sqlx::query(sqlx::AssertSqlSafe(query.as_str()))
        .bind(tenant_id)
        .bind(NOTARY_FILE_PROPERTY_VISIBILITY);
    for node_id in node_ids {
        sql_query = sql_query.bind(node_id);
    }

    let rows = sql_query.fetch_all(pool).await.map_err(|error| {
        NotaryServiceError::storage(format!("load file metadata failed: {error}"))
    })?;

    for row in rows {
        use sqlx::Row;
        let node_id: String = row.get("node_id");
        let property_key: String = row.get("property_key");
        let property_value: String = row.get("property_value");
        let entry = metadata_by_node
            .entry(node_id)
            .or_insert_with(FileNodeMetadata::default);
        if property_key == NOTARY_FILE_CATEGORY_PROPERTY {
            entry.category = Some(property_value);
        } else if property_key == NOTARY_FILE_REVIEW_STATUS_PROPERTY {
            entry.review_status = Some(property_value);
        } else if property_key == NOTARY_FILE_MATERIAL_CODE_PROPERTY {
            entry.material_code = Some(property_value);
        } else if property_key == NOTARY_FILE_PARTY_ID_PROPERTY {
            entry.party_id = Some(property_value);
        }
    }
    Ok(metadata_by_node)
}

async fn upsert_node_property(
    pool: &PgPool,
    tenant_id: &str,
    node_id: &str,
    property_key: &str,
    property_value: &str,
    operator_id: &str,
) -> Result<(), NotaryServiceError> {
    let property_id = build_node_property_id(tenant_id, node_id, property_key);
    sqlx::query(
        "INSERT INTO dr_drive_node_property (
            id, tenant_id, node_id, property_key, property_value, visibility,
            lifecycle_status, version, created_by, updated_by, created_at, updated_at
         ) VALUES ($1, $2, $3, $4, $5, $6, 'active', 1, $7, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         ON CONFLICT(tenant_id, node_id, property_key, visibility) DO UPDATE SET
            property_value = excluded.property_value,
            updated_by = excluded.updated_by,
            updated_at = CURRENT_TIMESTAMP,
            version = dr_drive_node_property.version + 1",
    )
    .bind(property_id)
    .bind(tenant_id)
    .bind(node_id)
    .bind(property_key)
    .bind(property_value)
    .bind(NOTARY_FILE_PROPERTY_VISIBILITY)
    .bind(operator_id)
    .execute(pool)
    .await
    .map_err(|error| NotaryServiceError::storage(format!("upsert file metadata failed: {error}")))?;
    Ok(())
}

fn build_node_property_id(tenant_id: &str, node_id: &str, property_key: &str) -> String {
    let digest = sha256_hash(
        format!("{tenant_id}\0{node_id}\0{NOTARY_FILE_PROPERTY_VISIBILITY}\0{property_key}")
            .as_bytes(),
    );
    format!("p:{}", &digest[..62])
}

fn file_category_matches(
    metadata: &HashMap<String, FileNodeMetadata>,
    node_id: &str,
    category_filter: Option<&str>,
) -> bool {
    let Some(filter) = category_filter else {
        return true;
    };
    metadata
        .get(node_id)
        .and_then(|meta| meta.category.as_deref())
        .unwrap_or("evidence")
        == filter
}

fn parse_list_cursor(cursor: Option<&str>) -> Result<(i64, usize), NotaryServiceError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok((0, 0));
    };
    let encoded = cursor
        .strip_prefix(NOTARY_DRIVE_CURSOR_PREFIX)
        .ok_or_else(|| NotaryServiceError::validation("invalid drive file cursor"))?;
    let bytes = base64_decode(encoded)
        .ok_or_else(|| NotaryServiceError::validation("invalid drive file cursor"))?;
    let payload = std::str::from_utf8(&bytes)
        .map_err(|_| NotaryServiceError::validation("invalid drive file cursor"))?;
    let (offset, skip) = payload
        .split_once(':')
        .ok_or_else(|| NotaryServiceError::validation("invalid drive file cursor"))?;
    let offset = offset
        .parse::<i64>()
        .ok()
        .filter(|value| *value >= 0)
        .ok_or_else(|| NotaryServiceError::validation("invalid drive file cursor"))?;
    let skip = skip
        .parse::<usize>()
        .map_err(|_| NotaryServiceError::validation("invalid drive file cursor"))?;
    Ok((offset, skip))
}

fn encode_list_cursor(offset: i64, skip: usize) -> String {
    format!(
        "{NOTARY_DRIVE_CURSOR_PREFIX}{}",
        base64_encode(format!("{offset}:{skip}").as_bytes())
    )
}

fn resolve_list_continuation(
    scan_offset: i64,
    matched_files_seen: usize,
    total_matched_on_page: usize,
    next_offset: Option<i64>,
) -> (bool, Option<String>) {
    if matched_files_seen < total_matched_on_page {
        return (
            true,
            Some(encode_list_cursor(scan_offset, matched_files_seen)),
        );
    }
    next_offset
        .map(|offset| (true, Some(encode_list_cursor(offset, 0))))
        .unwrap_or((false, None))
}

fn slug_segment(value: &str) -> String {
    let digest = sha256_hash(value.as_bytes());
    digest.chars().take(12).collect()
}

fn map_drive_error(error: DriveServiceError) -> NotaryServiceError {
    match error {
        DriveServiceError::Validation(message) => NotaryServiceError::validation(message),
        DriveServiceError::NotFound(message) => NotaryServiceError::not_found(message),
        DriveServiceError::Conflict(message) => NotaryServiceError::conflict(message),
        DriveServiceError::PermissionDenied(message) => NotaryServiceError::unauthorized(message),
        DriveServiceError::Internal(message) => NotaryServiceError::storage(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::any::AnyPoolOptions;

    #[tokio::test]
    async fn notary_file_business_metadata_round_trips_through_drive_properties() {
        sqlx::any::install_default_drivers();
        let pool = AnyPoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("connect sqlite");
        sqlx::query(
            "CREATE TABLE dr_drive_node_property (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                node_id TEXT NOT NULL,
                property_key TEXT NOT NULL,
                property_value TEXT NOT NULL,
                visibility TEXT NOT NULL,
                lifecycle_status TEXT NOT NULL,
                version INTEGER NOT NULL,
                created_by TEXT NOT NULL,
                updated_by TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE (tenant_id, node_id, property_key, visibility)
            )",
        )
        .execute(&pool)
        .await
        .expect("create property table");

        for (key, value) in [
            (NOTARY_FILE_CATEGORY_PROPERTY, "identity"),
            (NOTARY_FILE_REVIEW_STATUS_PROPERTY, "verified"),
            (NOTARY_FILE_MATERIAL_CODE_PROPERTY, "identity_front"),
            (NOTARY_FILE_PARTY_ID_PROPERTY, "party-1"),
        ] {
            upsert_node_property(&pool, "tenant-1", "node-1", key, value, "operator-1")
                .await
                .expect("upsert property");
        }

        let metadata = load_file_metadata(
            &pool,
            "tenant-1",
            &["node-1".to_string(), "node-without-metadata".to_string()],
        )
        .await
        .expect("load metadata");
        let node = metadata.get("node-1").expect("node metadata");
        assert_eq!(node.category.as_deref(), Some("identity"));
        assert_eq!(node.review_status.as_deref(), Some("verified"));
        assert_eq!(node.material_code.as_deref(), Some("identity_front"));
        assert_eq!(node.party_id.as_deref(), Some("party-1"));
        assert!(!metadata.contains_key("node-without-metadata"));
    }

    #[test]
    fn drive_file_cursor_is_opaque_and_rejects_numeric_aliases() {
        let cursor = encode_list_cursor(40, 3);
        assert!(cursor.starts_with(NOTARY_DRIVE_CURSOR_PREFIX));
        assert_eq!(parse_list_cursor(Some(&cursor)).unwrap(), (40, 3));
        assert!(parse_list_cursor(Some("40")).is_err());
        assert!(parse_list_cursor(Some("40:3")).is_err());
    }

    #[test]
    fn drive_file_continuation_does_not_repeat_a_full_final_page() {
        assert_eq!(resolve_list_continuation(0, 2, 2, None), (false, None));

        let (has_more_on_page, cursor_on_page) = resolve_list_continuation(20, 2, 3, None);
        assert!(has_more_on_page);
        assert_eq!(
            parse_list_cursor(cursor_on_page.as_deref()).unwrap(),
            (20, 2)
        );

        let (has_more_next_page, cursor_next_page) = resolve_list_continuation(20, 2, 2, Some(40));
        assert!(has_more_next_page);
        assert_eq!(
            parse_list_cursor(cursor_next_page.as_deref()).unwrap(),
            (40, 0)
        );
    }
}

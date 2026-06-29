use std::collections::HashMap;

use async_trait::async_trait;
use sdkwork_database_sqlx::{create_any_pool_from_config, DatabasePool};
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
use sdkwork_drive_workspace_service::DriveServiceError;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveFolderReference, DriveListNodesPage,
    DriveListNodesQuery, DriveNodeReference, DrivePort, DriveRegisterCaseFileCommand,
    NOTARY_FILE_CATEGORY_PROPERTY, NOTARY_FILE_REVIEW_STATUS_PROPERTY,
};
use sdkwork_utils_rust::{format_bytes, sha256_hash};
use sqlx::AnyPool;

const NOTARY_FILE_PROPERTY_VISIBILITY: &str = "app_public";

pub struct DriveWorkspacePort {
    tenant_id: String,
    operator_id: String,
    space_service: SqlDriveSpaceService,
    node_service: DriveNodeService<SqlNodeStore>,
    workspace_service: SqlDriveWorkspaceService,
    pool: AnyPool,
}

impl DriveWorkspacePort {
    pub async fn new(
        pool: DatabasePool,
        tenant_id: impl Into<String>,
        operator_id: impl Into<String>,
    ) -> Result<Self, NotaryServiceError> {
        let any_pool = create_any_pool_from_config(pool.config().clone())
            .await
            .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
        Ok(Self::from_any_pool(
            any_pool,
            tenant_id.into(),
            operator_id.into(),
        ))
    }

    fn from_any_pool(any_pool: AnyPool, tenant_id: String, operator_id: String) -> Self {
        Self {
            tenant_id,
            operator_id,
            space_service: SqlDriveSpaceService::new(any_pool.clone()),
            node_service: DriveNodeService::new(SqlNodeStore::new(any_pool.clone())),
            workspace_service: SqlDriveWorkspaceService::new(any_pool.clone()),
            pool: any_pool,
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
        let page_size = query.page_size.clamp(1, 500);
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
                });

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
                    has_more =
                        matched_files_seen < total_matched_on_page || page.next_offset.is_some();
                    next_cursor = if matched_files_seen < total_matched_on_page {
                        Some(format!("{scan_offset}:{matched_files_seen}"))
                    } else if let Some(next_offset) = page.next_offset {
                        Some(next_offset.to_string())
                    } else {
                        has_more = false;
                        None
                    };
                    break 'paginate;
                }

                matched_files_seen += 1;
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
        Ok(())
    }
}

#[derive(Default)]
struct FileNodeMetadata {
    category: Option<String>,
    review_status: Option<String>,
}

async fn load_file_metadata(
    pool: &AnyPool,
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
           AND property_key IN ('{NOTARY_FILE_CATEGORY_PROPERTY}', '{NOTARY_FILE_REVIEW_STATUS_PROPERTY}')
           AND node_id IN ({placeholders})"
    );

    let mut sql_query = sqlx::query(&query)
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
        }
    }
    Ok(metadata_by_node)
}

async fn upsert_node_property(
    pool: &AnyPool,
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
    let Some(cursor) = cursor.filter(|value| !value.is_empty()) else {
        return Ok((0, 0));
    };
    if let Some((offset, skip)) = cursor.split_once(':') {
        Ok((
            offset
                .parse::<i64>()
                .map_err(|_| NotaryServiceError::validation("cursor offset must be numeric"))?,
            skip.parse::<usize>()
                .map_err(|_| NotaryServiceError::validation("cursor file skip must be numeric"))?,
        ))
    } else {
        Ok((
            cursor
                .parse::<i64>()
                .map_err(|_| NotaryServiceError::validation("cursor must be numeric"))?,
            0,
        ))
    }
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

use async_trait::async_trait;
use sdkwork_database_sqlx::{create_any_pool_from_config, DatabasePool};
use sdkwork_drive_workspace_service::application::node_service::{CreateNodeCommand, DriveNodeService};
use sdkwork_drive_workspace_service::application::space_service::{
    CreateSpaceCommand, SqlDriveSpaceService,
};
use sdkwork_drive_workspace_service::domain::node::DriveNodeType;
use sdkwork_drive_workspace_service::domain::space::DriveSpaceType;
use sdkwork_drive_workspace_service::infrastructure::sql::node_store::SqlNodeStore;
use sdkwork_drive_workspace_service::DriveServiceError;
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_notary_case_service::{
    DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveFolderReference, DriveListNodesQuery,
    DriveNodeReference, DrivePort,
};
use sdkwork_utils_rust::sha256_hash;
use sqlx::AnyPool;

pub struct DriveWorkspacePort {
    tenant_id: String,
    operator_id: String,
    space_service: SqlDriveSpaceService,
    node_service: DriveNodeService<SqlNodeStore>,
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
            node_service: DriveNodeService::new(SqlNodeStore::new(any_pool)),
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
    ) -> Result<Vec<DriveNodeReference>, NotaryServiceError> {
        let _ = query;
        Ok(Vec::new())
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

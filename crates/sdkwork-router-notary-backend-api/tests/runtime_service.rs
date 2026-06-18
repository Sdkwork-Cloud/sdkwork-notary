use std::collections::BTreeMap;
use std::sync::{Arc, Mutex as StdMutex};

use async_trait::async_trait;
use sdkwork_notary_case_contract::{NotaryCaseRecord, NotaryPartyCommand, NotaryServiceError};
use sdkwork_notary_case_service::{
    AppbaseOrganizationMember, AppbasePort, CommerceCreateOrderCommand, CommerceOrderReference,
    CommercePort, DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveFolderReference,
    DriveListNodesQuery, DriveNodeReference, DrivePort, NotaryCaseEventListPage,
    NotaryCaseEventListQuery, NotaryCaseListPage, NotaryCaseListQuery, NotaryCaseRepositoryPort,
    NotaryCaseUpdateCommand, NotaryOrganizationProfile, NotaryPartyRecord,
};
use sdkwork_router_notary_backend_api::{
    NotaryBackendApiServicePort, NotaryBackendRuntimeService, NotaryRequestContext,
};
use serde_json::json;

#[tokio::test]
async fn backend_runtime_service_dispatches_route_operations_to_notary_runtime() {
    let service = NotaryBackendRuntimeService::new(
        RecordingAppbase::with_admin_member(),
        RecordingCommerce::default(),
        RecordingDrive::default(),
        RecordingNotaryRepository::default(),
    );

    let opened = service
        .handle(
            request_context(),
            "notary.organizationProfiles.create",
            BTreeMap::new(),
            json!({
                "organizationId": "org-1",
                "openedByMembershipId": "member-admin-1"
            }),
        )
        .await
        .unwrap();

    assert_eq!(opened["organizationId"], "org-1");
    assert_eq!(opened["driveSpaceType"], "notary");
    assert_eq!(opened["driveSpaceId"], "space-notary-org-1");
}

fn request_context() -> NotaryRequestContext {
    NotaryRequestContext {
        tenant_id: "tenant-1".to_string(),
        organization_id: Some("org-1".to_string()),
        user_id: "user-1".to_string(),
        membership_id: Some("member-admin-1".to_string()),
        session_id: "session-1".to_string(),
        app_id: "sdkwork-admin".to_string(),
    }
}

#[derive(Default)]
struct RecordingAppbase {
    members: Vec<AppbaseOrganizationMember>,
}

impl RecordingAppbase {
    fn with_admin_member() -> Self {
        Self {
            members: vec![AppbaseOrganizationMember {
                membership_id: "member-admin-1".to_string(),
                user_id: "user-1".to_string(),
                organization_id: "org-1".to_string(),
                display_name: "Admin".to_string(),
                enterprise_verified: true,
                notary_enabled: true,
                roles: vec!["notary_admin".to_string()],
                positions: vec!["notary director".to_string()],
                departments: vec!["notary-office".to_string()],
            }],
        }
    }
}

#[async_trait]
impl AppbasePort for RecordingAppbase {
    async fn get_organization_member(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(self
            .members
            .iter()
            .find(|member| {
                member.organization_id == organization_id && member.membership_id == membership_id
            })
            .cloned())
    }

    async fn list_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(self
            .members
            .iter()
            .filter(|member| member.organization_id == organization_id)
            .cloned()
            .collect())
    }
}

#[derive(Default)]
struct RecordingCommerce;

#[async_trait]
impl CommercePort for RecordingCommerce {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        Ok(CommerceOrderReference {
            order_id: format!("order-{}", command.sku_id),
            order_item_id: format!("item-{}", command.sku_id),
            sku_id: command.sku_id,
            matter_title: command.title,
            fee_amount: "500.00".to_string(),
            currency_code: "CNY".to_string(),
        })
    }
}

#[derive(Default)]
struct RecordingDrive;

#[async_trait]
impl DrivePort for RecordingDrive {
    async fn create_notary_space(
        &self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError> {
        Ok(format!("space-notary-{}", command.owner_subject_id))
    }

    async fn create_case_folder(
        &self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError> {
        Ok(DriveFolderReference {
            folder_node_id: format!("folder-{}", command.order_id),
            space_id: command.space_id,
            space_type: command.space_type,
        })
    }

    async fn list_nodes(
        &self,
        _query: DriveListNodesQuery,
    ) -> Result<Vec<DriveNodeReference>, NotaryServiceError> {
        Ok(Vec::new())
    }
}

#[derive(Clone, Default)]
struct RecordingNotaryRepository {
    inner: Arc<StdMutex<RecordingNotaryRepositoryState>>,
}

#[derive(Default)]
struct RecordingNotaryRepositoryState {
    profile: Option<NotaryOrganizationProfile>,
}

#[async_trait]
impl NotaryCaseRepositoryPort for RecordingNotaryRepository {
    async fn upsert_organization_profile(
        &self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        let profile = NotaryOrganizationProfile {
            organization_id: organization_id.to_string(),
            drive_space_id: drive_space_id.to_string(),
            drive_space_type: drive_space_type.to_string(),
            status: "active".to_string(),
        };
        self.inner.lock().unwrap().profile = Some(profile.clone());
        Ok(profile)
    }

    async fn get_organization_profile(
        &self,
        _organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError> {
        Ok(self.inner.lock().unwrap().profile.clone())
    }

    async fn insert_case(
        &self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        Ok(record)
    }

    async fn delete_case(&self, _case_id: &str) -> Result<(), NotaryServiceError> {
        Ok(())
    }

    async fn insert_party(
        &self,
        _case_id: &str,
        _party: &NotaryPartyCommand,
        _order_id: &str,
        _order_item_id: &str,
        _sku_id: &str,
    ) -> Result<(), NotaryServiceError> {
        Ok(())
    }

    async fn append_event(
        &self,
        _case_id: &str,
        _event_type: &str,
    ) -> Result<(), NotaryServiceError> {
        Ok(())
    }

    async fn get_case(
        &self,
        _case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(None)
    }

    async fn get_case_by_idempotency_key(
        &self,
        _idempotency_key: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(None)
    }

    async fn update_case(
        &self,
        _command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        Err(NotaryServiceError::not_found("notary case not found"))
    }

    async fn list_cases(
        &self,
        _query: NotaryCaseListQuery,
    ) -> Result<NotaryCaseListPage, NotaryServiceError> {
        Ok(NotaryCaseListPage {
            items: Vec::new(),
            has_more: false,
        })
    }

    async fn list_parties(
        &self,
        _case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError> {
        Ok(Vec::new())
    }

    async fn list_events(
        &self,
        _query: NotaryCaseEventListQuery,
    ) -> Result<NotaryCaseEventListPage, NotaryServiceError> {
        Ok(NotaryCaseEventListPage {
            items: Vec::new(),
            has_more: false,
        })
    }
}

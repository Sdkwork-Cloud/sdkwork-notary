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
use sdkwork_router_notary_app_api::{
    NotaryAppApiServicePort, NotaryAppRuntimeService, NotaryRequestContext,
};
use serde_json::json;

#[tokio::test]
async fn app_runtime_service_dispatches_route_operations_to_notary_runtime() {
    let service = NotaryAppRuntimeService::new(
        RecordingAppbase::with_notary_member(),
        RecordingCommerce::default(),
        RecordingDrive::default(),
        RecordingNotaryRepository::with_profile(),
    );

    let created = service
        .handle(
            request_context(),
            "notary.cases.create",
            BTreeMap::new(),
            json!({
                "organizationId": "org-1",
                "skuId": "sku-electronic-contract",
                "title": "Electronic contract preservation",
                "applicantName": "Zhang San Network",
                "primaryNotaryMembershipId": "member-notary-1",
                "idempotencyKey": "idem-route-service-1"
            }),
        )
        .await
        .unwrap();

    assert_eq!(created["orderId"], "order-sku-electronic-contract");
    assert_eq!(created["orderItemId"], "item-sku-electronic-contract");
    assert_eq!(created["driveSpaceType"], "notary");
    assert_eq!(
        created["driveFolderNodeId"],
        "folder-order-sku-electronic-contract"
    );
}

fn request_context() -> NotaryRequestContext {
    NotaryRequestContext {
        tenant_id: "tenant-1".to_string(),
        organization_id: Some("org-1".to_string()),
        user_id: "user-1".to_string(),
        membership_id: Some("member-notary-1".to_string()),
        session_id: "session-1".to_string(),
        app_id: "sdkwork-im-pc".to_string(),
    }
}

#[derive(Default)]
struct RecordingAppbase {
    member: Option<AppbaseOrganizationMember>,
}

impl RecordingAppbase {
    fn with_notary_member() -> Self {
        Self {
            member: Some(AppbaseOrganizationMember {
                membership_id: "member-notary-1".to_string(),
                user_id: "user-1".to_string(),
                organization_id: "org-1".to_string(),
                display_name: "李明".to_string(),
                enterprise_verified: true,
                notary_enabled: true,
                roles: vec!["notary".to_string()],
                positions: vec!["notary".to_string()],
                departments: vec!["notary-office".to_string()],
            }),
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
            .member
            .as_ref()
            .filter(|member| {
                member.organization_id == organization_id && member.membership_id == membership_id
            })
            .cloned())
    }

    async fn list_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(self
            .member
            .as_ref()
            .filter(|member| member.organization_id == organization_id)
            .cloned()
            .into_iter()
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
    cases: Vec<NotaryCaseRecord>,
}

impl RecordingNotaryRepository {
    fn with_profile() -> Self {
        Self {
            inner: Arc::new(StdMutex::new(RecordingNotaryRepositoryState {
                profile: Some(NotaryOrganizationProfile {
                    organization_id: "org-1".to_string(),
                    drive_space_id: "space-notary-org-1".to_string(),
                    drive_space_type: "notary".to_string(),
                    status: "active".to_string(),
                }),
                cases: Vec::new(),
            })),
        }
    }
}

#[async_trait]
impl NotaryCaseRepositoryPort for RecordingNotaryRepository {
    async fn upsert_organization_profile(
        &self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        let mut state = self.inner.lock().unwrap();
        let profile = NotaryOrganizationProfile {
            organization_id: organization_id.to_string(),
            drive_space_id: drive_space_id.to_string(),
            drive_space_type: drive_space_type.to_string(),
            status: "active".to_string(),
        };
        state.profile = Some(profile.clone());
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
        let mut state = self.inner.lock().unwrap();
        state.cases.push(record.clone());
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
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(self
            .inner
            .lock()
            .unwrap()
            .cases
            .iter()
            .find(|record| record.case_id == case_id)
            .cloned())
    }

    async fn get_case_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(self
            .inner
            .lock()
            .unwrap()
            .cases
            .iter()
            .find(|record| record.idempotency_key == idempotency_key)
            .cloned())
    }

    async fn update_case(
        &self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        let mut state = self.inner.lock().unwrap();
        let record = state
            .cases
            .iter_mut()
            .find(|record| record.case_id == command.case_id)
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

        Ok(record.clone())
    }

    async fn list_cases(
        &self,
        _query: NotaryCaseListQuery,
    ) -> Result<NotaryCaseListPage, NotaryServiceError> {
        Ok(NotaryCaseListPage {
            items: self.inner.lock().unwrap().cases.clone(),
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

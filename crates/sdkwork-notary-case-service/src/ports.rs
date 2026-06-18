use async_trait::async_trait;
use sdkwork_notary_core::{
    NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand, NotaryServiceError,
};
use serde_json::Value;

pub const NOTARY_IAM_PORT: &str = "appbase.iam.organization_member";
pub const NOTARY_COMMERCE_PORT: &str = "commerce.order";
pub const NOTARY_DRIVE_PORT: &str = "drive.notary_space";
pub const NOTARY_CASE_REPOSITORY_PORT: &str = "notary.case.repository";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppbaseOrganizationMember {
    pub membership_id: String,
    pub user_id: String,
    pub organization_id: String,
    pub enterprise_verified: bool,
    pub notary_enabled: bool,
    pub roles: Vec<String>,
    pub positions: Vec<String>,
    pub departments: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceCreateOrderCommand {
    pub organization_id: String,
    pub sku_id: String,
    pub title: String,
    pub applicant_name: String,
    pub product_type: String,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceOrderReference {
    pub order_id: String,
    pub order_item_id: String,
    pub sku_id: String,
    pub matter_title: String,
    pub fee_amount: String,
    pub currency_code: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceMatterListQuery {
    pub organization_id: Option<String>,
    pub search_term: Option<String>,
    pub status: Option<String>,
    pub page_size: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommerceMatterCommand {
    pub organization_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub price_amount: String,
    pub original_price_amount: Option<String>,
    pub currency_code: String,
    pub status: String,
    pub spec: Value,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommerceMatterUpdateCommand {
    pub sku_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price_amount: Option<String>,
    pub original_price_amount: Option<String>,
    pub currency_code: Option<String>,
    pub status: Option<String>,
    pub spec: Option<Value>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommerceMatterRecord {
    pub sku_id: String,
    pub spu_id: String,
    pub sku_no: String,
    pub title: String,
    pub description: Option<String>,
    pub price_amount: String,
    pub original_price_amount: Option<String>,
    pub currency_code: String,
    pub status: String,
    pub spec: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreateSpaceCommand {
    pub owner_subject_type: String,
    pub owner_subject_id: String,
    pub space_type: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreateFolderCommand {
    pub space_id: String,
    pub space_type: String,
    pub parent_node_id: Option<String>,
    pub folder_name: String,
    pub order_id: String,
    pub case_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveFolderReference {
    pub folder_node_id: String,
    pub space_id: String,
    pub space_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveListNodesQuery {
    pub space_id: String,
    pub space_type: String,
    pub parent_node_id: String,
    pub category: Option<String>,
    pub page_size: i64,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveNodeReference {
    pub node_id: String,
    pub node_name: String,
    pub category: String,
    pub size_label: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryOrganizationProfile {
    pub organization_id: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NotaryOrganizationProfileUpdateCommand {
    pub organization_id: String,
    pub status: Option<String>,
    pub settings: Option<Value>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseListQuery {
    pub organization_id: String,
    pub status: Option<String>,
    pub sku_id: Option<String>,
    pub search_term: Option<String>,
    pub page_size: i64,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseEventListQuery {
    pub case_id: String,
    pub page_size: i64,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseUpdateCommand {
    pub case_id: String,
    pub title: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<NotaryCaseStatus>,
    pub chain_hash: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryPartyUpdateCommand {
    pub case_id: String,
    pub party_id: String,
    pub name: Option<String>,
    pub party_role: Option<String>,
    pub identity_no: Option<String>,
    pub phone: Option<String>,
    pub signature_node_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryPartyRecord {
    pub party_id: String,
    pub case_id: String,
    pub order_id: String,
    pub order_item_id: String,
    pub sku_id: String,
    pub name: String,
    pub party_role: String,
    pub identity_no_last4: String,
    pub phone_masked: Option<String>,
    pub status: String,
    pub signature_node_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseEventRecord {
    pub event_id: String,
    pub case_id: String,
    pub event_type: String,
    pub event_title: String,
    pub actor_user_id: Option<String>,
    pub occurred_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseAssignmentCommand {
    pub case_id: String,
    pub organization_id: String,
    pub organization_membership_id: String,
    pub user_id: String,
    pub assignment_role: String,
    pub assigned_by_membership_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseAssignmentRecord {
    pub assignment_id: String,
    pub case_id: String,
    pub organization_membership_id: String,
    pub user_id: String,
    pub assignment_role: String,
    pub status: String,
    pub assigned_at: String,
}

#[async_trait]
pub trait AppbasePort: Send {
    async fn get_organization_member(
        &mut self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError>;

    async fn list_organization_members(
        &mut self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError>;
}

#[async_trait]
pub trait CommercePort: Send {
    async fn create_notary_order(
        &mut self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError>;

    async fn list_notary_matters(
        &mut self,
        _query: CommerceMatterListQuery,
    ) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter listing is not configured",
        ))
    }

    async fn create_notary_matter(
        &mut self,
        _command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter creation is not configured",
        ))
    }

    async fn update_notary_matter(
        &mut self,
        _command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter update is not configured",
        ))
    }
}

#[async_trait]
pub trait DrivePort: Send {
    async fn create_notary_space(
        &mut self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError>;

    async fn create_case_folder(
        &mut self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError>;

    async fn list_nodes(
        &mut self,
        query: DriveListNodesQuery,
    ) -> Result<Vec<DriveNodeReference>, NotaryServiceError>;
}

#[async_trait]
pub trait NotaryCaseRepositoryPort: Send {
    async fn upsert_organization_profile(
        &mut self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError>;

    async fn get_organization_profile(
        &mut self,
        organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError>;

    async fn list_organization_profiles(
        &mut self,
        _organization_id: Option<&str>,
        _page_size: i64,
    ) -> Result<Vec<NotaryOrganizationProfile>, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary organization profile listing is not configured",
        ))
    }

    async fn update_organization_profile(
        &mut self,
        _command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary organization profile update is not configured",
        ))
    }

    async fn insert_case(
        &mut self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError>;

    async fn insert_party(
        &mut self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError>;

    async fn append_event(
        &mut self,
        case_id: &str,
        event_type: &str,
    ) -> Result<(), NotaryServiceError>;

    async fn get_case(
        &mut self,
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError>;

    async fn update_case(
        &mut self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError>;

    async fn update_party(
        &mut self,
        _command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary party update is not configured",
        ))
    }

    async fn remove_party(
        &mut self,
        _case_id: &str,
        _party_id: &str,
    ) -> Result<(), NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary party removal is not configured",
        ))
    }

    async fn insert_assignment(
        &mut self,
        _command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary case assignment is not configured",
        ))
    }

    async fn release_assignment(&mut self, _assignment_id: &str) -> Result<(), NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary case assignment release is not configured",
        ))
    }

    async fn list_cases(
        &mut self,
        query: NotaryCaseListQuery,
    ) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError>;

    async fn list_parties(
        &mut self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError>;

    async fn list_events(
        &mut self,
        query: NotaryCaseEventListQuery,
    ) -> Result<Vec<NotaryCaseEventRecord>, NotaryServiceError>;
}

pub struct NotaryRuntimePorts<'a> {
    pub appbase: &'a mut dyn AppbasePort,
    pub commerce: &'a mut dyn CommercePort,
    pub drive: &'a mut dyn DrivePort,
    pub repository: &'a mut dyn NotaryCaseRepositoryPort,
}

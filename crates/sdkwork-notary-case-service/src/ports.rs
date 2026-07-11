use async_trait::async_trait;
use sdkwork_notary_case_contract::{
    NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand, NotaryServiceError,
};
use serde_json::Value;

pub const NOTARY_IAM_PORT: &str = "appbase.iam.organization_member";
pub const NOTARY_COMMERCE_PORT: &str = "commerce.order";
pub const NOTARY_DRIVE_PORT: &str = "drive.notary_space";
pub const NOTARY_CASE_REPOSITORY_PORT: &str = "notary.case.repository";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NotaryOperationMetadata {
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppbaseOrganizationMember {
    pub membership_id: String,
    pub user_id: String,
    pub organization_id: String,
    pub display_name: String,
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
pub struct CommerceOrderFulfillmentState {
    pub order_id: String,
    pub order_status: String,
    pub payment_status: Option<String>,
    pub payable_amount: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceMatterListQuery {
    pub organization_id: Option<String>,
    pub search_term: Option<String>,
    pub status: Option<String>,
    pub page_size: i64,
    pub offset: i64,
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
    pub organization_id: String,
    pub sku_id: String,
    pub title: Option<String>,
    /// PATCH tri-state: `None` preserves the value, `Some(Some(_))` replaces it,
    /// and `Some(None)` clears it.
    pub description: Option<Option<String>>,
    pub price_amount: Option<String>,
    /// PATCH tri-state: `None` preserves the value, `Some(Some(_))` replaces it,
    /// and `Some(None)` clears it.
    pub original_price_amount: Option<Option<String>>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct CommerceMatterListPage {
    pub items: Vec<CommerceMatterRecord>,
    pub has_more: bool,
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
    pub material_code: Option<String>,
    pub party_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveListNodesPage {
    pub items: Vec<DriveNodeReference>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

pub const NOTARY_FILE_CATEGORY_PROPERTY: &str = "notary.category";
pub const NOTARY_FILE_REVIEW_STATUS_PROPERTY: &str = "notary.review_status";
pub const NOTARY_FILE_MATERIAL_CODE_PROPERTY: &str = "notary.material_code";
pub const NOTARY_FILE_PARTY_ID_PROPERTY: &str = "notary.party_id";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveRegisterCaseFileCommand {
    pub space_id: String,
    pub node_id: String,
    pub category: String,
    pub review_status: String,
    pub material_code: Option<String>,
    pub party_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreateDownloadPackageCommand {
    pub space_id: String,
    pub space_type: String,
    pub case_id: String,
    pub node_ids: Vec<String>,
    pub package_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveDownloadPackageReference {
    pub package_id: String,
    pub case_id: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub status: String,
    pub package_name: String,
    pub download_url: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreatePartyVideoInviteCommand {
    pub case_id: String,
    pub party_id: String,
    pub party_name: String,
    pub purpose: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub drive_folder_node_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrivePartyVideoInviteReference {
    pub invite_id: String,
    pub case_id: String,
    pub party_id: String,
    pub party_name: String,
    pub purpose: String,
    pub conversation_id: String,
    pub invite_url: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub drive_folder_node_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreatePartySignatureInviteCommand {
    pub case_id: String,
    pub party_id: String,
    pub party_name: String,
    pub purpose: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub drive_folder_node_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrivePartySignatureInviteReference {
    pub invite_id: String,
    pub case_id: String,
    pub party_id: String,
    pub party_name: String,
    pub purpose: String,
    pub invite_url: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub drive_folder_node_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveCreateMonthlyReportCommand {
    pub month: String,
    pub format: String,
    pub case_count: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveMonthlyReportReference {
    pub report_id: String,
    pub month: String,
    pub format: String,
    pub download_url: String,
    pub file_size: i64,
    pub case_count: i64,
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
pub struct NotaryCaseListPage {
    pub items: Vec<NotaryCaseRecord>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
    pub total_items: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseEventListPage {
    pub items: Vec<NotaryCaseEventRecord>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryPartyListPage {
    pub items: Vec<NotaryPartyRecord>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryPartyListQuery {
    pub case_id: String,
    pub page_size: i64,
    pub cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryOrganizationProfileListPage {
    pub items: Vec<NotaryOrganizationProfile>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryStaffListQuery {
    pub organization_id: String,
    pub staff_role: Option<String>,
    pub page_size: i64,
    pub offset: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryStaffListPage {
    pub items: Vec<AppbaseOrganizationMember>,
    pub has_more: bool,
    pub next_offset: i64,
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
pub struct NotaryDashboardStatisticsQuery {
    pub organization_id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NotaryDashboardStatisticsAggregate {
    pub pending_review_count: i64,
    pub today_completed_count: i64,
    pub yesterday_completed_count: i64,
    pub monthly_case_count: i64,
    pub anomaly_intercepted_count: i64,
    pub unsynced_completed_count: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryMonthlyCaseCountQuery {
    organization_id: String,
    month: String,
}

impl NotaryMonthlyCaseCountQuery {
    pub fn new(
        organization_id: impl Into<String>,
        month: impl Into<String>,
    ) -> Result<Self, NotaryServiceError> {
        let organization_id = organization_id.into();
        let month = month.into();
        let bytes = month.as_bytes();
        let valid_shape = bytes.len() == 7
            && bytes[4] == b'-'
            && bytes[..4].iter().all(u8::is_ascii_digit)
            && bytes[5..].iter().all(u8::is_ascii_digit);
        let valid_calendar_month = valid_shape
            && month[..4].parse::<u16>().is_ok_and(|year| year > 0)
            && month[5..]
                .parse::<u8>()
                .is_ok_and(|month| (1..=12).contains(&month));
        if !valid_calendar_month {
            return Err(NotaryServiceError::validation(
                "month must use YYYY-MM with a calendar month from 01 to 12",
            ));
        }
        Ok(Self {
            organization_id,
            month,
        })
    }

    pub fn organization_id(&self) -> &str {
        &self.organization_id
    }

    pub fn month(&self) -> &str {
        &self.month
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NotaryMonthlyCaseCount {
    pub count: i64,
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
    pub expected_version: i64,
    pub title: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<NotaryCaseStatus>,
    pub chain_hash: Option<String>,
    pub reject_reason: Option<String>,
    pub event_type: String,
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
pub trait AppbasePort: Send + Sync {
    async fn get_organization_member(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError>;

    async fn list_organization_members(
        &self,
        organization_id: &str,
    ) -> Result<Vec<AppbaseOrganizationMember>, NotaryServiceError>;

    async fn list_notary_staff_page(
        &self,
        _query: NotaryStaffListQuery,
    ) -> Result<NotaryStaffListPage, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary staff listing is not configured",
        ))
    }
}

#[async_trait]
pub trait CommercePort: Send + Sync {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError>;

    async fn cancel_notary_order(
        &self,
        organization_id: &str,
        order_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let _ = (organization_id, order_id);
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary order cancellation is not configured",
        ))
    }

    async fn get_notary_order_fulfillment_state(
        &self,
        organization_id: &str,
        order_id: &str,
    ) -> Result<CommerceOrderFulfillmentState, NotaryServiceError> {
        let _ = (organization_id, order_id);
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary order fulfillment state is not configured",
        ))
    }

    async fn list_notary_matters(
        &self,
        _query: CommerceMatterListQuery,
    ) -> Result<CommerceMatterListPage, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter listing is not configured",
        ))
    }

    async fn create_notary_matter(
        &self,
        _command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter creation is not configured",
        ))
    }

    async fn update_notary_matter(
        &self,
        _command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "commerce notary matter update is not configured",
        ))
    }
}

#[async_trait]
pub trait DrivePort: Send + Sync {
    async fn create_notary_space(
        &self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError>;

    async fn create_case_folder(
        &self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError>;

    async fn delete_case_folder(
        &self,
        folder_node_id: &str,
        space_id: &str,
        space_type: &str,
    ) -> Result<(), NotaryServiceError> {
        let _ = (folder_node_id, space_id, space_type);
        Err(NotaryServiceError::provider_unavailable(
            "drive notary case folder deletion is not configured",
        ))
    }

    async fn list_nodes(
        &self,
        query: DriveListNodesQuery,
    ) -> Result<DriveListNodesPage, NotaryServiceError>;

    async fn register_case_file(
        &self,
        command: DriveRegisterCaseFileCommand,
    ) -> Result<(), NotaryServiceError> {
        let _ = command;
        Err(NotaryServiceError::provider_unavailable(
            "drive notary case file registration is not configured",
        ))
    }

    async fn create_download_package(
        &self,
        command: DriveCreateDownloadPackageCommand,
    ) -> Result<DriveDownloadPackageReference, NotaryServiceError> {
        let _ = command;
        Err(NotaryServiceError::provider_unavailable(
            "drive notary download package creation is not configured",
        ))
    }

    async fn create_party_video_invite(
        &self,
        command: DriveCreatePartyVideoInviteCommand,
    ) -> Result<DrivePartyVideoInviteReference, NotaryServiceError> {
        let _ = command;
        Err(NotaryServiceError::provider_unavailable(
            "drive notary party video invite creation is not configured",
        ))
    }

    async fn create_party_signature_invite(
        &self,
        command: DriveCreatePartySignatureInviteCommand,
    ) -> Result<DrivePartySignatureInviteReference, NotaryServiceError> {
        let _ = command;
        Err(NotaryServiceError::provider_unavailable(
            "drive notary party signature invite creation is not configured",
        ))
    }

    async fn create_monthly_report(
        &self,
        command: DriveCreateMonthlyReportCommand,
    ) -> Result<DriveMonthlyReportReference, NotaryServiceError> {
        let _ = command;
        Err(NotaryServiceError::provider_unavailable(
            "drive notary monthly report generation is not configured",
        ))
    }
}

#[async_trait]
pub trait NotaryCaseRepositoryPort: Send + Sync {
    async fn upsert_organization_profile(
        &self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError>;

    async fn get_organization_profile(
        &self,
        organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError>;

    async fn list_organization_profiles(
        &self,
        _organization_id: Option<&str>,
        _page_size: i64,
        _cursor: Option<&str>,
    ) -> Result<NotaryOrganizationProfileListPage, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary organization profile listing is not configured",
        ))
    }

    async fn update_organization_profile(
        &self,
        _command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary organization profile update is not configured",
        ))
    }

    async fn insert_case(
        &self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError>;

    async fn delete_case(&self, case_id: &str) -> Result<(), NotaryServiceError>;

    async fn insert_party(
        &self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError>;

    async fn append_event(&self, case_id: &str, event_type: &str)
        -> Result<(), NotaryServiceError>;

    async fn get_case(&self, case_id: &str)
        -> Result<Option<NotaryCaseRecord>, NotaryServiceError>;

    async fn get_case_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError>;

    async fn update_case(
        &self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError>;

    async fn update_party(
        &self,
        _command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary party update is not configured",
        ))
    }

    async fn remove_party(
        &self,
        _case_id: &str,
        _party_id: &str,
    ) -> Result<(), NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary party removal is not configured",
        ))
    }

    async fn insert_assignment(
        &self,
        _command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary case assignment is not configured",
        ))
    }

    async fn release_assignment(
        &self,
        _case_id: &str,
        _assignment_id: &str,
    ) -> Result<(), NotaryServiceError> {
        Err(NotaryServiceError::provider_unavailable(
            "notary case assignment release is not configured",
        ))
    }

    async fn list_cases(
        &self,
        query: NotaryCaseListQuery,
    ) -> Result<NotaryCaseListPage, NotaryServiceError>;

    async fn get_dashboard_statistics(
        &self,
        query: NotaryDashboardStatisticsQuery,
    ) -> Result<NotaryDashboardStatisticsAggregate, NotaryServiceError>;

    async fn count_cases_for_month(
        &self,
        query: NotaryMonthlyCaseCountQuery,
    ) -> Result<NotaryMonthlyCaseCount, NotaryServiceError>;

    async fn list_parties(
        &self,
        query: NotaryPartyListQuery,
    ) -> Result<NotaryPartyListPage, NotaryServiceError>;

    async fn list_events(
        &self,
        query: NotaryCaseEventListQuery,
    ) -> Result<NotaryCaseEventListPage, NotaryServiceError>;
}

pub struct NotaryRuntimePorts<'a> {
    pub appbase: &'a dyn AppbasePort,
    pub commerce: &'a dyn CommercePort,
    pub drive: &'a dyn DrivePort,
    pub repository: &'a dyn NotaryCaseRepositoryPort,
}

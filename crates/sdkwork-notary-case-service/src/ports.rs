use async_trait::async_trait;
use sdkwork_notary_case_contract::{
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
pub struct DriveListNodesPage {
    pub items: Vec<DriveNodeReference>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

pub const NOTARY_FILE_CATEGORY_PROPERTY: &str = "notary.category";
pub const NOTARY_FILE_REVIEW_STATUS_PROPERTY: &str = "notary.review_status";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveRegisterCaseFileCommand {
    pub space_id: String,
    pub node_id: String,
    pub category: String,
    pub review_status: String,
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseEventListPage {
    pub items: Vec<NotaryCaseEventRecord>,
    pub has_more: bool,
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
}

#[async_trait]
pub trait CommercePort: Send + Sync {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError>;

    async fn cancel_notary_order(&self, order_id: &str) -> Result<(), NotaryServiceError> {
        let _ = order_id;
        Ok(())
    }

    async fn list_notary_matters(
        &self,
        _query: CommerceMatterListQuery,
    ) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
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
        Ok(())
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
        Ok(())
    }

    async fn create_download_package(
        &self,
        command: DriveCreateDownloadPackageCommand,
    ) -> Result<DriveDownloadPackageReference, NotaryServiceError> {
        Ok(DriveDownloadPackageReference {
            package_id: format!("download-package-{}", command.case_id),
            case_id: command.case_id,
            drive_space_id: command.space_id,
            drive_space_type: command.space_type,
            status: "preparing".to_string(),
            package_name: command.package_name,
            download_url: None,
        })
    }

    async fn create_party_video_invite(
        &self,
        command: DriveCreatePartyVideoInviteCommand,
    ) -> Result<DrivePartyVideoInviteReference, NotaryServiceError> {
        let conversation_id = format!(
            "notary-{}-{}-video",
            port_slug_segment(&command.case_id),
            port_slug_segment(&command.party_id)
        );
        let invite_id = format!(
            "video-invite-{}-{}",
            port_slug_segment(&command.case_id),
            port_slug_segment(&command.party_id)
        );
        let invite_url = format!(
            "sdkwork://notary/video?inviteId={}&caseId={}&partyId={}&conversationId={}",
            port_url_component(&invite_id),
            port_url_component(&command.case_id),
            port_url_component(&command.party_id),
            port_url_component(&conversation_id)
        );
        Ok(DrivePartyVideoInviteReference {
            invite_id,
            case_id: command.case_id,
            party_id: command.party_id,
            party_name: command.party_name,
            purpose: command.purpose,
            conversation_id,
            invite_url,
            drive_space_id: command.drive_space_id,
            drive_space_type: command.drive_space_type,
            drive_folder_node_id: command.drive_folder_node_id,
        })
    }

    async fn create_party_signature_invite(
        &self,
        command: DriveCreatePartySignatureInviteCommand,
    ) -> Result<DrivePartySignatureInviteReference, NotaryServiceError> {
        let invite_id = format!(
            "signature-invite-{}-{}",
            port_slug_segment(&command.case_id),
            port_slug_segment(&command.party_id)
        );
        let invite_url = format!(
            "sdkwork://notary/signature?inviteId={}&caseId={}&partyId={}&driveFolderNodeId={}",
            port_url_component(&invite_id),
            port_url_component(&command.case_id),
            port_url_component(&command.party_id),
            port_url_component(&command.drive_folder_node_id)
        );
        Ok(DrivePartySignatureInviteReference {
            invite_id,
            case_id: command.case_id,
            party_id: command.party_id,
            party_name: command.party_name,
            purpose: command.purpose,
            invite_url,
            drive_space_id: command.drive_space_id,
            drive_space_type: command.drive_space_type,
            drive_folder_node_id: command.drive_folder_node_id,
        })
    }

    async fn create_monthly_report(
        &self,
        command: DriveCreateMonthlyReportCommand,
    ) -> Result<DriveMonthlyReportReference, NotaryServiceError> {
        let report_id = format!("notary-monthly-{}-{}", command.month, command.format);
        Ok(DriveMonthlyReportReference {
            report_id: report_id.clone(),
            month: command.month,
            format: command.format.clone(),
            download_url: format!("sdkwork://notary/reports/{report_id}.{}", command.format),
            file_size: command.case_count * 4096,
            case_count: command.case_count,
        })
    }
}

fn port_slug_segment(value: &str) -> String {
    let mut result = String::new();
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash && !result.is_empty() {
            result.push('-');
            previous_dash = true;
        }
    }
    result.trim_end_matches('-').to_string()
}

fn port_url_component(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
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
    ) -> Result<Vec<NotaryOrganizationProfile>, NotaryServiceError> {
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

    async fn list_parties(
        &self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError>;

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

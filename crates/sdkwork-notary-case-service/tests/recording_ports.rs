use std::sync::{Arc, Mutex as StdMutex};

use async_trait::async_trait;
use sdkwork_notary_case_contract::{NotaryCaseRecord, NotaryPartyCommand, NotaryServiceError};
use sdkwork_notary_case_service::{
    AppbaseOrganizationMember, AppbasePort, CommerceCreateOrderCommand, CommerceMatterCommand,
    CommerceMatterListPage, CommerceMatterListQuery, CommerceMatterRecord,
    CommerceMatterUpdateCommand, CommerceOrderFulfillmentState, CommerceOrderReference,
    CommercePort, DriveCreateDownloadPackageCommand, DriveCreateFolderCommand,
    DriveCreateMonthlyReportCommand, DriveCreatePartySignatureInviteCommand,
    DriveCreatePartyVideoInviteCommand, DriveCreateSpaceCommand, DriveDownloadPackageReference,
    DriveFolderReference, DriveListNodesPage, DriveListNodesQuery, DriveMonthlyReportReference,
    DriveNodeReference, DrivePartySignatureInviteReference, DrivePartyVideoInviteReference,
    DrivePort, DriveRegisterCaseFileCommand, NotaryCaseAssignmentCommand,
    NotaryCaseAssignmentRecord, NotaryCaseEventListPage, NotaryCaseEventListQuery,
    NotaryCaseEventRecord, NotaryCaseListPage, NotaryCaseListQuery, NotaryCaseRepositoryPort,
    NotaryCaseUpdateCommand, NotaryDashboardStatisticsAggregate, NotaryDashboardStatisticsQuery,
    NotaryMonthlyCaseCount, NotaryMonthlyCaseCountQuery, NotaryOrganizationProfile,
    NotaryOrganizationProfileListPage, NotaryOrganizationProfileUpdateCommand, NotaryPartyListPage,
    NotaryPartyListQuery, NotaryPartyRecord, NotaryPartyUpdateCommand, NotaryStaffListPage,
    NotaryStaffListQuery,
};
use serde_json::json;

type Shared<T> = Arc<StdMutex<T>>;

fn lock<T>(shared: &Shared<T>) -> std::sync::MutexGuard<'_, T> {
    shared
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[derive(Clone, Default)]
pub struct RecordingAppbase {
    inner: Shared<RecordingAppbaseState>,
}

#[derive(Default)]
struct RecordingAppbaseState {
    members: Vec<AppbaseOrganizationMember>,
}

impl RecordingAppbase {
    pub fn with_member(self, member: AppbaseOrganizationMember) -> Self {
        lock(&self.inner).members.push(member);
        self
    }
}

#[async_trait]
impl AppbasePort for RecordingAppbase {
    async fn get_organization_member(
        &self,
        organization_id: &str,
        membership_id: &str,
    ) -> Result<Option<AppbaseOrganizationMember>, NotaryServiceError> {
        Ok(lock(&self.inner)
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
        Ok(lock(&self.inner)
            .members
            .iter()
            .filter(|member| member.organization_id == organization_id)
            .cloned()
            .collect())
    }

    async fn list_notary_staff_page(
        &self,
        query: NotaryStaffListQuery,
    ) -> Result<NotaryStaffListPage, NotaryServiceError> {
        let members = self
            .list_organization_members(&query.organization_id)
            .await?;
        let page_size = query.page_size.max(1);
        let offset = query.offset.max(0) as usize;
        let filtered: Vec<_> = members
            .into_iter()
            .filter(|member| member.enterprise_verified && member.notary_enabled)
            .filter(|member| {
                query
                    .staff_role
                    .as_ref()
                    .is_none_or(|role| member.roles.iter().any(|value| value == role))
            })
            .skip(offset)
            .take((page_size + 1) as usize)
            .collect();
        let has_more = filtered.len() as i64 > page_size;
        let items = filtered
            .into_iter()
            .take(page_size as usize)
            .collect::<Vec<_>>();
        let next_offset = query.offset + items.len() as i64;
        Ok(NotaryStaffListPage {
            items,
            has_more,
            next_offset,
        })
    }
}

#[derive(Clone, Default)]
pub struct RecordingCommerce {
    inner: Shared<RecordingCommerceState>,
}

#[derive(Default)]
struct RecordingCommerceState {
    events: Vec<String>,
    matters: Vec<CommerceMatterRecord>,
    order_states: Vec<RecordingCommerceOrderState>,
    strict_order_state_lookup: bool,
}

struct RecordingCommerceOrderState {
    organization_id: String,
    state: CommerceOrderFulfillmentState,
}

impl RecordingCommerce {
    pub fn with_matter(self, record: CommerceMatterRecord) -> Self {
        lock(&self.inner).matters.push(record);
        self
    }

    pub fn with_order_fulfillment_state(
        self,
        organization_id: &str,
        state: CommerceOrderFulfillmentState,
    ) -> Self {
        {
            let mut inner = lock(&self.inner);
            inner.strict_order_state_lookup = true;
            inner.order_states.push(RecordingCommerceOrderState {
                organization_id: organization_id.to_owned(),
                state,
            });
        }
        self
    }

    pub fn events(&self) -> Vec<String> {
        lock(&self.inner).events.clone()
    }

    pub fn is_empty(&self) -> bool {
        lock(&self.inner).events.is_empty()
    }
}

#[async_trait]
impl CommercePort for RecordingCommerce {
    async fn create_notary_order(
        &self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "create_order:{}:{}:{}",
            command.sku_id, command.product_type, command.idempotency_key
        ));
        Ok(CommerceOrderReference {
            order_id: format!("order-{}", command.sku_id),
            order_item_id: format!("item-{}", command.sku_id),
            sku_id: command.sku_id,
            matter_title: command.title,
            fee_amount: "500.00".to_string(),
            currency_code: "CNY".to_string(),
        })
    }

    async fn cancel_notary_order(
        &self,
        _organization_id: &str,
        order_id: &str,
    ) -> Result<(), NotaryServiceError> {
        lock(&self.inner)
            .events
            .push(format!("cancel_order:{order_id}"));
        Ok(())
    }

    async fn get_notary_order_fulfillment_state(
        &self,
        organization_id: &str,
        order_id: &str,
    ) -> Result<CommerceOrderFulfillmentState, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "get_order_fulfillment_state:{organization_id}:{order_id}"
        ));
        if let Some(order_state) = state.order_states.iter().find(|order_state| {
            order_state.organization_id == organization_id && order_state.state.order_id == order_id
        }) {
            return Ok(order_state.state.clone());
        }
        if state.strict_order_state_lookup {
            return Err(NotaryServiceError::not_found(
                "commerce order was not found for the notary organization",
            ));
        }
        Ok(CommerceOrderFulfillmentState {
            order_id: order_id.to_owned(),
            order_status: "paid".to_owned(),
            payment_status: Some("success".to_owned()),
            payable_amount: "50000".to_owned(),
        })
    }

    async fn list_notary_matters(
        &self,
        query: CommerceMatterListQuery,
    ) -> Result<CommerceMatterListPage, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_matters:{}:{}:{}:{}:{}",
            query.organization_id.clone().unwrap_or_default(),
            query.search_term.clone().unwrap_or_default(),
            query.status.clone().unwrap_or_default(),
            query.page_size,
            query.offset
        ));
        let mut items = state
            .matters
            .iter()
            .filter(|record| {
                query
                    .status
                    .as_ref()
                    .is_none_or(|status| record.status.eq_ignore_ascii_case(status))
            })
            .filter(|record| {
                query.search_term.as_ref().is_none_or(|search_term| {
                    record
                        .title
                        .to_ascii_lowercase()
                        .contains(&search_term.to_ascii_lowercase())
                })
            })
            .skip(query.offset.max(0) as usize)
            .take(query.page_size.max(0).saturating_add(1) as usize)
            .cloned()
            .collect::<Vec<_>>();
        let has_more = items.len() as i64 > query.page_size;
        items.truncate(query.page_size.max(0) as usize);
        Ok(CommerceMatterListPage { items, has_more })
    }

    async fn create_notary_matter(
        &self,
        command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let sku_id = format!("sku-{}", slug(&command.title));
        state.events.push(format!(
            "create_matter:{}:{}:{}",
            sku_id, command.price_amount, command.idempotency_key
        ));
        let record = CommerceMatterRecord {
            sku_id: sku_id.clone(),
            spu_id: format!("spu-{sku_id}"),
            sku_no: format!("SKU-{}", slug(&command.title).to_ascii_uppercase()),
            title: command.title,
            description: command.description,
            price_amount: command.price_amount,
            original_price_amount: command.original_price_amount,
            currency_code: command.currency_code,
            status: command.status,
            spec: command.spec,
        };
        state.matters.push(record.clone());
        Ok(record)
    }

    async fn update_notary_matter(
        &self,
        command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let record = state
            .matters
            .iter_mut()
            .find(|record| record.sku_id == command.sku_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary matter sku not found"))?;

        if let Some(title) = command.title {
            record.title = title;
        }
        if let Some(description) = command.description {
            record.description = description;
        }
        if let Some(price_amount) = command.price_amount {
            record.price_amount = price_amount;
        }
        if let Some(original_price_amount) = command.original_price_amount {
            record.original_price_amount = original_price_amount;
        }
        if let Some(currency_code) = command.currency_code {
            record.currency_code = currency_code;
        }
        if let Some(status) = command.status {
            record.status = status;
        }
        if let Some(spec) = command.spec {
            record.spec = spec;
        }
        let updated = record.clone();
        state
            .events
            .push(format!("update_matter:{}", command.sku_id));
        Ok(updated)
    }
}

#[derive(Clone, Default)]
pub struct RecordingDrive {
    inner: Shared<RecordingDriveState>,
}

#[derive(Default)]
struct RecordingDriveState {
    events: Vec<String>,
    files: Vec<DriveNodeReference>,
}

impl RecordingDrive {
    pub fn events(&self) -> Vec<String> {
        lock(&self.inner).events.clone()
    }
}

#[async_trait]
impl DrivePort for RecordingDrive {
    async fn create_notary_space(
        &self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError> {
        lock(&self.inner).events.push(format!(
            "create_space:{}:{}:{}",
            command.space_type, command.owner_subject_type, command.owner_subject_id
        ));
        Ok(format!("space-notary-{}", command.owner_subject_id))
    }

    async fn create_case_folder(
        &self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError> {
        lock(&self.inner).events.push(format!(
            "create_folder:{}:{}:{}",
            command.space_type, command.space_id, command.folder_name
        ));
        Ok(DriveFolderReference {
            folder_node_id: format!("folder-{}", command.order_id),
            space_id: command.space_id,
            space_type: command.space_type,
        })
    }

    async fn delete_case_folder(
        &self,
        folder_node_id: &str,
        space_id: &str,
        space_type: &str,
    ) -> Result<(), NotaryServiceError> {
        lock(&self.inner).events.push(format!(
            "delete_folder:{space_type}:{space_id}:{folder_node_id}"
        ));
        Ok(())
    }

    async fn list_nodes(
        &self,
        query: DriveListNodesQuery,
    ) -> Result<DriveListNodesPage, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_nodes:{}:{}:{}:{}:{}:{}",
            query.space_type,
            query.space_id,
            query.parent_node_id,
            query.category.clone().unwrap_or_default(),
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        if !state.files.is_empty() {
            let items = state
                .files
                .iter()
                .filter(|file| {
                    query
                        .category
                        .as_deref()
                        .is_none_or(|category| file.category == category)
                })
                .cloned()
                .collect();
            return Ok(DriveListNodesPage {
                items,
                has_more: false,
                next_cursor: None,
            });
        }
        Ok(DriveListNodesPage {
            items: vec![DriveNodeReference {
                node_id: format!("node-{}", query.parent_node_id),
                node_name: "合同.pdf".to_string(),
                category: query.category.unwrap_or_else(|| "evidence".to_string()),
                size_label: "2.4 MB".to_string(),
                status: "verified".to_string(),
                material_code: None,
                party_id: None,
            }],
            has_more: false,
            next_cursor: None,
        })
    }

    async fn register_case_file(
        &self,
        command: DriveRegisterCaseFileCommand,
    ) -> Result<(), NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "register_case_file:{}:{}:{}:{}",
            command.space_id, command.node_id, command.category, command.review_status
        ));
        state.files.push(DriveNodeReference {
            node_name: command
                .material_code
                .clone()
                .unwrap_or_else(|| command.node_id.clone()),
            node_id: command.node_id,
            category: command.category,
            size_label: String::new(),
            status: command.review_status,
            material_code: command.material_code,
            party_id: command.party_id,
        });
        Ok(())
    }

    async fn create_download_package(
        &self,
        command: DriveCreateDownloadPackageCommand,
    ) -> Result<DriveDownloadPackageReference, NotaryServiceError> {
        lock(&self.inner).events.push(format!(
            "create_download_package:{}:{}:{}",
            command.space_type, command.case_id, command.package_name
        ));
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
        let invite_id = format!("video-invite-{}-{}", command.case_id, command.party_id);
        let conversation_id = format!("notary-{}-{}-video", command.case_id, command.party_id);
        let invite_url = format!(
            "sdkwork://notary/video-invite?inviteId={invite_id}&conversationId={conversation_id}&caseId={}&partyId={}",
            command.case_id, command.party_id
        );
        lock(&self.inner).events.push(format!(
            "create_party_video_invite:{}:{}:{}",
            command.case_id, command.party_id, command.purpose
        ));
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
        let invite_id = format!("signature-invite-{}-{}", command.case_id, command.party_id);
        let invite_url = format!(
            "sdkwork://notary/signature-invite?inviteId={invite_id}&caseId={}&partyId={}",
            command.case_id, command.party_id
        );
        lock(&self.inner).events.push(format!(
            "create_party_signature_invite:{}:{}:{}",
            command.case_id, command.party_id, command.purpose
        ));
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
        lock(&self.inner).events.push(format!(
            "create_monthly_report:{}:{}:{}",
            command.month, command.format, command.case_count
        ));
        Ok(DriveMonthlyReportReference {
            download_url: format!("sdkwork://notary/reports/{report_id}.{}", command.format),
            report_id,
            month: command.month,
            format: command.format,
            file_size: 1_024,
            case_count: command.case_count,
        })
    }
}

#[derive(Clone, Default)]
pub struct RecordingNotaryCaseRepository {
    inner: Shared<RecordingNotaryCaseRepositoryState>,
}

#[derive(Default)]
struct RecordingNotaryCaseRepositoryState {
    events: Vec<String>,
    case_update_commands: Vec<NotaryCaseUpdateCommand>,
    profiles: Vec<(String, String, String)>,
    cases: Vec<NotaryCaseRecord>,
    parties: Vec<NotaryPartyRecord>,
    case_events: Vec<NotaryCaseEventRecord>,
    assignments: Vec<NotaryCaseAssignmentRecord>,
    dashboard_statistics: NotaryDashboardStatisticsAggregate,
    monthly_case_count: i64,
    insert_party_failure_case_id: Option<String>,
}

impl RecordingNotaryCaseRepository {
    pub fn with_profile(self, organization_id: &str, drive_space_id: &str) -> Self {
        lock(&self.inner).profiles.push((
            organization_id.to_string(),
            drive_space_id.to_string(),
            "active".to_string(),
        ));
        self
    }

    pub fn with_case(self, record: NotaryCaseRecord) -> Self {
        lock(&self.inner).cases.push(record);
        self
    }

    pub fn with_party(self, record: NotaryPartyRecord) -> Self {
        lock(&self.inner).parties.push(record);
        self
    }

    pub fn with_event(self, record: NotaryCaseEventRecord) -> Self {
        lock(&self.inner).case_events.push(record);
        self
    }

    pub fn with_dashboard_statistics(self, statistics: NotaryDashboardStatisticsAggregate) -> Self {
        lock(&self.inner).dashboard_statistics = statistics;
        self
    }

    pub fn with_monthly_case_count(self, count: i64) -> Self {
        lock(&self.inner).monthly_case_count = count;
        self
    }

    pub fn with_insert_party_failure(self, case_id: &str) -> Self {
        lock(&self.inner).insert_party_failure_case_id = Some(case_id.to_string());
        self
    }

    pub fn events(&self) -> Vec<String> {
        lock(&self.inner).events.clone()
    }

    pub fn case_update_commands(&self) -> Vec<NotaryCaseUpdateCommand> {
        lock(&self.inner).case_update_commands.clone()
    }

    pub fn is_empty(&self) -> bool {
        lock(&self.inner).events.is_empty()
    }
}

#[async_trait]
impl NotaryCaseRepositoryPort for RecordingNotaryCaseRepository {
    async fn upsert_organization_profile(
        &self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "upsert_profile:{organization_id}:{drive_space_id}:{drive_space_type}"
        ));
        if let Some(profile) = state
            .profiles
            .iter_mut()
            .find(|(profile_org_id, _, _)| profile_org_id == organization_id)
        {
            profile.1 = drive_space_id.to_string();
            profile.2 = "active".to_string();
        } else {
            state.profiles.push((
                organization_id.to_string(),
                drive_space_id.to_string(),
                "active".to_string(),
            ));
        }
        Ok(NotaryOrganizationProfile {
            organization_id: organization_id.to_string(),
            drive_space_id: drive_space_id.to_string(),
            drive_space_type: drive_space_type.to_string(),
            status: "active".to_string(),
        })
    }

    async fn get_organization_profile(
        &self,
        organization_id: &str,
    ) -> Result<Option<NotaryOrganizationProfile>, NotaryServiceError> {
        Ok(lock(&self.inner)
            .profiles
            .iter()
            .find(|(profile_org_id, _, _)| profile_org_id == organization_id)
            .map(
                |(profile_org_id, drive_space_id, status)| NotaryOrganizationProfile {
                    organization_id: profile_org_id.clone(),
                    drive_space_id: drive_space_id.clone(),
                    drive_space_type: "notary".to_string(),
                    status: status.clone(),
                },
            ))
    }

    async fn list_organization_profiles(
        &self,
        organization_id: Option<&str>,
        page_size: i64,
        cursor: Option<&str>,
    ) -> Result<NotaryOrganizationProfileListPage, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_profiles:{}:{}:{}",
            organization_id.unwrap_or_default(),
            page_size,
            cursor.unwrap_or_default()
        ));
        let items = state
            .profiles
            .iter()
            .filter(|(profile_org_id, _, _)| {
                organization_id.is_none_or(|organization_id| profile_org_id == organization_id)
            })
            .take(page_size as usize)
            .map(
                |(profile_org_id, drive_space_id, status)| NotaryOrganizationProfile {
                    organization_id: profile_org_id.clone(),
                    drive_space_id: drive_space_id.clone(),
                    drive_space_type: "notary".to_string(),
                    status: status.clone(),
                },
            )
            .collect();
        Ok(NotaryOrganizationProfileListPage {
            items,
            has_more: false,
            next_cursor: None,
        })
    }

    async fn update_organization_profile(
        &self,
        command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let profile = state
            .profiles
            .iter_mut()
            .find(|(profile_org_id, _, _)| profile_org_id == &command.organization_id)
            .ok_or_else(|| {
                NotaryServiceError::not_found("notary organization profile not found")
            })?;
        if let Some(status) = command.status {
            profile.2 = status;
        }
        let updated = NotaryOrganizationProfile {
            organization_id: profile.0.clone(),
            drive_space_id: profile.1.clone(),
            drive_space_type: "notary".to_string(),
            status: profile.2.clone(),
        };
        state
            .events
            .push(format!("update_profile:{}", command.organization_id));
        Ok(updated)
    }

    async fn insert_case(
        &self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "insert_case:{}:{}:{}:{}",
            record.order_id, record.order_item_id, record.sku_id, record.drive_folder_node_id
        ));
        state.cases.push(record.clone());
        Ok(record)
    }

    async fn delete_case(&self, case_id: &str) -> Result<(), NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!("delete_case:{case_id}"));
        state.cases.retain(|record| record.case_id != case_id);
        state.parties.retain(|party| party.case_id != case_id);
        state.case_events.retain(|event| event.case_id != case_id);
        state
            .assignments
            .retain(|assignment| assignment.case_id != case_id);
        Ok(())
    }

    async fn insert_party(
        &self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let mut state = lock(&self.inner);
        if state
            .insert_party_failure_case_id
            .as_deref()
            .is_some_and(|failed_case_id| failed_case_id == case_id)
        {
            return Err(NotaryServiceError::provider_unavailable(
                "simulated party insert failure",
            ));
        }
        let _ = (case_id, order_item_id);
        state.events.push(format!(
            "insert_party:{}:{}:{}",
            party.name, order_id, sku_id
        ));
        let party_id = format!("party-{case_id}-{}", state.parties.len() + 1);
        state.parties.push(NotaryPartyRecord {
            party_id,
            case_id: case_id.to_string(),
            order_id: order_id.to_string(),
            order_item_id: order_item_id.to_string(),
            sku_id: sku_id.to_string(),
            name: party.name.clone(),
            party_role: party.party_role.clone(),
            identity_no_last4: party
                .identity_no
                .chars()
                .rev()
                .take(4)
                .collect::<String>()
                .chars()
                .rev()
                .collect(),
            phone_masked: party.phone.clone(),
            status: "active".to_string(),
            signature_node_id: None,
        });
        Ok(())
    }

    async fn append_event(
        &self,
        case_id: &str,
        event_type: &str,
    ) -> Result<(), NotaryServiceError> {
        let _ = case_id;
        lock(&self.inner)
            .events
            .push(format!("append_event:{event_type}"));
        Ok(())
    }

    async fn get_case(
        &self,
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(lock(&self.inner)
            .cases
            .iter()
            .find(|record| record.case_id == case_id)
            .cloned())
    }

    async fn update_case(
        &self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let record_index = state
            .cases
            .iter()
            .position(|record| record.case_id == command.case_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
        if state.cases[record_index].version != command.expected_version {
            return Err(NotaryServiceError::conflict("notary case version conflict"));
        }

        state.case_update_commands.push(command.clone());
        let event_type = command.event_type.clone();
        let case_id = command.case_id.clone();
        let updated = {
            let record = &mut state.cases[record_index];

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
            record.version += 1;
            record.updated_at = "2026-06-10 10:30".to_string();
            record.clone()
        };

        state.events.push(format!("update_case:{event_type}"));
        let next_event_order = state.case_events.len() + 1;
        state.case_events.push(NotaryCaseEventRecord {
            event_id: format!("event-{case_id}-{next_event_order}"),
            case_id,
            event_type: event_type.clone(),
            event_title: event_type,
            actor_user_id: Some("1".to_string()),
            occurred_at: updated.updated_at.clone(),
        });

        Ok(updated)
    }

    async fn update_party(
        &self,
        command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let record = state
            .parties
            .iter_mut()
            .find(|record| record.case_id == command.case_id && record.party_id == command.party_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))?;
        if let Some(name) = command.name {
            record.name = name;
        }
        if let Some(party_role) = command.party_role {
            record.party_role = party_role;
        }
        if let Some(identity_no) = command.identity_no {
            record.identity_no_last4 = identity_no
                .chars()
                .rev()
                .take(4)
                .collect::<String>()
                .chars()
                .rev()
                .collect();
        }
        if let Some(phone) = command.phone {
            record.phone_masked = Some(phone);
        }
        if let Some(signature_node_id) = command.signature_node_id {
            record.signature_node_id = Some(signature_node_id);
        }
        let party_id = record.party_id.clone();
        let updated = record.clone();
        state.events.push(format!("update_party:{party_id}"));
        Ok(updated)
    }

    async fn remove_party(&self, case_id: &str, party_id: &str) -> Result<(), NotaryServiceError> {
        let mut state = lock(&self.inner);
        let record = state
            .parties
            .iter_mut()
            .find(|record| record.case_id == case_id && record.party_id == party_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))?;
        record.status = "removed".to_string();
        state.events.push(format!("remove_party:{party_id}"));
        Ok(())
    }

    async fn insert_assignment(
        &self,
        command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
        let mut state = lock(&self.inner);
        let assignment = NotaryCaseAssignmentRecord {
            assignment_id: format!(
                "assignment-{}-{}-{}",
                command.case_id, command.organization_membership_id, command.assignment_role
            ),
            case_id: command.case_id,
            organization_membership_id: command.organization_membership_id,
            user_id: command.user_id,
            assignment_role: command.assignment_role,
            status: "active".to_string(),
            assigned_at: "2026-06-10 10:30".to_string(),
        };
        state
            .events
            .push(format!("insert_assignment:{}", assignment.assignment_id));
        state.assignments.push(assignment.clone());
        Ok(assignment)
    }

    async fn release_assignment(
        &self,
        case_id: &str,
        assignment_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let mut state = lock(&self.inner);
        let assignment = state
            .assignments
            .iter_mut()
            .find(|assignment| {
                assignment.assignment_id == assignment_id && assignment.case_id == case_id
            })
            .ok_or_else(|| NotaryServiceError::not_found("notary case assignment not found"))?;
        assignment.status = "released".to_string();
        state
            .events
            .push(format!("release_assignment:{case_id}:{assignment_id}"));
        Ok(())
    }

    async fn get_case_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(lock(&self.inner)
            .cases
            .iter()
            .find(|record| record.idempotency_key == idempotency_key)
            .cloned())
    }

    async fn list_cases(
        &self,
        query: NotaryCaseListQuery,
    ) -> Result<NotaryCaseListPage, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_cases:{}:{}:{}:{}:{}:{}",
            query.organization_id,
            query.status.clone().unwrap_or_default(),
            query.sku_id.clone().unwrap_or_default(),
            query.search_term.clone().unwrap_or_default(),
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        let page_size = query.page_size.max(1);
        let filtered_items: Vec<NotaryCaseRecord> = state
            .cases
            .iter()
            .filter(|record| record.organization_id == query.organization_id)
            .filter(|record| {
                query
                    .status
                    .as_ref()
                    .is_none_or(|status| record.status.as_storage_value() == status)
            })
            .filter(|record| {
                query
                    .sku_id
                    .as_ref()
                    .is_none_or(|sku_id| &record.sku_id == sku_id)
            })
            .filter(|record| {
                query.search_term.as_ref().is_none_or(|search_term| {
                    record
                        .title
                        .to_ascii_lowercase()
                        .contains(&search_term.to_ascii_lowercase())
                        || record
                            .applicant_name
                            .to_ascii_lowercase()
                            .contains(&search_term.to_ascii_lowercase())
                })
            })
            .cloned()
            .collect();
        let total_items = filtered_items.len() as i64;
        let mut items = filtered_items
            .into_iter()
            .filter(|record| {
                query
                    .cursor
                    .as_ref()
                    .is_none_or(|cursor| record.case_id.as_str() < cursor.as_str())
            })
            .take((page_size + 1) as usize)
            .collect::<Vec<_>>();
        let has_more = items.len() as i64 > page_size;
        if has_more {
            items.truncate(page_size as usize);
        }
        let next_cursor = items.last().map(|record| record.case_id.clone());
        Ok(NotaryCaseListPage {
            items,
            has_more,
            next_cursor,
            total_items,
        })
    }

    async fn get_dashboard_statistics(
        &self,
        query: NotaryDashboardStatisticsQuery,
    ) -> Result<NotaryDashboardStatisticsAggregate, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "get_dashboard_statistics:{}",
            query.organization_id
        ));
        Ok(state.dashboard_statistics.clone())
    }

    async fn count_cases_for_month(
        &self,
        query: NotaryMonthlyCaseCountQuery,
    ) -> Result<NotaryMonthlyCaseCount, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "count_cases_for_month:{}:{}",
            query.organization_id(),
            query.month()
        ));
        Ok(NotaryMonthlyCaseCount {
            count: state.monthly_case_count,
        })
    }

    async fn list_parties(
        &self,
        query: NotaryPartyListQuery,
    ) -> Result<NotaryPartyListPage, NotaryServiceError> {
        let items = lock(&self.inner)
            .parties
            .iter()
            .filter(|record| record.case_id == query.case_id)
            .take(query.page_size.max(1) as usize)
            .cloned()
            .collect();
        Ok(NotaryPartyListPage {
            items,
            has_more: false,
            next_cursor: None,
        })
    }

    async fn list_events(
        &self,
        query: NotaryCaseEventListQuery,
    ) -> Result<NotaryCaseEventListPage, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_events:{}:{}:{}",
            query.case_id,
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        let page_size = query.page_size.max(1);
        let mut items: Vec<NotaryCaseEventRecord> = state
            .case_events
            .iter()
            .filter(|record| record.case_id == query.case_id)
            .filter(|record| {
                query
                    .cursor
                    .as_ref()
                    .is_none_or(|cursor| record.event_id.as_str() > cursor.as_str())
            })
            .take((page_size + 1) as usize)
            .cloned()
            .collect();
        let has_more = items.len() as i64 > page_size;
        if has_more {
            items.truncate(page_size as usize);
        }
        let next_cursor = items.last().map(|record| record.event_id.clone());
        Ok(NotaryCaseEventListPage {
            items,
            has_more,
            next_cursor,
        })
    }
}

fn slug(value: &str) -> String {
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

pub fn sample_matter_record(sku_id: &str, title: &str) -> CommerceMatterRecord {
    CommerceMatterRecord {
        sku_id: sku_id.to_string(),
        spu_id: format!("spu-{sku_id}"),
        sku_no: format!("SKU-{}", slug(title).to_ascii_uppercase()),
        title: title.to_string(),
        description: Some(format!("{title} service")),
        price_amount: "500.00".to_string(),
        original_price_amount: None,
        currency_code: "CNY".to_string(),
        status: "active".to_string(),
        spec: json!({
            "productType": "notary",
            "skuPolicy": "one_spu_one_sku"
        }),
    }
}

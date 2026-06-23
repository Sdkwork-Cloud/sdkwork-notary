use std::sync::{Arc, Mutex as StdMutex};

use async_trait::async_trait;
use sdkwork_notary_case_contract::{NotaryCaseRecord, NotaryPartyCommand, NotaryServiceError};
use sdkwork_notary_case_service::{
    AppbaseOrganizationMember, AppbasePort, CommerceCreateOrderCommand, CommerceMatterCommand,
    CommerceMatterListQuery, CommerceMatterRecord, CommerceMatterUpdateCommand,
    CommerceOrderReference, CommercePort, DriveCreateDownloadPackageCommand,
    DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveDownloadPackageReference,
    DriveFolderReference, DriveListNodesQuery, DriveNodeReference, DrivePort,
    NotaryCaseAssignmentCommand, NotaryCaseAssignmentRecord, NotaryCaseEventListPage,
    NotaryCaseEventListQuery, NotaryCaseEventRecord, NotaryCaseListPage, NotaryCaseListQuery,
    NotaryCaseRepositoryPort, NotaryCaseUpdateCommand, NotaryOrganizationProfile,
    NotaryOrganizationProfileUpdateCommand, NotaryPartyRecord, NotaryPartyUpdateCommand,
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
}

#[derive(Clone, Default)]
pub struct RecordingCommerce {
    inner: Shared<RecordingCommerceState>,
}

#[derive(Default)]
struct RecordingCommerceState {
    events: Vec<String>,
    matters: Vec<CommerceMatterRecord>,
}

impl RecordingCommerce {
    pub fn with_matter(self, record: CommerceMatterRecord) -> Self {
        lock(&self.inner).matters.push(record);
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

    async fn cancel_notary_order(&self, order_id: &str) -> Result<(), NotaryServiceError> {
        lock(&self.inner)
            .events
            .push(format!("cancel_order:{order_id}"));
        Ok(())
    }

    async fn list_notary_matters(
        &self,
        query: CommerceMatterListQuery,
    ) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_matters:{}:{}:{}:{}",
            query.organization_id.clone().unwrap_or_default(),
            query.search_term.clone().unwrap_or_default(),
            query.status.clone().unwrap_or_default(),
            query.page_size
        ));
        Ok(state
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
            .take(query.page_size as usize)
            .cloned()
            .collect())
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
            record.description = Some(description);
        }
        if let Some(price_amount) = command.price_amount {
            record.price_amount = price_amount;
        }
        if let Some(original_price_amount) = command.original_price_amount {
            record.original_price_amount = Some(original_price_amount);
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
    ) -> Result<Vec<DriveNodeReference>, NotaryServiceError> {
        lock(&self.inner).events.push(format!(
            "list_nodes:{}:{}:{}:{}:{}:{}",
            query.space_type,
            query.space_id,
            query.parent_node_id,
            query.category.clone().unwrap_or_default(),
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        Ok(vec![DriveNodeReference {
            node_id: format!("node-{}", query.parent_node_id),
            node_name: "合同.pdf".to_string(),
            category: query.category.unwrap_or_else(|| "evidence".to_string()),
            size_label: "2.4 MB".to_string(),
            status: "verified".to_string(),
        }])
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
}

#[derive(Clone, Default)]
pub struct RecordingNotaryCaseRepository {
    inner: Shared<RecordingNotaryCaseRepositoryState>,
}

#[derive(Default)]
struct RecordingNotaryCaseRepositoryState {
    events: Vec<String>,
    profiles: Vec<(String, String, String)>,
    cases: Vec<NotaryCaseRecord>,
    parties: Vec<NotaryPartyRecord>,
    case_events: Vec<NotaryCaseEventRecord>,
    assignments: Vec<NotaryCaseAssignmentRecord>,
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

    pub fn with_insert_party_failure(self, case_id: &str) -> Self {
        lock(&self.inner).insert_party_failure_case_id = Some(case_id.to_string());
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
    ) -> Result<Vec<NotaryOrganizationProfile>, NotaryServiceError> {
        let mut state = lock(&self.inner);
        state.events.push(format!(
            "list_profiles:{}:{}",
            organization_id.unwrap_or_default(),
            page_size
        ));
        Ok(state
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
            .collect())
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
        record.updated_at = "2026-06-10 10:30".to_string();

        Ok(record.clone())
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
        let mut items: Vec<NotaryCaseRecord> = state
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
            .filter(|record| {
                query
                    .cursor
                    .as_ref()
                    .is_none_or(|cursor| record.case_id.as_str() < cursor.as_str())
            })
            .take((page_size + 1) as usize)
            .cloned()
            .collect();
        let has_more = items.len() as i64 > page_size;
        if has_more {
            items.truncate(page_size as usize);
        }
        Ok(NotaryCaseListPage { items, has_more })
    }

    async fn list_parties(
        &self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError> {
        Ok(lock(&self.inner)
            .parties
            .iter()
            .filter(|record| record.case_id == case_id)
            .cloned()
            .collect())
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
        Ok(NotaryCaseEventListPage { items, has_more })
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

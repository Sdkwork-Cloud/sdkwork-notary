use std::collections::BTreeMap;

use sdkwork_notary_core::{
    NotaryCaseCommand, NotaryCaseRecord, NotaryCaseStatus, NotaryRuntimeContext,
    NotaryServiceContract, NotaryServiceError,
};
use serde_json::{json, Value};

use crate::{
    AppbaseOrganizationMember, CommerceCreateOrderCommand, CommerceMatterCommand,
    CommerceMatterListQuery, CommerceMatterRecord, CommerceMatterUpdateCommand,
    DriveCreateFolderCommand, DriveCreateSpaceCommand, DriveListNodesQuery, DriveNodeReference,
    NotaryCaseAssignmentCommand, NotaryCaseAssignmentRecord, NotaryCaseEventListQuery,
    NotaryCaseEventRecord, NotaryCaseListQuery, NotaryCaseUpdateCommand, NotaryOrganizationProfile,
    NotaryOrganizationProfileUpdateCommand, NotaryPartyRecord, NotaryPartyUpdateCommand,
    NotaryRuntimePorts, NOTARY_CASE_REPOSITORY_PORT, NOTARY_COMMERCE_PORT, NOTARY_DRIVE_PORT,
    NOTARY_IAM_PORT,
};

pub fn notary_runtime_contract() -> NotaryServiceContract {
    NotaryServiceContract::new(
        "notary",
        "notary.case",
        vec![
            "notary.business.open",
            "notary.organizationProfiles.create",
            "notary.organizationProfiles.update",
            "notary.matters.create",
            "notary.matters.update",
            "notary.cases.create",
            "notary.cases.update",
            "notary.cases.acceptances.create",
            "notary.cases.rejections.create",
            "notary.cases.completions.create",
            "notary.cases.assignments.create",
            "notary.cases.assignments.delete",
            "notary.cases.files.create",
            "notary.cases.downloadPackages.create",
            "notary.cases.parties.create",
            "notary.cases.parties.update",
            "notary.cases.parties.delete",
            "notary.cases.parties.signatures.create",
            "notary.cases.parties.videoInvites.create",
            "notary.cases.parties.signatureInvites.create",
        ],
        vec![
            "notary.access.retrieve",
            "notary.organizationProfiles.list",
            "notary.organizationProfiles.retrieve",
            "notary.matters.list",
            "notary.matters.management.list",
            "notary.staff.list",
            "notary.cases.list",
            "notary.cases.retrieve",
            "notary.cases.management.list",
            "notary.cases.management.retrieve",
            "notary.cases.parties.list",
            "notary.cases.files.list",
            "notary.cases.events.list",
            "notary.reports.caseSummary.retrieve",
            "notary.dashboard.statistics.retrieve",
            "notary.reports.monthly.retrieve",
        ],
        vec![
            NOTARY_IAM_PORT,
            NOTARY_COMMERCE_PORT,
            NOTARY_DRIVE_PORT,
            NOTARY_CASE_REPOSITORY_PORT,
        ],
        true,
    )
}

pub async fn ensure_notary_business_open(
    context: &NotaryRuntimeContext,
    organization_id: &str,
    opened_by_membership_id: &str,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<NotaryOrganizationProfile, NotaryServiceError> {
    validate_context(context)?;
    let member = require_notary_member(
        ports.appbase,
        organization_id,
        opened_by_membership_id,
        true,
    )
    .await?;
    if member.user_id != context.user_id {
        return Err(NotaryServiceError::unauthorized(
            "organization member does not match current user",
        ));
    }

    let drive_space_id = ports
        .drive
        .create_notary_space(DriveCreateSpaceCommand {
            owner_subject_type: "organization".to_string(),
            owner_subject_id: organization_id.to_string(),
            space_type: "notary".to_string(),
            display_name: "Notary".to_string(),
        })
        .await?;

    ports
        .repository
        .upsert_organization_profile(organization_id, &drive_space_id, "notary")
        .await
}

pub async fn create_notary_case(
    context: &NotaryRuntimeContext,
    command: NotaryCaseCommand,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<NotaryCaseRecord, NotaryServiceError> {
    validate_context(context)?;
    validate_create_case_command(&command)?;

    let primary_notary_member =
        if let Some(membership_id) = command.primary_notary_membership_id.as_deref() {
            Some(
                require_notary_member(
                    ports.appbase,
                    &command.organization_id,
                    membership_id,
                    false,
                )
                .await?,
            )
        } else {
            None
        };

    let primary_notary_membership_id = primary_notary_member
        .as_ref()
        .map(|member| member.membership_id.clone());
    let primary_notary_user_id = primary_notary_member
        .as_ref()
        .map(|member| member.user_id.clone());
    let primary_notary_name = primary_notary_member
        .as_ref()
        .map(|member| member.membership_id.clone());

    let profile = ports
        .repository
        .get_organization_profile(&command.organization_id)
        .await?
        .ok_or_else(|| NotaryServiceError::not_found("notary business is not open"))?;

    if profile.drive_space_type != "notary" {
        return Err(NotaryServiceError::invalid_state(
            "notary organization profile must use drive space_type notary",
        ));
    }

    let order = ports
        .commerce
        .create_notary_order(CommerceCreateOrderCommand {
            organization_id: command.organization_id.clone(),
            sku_id: command.sku_id.clone(),
            title: command.title.clone(),
            applicant_name: command.applicant_name.clone(),
            product_type: "notary".to_string(),
            idempotency_key: command.idempotency_key.clone(),
        })
        .await?;

    let case_id = format!("case-{}", order.order_item_id);
    let folder = ports
        .drive
        .create_case_folder(DriveCreateFolderCommand {
            space_id: profile.drive_space_id.clone(),
            space_type: "notary".to_string(),
            parent_node_id: None,
            folder_name: command
                .drive_folder_name
                .clone()
                .unwrap_or_else(|| command.title.clone()),
            order_id: order.order_id.clone(),
            case_id: case_id.clone(),
        })
        .await?;

    let now = "2026-06-10 00:00".to_string();
    let record = NotaryCaseRecord {
        case_id: case_id.clone(),
        case_no: build_case_no(&order.order_item_id),
        organization_id: command.organization_id.clone(),
        title: command.title,
        applicant_name: command.applicant_name,
        primary_notary_membership_id,
        primary_notary_user_id,
        primary_notary_name,
        status: NotaryCaseStatus::PendingReview,
        order_id: order.order_id.clone(),
        order_item_id: order.order_item_id.clone(),
        sku_id: order.sku_id.clone(),
        matter_title: order.matter_title,
        fee_amount: order.fee_amount,
        currency_code: order.currency_code,
        drive_space_id: folder.space_id,
        drive_space_type: folder.space_type,
        drive_folder_node_id: folder.folder_node_id,
        chain_hash: None,
        remarks: command.remarks,
        request_no: build_case_no(&order.order_item_id),
        idempotency_key: command.idempotency_key,
        created_at: now.clone(),
        updated_at: now,
    };

    let inserted = ports.repository.insert_case(record).await?;
    for party in &command.parties {
        ports
            .repository
            .insert_party(
                &inserted.case_id,
                party,
                &inserted.order_id,
                &inserted.order_item_id,
                &inserted.sku_id,
            )
            .await?;
    }
    ports
        .repository
        .append_event(&inserted.case_id, "notary.case.submitted")
        .await?;

    Ok(inserted)
}

pub async fn list_case_files(
    context: &NotaryRuntimeContext,
    case_id: &str,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Vec<DriveNodeReference>, NotaryServiceError> {
    validate_context(context)?;
    let record = ports
        .repository
        .get_case(case_id)
        .await?
        .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;

    ports
        .drive
        .list_nodes(DriveListNodesQuery {
            space_id: record.drive_space_id,
            space_type: record.drive_space_type,
            parent_node_id: record.drive_folder_node_id,
            category: None,
            page_size: 50,
            cursor: None,
        })
        .await
}

pub async fn handle_notary_app_operation(
    context: &NotaryRuntimeContext,
    operation_id: &str,
    path_params: BTreeMap<String, String>,
    body: Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    match operation_id {
        "notary.access.retrieve" => retrieve_notary_access(context, ports).await,
        "notary.matters.list" => list_notary_matters(context, &body, ports).await,
        "notary.dashboard.statistics.retrieve" => {
            let cases = list_app_cases_for_report(context, &body, ports).await?;
            Ok(notary_statistics_to_value(&cases))
        }
        "notary.reports.monthly.retrieve" => {
            let cases = list_app_cases_for_report(context, &body, ports).await?;
            Ok(monthly_report_to_value(&cases, &body))
        }
        "notary.cases.create" => {
            let command = create_case_command_from_body(context, &body)?;
            let record = create_notary_case(context, command, ports).await?;
            Ok(case_record_to_value(&record))
        }
        "notary.staff.list" => list_notary_staff(context, &body, ports).await,
        "notary.cases.list" => {
            let organization_id = string_field(&body, &["organizationId", "organization_id"])
                .or_else(|| context.organization_id.clone())
                .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
            let cases = ports
                .repository
                .list_cases(NotaryCaseListQuery {
                    organization_id,
                    status: string_field(&body, &["status"]).map(|value| status_to_storage(&value)),
                    sku_id: string_field(&body, &["skuId", "sku_id"]),
                    search_term: string_field(&body, &["q", "searchTerm", "search_term"]),
                    page_size: integer_field(&body, &["pageSize", "page_size"]).unwrap_or(50),
                    cursor: string_field(&body, &["cursor"]),
                })
                .await?;
            Ok(json!({
                "items": cases
                    .iter()
                    .map(case_record_to_value)
                    .collect::<Vec<_>>(),
                "pageInfo": {
                    "hasMore": false
                }
            }))
        }
        "notary.cases.retrieve" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = ports
                .repository
                .get_case(case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.update" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = update_case_from_body(case_id, &body, ports).await?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.acceptances.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = update_case_status(
                case_id,
                NotaryCaseStatus::Processing,
                "notary.case.accepted",
                None,
                ports,
            )
            .await?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.rejections.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = update_case_status(
                case_id,
                NotaryCaseStatus::Rejected,
                "notary.case.rejected",
                None,
                ports,
            )
            .await?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.completions.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = update_case_status(
                case_id,
                NotaryCaseStatus::Completed,
                "notary.case.completed",
                string_field(&body, &["chainHash", "chain_hash"]),
                ports,
            )
            .await?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.assignments.create" => {
            create_case_assignment(context, &path_params, &body, ports).await
        }
        "notary.cases.parties.list" => {
            let case_id = path_param(&path_params, "caseId")?;
            let parties = ports.repository.list_parties(case_id).await?;
            Ok(json!({
                "items": parties
                    .iter()
                    .map(party_record_to_value)
                    .collect::<Vec<_>>()
            }))
        }
        "notary.cases.parties.create" => {
            let case_id = path_param(&path_params, "caseId")?.to_string();
            let record = ports
                .repository
                .get_case(&case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            let party = party_command_from_value(&body)?;
            ports
                .repository
                .insert_party(
                    &record.case_id,
                    &party,
                    &record.order_id,
                    &record.order_item_id,
                    &record.sku_id,
                )
                .await?;
            ports
                .repository
                .append_event(&record.case_id, "notary.party.created")
                .await?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.parties.update" => {
            let case_id = path_param(&path_params, "caseId")?.to_string();
            let party_id = path_param(&path_params, "partyId")?.to_string();
            let party = ports
                .repository
                .update_party(party_update_command_from_body(&case_id, &party_id, &body))
                .await?;
            ports
                .repository
                .append_event(&case_id, "notary.party.updated")
                .await?;
            Ok(party_record_to_value(&party))
        }
        "notary.cases.parties.delete" => {
            let case_id = path_param(&path_params, "caseId")?;
            let party_id = path_param(&path_params, "partyId")?;
            ports.repository.remove_party(case_id, party_id).await?;
            ports
                .repository
                .append_event(case_id, "notary.party.removed")
                .await?;
            Ok(json!({
                "deleted": true,
                "caseId": case_id,
                "partyId": party_id
            }))
        }
        "notary.cases.parties.signatures.create" => {
            let case_id = path_param(&path_params, "caseId")?.to_string();
            let party_id = path_param(&path_params, "partyId")?.to_string();
            let drive_node_id = string_field(
                &body,
                &[
                    "signatureNodeId",
                    "signature_node_id",
                    "driveNodeId",
                    "drive_node_id",
                    "nodeId",
                ],
            )
            .ok_or_else(|| NotaryServiceError::validation("signatureNodeId is required"))?;
            let party = ports
                .repository
                .update_party(NotaryPartyUpdateCommand {
                    case_id: case_id.clone(),
                    party_id,
                    name: None,
                    party_role: None,
                    identity_no: None,
                    phone: None,
                    signature_node_id: Some(drive_node_id),
                })
                .await?;
            ports
                .repository
                .append_event(&case_id, "notary.party.signature_attached")
                .await?;
            Ok(party_record_to_value(&party))
        }
        "notary.cases.parties.videoInvites.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let party_id = path_param(&path_params, "partyId")?;
            create_party_video_invite(case_id, party_id, &body, ports).await
        }
        "notary.cases.parties.signatureInvites.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let party_id = path_param(&path_params, "partyId")?;
            create_party_signature_invite(case_id, party_id, &body, ports).await
        }
        "notary.cases.files.list" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = ports
                .repository
                .get_case(case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            let files = ports
                .drive
                .list_nodes(DriveListNodesQuery {
                    space_id: record.drive_space_id.clone(),
                    space_type: record.drive_space_type.clone(),
                    parent_node_id: record.drive_folder_node_id.clone(),
                    category: string_field(&body, &["category"]),
                    page_size: integer_field(&body, &["pageSize", "page_size"]).unwrap_or(50),
                    cursor: string_field(&body, &["cursor"]),
                })
                .await?;
            Ok(json!({
                "items": files
                    .iter()
                    .map(|file| drive_node_to_document_value(file, &record))
                    .collect::<Vec<_>>(),
                "pageInfo": {
                    "hasMore": false
                }
            }))
        }
        "notary.cases.files.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = ports
                .repository
                .get_case(case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            let document = file_create_to_document_value(&body, &record)?;
            ports
                .repository
                .append_event(&record.case_id, "notary.case.file_attached")
                .await?;
            Ok(document)
        }
        "notary.cases.downloadPackages.create" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = ports
                .repository
                .get_case(case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            ports
                .repository
                .append_event(&record.case_id, "notary.case.download_package_requested")
                .await?;
            Ok(json!({
                "packageId": format!("download-package-{}", record.case_id),
                "caseId": record.case_id,
                "driveSpaceId": record.drive_space_id,
                "driveSpaceType": record.drive_space_type,
                "status": "preparing",
                "packageName": string_field(&body, &["packageName", "package_name"])
                    .unwrap_or_else(|| format!("{}.zip", record.case_no))
            }))
        }
        "notary.cases.events.list" => {
            let case_id = path_param(&path_params, "caseId")?;
            let events = ports
                .repository
                .list_events(NotaryCaseEventListQuery {
                    case_id: case_id.to_string(),
                    page_size: integer_field(&body, &["pageSize", "page_size"]).unwrap_or(50),
                    cursor: string_field(&body, &["cursor"]),
                })
                .await?;
            Ok(json!({
                "items": events
                    .iter()
                    .map(event_record_to_value)
                    .collect::<Vec<_>>(),
                "pageInfo": {
                    "hasMore": false
                }
            }))
        }
        _ => Err(NotaryServiceError::provider_unavailable(format!(
            "unsupported notary app operation: {operation_id}"
        ))),
    }
}

async fn list_app_cases_for_report(
    context: &NotaryRuntimeContext,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError> {
    let organization_id = string_field(body, &["organizationId", "organization_id"])
        .or_else(|| context.organization_id.clone())
        .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
    ports
        .repository
        .list_cases(NotaryCaseListQuery {
            organization_id,
            status: None,
            sku_id: None,
            search_term: None,
            page_size: 500,
            cursor: None,
        })
        .await
}

pub async fn handle_notary_backend_operation(
    context: &NotaryRuntimeContext,
    operation_id: &str,
    path_params: BTreeMap<String, String>,
    body: Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    match operation_id {
        "notary.organizationProfiles.create" => {
            let organization_id = string_field(&body, &["organizationId", "organization_id"])
                .or_else(|| context.organization_id.clone())
                .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
            let opened_by_membership_id = string_field(
                &body,
                &[
                    "openedByMembershipId",
                    "opened_by_membership_id",
                    "membershipId",
                ],
            )
            .or_else(|| context.membership_id.clone())
            .ok_or_else(|| NotaryServiceError::validation("openedByMembershipId is required"))?;
            let profile = ensure_notary_business_open(
                context,
                &organization_id,
                &opened_by_membership_id,
                ports,
            )
            .await?;
            Ok(organization_profile_to_value(&profile, context))
        }
        "notary.organizationProfiles.list" => {
            let organization_id = string_field(&body, &["organizationId", "organization_id"])
                .or_else(|| context.organization_id.clone());
            let profiles = ports
                .repository
                .list_organization_profiles(
                    organization_id.as_deref(),
                    integer_field(&body, &["pageSize", "page_size"]).unwrap_or(20),
                )
                .await?;
            Ok(json!({
                "items": profiles
                    .iter()
                    .map(|profile| organization_profile_to_value(profile, context))
                    .collect::<Vec<_>>(),
                "pageInfo": {
                    "hasMore": false
                }
            }))
        }
        "notary.organizationProfiles.retrieve" => {
            let organization_id = path_param(&path_params, "organizationProfileId")?;
            let profile = ports
                .repository
                .get_organization_profile(organization_id)
                .await?
                .ok_or_else(|| {
                    NotaryServiceError::not_found("notary organization profile not found")
                })?;
            Ok(organization_profile_to_value(&profile, context))
        }
        "notary.organizationProfiles.update" => {
            let organization_id = path_param(&path_params, "organizationProfileId")?.to_string();
            let status = string_field(&body, &["status"]);
            if let Some(status) = status.as_deref() {
                validate_profile_status(status)?;
            }
            let profile = ports
                .repository
                .update_organization_profile(NotaryOrganizationProfileUpdateCommand {
                    organization_id,
                    status,
                    settings: body.get("settings").cloned(),
                })
                .await?;
            Ok(organization_profile_to_value(&profile, context))
        }
        "notary.matters.management.list" => list_notary_matters(context, &body, ports).await,
        "notary.matters.create" => {
            let matter = ports
                .commerce
                .create_notary_matter(matter_command_from_body(context, &body)?)
                .await?;
            Ok(matter_record_to_value(&matter))
        }
        "notary.matters.update" => {
            let sku_id = path_param(&path_params, "skuId")?.to_string();
            let matter = ports
                .commerce
                .update_notary_matter(matter_update_command_from_body(&sku_id, &body))
                .await?;
            Ok(matter_record_to_value(&matter))
        }
        "notary.cases.management.list" => {
            let cases = list_backend_cases(context, &body, ports).await?;
            Ok(json!({
                "items": cases.iter().map(case_record_to_value).collect::<Vec<_>>(),
                "pageInfo": {
                    "hasMore": false
                }
            }))
        }
        "notary.cases.management.retrieve" => {
            let case_id = path_param(&path_params, "caseId")?;
            let record = ports
                .repository
                .get_case(case_id)
                .await?
                .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
            case_detail_to_value(&record, ports).await
        }
        "notary.cases.assignments.create" => {
            create_case_assignment(context, &path_params, &body, ports).await
        }
        "notary.cases.assignments.delete" => {
            let case_id = path_param(&path_params, "caseId")?.to_string();
            let assignment_id = path_param(&path_params, "assignmentId")?.to_string();
            ports.repository.release_assignment(&assignment_id).await?;
            ports
                .repository
                .append_event(&case_id, "notary.case.assignment_released")
                .await?;
            Ok(json!({
                "released": true,
                "caseId": case_id,
                "assignmentId": assignment_id
            }))
        }
        "notary.staff.list" => list_notary_staff(context, &body, ports).await,
        "notary.reports.caseSummary.retrieve" => {
            let cases = list_backend_cases(context, &body, ports).await?;
            Ok(case_summary_to_value(&cases))
        }
        _ => Err(NotaryServiceError::provider_unavailable(format!(
            "unsupported notary backend operation: {operation_id}"
        ))),
    }
}

async fn list_backend_cases(
    context: &NotaryRuntimeContext,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError> {
    let organization_id = string_field(body, &["organizationId", "organization_id"])
        .or_else(|| context.organization_id.clone())
        .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
    ports
        .repository
        .list_cases(NotaryCaseListQuery {
            organization_id,
            status: string_field(body, &["status"]).map(|value| status_to_storage(&value)),
            sku_id: None,
            search_term: string_field(body, &["q", "searchTerm", "search_term"]),
            page_size: integer_field(body, &["pageSize", "page_size"]).unwrap_or(20),
            cursor: string_field(body, &["cursor"]),
        })
        .await
}

async fn create_case_assignment(
    context: &NotaryRuntimeContext,
    path_params: &BTreeMap<String, String>,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    let case_id = path_param(path_params, "caseId")?.to_string();
    let record = ports
        .repository
        .get_case(&case_id)
        .await?
        .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
    let organization_membership_id = string_field(
        body,
        &[
            "organizationMembershipId",
            "organization_membership_id",
            "membershipId",
        ],
    )
    .ok_or_else(|| NotaryServiceError::validation("organizationMembershipId is required"))?;
    let assignment_role = string_field(body, &["assignmentRole", "assignment_role"])
        .ok_or_else(|| NotaryServiceError::validation("assignmentRole is required"))?;
    validate_assignment_role(&assignment_role)?;
    let member = require_notary_member(
        ports.appbase,
        &record.organization_id,
        &organization_membership_id,
        false,
    )
    .await?;
    let assignment = ports
        .repository
        .insert_assignment(NotaryCaseAssignmentCommand {
            case_id: case_id.clone(),
            organization_id: record.organization_id,
            organization_membership_id,
            user_id: member.user_id,
            assignment_role,
            assigned_by_membership_id: context.membership_id.clone(),
        })
        .await?;
    ports
        .repository
        .append_event(&case_id, "notary.case.assignment_created")
        .await?;
    Ok(assignment_record_to_value(&assignment))
}

async fn list_notary_staff(
    context: &NotaryRuntimeContext,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    let organization_id = string_field(body, &["organizationId", "organization_id"])
        .or_else(|| context.organization_id.clone())
        .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
    let staff_role = string_field(body, &["staffRole", "staff_role"]);
    let members = ports
        .appbase
        .list_organization_members(&organization_id)
        .await?;
    let items = members
        .iter()
        .filter(|member| member.enterprise_verified && member.notary_enabled)
        .filter(|member| {
            staff_role.as_ref().is_none_or(|role| {
                member_notary_staff_role(member).as_deref() == Some(role.as_str())
            })
        })
        .map(staff_member_to_value)
        .collect::<Vec<_>>();

    Ok(json!({
        "items": items,
        "pageInfo": {
            "hasMore": false
        }
    }))
}

async fn retrieve_notary_access(
    context: &NotaryRuntimeContext,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    validate_context(context)?;
    let organization_id = context.organization_id.clone().unwrap_or_default();
    let membership_id = context.membership_id.clone().unwrap_or_default();
    if organization_id.is_empty() || membership_id.is_empty() {
        return Ok(notary_access_value(
            context,
            None,
            false,
            "organization membership is required",
        ));
    }

    let member = ports
        .appbase
        .get_organization_member(&organization_id, &membership_id)
        .await?;
    let Some(member) = member else {
        return Ok(notary_access_value(
            context,
            None,
            false,
            "organization member is required",
        ));
    };

    let profile = ports
        .repository
        .get_organization_profile(&organization_id)
        .await?;
    let profile_active = profile
        .as_ref()
        .is_some_and(|profile| profile.status == "active" && profile.drive_space_type == "notary");
    let visible = member.enterprise_verified && member.notary_enabled && profile_active;
    let reason = if visible {
        ""
    } else if !member.enterprise_verified {
        "organization member is not enterprise verified"
    } else if !member.notary_enabled {
        "organization member is not enabled for notary business"
    } else {
        "notary business is not open"
    };

    Ok(notary_access_value(context, Some(&member), visible, reason))
}

async fn list_notary_matters(
    context: &NotaryRuntimeContext,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    validate_context(context)?;
    let matters = ports
        .commerce
        .list_notary_matters(CommerceMatterListQuery {
            organization_id: string_field(body, &["organizationId", "organization_id"])
                .or_else(|| context.organization_id.clone()),
            search_term: string_field(body, &["q", "searchTerm", "search_term"]),
            status: string_field(body, &["status"]),
            page_size: integer_field(body, &["pageSize", "page_size"]).unwrap_or(20),
        })
        .await?;
    Ok(json!({
        "items": matters
            .iter()
            .map(matter_record_to_value)
            .collect::<Vec<_>>(),
        "pageInfo": {
            "hasMore": false
        }
    }))
}

fn notary_access_value(
    context: &NotaryRuntimeContext,
    member: Option<&AppbaseOrganizationMember>,
    visible: bool,
    reason: &str,
) -> Value {
    let permissions = if visible {
        vec![
            "notary.matters.read",
            "notary.cases.read",
            "notary.cases.create",
            "notary.cases.update",
            "notary.files.read",
            "notary.files.create",
        ]
    } else {
        Vec::new()
    };
    json!({
        "visible": visible,
        "organizationVerified": member.is_some_and(|member| member.enterprise_verified),
        "notaryBusinessEnabled": visible,
        "tenantId": context.tenant_id,
        "organizationId": context.organization_id,
        "memberId": member
            .map(|member| member.membership_id.clone())
            .or_else(|| context.membership_id.clone())
            .unwrap_or_default(),
        "roles": member.map(|member| member.roles.clone()).unwrap_or_default(),
        "positions": member.map(|member| member.positions.clone()).unwrap_or_default(),
        "departments": member.map(|member| member.departments.clone()).unwrap_or_default(),
        "permissions": permissions,
        "reason": reason
    })
}

fn matter_record_to_value(record: &CommerceMatterRecord) -> Value {
    json!({
        "skuId": record.sku_id,
        "spuId": record.spu_id,
        "skuNo": record.sku_no,
        "title": record.title,
        "description": record.description,
        "priceAmount": record.price_amount,
        "originalPriceAmount": record.original_price_amount,
        "currencyCode": record.currency_code,
        "status": record.status,
        "spec": record.spec
    })
}

fn assignment_record_to_value(record: &NotaryCaseAssignmentRecord) -> Value {
    json!({
        "id": record.assignment_id,
        "caseId": record.case_id,
        "organizationMembershipId": record.organization_membership_id,
        "userId": record.user_id,
        "displayName": record.organization_membership_id,
        "assignmentRole": record.assignment_role,
        "status": record.status,
        "assignedAt": record.assigned_at
    })
}

fn organization_profile_to_value(
    profile: &NotaryOrganizationProfile,
    context: &NotaryRuntimeContext,
) -> Value {
    json!({
        "id": format!("notary-profile-{}", profile.organization_id),
        "tenantId": context.tenant_id,
        "organizationId": profile.organization_id,
        "status": profile.status,
        "driveSpaceId": profile.drive_space_id,
        "driveSpaceType": profile.drive_space_type,
        "openedAt": "2026-06-10T00:00:00Z",
        "settings": {
            "driveSpaceType": profile.drive_space_type
        },
        "version": "1"
    })
}

fn staff_member_to_value(member: &AppbaseOrganizationMember) -> Value {
    json!({
        "membershipId": member.membership_id,
        "userId": member.user_id,
        "displayName": member.membership_id,
        "status": "active",
        "roles": member.roles,
        "positions": member.positions,
        "departments": member.departments,
        "notaryStaffRole": member_notary_staff_role(member).unwrap_or_else(|| "assistant".to_string())
    })
}

fn member_notary_staff_role(member: &AppbaseOrganizationMember) -> Option<String> {
    for role in ["notary", "assistant", "reviewer", "approver"] {
        if member.roles.iter().any(|candidate| candidate == role) {
            return Some(role.to_string());
        }
    }
    if member
        .positions
        .iter()
        .any(|position| position.eq_ignore_ascii_case("notary"))
    {
        return Some("notary".to_string());
    }
    None
}

fn case_summary_to_value(cases: &[NotaryCaseRecord]) -> Value {
    let total_count = cases.len();
    let pending_review_count = cases
        .iter()
        .filter(|record| record.status == NotaryCaseStatus::PendingReview)
        .count();
    let processing_count = cases
        .iter()
        .filter(|record| record.status == NotaryCaseStatus::Processing)
        .count();
    let completed_count = cases
        .iter()
        .filter(|record| record.status == NotaryCaseStatus::Completed)
        .count();
    let rejected_count = cases
        .iter()
        .filter(|record| {
            matches!(
                record.status,
                NotaryCaseStatus::Rejected
                    | NotaryCaseStatus::Cancelled
                    | NotaryCaseStatus::CreateFailed
            )
        })
        .count();
    let fee_amount_total = cases
        .iter()
        .filter_map(|record| record.fee_amount.parse::<f64>().ok())
        .sum::<f64>();

    json!({
        "totalCount": total_count,
        "pendingReviewCount": pending_review_count,
        "processingCount": processing_count,
        "completedCount": completed_count,
        "rejectedCount": rejected_count,
        "feeAmountTotal": format!("{fee_amount_total:.2}")
    })
}

fn notary_statistics_to_value(cases: &[NotaryCaseRecord]) -> Value {
    let pending_review_count = cases
        .iter()
        .filter(|record| record.status == NotaryCaseStatus::PendingReview)
        .count();
    let completed_count = cases
        .iter()
        .filter(|record| record.status == NotaryCaseStatus::Completed)
        .count();
    let anomaly_count = cases
        .iter()
        .filter(|record| {
            matches!(
                record.status,
                NotaryCaseStatus::Rejected
                    | NotaryCaseStatus::Cancelled
                    | NotaryCaseStatus::CreateFailed
            )
        })
        .count();

    json!({
        "pendingReviewQueue": {
            "count": pending_review_count,
            "estimatedProcessHours": pending_review_count as f64 * 2.0
        },
        "todayCompleted": {
            "count": completed_count,
            "comparedToYesterday": 0
        },
        "anomalyIntercepted": {
            "count": anomaly_count,
            "interceptorType": "notary-risk-control"
        },
        "monthlyPreservationTotal": {
            "count": cases.len(),
            "blockchainSyncStatus": "OK"
        },
        "timestamp": "2026-06-10T00:00:00Z"
    })
}

fn monthly_report_to_value(cases: &[NotaryCaseRecord], body: &Value) -> Value {
    let month = string_field(body, &["month"]).unwrap_or_else(|| "2026-06".to_string());
    let format = string_field(body, &["format"]).unwrap_or_else(|| "pdf".to_string());
    let report_id = format!("notary-monthly-{month}-{format}");

    json!({
        "downloadUrl": format!("sdkwork://notary/reports/{report_id}.{format}"),
        "reportId": report_id,
        "month": month,
        "format": format,
        "generatedAt": "2026-06-10T00:00:00Z",
        "expiresAt": "2026-06-17T00:00:00Z",
        "fileSize": 0,
        "caseCount": cases.len()
    })
}

async fn require_notary_member(
    appbase: &mut dyn crate::AppbasePort,
    organization_id: &str,
    membership_id: &str,
    require_admin: bool,
) -> Result<AppbaseOrganizationMember, NotaryServiceError> {
    let member = appbase
        .get_organization_member(organization_id, membership_id)
        .await?
        .ok_or_else(|| NotaryServiceError::unauthorized("organization member is required"))?;

    if !member.enterprise_verified || !member.notary_enabled {
        return Err(NotaryServiceError::unauthorized(
            "organization member is not enabled for notary business",
        ));
    }

    if require_admin
        && !member
            .roles
            .iter()
            .any(|role| role == "notary_admin" || role == "owner")
    {
        return Err(NotaryServiceError::unauthorized(
            "notary business opening requires a notary admin role",
        ));
    }

    let has_notary_staff_role = member.roles.iter().any(|role| {
        matches!(
            role.as_str(),
            "notary" | "notary_admin" | "assistant" | "reviewer" | "approver"
        )
    });
    let has_notary_position = member
        .positions
        .iter()
        .any(|position| position.contains("公证") || position.eq_ignore_ascii_case("notary"));

    if !require_admin && !has_notary_staff_role && !has_notary_position {
        return Err(NotaryServiceError::unauthorized(
            "case assignment requires notary staff role or position",
        ));
    }

    Ok(member)
}

fn validate_context(context: &NotaryRuntimeContext) -> Result<(), NotaryServiceError> {
    if context.tenant_id.trim().is_empty() {
        return Err(NotaryServiceError::unauthenticated("tenant_id is required"));
    }
    if context.user_id.trim().is_empty() {
        return Err(NotaryServiceError::unauthenticated("user_id is required"));
    }
    if context.session_id.trim().is_empty() {
        return Err(NotaryServiceError::unauthenticated(
            "session_id is required",
        ));
    }
    if context.app_id.trim().is_empty() {
        return Err(NotaryServiceError::unauthenticated("app_id is required"));
    }

    Ok(())
}

fn validate_create_case_command(command: &NotaryCaseCommand) -> Result<(), NotaryServiceError> {
    if command.organization_id.trim().is_empty() {
        return Err(NotaryServiceError::validation(
            "organization_id is required",
        ));
    }
    if command.sku_id.trim().is_empty() {
        return Err(NotaryServiceError::validation("sku_id is required"));
    }
    if command.title.trim().is_empty() {
        return Err(NotaryServiceError::validation("title is required"));
    }
    if command.applicant_name.trim().is_empty() {
        return Err(NotaryServiceError::validation("applicant_name is required"));
    }
    if command.idempotency_key.trim().is_empty() {
        return Err(NotaryServiceError::validation(
            "idempotency_key is required",
        ));
    }

    Ok(())
}

fn build_case_no(order_item_id: &str) -> String {
    let suffix = order_item_id
        .chars()
        .filter(|value| value.is_ascii_alphanumeric())
        .rev()
        .take(6)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    format!("NT-20260610-{}", suffix.to_uppercase())
}

async fn update_case_from_body(
    case_id: &str,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<NotaryCaseRecord, NotaryServiceError> {
    let status = string_field(body, &["status"]).map(|value| case_status_from_api(&value));
    let status = status.transpose()?;
    let record = ports
        .repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: case_id.to_string(),
            title: string_field(body, &["title"]),
            remarks: string_field(body, &["remarks", "description"]),
            status,
            chain_hash: string_field(body, &["chainHash", "chain_hash"]),
        })
        .await?;
    ports
        .repository
        .append_event(&record.case_id, "notary.case.updated")
        .await?;
    Ok(record)
}

async fn update_case_status(
    case_id: &str,
    status: NotaryCaseStatus,
    event_type: &str,
    chain_hash: Option<String>,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<NotaryCaseRecord, NotaryServiceError> {
    let record = ports
        .repository
        .update_case(NotaryCaseUpdateCommand {
            case_id: case_id.to_string(),
            title: None,
            remarks: None,
            status: Some(status),
            chain_hash,
        })
        .await?;
    ports
        .repository
        .append_event(&record.case_id, event_type)
        .await?;
    Ok(record)
}

fn create_case_command_from_body(
    context: &NotaryRuntimeContext,
    body: &Value,
) -> Result<NotaryCaseCommand, NotaryServiceError> {
    let organization_id = string_field(body, &["organizationId", "organization_id"])
        .or_else(|| context.organization_id.clone())
        .ok_or_else(|| NotaryServiceError::validation("organizationId is required"))?;
    let sku_id = string_field(body, &["skuId", "sku_id"])
        .ok_or_else(|| NotaryServiceError::validation("skuId is required"))?;
    let title = string_field(body, &["title"])
        .ok_or_else(|| NotaryServiceError::validation("title is required"))?;
    let applicant_name = string_field(body, &["applicantName", "applicant_name", "applicant"])
        .ok_or_else(|| NotaryServiceError::validation("applicantName is required"))?;
    let idempotency_key = string_field(body, &["idempotencyKey", "idempotency_key"])
        .unwrap_or_else(|| {
            format!(
                "notary-case:{}:{}:{}",
                context.user_id, organization_id, sku_id
            )
        });

    Ok(NotaryCaseCommand {
        organization_id,
        sku_id,
        title,
        drive_folder_name: string_field(body, &["driveFolderName", "drive_folder_name"]),
        applicant_name,
        remarks: string_field(body, &["remarks", "description"]),
        primary_notary_membership_id: string_field(
            body,
            &["primaryNotaryMembershipId", "primary_notary_membership_id"],
        ),
        idempotency_key,
        parties: party_commands_from_body(body)?,
    })
}

fn party_commands_from_body(
    body: &Value,
) -> Result<Vec<sdkwork_notary_core::NotaryPartyCommand>, NotaryServiceError> {
    let Some(parties) = body.get("parties") else {
        return Ok(Vec::new());
    };
    let parties = parties
        .as_array()
        .ok_or_else(|| NotaryServiceError::validation("parties must be an array"))?;

    parties.iter().map(party_command_from_value).collect()
}

fn party_command_from_value(
    party: &Value,
) -> Result<sdkwork_notary_core::NotaryPartyCommand, NotaryServiceError> {
    let name = string_field(party, &["name"])
        .ok_or_else(|| NotaryServiceError::validation("party.name is required"))?;
    let party_role = string_field(party, &["role", "partyRole", "party_role"])
        .ok_or_else(|| NotaryServiceError::validation("party.role is required"))?;
    let identity_no = string_field(party, &["identityNo", "identity_no", "identityId"])
        .ok_or_else(|| NotaryServiceError::validation("party.identityNo is required"))?;
    Ok(sdkwork_notary_core::NotaryPartyCommand {
        name,
        party_role,
        identity_no,
        phone: string_field(party, &["phone"]),
    })
}

fn party_update_command_from_body(
    case_id: &str,
    party_id: &str,
    body: &Value,
) -> NotaryPartyUpdateCommand {
    NotaryPartyUpdateCommand {
        case_id: case_id.to_string(),
        party_id: party_id.to_string(),
        name: string_field(body, &["name"]),
        party_role: string_field(body, &["role", "partyRole", "party_role"]),
        identity_no: string_field(body, &["identityNo", "identity_no", "identityId"]),
        phone: string_field(body, &["phone"]),
        signature_node_id: string_field(body, &["signatureNodeId", "signature_node_id"]),
    }
}

fn matter_command_from_body(
    context: &NotaryRuntimeContext,
    body: &Value,
) -> Result<CommerceMatterCommand, NotaryServiceError> {
    let title = string_field(body, &["title"])
        .ok_or_else(|| NotaryServiceError::validation("title is required"))?;
    let price_amount = string_field(body, &["priceAmount", "price_amount"])
        .ok_or_else(|| NotaryServiceError::validation("priceAmount is required"))?;
    let currency_code = string_field(body, &["currencyCode", "currency_code"])
        .ok_or_else(|| NotaryServiceError::validation("currencyCode is required"))?;
    let status = string_field(body, &["status"]).unwrap_or_else(|| "active".to_string());
    validate_matter_status(&status)?;

    Ok(CommerceMatterCommand {
        organization_id: string_field(body, &["organizationId", "organization_id"])
            .or_else(|| context.organization_id.clone()),
        title,
        description: string_field(body, &["description"]),
        price_amount,
        original_price_amount: string_field(
            body,
            &["originalPriceAmount", "original_price_amount"],
        ),
        currency_code,
        status,
        spec: body.get("spec").cloned().unwrap_or_else(|| json!({})),
        idempotency_key: string_field(body, &["idempotencyKey", "idempotency_key"])
            .unwrap_or_else(|| format!("notary-matter:{}:{}", context.user_id, uuid_seed(body))),
    })
}

fn matter_update_command_from_body(sku_id: &str, body: &Value) -> CommerceMatterUpdateCommand {
    CommerceMatterUpdateCommand {
        sku_id: sku_id.to_string(),
        title: string_field(body, &["title"]),
        description: string_field(body, &["description"]),
        price_amount: string_field(body, &["priceAmount", "price_amount"]),
        original_price_amount: string_field(
            body,
            &["originalPriceAmount", "original_price_amount"],
        ),
        currency_code: string_field(body, &["currencyCode", "currency_code"]),
        status: string_field(body, &["status"]),
        spec: body.get("spec").cloned(),
    }
}

fn validate_profile_status(status: &str) -> Result<(), NotaryServiceError> {
    if matches!(status, "active" | "suspended" | "closed") {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary organization profile status: {status}"
        )))
    }
}

fn validate_matter_status(status: &str) -> Result<(), NotaryServiceError> {
    if matches!(status, "draft" | "active" | "inactive") {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary matter status: {status}"
        )))
    }
}

fn validate_assignment_role(role: &str) -> Result<(), NotaryServiceError> {
    if matches!(
        role,
        "primary_notary" | "assistant" | "reviewer" | "approver"
    ) {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary assignment role: {role}"
        )))
    }
}

fn validate_video_invite_purpose(purpose: &str) -> Result<(), NotaryServiceError> {
    if matches!(
        purpose,
        "identity_verification" | "material_confirmation" | "remote_inquiry"
    ) {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary video invite purpose: {purpose}"
        )))
    }
}

fn validate_signature_invite_purpose(purpose: &str) -> Result<(), NotaryServiceError> {
    if matches!(
        purpose,
        "remote_signature" | "onsite_signature_confirmation" | "material_signature"
    ) {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary signature invite purpose: {purpose}"
        )))
    }
}

fn uuid_seed(value: &Value) -> String {
    let seed = value.to_string();
    let mut hash = 0xcbf29ce484222325u64;
    for byte in seed.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn slug_segment(value: &str) -> String {
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

fn url_component(value: &str) -> String {
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

fn case_record_to_value(record: &NotaryCaseRecord) -> Value {
    json!({
        "id": record.case_id,
        "caseId": record.case_id,
        "caseNo": record.case_no,
        "createTime": record.created_at,
        "createdAt": record.created_at,
        "processTime": record.updated_at,
        "updatedAt": record.updated_at,
        "applicant": record.applicant_name,
        "applicantName": record.applicant_name,
        "title": record.title,
        "notary": record.primary_notary_name.clone().unwrap_or_else(|| "Unassigned".to_string()),
        "primaryNotaryMembershipId": record.primary_notary_membership_id,
        "primaryNotaryName": record.primary_notary_name,
        "remarks": record.remarks.clone().unwrap_or_default(),
        "type": record.matter_title,
        "matterTitle": record.matter_title,
        "status": record.status.as_frontend_value(),
        "fee": record.fee_amount,
        "feeAmount": record.fee_amount,
        "currencyCode": record.currency_code,
        "hash": record.chain_hash.clone().unwrap_or_default(),
        "chainHash": record.chain_hash,
        "orderId": record.order_id,
        "orderItemId": record.order_item_id,
        "skuId": record.sku_id,
        "driveSpaceId": record.drive_space_id,
        "driveSpaceType": record.drive_space_type,
        "driveFolderNodeId": record.drive_folder_node_id,
        "parties": [],
        "documents": [],
        "timeline": []
    })
}

async fn case_detail_to_value(
    record: &NotaryCaseRecord,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    let parties = ports.repository.list_parties(&record.case_id).await?;
    let events = ports
        .repository
        .list_events(NotaryCaseEventListQuery {
            case_id: record.case_id.clone(),
            page_size: 50,
            cursor: None,
        })
        .await?;
    let files = ports
        .drive
        .list_nodes(DriveListNodesQuery {
            space_id: record.drive_space_id.clone(),
            space_type: record.drive_space_type.clone(),
            parent_node_id: record.drive_folder_node_id.clone(),
            category: None,
            page_size: 50,
            cursor: None,
        })
        .await?;
    let mut value = case_record_to_value(record);
    if let Some(object) = value.as_object_mut() {
        object.insert(
            "parties".to_string(),
            Value::Array(parties.iter().map(party_record_to_value).collect()),
        );
        object.insert(
            "documents".to_string(),
            Value::Array(
                files
                    .iter()
                    .map(|file| drive_node_to_document_value(file, record))
                    .collect(),
            ),
        );
        object.insert(
            "timeline".to_string(),
            Value::Array(events.iter().map(event_record_to_value).collect()),
        );
    }
    Ok(value)
}

fn party_record_to_value(party: &NotaryPartyRecord) -> Value {
    let signature_url = party
        .signature_node_id
        .as_ref()
        .map(|node_id| format!("drive://notary/signatures/{node_id}"));
    json!({
        "id": party.party_id,
        "partyId": party.party_id,
        "caseId": party.case_id,
        "orderId": party.order_id,
        "orderItemId": party.order_item_id,
        "skuId": party.sku_id,
        "name": party.name,
        "role": party.party_role,
        "partyRole": party.party_role,
        "identityId": format!("****{}", party.identity_no_last4),
        "identityNoLast4": party.identity_no_last4,
        "phone": party.phone_masked,
        "phoneMasked": party.phone_masked,
        "verificationStatus": "pending",
        "status": party.status,
        "signatureNodeId": party.signature_node_id,
        "signatureUrl": signature_url
    })
}

fn event_record_to_value(event: &NotaryCaseEventRecord) -> Value {
    json!({
        "id": event.event_id,
        "eventId": event.event_id,
        "caseId": event.case_id,
        "time": event.occurred_at,
        "occurredAt": event.occurred_at,
        "event": event.event_title,
        "eventTitle": event.event_title,
        "eventType": event.event_type,
        "actor": event.actor_user_id.clone().unwrap_or_else(|| "System".to_string()),
        "actorUserId": event.actor_user_id
    })
}

fn drive_node_to_document_value(node: &DriveNodeReference, record: &NotaryCaseRecord) -> Value {
    json!({
        "nodeId": node.node_id,
        "driveNodeId": node.node_id,
        "driveSpaceId": record.drive_space_id,
        "driveSpaceType": record.drive_space_type,
        "parentNodeId": record.drive_folder_node_id,
        "name": node.node_name,
        "nodeName": node.node_name,
        "size": node.size_label,
        "sizeLabel": node.size_label,
        "status": node.status,
        "reviewStatus": node.status,
        "category": node.category
    })
}

fn file_create_to_document_value(
    body: &Value,
    record: &NotaryCaseRecord,
) -> Result<Value, NotaryServiceError> {
    let node_id = string_field(body, &["driveNodeId", "drive_node_id", "nodeId"])
        .ok_or_else(|| NotaryServiceError::validation("driveNodeId is required"))?;
    let category = string_field(body, &["category"]).unwrap_or_else(|| "evidence".to_string());
    let status = string_field(body, &["reviewStatus", "review_status", "status"])
        .unwrap_or_else(|| "pending".to_string());
    let name = string_field(body, &["materialCode", "material_code", "name"])
        .unwrap_or_else(|| node_id.clone());
    Ok(json!({
        "nodeId": node_id,
        "driveNodeId": node_id,
        "driveSpaceId": record.drive_space_id,
        "driveSpaceType": record.drive_space_type,
        "parentNodeId": record.drive_folder_node_id,
        "name": name,
        "nodeName": name,
        "size": "",
        "sizeLabel": "",
        "status": status,
        "reviewStatus": status,
        "category": category,
        "materialCode": string_field(body, &["materialCode", "material_code"]),
        "partyId": string_field(body, &["partyId", "party_id"])
    }))
}

async fn create_party_video_invite(
    case_id: &str,
    party_id: &str,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    let record = ports
        .repository
        .get_case(case_id)
        .await?
        .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
    let parties = ports.repository.list_parties(case_id).await?;
    let party = parties
        .iter()
        .find(|party| party.party_id == party_id && party.status != "removed")
        .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))?;
    let purpose =
        string_field(body, &["purpose"]).unwrap_or_else(|| "identity_verification".to_string());
    validate_video_invite_purpose(&purpose)?;

    let conversation_id = format!(
        "notary-{}-{}-video",
        slug_segment(&record.case_id),
        slug_segment(&party.party_id)
    );
    let invite_id = format!(
        "video-invite-{}-{}",
        slug_segment(&record.case_id),
        slug_segment(&party.party_id)
    );
    let invite_url = format!(
        "sdkwork://notary/video?inviteId={}&caseId={}&partyId={}&conversationId={}",
        url_component(&invite_id),
        url_component(&record.case_id),
        url_component(&party.party_id),
        url_component(&conversation_id)
    );

    ports
        .repository
        .append_event(&record.case_id, "notary.party.video_invite.created")
        .await?;

    Ok(json!({
        "inviteId": invite_id,
        "caseId": record.case_id,
        "partyId": party.party_id,
        "partyName": party.name,
        "purpose": purpose,
        "conversationId": conversation_id,
        "inviteUrl": invite_url,
        "expiresAt": "2026-06-10T00:10:00Z",
        "driveSpaceId": record.drive_space_id,
        "driveSpaceType": record.drive_space_type,
        "driveFolderNodeId": record.drive_folder_node_id
    }))
}

async fn create_party_signature_invite(
    case_id: &str,
    party_id: &str,
    body: &Value,
    ports: &mut NotaryRuntimePorts<'_>,
) -> Result<Value, NotaryServiceError> {
    let record = ports
        .repository
        .get_case(case_id)
        .await?
        .ok_or_else(|| NotaryServiceError::not_found("notary case not found"))?;
    let parties = ports.repository.list_parties(case_id).await?;
    let party = parties
        .iter()
        .find(|party| party.party_id == party_id && party.status != "removed")
        .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))?;
    let purpose =
        string_field(body, &["purpose"]).unwrap_or_else(|| "remote_signature".to_string());
    validate_signature_invite_purpose(&purpose)?;

    let invite_id = format!(
        "signature-invite-{}-{}",
        slug_segment(&record.case_id),
        slug_segment(&party.party_id)
    );
    let invite_url = format!(
        "sdkwork://notary/signature?inviteId={}&caseId={}&partyId={}&driveFolderNodeId={}",
        url_component(&invite_id),
        url_component(&record.case_id),
        url_component(&party.party_id),
        url_component(&record.drive_folder_node_id)
    );

    ports
        .repository
        .append_event(&record.case_id, "notary.party.signature_invite.created")
        .await?;

    Ok(json!({
        "inviteId": invite_id,
        "caseId": record.case_id,
        "partyId": party.party_id,
        "partyName": party.name,
        "purpose": purpose,
        "inviteUrl": invite_url,
        "signingUrl": invite_url,
        "expiresAt": "2026-06-10T00:10:00Z",
        "driveSpaceId": record.drive_space_id,
        "driveSpaceType": record.drive_space_type,
        "driveFolderNodeId": record.drive_folder_node_id
    }))
}

fn path_param<'a>(
    path_params: &'a BTreeMap<String, String>,
    name: &str,
) -> Result<&'a str, NotaryServiceError> {
    path_params
        .get(name)
        .or_else(|| path_params.get(&to_snake_case(name)))
        .map(String::as_str)
        .ok_or_else(|| NotaryServiceError::validation(format!("{name} path parameter is required")))
}

fn string_field(value: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        value
            .get(*name)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn integer_field(value: &Value, names: &[&str]) -> Option<i64> {
    names.iter().find_map(|name| {
        value
            .get(*name)
            .and_then(|value| {
                value
                    .as_i64()
                    .or_else(|| value.as_str()?.parse::<i64>().ok())
            })
            .filter(|value| *value > 0)
    })
}

fn status_to_storage(value: &str) -> String {
    match value {
        "PENDING_REVIEW" => "pending_review".to_string(),
        "PROCESSING" => "processing".to_string(),
        "COMPLETED" => "completed".to_string(),
        "REJECTED" => "rejected".to_string(),
        "CANCELLED" => "cancelled".to_string(),
        "CREATE_FAILED" => "create_failed".to_string(),
        _ => value.to_string(),
    }
}

fn case_status_from_api(value: &str) -> Result<NotaryCaseStatus, NotaryServiceError> {
    let storage_value = status_to_storage(value);
    NotaryCaseStatus::from_storage_value(&storage_value).ok_or_else(|| {
        NotaryServiceError::validation(format!("unsupported notary case status: {value}"))
    })
}

fn to_snake_case(value: &str) -> String {
    let mut result = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index > 0 {
                result.push('_');
            }
            result.push(character.to_ascii_lowercase());
        } else {
            result.push(character);
        }
    }
    result
}

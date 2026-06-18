use async_trait::async_trait;
use sdkwork_notary_core::{
    NotaryCaseCommand, NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand,
    NotaryRuntimeContext, NotaryServiceError,
};
use sdkwork_notary_runtime::{
    create_notary_case, ensure_notary_business_open, handle_notary_app_operation,
    handle_notary_backend_operation, list_case_files, notary_runtime_contract,
    AppbaseOrganizationMember, AppbasePort, CommerceCreateOrderCommand, CommerceMatterCommand,
    CommerceMatterListQuery, CommerceMatterRecord, CommerceMatterUpdateCommand,
    CommerceOrderReference, CommercePort, DriveCreateFolderCommand, DriveCreateSpaceCommand,
    DriveFolderReference, DriveListNodesQuery, DriveNodeReference, DrivePort,
    NotaryCaseAssignmentCommand, NotaryCaseAssignmentRecord, NotaryCaseEventListQuery,
    NotaryCaseEventRecord, NotaryCaseListQuery, NotaryCaseRepositoryPort, NotaryCaseUpdateCommand,
    NotaryOrganizationProfileUpdateCommand, NotaryPartyRecord, NotaryPartyUpdateCommand,
    NotaryRuntimePorts, NOTARY_CASE_REPOSITORY_PORT, NOTARY_COMMERCE_PORT, NOTARY_DRIVE_PORT,
    NOTARY_IAM_PORT,
};
use serde_json::json;
use std::collections::BTreeMap;

#[test]
fn runtime_contract_declares_commerce_drive_iam_and_notary_storage_ports() {
    let contract = notary_runtime_contract();

    assert_eq!(contract.domain, "notary");
    assert_eq!(contract.service_name, "notary.case");
    assert_eq!(
        contract.write_commands,
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
    );
    assert_eq!(
        contract.read_queries,
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
    );
    assert_eq!(
        contract.ports,
        vec![
            NOTARY_IAM_PORT,
            NOTARY_COMMERCE_PORT,
            NOTARY_DRIVE_PORT,
            NOTARY_CASE_REPOSITORY_PORT,
        ],
    );
    assert!(contract.requires_idempotency_for_writes);
}

#[tokio::test]
async fn opening_notary_business_creates_notary_drive_space_before_profile() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-owner".to_string(),
        user_id: "user-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary_admin".to_string()],
        positions: vec!["公证业务负责人".to_string()],
        departments: vec!["公证业务部".to_string()],
    });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default();
    let context = runtime_context();

    let profile = ensure_notary_business_open(
        &context,
        "org-1",
        "member-owner",
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(profile.organization_id, "org-1");
    assert_eq!(profile.drive_space_id, "space-notary-org-1");
    assert_eq!(profile.drive_space_type, "notary");
    assert_eq!(drive.events, vec!["create_space:notary:organization:org-1"],);
    assert_eq!(
        repository.events,
        vec!["upsert_profile:org-1:space-notary-org-1:notary"],
    );
}

#[tokio::test]
async fn creating_case_reuses_sku_order_item_and_creates_notary_drive_folder() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "user-notary-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["公证员".to_string()],
        departments: vec!["公证一部".to_string()],
    });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_profile("org-1", "space-notary-org-1");
    let context = runtime_context();

    let created = create_notary_case(
        &context,
        NotaryCaseCommand {
            organization_id: "org-1".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            drive_folder_name: None,
            title: "电子合同存证办理".to_string(),
            applicant_name: "张三网络科技".to_string(),
            remarks: Some("优先处理".to_string()),
            primary_notary_membership_id: Some("member-notary-1".to_string()),
            idempotency_key: "idem-case-1".to_string(),
            parties: vec![NotaryPartyCommand {
                name: "张三".to_string(),
                party_role: "申请人".to_string(),
                identity_no: "110105199001011234".to_string(),
                phone: Some("13800138000".to_string()),
            }],
        },
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(created.status, NotaryCaseStatus::PendingReview);
    assert_eq!(
        created.primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(
        created.primary_notary_user_id,
        Some("user-notary-1".to_string())
    );
    assert_eq!(
        created.primary_notary_name,
        Some("member-notary-1".to_string())
    );
    assert_eq!(created.order_id, "order-sku-electronic-contract");
    assert_eq!(created.order_item_id, "item-sku-electronic-contract");
    assert_eq!(created.sku_id, "sku-electronic-contract");
    assert_eq!(created.drive_space_id, "space-notary-org-1");
    assert_eq!(created.drive_space_type, "notary");
    assert_eq!(
        created.drive_folder_node_id,
        "folder-order-sku-electronic-contract"
    );
    assert_eq!(
        commerce.events,
        vec!["create_order:sku-electronic-contract:notary:idem-case-1"],
    );
    assert_eq!(
        drive.events,
        vec!["create_folder:notary:space-notary-org-1:电子合同存证办理"],
    );
    assert_eq!(
        repository.events,
        vec![
            "insert_case:order-sku-electronic-contract:item-sku-electronic-contract:sku-electronic-contract:folder-order-sku-electronic-contract",
            "insert_party:张三:order-sku-electronic-contract:sku-electronic-contract",
            "append_event:notary.case.submitted",
        ],
    );
}

#[tokio::test]
async fn creating_case_uses_frontend_drive_folder_name_when_provided() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "user-notary-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_profile("org-1", "space-notary-org-1");

    let _created = create_notary_case(
        &runtime_context(),
        NotaryCaseCommand {
            organization_id: "org-1".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            title: "Electronic contract preservation".to_string(),
            drive_folder_name: Some("NT-20260610-custom-folder".to_string()),
            applicant_name: "Zhang San Network".to_string(),
            remarks: None,
            primary_notary_membership_id: Some("member-notary-1".to_string()),
            idempotency_key: "idem-case-folder-name".to_string(),
            parties: Vec::new(),
        },
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        drive.events,
        vec!["create_folder:notary:space-notary-org-1:NT-20260610-custom-folder"]
    );
}

#[tokio::test]
async fn creating_case_rejects_users_without_notary_business_access() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-sales-1".to_string(),
        user_id: "user-sales-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: false,
        roles: vec!["sales".to_string()],
        positions: vec!["销售".to_string()],
        departments: vec!["商务部".to_string()],
    });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_profile("org-1", "space-notary-org-1");
    let context = runtime_context();

    let error = create_notary_case(
        &context,
        NotaryCaseCommand {
            organization_id: "org-1".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            drive_folder_name: None,
            title: "电子合同存证办理".to_string(),
            applicant_name: "张三网络科技".to_string(),
            remarks: None,
            primary_notary_membership_id: Some("member-sales-1".to_string()),
            idempotency_key: "idem-case-1".to_string(),
            parties: Vec::new(),
        },
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code(), "unauthorized");
    assert!(commerce.events.is_empty());
    assert!(drive.events.is_empty());
    assert!(repository.events.is_empty());
}

#[tokio::test]
async fn case_file_listing_uses_denormalized_drive_space_type_without_joining() {
    let mut appbase = RecordingAppbase::default();
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default().with_case(NotaryCaseRecord {
        case_id: "case-1".to_string(),
        case_no: "NT-20260610-001".to_string(),
        organization_id: "org-1".to_string(),
        title: "电子合同存证办理".to_string(),
        applicant_name: "张三网络科技".to_string(),
        primary_notary_name: Some("李明".to_string()),
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        primary_notary_user_id: Some("user-notary-1".to_string()),
        status: NotaryCaseStatus::Processing,
        order_id: "order-1".to_string(),
        order_item_id: "item-1".to_string(),
        sku_id: "sku-electronic-contract".to_string(),
        matter_title: "电子合同存证".to_string(),
        fee_amount: "500.00".to_string(),
        currency_code: "CNY".to_string(),
        drive_space_id: "space-notary-org-1".to_string(),
        drive_space_type: "notary".to_string(),
        drive_folder_node_id: "folder-case-1".to_string(),
        chain_hash: None,
        remarks: None,
        request_no: "REQ-20260610-001".to_string(),
        idempotency_key: "idem-case-1".to_string(),
        created_at: "2026-06-10 10:00".to_string(),
        updated_at: "2026-06-10 10:00".to_string(),
    });

    let files = list_case_files(
        &runtime_context(),
        "case-1",
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].node_id, "node-folder-case-1");
    assert_eq!(
        drive.events,
        vec!["list_nodes:notary:space-notary-org-1:folder-case-1::50:"],
    );
}

#[tokio::test]
async fn app_operation_dispatcher_creates_case_and_lists_drive_files() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "user-notary-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_profile("org-1", "space-notary-org-1");
    let context = runtime_context();

    let created = handle_notary_app_operation(
        &context,
        "notary.cases.create",
        BTreeMap::new(),
        json!({
            "organizationId": "org-1",
            "skuId": "sku-electronic-contract",
            "title": "Electronic contract preservation",
            "applicantName": "Zhang San Network",
            "primaryNotaryMembershipId": "member-notary-1",
            "idempotencyKey": "idem-route-case-1",
            "parties": [
                {
                    "name": "Zhang San",
                    "role": "applicant",
                    "identityNo": "110105199001011234",
                    "phone": "13800138000"
                }
            ]
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(created["orderId"], "order-sku-electronic-contract");
    assert_eq!(created["orderItemId"], "item-sku-electronic-contract");
    assert_eq!(created["skuId"], "sku-electronic-contract");
    assert_eq!(created["primaryNotaryMembershipId"], "member-notary-1");
    assert_eq!(created["driveSpaceType"], "notary");
    let case_id = created["id"].as_str().unwrap().to_string();

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), case_id);
    let files = handle_notary_app_operation(
        &context,
        "notary.cases.files.list",
        path_params,
        serde_json::Value::Null,
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(files["items"][0]["driveSpaceType"], "notary");
    assert_eq!(
        files["items"][0]["parentNodeId"],
        "folder-order-sku-electronic-contract"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_returns_frontend_case_details_from_notary_and_drive() {
    let case = sample_case_record("case-1");
    let mut appbase = RecordingAppbase::default();
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default()
        .with_case(case)
        .with_party(NotaryPartyRecord {
            party_id: "party-1".to_string(),
            case_id: "case-1".to_string(),
            order_id: "order-1".to_string(),
            order_item_id: "item-1".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            name: "Zhang San".to_string(),
            party_role: "applicant".to_string(),
            identity_no_last4: "1234".to_string(),
            phone_masked: Some("138****8000".to_string()),
            status: "active".to_string(),
            signature_node_id: Some("signature-node-1".to_string()),
        })
        .with_event(NotaryCaseEventRecord {
            event_id: "event-1".to_string(),
            case_id: "case-1".to_string(),
            event_type: "notary.case.submitted".to_string(),
            event_title: "Case submitted".to_string(),
            actor_user_id: Some("user-1".to_string()),
            occurred_at: "2026-06-10 10:00".to_string(),
        });

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let detail = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.retrieve",
        path_params,
        serde_json::Value::Null,
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(detail["id"], "case-1");
    assert_eq!(detail["parties"][0]["id"], "party-1");
    assert_eq!(detail["parties"][0]["identityId"], "****1234");
    assert_eq!(detail["parties"][0]["signatureNodeId"], "signature-node-1");
    assert_eq!(
        detail["parties"][0]["signatureUrl"],
        "drive://notary/signatures/signature-node-1"
    );
    assert_eq!(detail["documents"][0]["driveSpaceType"], "notary");
    assert_eq!(detail["timeline"][0]["event"], "Case submitted");

    let listed = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "status": "PROCESSING", "q": "contract"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(listed["items"][0]["id"], "case-1");
    assert_eq!(
        repository.events.last().unwrap(),
        "list_cases:org-1:processing::contract:50:"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_forwards_openapi_filters_to_repository_and_drive() {
    let case = sample_case_record("case-1");
    let mut other_case = sample_case_record("case-2");
    other_case.sku_id = "sku-other-notary-matter".to_string();
    other_case.title = "Other contract preservation".to_string();
    other_case.order_id = "order-2".to_string();
    other_case.order_item_id = "item-2".to_string();

    let mut appbase = RecordingAppbase::default();
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default()
        .with_case(case)
        .with_case(other_case)
        .with_event(NotaryCaseEventRecord {
            event_id: "event-1".to_string(),
            case_id: "case-1".to_string(),
            event_type: "notary.case.submitted".to_string(),
            event_title: "Case submitted".to_string(),
            actor_user_id: Some("user-1".to_string()),
            occurred_at: "2026-06-10 10:00".to_string(),
        });

    let listed = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.list",
        BTreeMap::new(),
        json!({
            "organizationId": "org-1",
            "status": "PROCESSING",
            "q": "contract",
            "sku_id": "sku-electronic-contract",
            "page_size": 25,
            "cursor": "case-z"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(listed["items"].as_array().unwrap().len(), 1);
    assert_eq!(listed["items"][0]["skuId"], "sku-electronic-contract");
    assert_eq!(
        repository.events.last().unwrap(),
        "list_cases:org-1:processing:sku-electronic-contract:contract:25:case-z"
    );

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let files = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.files.list",
        path_params.clone(),
        json!({
            "category": "identity",
            "page_size": 25,
            "cursor": "file-cursor"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(files["items"][0]["category"], "identity");
    assert_eq!(
        drive.events.last().unwrap(),
        "list_nodes:notary:space-notary-org-1:folder-case-1:identity:25:file-cursor"
    );

    let _events = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.events.list",
        path_params,
        json!({
            "page_size": 1,
            "cursor": "event-0"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        repository.events.last().unwrap(),
        "list_events:case-1:1:event-0"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_returns_dashboard_statistics_and_monthly_report() {
    let mut completed_case = sample_case_record("case-2");
    completed_case.status = NotaryCaseStatus::Completed;
    completed_case.case_no = "NT-20260610-002".to_string();
    completed_case.title = "Completed evidence preservation".to_string();

    let mut rejected_case = sample_case_record("case-3");
    rejected_case.status = NotaryCaseStatus::Rejected;
    rejected_case.case_no = "NT-20260610-003".to_string();
    rejected_case.title = "Rejected evidence preservation".to_string();

    let mut appbase = RecordingAppbase::default();
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default()
        .with_case(sample_case_record("case-1"))
        .with_case(completed_case)
        .with_case(rejected_case);

    let statistics = handle_notary_app_operation(
        &runtime_context(),
        "notary.dashboard.statistics.retrieve",
        BTreeMap::new(),
        serde_json::Value::Null,
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(statistics["pendingReviewQueue"]["count"], 0);
    assert_eq!(statistics["todayCompleted"]["count"], 1);
    assert_eq!(statistics["anomalyIntercepted"]["count"], 1);
    assert_eq!(statistics["monthlyPreservationTotal"]["count"], 3);
    assert_eq!(
        statistics["monthlyPreservationTotal"]["blockchainSyncStatus"],
        "OK"
    );
    assert_eq!(repository.events[0], "list_cases:org-1::::500:");

    let report = handle_notary_app_operation(
        &runtime_context(),
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "month": "2026-06", "format": "csv"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(report["reportId"], "notary-monthly-2026-06-csv");
    assert_eq!(report["month"], "2026-06");
    assert_eq!(report["format"], "csv");
    assert_eq!(report["caseCount"], 3);
    assert_eq!(
        report["downloadUrl"],
        "sdkwork://notary/reports/notary-monthly-2026-06-csv.csv"
    );
    assert_eq!(repository.events[1], "list_cases:org-1::::500:");
}

#[tokio::test]
async fn app_operation_dispatcher_mutates_case_party_and_drive_file_workflows() {
    let mut appbase = RecordingAppbase::default();
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let accepted = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        path_params.clone(),
        json!({"remarks": "materials accepted"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(accepted["status"], "PROCESSING");
    assert!(repository
        .events
        .contains(&"append_event:notary.case.accepted".to_string()));

    let with_party = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.create",
        path_params.clone(),
        json!({
            "name": "Li Si",
            "role": "counterparty",
            "identityNo": "110105199202021234",
            "phone": "13900139000"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(with_party["parties"][0]["name"], "Li Si");
    assert!(repository
        .events
        .contains(&"append_event:notary.party.created".to_string()));

    let party_id = with_party["parties"][0]["id"].as_str().unwrap().to_string();
    let mut invite_params = path_params.clone();
    invite_params.insert("partyId".to_string(), party_id.clone());
    let invite = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.videoInvites.create",
        invite_params,
        json!({"purpose": "identity_verification"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(invite["caseId"], "case-1");
    assert_eq!(invite["partyId"], party_id);
    assert_eq!(
        invite["conversationId"],
        "notary-case-1-party-case-1-1-video"
    );
    assert_eq!(invite["driveSpaceType"], "notary");
    let invite_url = invite["inviteUrl"].as_str().unwrap();
    assert!(invite_url.contains("conversationId=notary-case-1-party-case-1-1-video"));
    assert!(invite_url.contains("caseId=case-1"));
    assert!(invite_url.contains("partyId=party-case-1-1"));
    assert_eq!(
        repository.events.last().unwrap(),
        "append_event:notary.party.video_invite.created"
    );

    let mut signature_invite_params = path_params.clone();
    signature_invite_params.insert("partyId".to_string(), party_id.clone());
    let signature_invite = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.signatureInvites.create",
        signature_invite_params,
        json!({"purpose": "remote_signature"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(signature_invite["caseId"], "case-1");
    assert_eq!(signature_invite["partyId"], party_id);
    assert_eq!(signature_invite["driveSpaceType"], "notary");
    assert_eq!(signature_invite["driveFolderNodeId"], "folder-case-1");
    let signature_url = signature_invite["inviteUrl"].as_str().unwrap();
    assert!(signature_url.contains("inviteId=signature-invite-case-1-party-case-1-1"));
    assert!(signature_url.contains("caseId=case-1"));
    assert!(signature_url.contains("partyId=party-case-1-1"));
    assert_eq!(
        signature_invite["signingUrl"],
        signature_invite["inviteUrl"]
    );
    assert_eq!(
        repository.events.last().unwrap(),
        "append_event:notary.party.signature_invite.created"
    );

    let file = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.files.create",
        path_params.clone(),
        json!({
            "driveNodeId": "drive-node-1",
            "category": "evidence",
            "materialCode": "contract.pdf",
            "reviewStatus": "pending"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(file["nodeId"], "drive-node-1");
    assert_eq!(file["driveSpaceType"], "notary");
    assert_eq!(file["parentNodeId"], "folder-case-1");

    let package = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.downloadPackages.create",
        path_params,
        json!({"packageName": "case-1.zip"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(package["caseId"], "case-1");
    assert_eq!(package["driveSpaceType"], "notary");
    assert_eq!(package["status"], "preparing");
}

#[tokio::test]
async fn backend_operation_dispatcher_opens_business_lists_staff_cases_and_summary() {
    let mut completed_case = sample_case_record("case-2");
    completed_case.status = NotaryCaseStatus::Completed;
    completed_case.case_no = "NT-20260610-002".to_string();
    completed_case.title = "Completed evidence preservation".to_string();

    let mut appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-owner".to_string(),
            user_id: "user-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary_admin".to_string()],
            positions: vec!["notary director".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "user-notary-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default()
        .with_case(sample_case_record("case-1"))
        .with_case(completed_case);

    let opened = handle_notary_backend_operation(
        &runtime_context(),
        "notary.organizationProfiles.create",
        BTreeMap::new(),
        json!({
            "organizationId": "org-1",
            "openedByMembershipId": "member-owner"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(opened["driveSpaceType"], "notary");
    assert_eq!(opened["organizationId"], "org-1");

    let profiles = handle_notary_backend_operation(
        &runtime_context(),
        "notary.organizationProfiles.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "pageSize": 20}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(profiles["items"][0]["organizationId"], "org-1");
    assert!(repository
        .events
        .contains(&"list_profiles:org-1:20".to_string()));

    let staff = handle_notary_backend_operation(
        &runtime_context(),
        "notary.staff.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "staffRole": "notary"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(staff["items"][0]["membershipId"], "member-notary-1");
    assert_eq!(staff["items"][0]["notaryStaffRole"], "notary");

    let cases = handle_notary_backend_operation(
        &runtime_context(),
        "notary.cases.management.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "pageSize": 20}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(cases["items"].as_array().unwrap().len(), 2);

    let summary = handle_notary_backend_operation(
        &runtime_context(),
        "notary.reports.caseSummary.retrieve",
        BTreeMap::new(),
        json!({"organizationId": "org-1"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(summary["totalCount"], 2);
    assert_eq!(summary["processingCount"], 1);
    assert_eq!(summary["completedCount"], 1);
}

#[tokio::test]
async fn app_operation_dispatcher_returns_access_and_sku_backed_matters() {
    let mut appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "user-1".to_string(),
        organization_id: "org-1".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let mut commerce = RecordingCommerce::default().with_matter(sample_matter_record(
        "sku-electronic-contract",
        "Electronic contract preservation",
    ));
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_profile("org-1", "space-notary-org-1");

    let access = handle_notary_app_operation(
        &runtime_context(),
        "notary.access.retrieve",
        BTreeMap::new(),
        serde_json::Value::Null,
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(access["visible"], true);
    assert_eq!(access["organizationVerified"], true);
    assert_eq!(access["notaryBusinessEnabled"], true);
    assert_eq!(access["memberId"], "member-notary-1");
    assert!(access["permissions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|permission| permission == "notary.cases.create"));

    let matters = handle_notary_app_operation(
        &runtime_context(),
        "notary.matters.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "q": "contract", "pageSize": 20}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(matters["items"][0]["skuId"], "sku-electronic-contract");
    assert_eq!(matters["items"][0]["spuId"], "spu-sku-electronic-contract");
    assert_eq!(matters["items"][0]["priceAmount"], "500.00");
    assert_eq!(
        commerce.events.last().unwrap(),
        "list_matters:org-1:contract::20"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_lists_notary_staff_and_assigns_selected_member() {
    let mut appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "user-notary-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-assistant-1".to_string(),
            user_id: "user-assistant-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["assistant".to_string()],
            positions: vec!["assistant".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-sales-1".to_string(),
            user_id: "user-sales-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: false,
            roles: vec!["sales".to_string()],
            positions: vec!["sales".to_string()],
            departments: vec!["sales".to_string()],
        });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));

    let staff = handle_notary_app_operation(
        &runtime_context(),
        "notary.staff.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "staffRole": "notary"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(staff["items"].as_array().unwrap().len(), 1);
    assert_eq!(staff["items"][0]["membershipId"], "member-notary-1");
    assert_eq!(staff["items"][0]["notaryStaffRole"], "notary");

    let mut assignment_path = BTreeMap::new();
    assignment_path.insert("caseId".to_string(), "case-1".to_string());
    let assignment = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.assignments.create",
        assignment_path,
        json!({
            "organizationMembershipId": "member-notary-1",
            "assignmentRole": "primary_notary"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(assignment["caseId"], "case-1");
    assert_eq!(assignment["organizationMembershipId"], "member-notary-1");
    assert_eq!(assignment["userId"], "user-notary-1");
    assert_eq!(assignment["assignmentRole"], "primary_notary");
    assert_eq!(
        repository.events,
        vec![
            "insert_assignment:assignment-case-1-member-notary-1-primary_notary",
            "append_event:notary.case.assignment_created",
        ],
    );
}

#[tokio::test]
async fn backend_operation_dispatcher_manages_profile_matters_and_assignments() {
    let mut appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-owner".to_string(),
            user_id: "user-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary_admin".to_string()],
            positions: vec!["notary director".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "user-notary-1".to_string(),
            organization_id: "org-1".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        });
    let mut commerce = RecordingCommerce::default();
    let mut drive = RecordingDrive::default();
    let mut repository = RecordingNotaryCaseRepository::default()
        .with_profile("org-1", "space-notary-org-1")
        .with_case(sample_case_record("case-1"));

    let mut profile_path = BTreeMap::new();
    profile_path.insert("organizationProfileId".to_string(), "org-1".to_string());
    let updated_profile = handle_notary_backend_operation(
        &runtime_context(),
        "notary.organizationProfiles.update",
        profile_path,
        json!({"status": "suspended", "settings": {"reviewMode": "manual"}}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(updated_profile["organizationId"], "org-1");
    assert_eq!(updated_profile["status"], "suspended");

    let created_matter = handle_notary_backend_operation(
        &runtime_context(),
        "notary.matters.create",
        BTreeMap::new(),
        json!({
            "organizationId": "org-1",
            "title": "Electronic evidence preservation",
            "description": "Preserve electronic evidence",
            "priceAmount": "600.00",
            "originalPriceAmount": "800.00",
            "currencyCode": "CNY",
            "status": "active",
            "spec": {"materialCodes": ["identity", "evidence"]}
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        created_matter["skuId"],
        "sku-electronic-evidence-preservation"
    );
    assert_eq!(
        created_matter["spuId"],
        "spu-sku-electronic-evidence-preservation"
    );
    assert_eq!(created_matter["priceAmount"], "600.00");

    let matters = handle_notary_backend_operation(
        &runtime_context(),
        "notary.matters.management.list",
        BTreeMap::new(),
        json!({"organizationId": "org-1", "q": "evidence", "pageSize": 10}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        matters["items"][0]["skuId"],
        "sku-electronic-evidence-preservation"
    );

    let mut matter_path = BTreeMap::new();
    matter_path.insert(
        "skuId".to_string(),
        "sku-electronic-evidence-preservation".to_string(),
    );
    let updated_matter = handle_notary_backend_operation(
        &runtime_context(),
        "notary.matters.update",
        matter_path,
        json!({"title": "Updated evidence preservation", "status": "inactive"}),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(updated_matter["title"], "Updated evidence preservation");
    assert_eq!(updated_matter["status"], "inactive");

    let mut assignment_path = BTreeMap::new();
    assignment_path.insert("caseId".to_string(), "case-1".to_string());
    let assignment = handle_notary_backend_operation(
        &runtime_context(),
        "notary.cases.assignments.create",
        assignment_path.clone(),
        json!({
            "organizationMembershipId": "member-notary-1",
            "assignmentRole": "primary_notary"
        }),
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(assignment["caseId"], "case-1");
    assert_eq!(assignment["organizationMembershipId"], "member-notary-1");
    assert_eq!(assignment["assignmentRole"], "primary_notary");
    assert_eq!(
        repository.events.last().unwrap(),
        "append_event:notary.case.assignment_created"
    );

    assignment_path.insert(
        "assignmentId".to_string(),
        assignment["id"].as_str().unwrap().to_string(),
    );
    let released = handle_notary_backend_operation(
        &runtime_context(),
        "notary.cases.assignments.delete",
        assignment_path,
        serde_json::Value::Null,
        &mut NotaryRuntimePorts {
            appbase: &mut appbase,
            commerce: &mut commerce,
            drive: &mut drive,
            repository: &mut repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(released["released"], true);
    assert_eq!(
        repository.events.last().unwrap(),
        "append_event:notary.case.assignment_released"
    );
}

#[derive(Default)]
struct RecordingAppbase {
    members: Vec<AppbaseOrganizationMember>,
}

impl RecordingAppbase {
    fn with_member(mut self, member: AppbaseOrganizationMember) -> Self {
        self.members.push(member);
        self
    }
}

#[async_trait]
impl AppbasePort for RecordingAppbase {
    async fn get_organization_member(
        &mut self,
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
        &mut self,
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
struct RecordingCommerce {
    events: Vec<String>,
    matters: Vec<CommerceMatterRecord>,
}

impl RecordingCommerce {
    fn with_matter(mut self, record: CommerceMatterRecord) -> Self {
        self.matters.push(record);
        self
    }
}

#[async_trait]
impl CommercePort for RecordingCommerce {
    async fn create_notary_order(
        &mut self,
        command: CommerceCreateOrderCommand,
    ) -> Result<CommerceOrderReference, NotaryServiceError> {
        self.events.push(format!(
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

    async fn list_notary_matters(
        &mut self,
        query: CommerceMatterListQuery,
    ) -> Result<Vec<CommerceMatterRecord>, NotaryServiceError> {
        self.events.push(format!(
            "list_matters:{}:{}:{}:{}",
            query.organization_id.clone().unwrap_or_default(),
            query.search_term.clone().unwrap_or_default(),
            query.status.clone().unwrap_or_default(),
            query.page_size
        ));
        Ok(self
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
        &mut self,
        command: CommerceMatterCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let sku_id = format!("sku-{}", slug(&command.title));
        self.events.push(format!(
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
        self.matters.push(record.clone());
        Ok(record)
    }

    async fn update_notary_matter(
        &mut self,
        command: CommerceMatterUpdateCommand,
    ) -> Result<CommerceMatterRecord, NotaryServiceError> {
        let record = self
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
        self.events
            .push(format!("update_matter:{}", command.sku_id));
        Ok(record.clone())
    }
}

#[derive(Default)]
struct RecordingDrive {
    events: Vec<String>,
}

#[async_trait]
impl DrivePort for RecordingDrive {
    async fn create_notary_space(
        &mut self,
        command: DriveCreateSpaceCommand,
    ) -> Result<String, NotaryServiceError> {
        self.events.push(format!(
            "create_space:{}:{}:{}",
            command.space_type, command.owner_subject_type, command.owner_subject_id
        ));
        Ok(format!("space-notary-{}", command.owner_subject_id))
    }

    async fn create_case_folder(
        &mut self,
        command: DriveCreateFolderCommand,
    ) -> Result<DriveFolderReference, NotaryServiceError> {
        self.events.push(format!(
            "create_folder:{}:{}:{}",
            command.space_type, command.space_id, command.folder_name
        ));
        Ok(DriveFolderReference {
            folder_node_id: format!("folder-{}", command.order_id),
            space_id: command.space_id,
            space_type: command.space_type,
        })
    }

    async fn list_nodes(
        &mut self,
        query: DriveListNodesQuery,
    ) -> Result<Vec<DriveNodeReference>, NotaryServiceError> {
        self.events.push(format!(
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
}

#[derive(Default)]
struct RecordingNotaryCaseRepository {
    events: Vec<String>,
    profiles: Vec<(String, String, String)>,
    cases: Vec<NotaryCaseRecord>,
    parties: Vec<NotaryPartyRecord>,
    case_events: Vec<NotaryCaseEventRecord>,
    assignments: Vec<NotaryCaseAssignmentRecord>,
}

impl RecordingNotaryCaseRepository {
    fn with_profile(mut self, organization_id: &str, drive_space_id: &str) -> Self {
        self.profiles.push((
            organization_id.to_string(),
            drive_space_id.to_string(),
            "active".to_string(),
        ));
        self
    }

    fn with_case(mut self, record: NotaryCaseRecord) -> Self {
        self.cases.push(record);
        self
    }

    fn with_party(mut self, record: NotaryPartyRecord) -> Self {
        self.parties.push(record);
        self
    }

    fn with_event(mut self, record: NotaryCaseEventRecord) -> Self {
        self.case_events.push(record);
        self
    }
}

#[async_trait]
impl NotaryCaseRepositoryPort for RecordingNotaryCaseRepository {
    async fn upsert_organization_profile(
        &mut self,
        organization_id: &str,
        drive_space_id: &str,
        drive_space_type: &str,
    ) -> Result<sdkwork_notary_runtime::NotaryOrganizationProfile, NotaryServiceError> {
        self.events.push(format!(
            "upsert_profile:{organization_id}:{drive_space_id}:{drive_space_type}"
        ));
        if let Some(profile) = self
            .profiles
            .iter_mut()
            .find(|(profile_org_id, _, _)| profile_org_id == organization_id)
        {
            profile.1 = drive_space_id.to_string();
            profile.2 = "active".to_string();
        } else {
            self.profiles.push((
                organization_id.to_string(),
                drive_space_id.to_string(),
                "active".to_string(),
            ));
        }
        Ok(sdkwork_notary_runtime::NotaryOrganizationProfile {
            organization_id: organization_id.to_string(),
            drive_space_id: drive_space_id.to_string(),
            drive_space_type: drive_space_type.to_string(),
            status: "active".to_string(),
        })
    }

    async fn get_organization_profile(
        &mut self,
        organization_id: &str,
    ) -> Result<Option<sdkwork_notary_runtime::NotaryOrganizationProfile>, NotaryServiceError> {
        Ok(self
            .profiles
            .iter()
            .find(|(profile_org_id, _, _)| profile_org_id == organization_id)
            .map(|(profile_org_id, drive_space_id, status)| {
                sdkwork_notary_runtime::NotaryOrganizationProfile {
                    organization_id: profile_org_id.clone(),
                    drive_space_id: drive_space_id.clone(),
                    drive_space_type: "notary".to_string(),
                    status: status.clone(),
                }
            }))
    }

    async fn list_organization_profiles(
        &mut self,
        organization_id: Option<&str>,
        page_size: i64,
    ) -> Result<Vec<sdkwork_notary_runtime::NotaryOrganizationProfile>, NotaryServiceError> {
        self.events.push(format!(
            "list_profiles:{}:{}",
            organization_id.unwrap_or_default(),
            page_size
        ));
        Ok(self
            .profiles
            .iter()
            .filter(|(profile_org_id, _, _)| {
                organization_id.is_none_or(|organization_id| profile_org_id == organization_id)
            })
            .take(page_size as usize)
            .map(|(profile_org_id, drive_space_id, status)| {
                sdkwork_notary_runtime::NotaryOrganizationProfile {
                    organization_id: profile_org_id.clone(),
                    drive_space_id: drive_space_id.clone(),
                    drive_space_type: "notary".to_string(),
                    status: status.clone(),
                }
            })
            .collect())
    }

    async fn update_organization_profile(
        &mut self,
        command: NotaryOrganizationProfileUpdateCommand,
    ) -> Result<sdkwork_notary_runtime::NotaryOrganizationProfile, NotaryServiceError> {
        let profile = self
            .profiles
            .iter_mut()
            .find(|(profile_org_id, _, _)| profile_org_id == &command.organization_id)
            .ok_or_else(|| {
                NotaryServiceError::not_found("notary organization profile not found")
            })?;
        if let Some(status) = command.status {
            profile.2 = status;
        }
        self.events
            .push(format!("update_profile:{}", command.organization_id));
        Ok(sdkwork_notary_runtime::NotaryOrganizationProfile {
            organization_id: profile.0.clone(),
            drive_space_id: profile.1.clone(),
            drive_space_type: "notary".to_string(),
            status: profile.2.clone(),
        })
    }

    async fn insert_case(
        &mut self,
        record: NotaryCaseRecord,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        self.events.push(format!(
            "insert_case:{}:{}:{}:{}",
            record.order_id, record.order_item_id, record.sku_id, record.drive_folder_node_id
        ));
        self.cases.push(record.clone());
        Ok(record)
    }

    async fn insert_party(
        &mut self,
        case_id: &str,
        party: &NotaryPartyCommand,
        order_id: &str,
        order_item_id: &str,
        sku_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let _ = (case_id, order_item_id);
        self.events.push(format!(
            "insert_party:{}:{}:{}",
            party.name, order_id, sku_id
        ));
        self.parties.push(NotaryPartyRecord {
            party_id: format!("party-{case_id}-{}", self.parties.len() + 1),
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
        &mut self,
        case_id: &str,
        event_type: &str,
    ) -> Result<(), NotaryServiceError> {
        let _ = case_id;
        self.events.push(format!("append_event:{event_type}"));
        Ok(())
    }

    async fn get_case(
        &mut self,
        case_id: &str,
    ) -> Result<Option<NotaryCaseRecord>, NotaryServiceError> {
        Ok(self
            .cases
            .iter()
            .find(|record| record.case_id == case_id)
            .cloned())
    }

    async fn update_case(
        &mut self,
        command: NotaryCaseUpdateCommand,
    ) -> Result<NotaryCaseRecord, NotaryServiceError> {
        let record = self
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
        &mut self,
        command: NotaryPartyUpdateCommand,
    ) -> Result<NotaryPartyRecord, NotaryServiceError> {
        let record = self
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
        self.events
            .push(format!("update_party:{}", record.party_id));
        Ok(record.clone())
    }

    async fn remove_party(
        &mut self,
        case_id: &str,
        party_id: &str,
    ) -> Result<(), NotaryServiceError> {
        let record = self
            .parties
            .iter_mut()
            .find(|record| record.case_id == case_id && record.party_id == party_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary party not found"))?;
        record.status = "removed".to_string();
        self.events.push(format!("remove_party:{party_id}"));
        Ok(())
    }

    async fn insert_assignment(
        &mut self,
        command: NotaryCaseAssignmentCommand,
    ) -> Result<NotaryCaseAssignmentRecord, NotaryServiceError> {
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
        self.events
            .push(format!("insert_assignment:{}", assignment.assignment_id));
        self.assignments.push(assignment.clone());
        Ok(assignment)
    }

    async fn release_assignment(&mut self, assignment_id: &str) -> Result<(), NotaryServiceError> {
        let assignment = self
            .assignments
            .iter_mut()
            .find(|assignment| assignment.assignment_id == assignment_id)
            .ok_or_else(|| NotaryServiceError::not_found("notary case assignment not found"))?;
        assignment.status = "released".to_string();
        self.events
            .push(format!("release_assignment:{assignment_id}"));
        Ok(())
    }

    async fn list_cases(
        &mut self,
        query: NotaryCaseListQuery,
    ) -> Result<Vec<NotaryCaseRecord>, NotaryServiceError> {
        self.events.push(format!(
            "list_cases:{}:{}:{}:{}:{}:{}",
            query.organization_id,
            query.status.clone().unwrap_or_default(),
            query.sku_id.clone().unwrap_or_default(),
            query.search_term.clone().unwrap_or_default(),
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        Ok(self
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
            .take(query.page_size as usize)
            .cloned()
            .collect())
    }

    async fn list_parties(
        &mut self,
        case_id: &str,
    ) -> Result<Vec<NotaryPartyRecord>, NotaryServiceError> {
        Ok(self
            .parties
            .iter()
            .filter(|record| record.case_id == case_id)
            .cloned()
            .collect())
    }

    async fn list_events(
        &mut self,
        query: NotaryCaseEventListQuery,
    ) -> Result<Vec<NotaryCaseEventRecord>, NotaryServiceError> {
        self.events.push(format!(
            "list_events:{}:{}:{}",
            query.case_id,
            query.page_size,
            query.cursor.clone().unwrap_or_default()
        ));
        Ok(self
            .case_events
            .iter()
            .filter(|record| record.case_id == query.case_id)
            .filter(|record| {
                query
                    .cursor
                    .as_ref()
                    .is_none_or(|cursor| record.event_id.as_str() > cursor.as_str())
            })
            .take(query.page_size as usize)
            .cloned()
            .collect())
    }
}

fn runtime_context() -> NotaryRuntimeContext {
    NotaryRuntimeContext {
        tenant_id: "tenant-1".to_string(),
        organization_id: Some("org-1".to_string()),
        user_id: "user-1".to_string(),
        membership_id: Some("member-notary-1".to_string()),
        session_id: "session-1".to_string(),
        app_id: "sdkwork-chat-pc".to_string(),
    }
}

fn sample_case_record(case_id: &str) -> NotaryCaseRecord {
    NotaryCaseRecord {
        case_id: case_id.to_string(),
        case_no: "NT-20260610-001".to_string(),
        organization_id: "org-1".to_string(),
        title: "Electronic contract preservation".to_string(),
        applicant_name: "Zhang San Network".to_string(),
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        primary_notary_user_id: Some("user-notary-1".to_string()),
        primary_notary_name: Some("Li Ming".to_string()),
        status: NotaryCaseStatus::Processing,
        order_id: "order-1".to_string(),
        order_item_id: "item-1".to_string(),
        sku_id: "sku-electronic-contract".to_string(),
        matter_title: "Electronic contract preservation".to_string(),
        fee_amount: "500.00".to_string(),
        currency_code: "CNY".to_string(),
        drive_space_id: "space-notary-org-1".to_string(),
        drive_space_type: "notary".to_string(),
        drive_folder_node_id: "folder-case-1".to_string(),
        chain_hash: None,
        remarks: Some("priority".to_string()),
        request_no: "REQ-20260610-001".to_string(),
        idempotency_key: "idem-case-1".to_string(),
        created_at: "2026-06-10 10:00".to_string(),
        updated_at: "2026-06-10 10:00".to_string(),
    }
}

fn sample_matter_record(sku_id: &str, title: &str) -> CommerceMatterRecord {
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

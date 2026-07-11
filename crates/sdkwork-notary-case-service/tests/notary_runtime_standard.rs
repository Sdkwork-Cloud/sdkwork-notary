use sdkwork_notary_case_contract::{
    now_iso8601, NotaryCaseCommand, NotaryCaseRecord, NotaryCaseStatus, NotaryPartyCommand,
    NotaryRuntimeContext,
};
use sdkwork_notary_case_service::{
    create_notary_case, ensure_notary_business_open, handle_notary_app_operation,
    handle_notary_app_operation_with_metadata, handle_notary_backend_operation, list_case_files,
    notary_runtime_contract, AppbaseOrganizationMember, CommerceOrderFulfillmentState,
    NotaryCaseEventRecord, NotaryDashboardStatisticsAggregate, NotaryOperationMetadata,
    NotaryPartyRecord, NotaryRuntimePorts, NOTARY_CASE_REPOSITORY_PORT, NOTARY_COMMERCE_PORT,
    NOTARY_DRIVE_PORT, NOTARY_IAM_PORT,
};
use serde_json::json;
use std::collections::BTreeMap;

mod recording_ports;

use recording_ports::{
    sample_matter_record, RecordingAppbase, RecordingCommerce, RecordingDrive,
    RecordingNotaryCaseRepository,
};

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
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-owner".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary_admin".to_string()],
        positions: vec!["公证业务负责人".to_string()],
        departments: vec!["公证业务部".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default();
    let context = runtime_context();

    let profile = ensure_notary_business_open(
        &context,
        "200001",
        "member-owner",
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(profile.organization_id, "200001");
    assert_eq!(profile.drive_space_id, "space-notary-200001");
    assert_eq!(profile.drive_space_type, "notary");
    assert_eq!(
        drive.events(),
        vec!["create_space:notary:organization:200001"],
    );
    assert_eq!(
        repository.events(),
        vec!["upsert_profile:200001:space-notary-200001:notary"],
    );
}

#[tokio::test]
async fn creating_case_reuses_sku_order_item_and_creates_notary_drive_folder() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["公证员".to_string()],
        departments: vec!["公证一部".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");
    let context = runtime_context();

    let created = create_notary_case(
        &context,
        NotaryCaseCommand {
            organization_id: "200001".to_string(),
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(created.status, NotaryCaseStatus::PendingReview);
    assert_eq!(
        created.primary_notary_membership_id,
        Some("member-notary-1".to_string())
    );
    assert_eq!(created.primary_notary_user_id, Some("1".to_string()));
    assert_eq!(
        created.primary_notary_name,
        Some("Notary Staff".to_string())
    );
    assert_eq!(created.order_id, "order-sku-electronic-contract");
    assert_eq!(created.order_item_id, "item-sku-electronic-contract");
    assert_eq!(created.sku_id, "sku-electronic-contract");
    assert_eq!(created.drive_space_id, "space-notary-200001");
    assert_eq!(created.drive_space_type, "notary");
    assert_eq!(
        created.drive_folder_node_id,
        "folder-order-sku-electronic-contract"
    );
    assert_eq!(
        commerce.events(),
        vec!["create_order:sku-electronic-contract:notary:idem-case-1"],
    );
    assert_eq!(
        drive.events(),
        vec!["create_folder:notary:space-notary-200001:电子合同存证办理"],
    );
    assert_eq!(
        repository.events(),
        vec![
            "insert_case:order-sku-electronic-contract:item-sku-electronic-contract:sku-electronic-contract:folder-order-sku-electronic-contract",
            "insert_party:张三:order-sku-electronic-contract:sku-electronic-contract",
            "append_event:notary.case.submitted",
        ],
    );
}

#[tokio::test]
async fn creating_case_compensates_when_party_insert_fails() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["公证员".to_string()],
        departments: vec!["公证一部".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
        .with_profile("200001", "space-notary-200001")
        .with_insert_party_failure("case-item-sku-electronic-contract");
    let context = runtime_context();

    let error = create_notary_case(
        &context,
        NotaryCaseCommand {
            organization_id: "200001".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            drive_folder_name: None,
            title: "电子合同存证办理".to_string(),
            applicant_name: "张三网络科技".to_string(),
            remarks: None,
            primary_notary_membership_id: Some("member-notary-1".to_string()),
            idempotency_key: "idem-case-party-failure".to_string(),
            parties: vec![NotaryPartyCommand {
                name: "张三".to_string(),
                party_role: "申请人".to_string(),
                identity_no: "110105199001011234".to_string(),
                phone: Some("13800138000".to_string()),
            }],
        },
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code(), "provider-unavailable");
    assert_eq!(
        commerce.events(),
        vec![
            "create_order:sku-electronic-contract:notary:idem-case-party-failure",
            "cancel_order:order-sku-electronic-contract",
        ],
    );
    assert_eq!(
        drive.events(),
        vec![
            "create_folder:notary:space-notary-200001:电子合同存证办理",
            "delete_folder:notary:space-notary-200001:folder-order-sku-electronic-contract",
        ],
    );
    assert_eq!(
        repository.events(),
        vec![
            "insert_case:order-sku-electronic-contract:item-sku-electronic-contract:sku-electronic-contract:folder-order-sku-electronic-contract",
            "delete_case:case-item-sku-electronic-contract",
        ],
    );
}

#[tokio::test]
async fn creating_case_uses_frontend_drive_folder_name_when_provided() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");

    let _created = create_notary_case(
        &runtime_context(),
        NotaryCaseCommand {
            organization_id: "200001".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            title: "Electronic contract preservation".to_string(),
            drive_folder_name: Some("NT-20260610-custom-folder".to_string()),
            applicant_name: "Zhang San Network".to_string(),
            remarks: None,
            primary_notary_membership_id: Some("member-notary-1".to_string()),
            idempotency_key: "idem-case-folder-name".to_string(),
            parties: Vec::new(),
        },
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(
        drive.events(),
        vec!["create_folder:notary:space-notary-200001:NT-20260610-custom-folder"]
    );
}

#[tokio::test]
async fn creating_case_rejects_users_without_notary_business_access() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-sales-1".to_string(),
        user_id: "user-sales-1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: false,
        roles: vec!["sales".to_string()],
        positions: vec!["销售".to_string()],
        departments: vec!["商务部".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");
    let context = runtime_context();

    let error = create_notary_case(
        &context,
        NotaryCaseCommand {
            organization_id: "200001".to_string(),
            sku_id: "sku-electronic-contract".to_string(),
            drive_folder_name: None,
            title: "电子合同存证办理".to_string(),
            applicant_name: "张三网络科技".to_string(),
            remarks: None,
            primary_notary_membership_id: Some("member-sales-1".to_string()),
            idempotency_key: "idem-case-1".to_string(),
            parties: Vec::new(),
        },
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code(), "unauthorized");
    assert!(commerce.is_empty());
    assert!(drive.events().is_empty());
    assert!(repository.is_empty());
}

#[tokio::test]
async fn creating_case_replays_existing_idempotency_key_without_duplicate_side_effects() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["公证员".to_string()],
        departments: vec!["公证一部".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");
    let context = runtime_context();
    let command = NotaryCaseCommand {
        organization_id: "200001".to_string(),
        sku_id: "sku-electronic-contract".to_string(),
        drive_folder_name: None,
        title: "电子合同存证办理".to_string(),
        applicant_name: "张三网络科技".to_string(),
        remarks: None,
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        idempotency_key: "idem-case-replay".to_string(),
        parties: Vec::new(),
    };

    let first = create_notary_case(
        &context,
        command.clone(),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    let commerce_calls_after_first = commerce.events().len();
    let drive_calls_after_first = drive.events().len();

    let replayed = create_notary_case(
        &context,
        command,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(replayed.case_id, first.case_id);
    assert_eq!(commerce.events().len(), commerce_calls_after_first);
    assert_eq!(drive.events().len(), drive_calls_after_first);
}

#[tokio::test]
async fn retrieving_case_rejects_cross_organization_access() {
    let appbase = RecordingAppbase::default();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));
    let mut context = runtime_context();
    context.organization_id = Some("200002".to_string());

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let error = handle_notary_app_operation(
        &context,
        "notary.cases.retrieve",
        path_params,
        serde_json::Value::Null,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code(), "unauthorized");
}

#[tokio::test]
async fn accepting_case_rejects_invalid_status_transitions() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        path_params,
        json!({"remarks": "should fail"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap_err();

    assert_eq!(error.code(), "invalid-state");
    assert!(error.message().contains("pending_review"));
}

#[tokio::test]
async fn accepting_case_rejects_unpaid_order_before_repository_mutation() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default().with_order_fulfillment_state(
        "200001",
        CommerceOrderFulfillmentState {
            order_id: "order-1".to_string(),
            order_status: "pending_payment".to_string(),
            payment_status: Some("pending".to_string()),
            payable_amount: "50000".to_string(),
        },
    );
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
        json!({}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("unpaid order must not be accepted");

    assert_eq!(error.code(), "conflict");
    assert!(error.message().contains("payment succeeds"));
    assert!(repository.case_update_commands().is_empty());
    assert!(repository.events().is_empty());
    assert_eq!(
        commerce.events(),
        vec!["get_order_fulfillment_state:200001:order-1"]
    );
}

#[tokio::test]
async fn accepting_case_allows_paid_order_and_writes_one_acceptance_event() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default().with_order_fulfillment_state(
        "200001",
        CommerceOrderFulfillmentState {
            order_id: "order-1".to_string(),
            order_status: "paid".to_string(),
            payment_status: Some("success".to_string()),
            payable_amount: "50000".to_string(),
        },
    );
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let accepted = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
        json!({}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect("paid order must be accepted");

    assert_eq!(accepted["status"], "PROCESSING");
    assert_eq!(accepted["version"], "2");
    let updates = repository.case_update_commands();
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].event_type, "notary.case.accepted");
    assert_eq!(
        commerce.events(),
        vec!["get_order_fulfillment_state:200001:order-1"]
    );
}

#[tokio::test]
async fn accepting_case_rejects_terminal_order_even_after_payment_succeeds() {
    for terminal_status in [
        "fulfilled",
        "completed",
        "finished",
        "cancelled",
        "canceled",
        "closed",
        "expired",
        "refunded",
    ] {
        let appbase = appbase_with_notary_staff();
        let commerce = RecordingCommerce::default().with_order_fulfillment_state(
            "200001",
            CommerceOrderFulfillmentState {
                order_id: "order-1".to_string(),
                order_status: terminal_status.to_string(),
                payment_status: Some("success".to_string()),
                payable_amount: "50000".to_string(),
            },
        );
        let drive = RecordingDrive::default();
        let mut case = sample_case_record("case-1");
        case.status = NotaryCaseStatus::PendingReview;
        let repository = RecordingNotaryCaseRepository::default().with_case(case);

        let error = handle_notary_app_operation(
            &runtime_context(),
            "notary.cases.acceptances.create",
            BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
            json!({}),
            &NotaryRuntimePorts {
                appbase: &appbase,
                commerce: &commerce,
                drive: &drive,
                repository: &repository,
            },
        )
        .await
        .expect_err("terminal commerce order must not be accepted");

        assert_eq!(error.code(), "conflict", "status: {terminal_status}");
        assert!(error.message().contains("terminal commerce order"));
        assert!(repository.case_update_commands().is_empty());
        assert!(repository.events().is_empty());
    }
}

#[tokio::test]
async fn accepting_case_allows_zero_total_order_without_success_payment_status() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default().with_order_fulfillment_state(
        "200001",
        CommerceOrderFulfillmentState {
            order_id: "order-1".to_string(),
            order_status: "pending_payment".to_string(),
            payment_status: Some("pending".to_string()),
            payable_amount: "0".to_string(),
        },
    );
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let accepted = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
        json!({}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect("zero-total order must be accepted");

    assert_eq!(accepted["status"], "PROCESSING");
    assert_eq!(repository.case_update_commands().len(), 1);
}

#[tokio::test]
async fn accepting_case_scopes_order_lookup_to_case_organization() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default().with_order_fulfillment_state(
        "200002",
        CommerceOrderFulfillmentState {
            order_id: "order-1".to_string(),
            order_status: "paid".to_string(),
            payment_status: Some("success".to_string()),
            payable_amount: "50000".to_string(),
        },
    );
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
        json!({}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("order from another organization must not authorize acceptance");

    assert_eq!(error.code(), "not-found");
    assert!(repository.case_update_commands().is_empty());
    assert!(repository.events().is_empty());
    assert_eq!(
        commerce.events(),
        vec!["get_order_fulfillment_state:200001:order-1"]
    );
}

#[tokio::test]
async fn completing_processing_case_does_not_query_order_payment_again() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));

    let completed = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.completions.create",
        BTreeMap::from([(String::from("caseId"), String::from("case-1"))]),
        json!({}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect("processing case must be completable");

    assert_eq!(completed["status"], "COMPLETED");
    assert!(commerce.events().is_empty());
    assert_eq!(repository.case_update_commands().len(), 1);
}

#[tokio::test]
async fn case_status_mutations_reject_malformed_wire_versions_before_repository_updates() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let path_params = BTreeMap::from([("caseId".to_string(), "case-1".to_string())]);

    for (body, expected_message) in [
        (json!({"version": 1}), "version must be an int64 string"),
        (
            json!({"version": "not-an-integer"}),
            "version must be an int64 string",
        ),
        (json!({"version": "0"}), "version must be greater than zero"),
        (
            json!({"version": "-1"}),
            "version must be greater than zero",
        ),
    ] {
        let mut case = sample_case_record("case-1");
        case.status = NotaryCaseStatus::PendingReview;
        let repository = RecordingNotaryCaseRepository::default().with_case(case);

        let error = handle_notary_app_operation(
            &runtime_context(),
            "notary.cases.acceptances.create",
            path_params.clone(),
            body,
            &NotaryRuntimePorts {
                appbase: &appbase,
                commerce: &commerce,
                drive: &drive,
                repository: &repository,
            },
        )
        .await
        .expect_err("malformed version must be rejected");

        assert_eq!(error.code(), "validation");
        assert_eq!(error.message(), expected_message);
        assert!(repository.case_update_commands().is_empty());
    }
}

#[tokio::test]
async fn rejecting_case_requires_a_reason_and_passes_it_into_the_atomic_update() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);
    let path_params = BTreeMap::from([("caseId".to_string(), "case-1".to_string())]);

    let missing_reason_error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.rejections.create",
        path_params.clone(),
        json!({"reason": "  ", "version": "1"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("blank rejection reason");
    assert_eq!(missing_reason_error.code(), "validation");
    assert_eq!(missing_reason_error.message(), "reason is required");
    assert!(repository.case_update_commands().is_empty());

    let rejected = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.rejections.create",
        path_params,
        json!({"reason": "  identity document expired  ", "version": "1"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(rejected["status"], "REJECTED");
    assert_eq!(rejected["version"], "2");
    let commands = repository.case_update_commands();
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].expected_version, 1);
    assert_eq!(commands[0].event_type, "notary.case.rejected");
    assert_eq!(
        commands[0].reject_reason.as_deref(),
        Some("identity document expired")
    );
}

#[tokio::test]
async fn case_status_mutation_surfaces_a_stale_version_as_conflict() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    case.version = 2;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        BTreeMap::from([("caseId".to_string(), "case-1".to_string())]),
        json!({"version": "1"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("stale version must fail compare-and-swap");

    assert_eq!(error.code(), "conflict");
    assert_eq!(error.message(), "notary case version conflict");
    assert!(repository.case_update_commands().is_empty());
    assert!(repository.events().is_empty());
}

#[tokio::test]
async fn terminal_cases_reject_content_mutations_before_repository_writes() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let mut cancelled_case = sample_case_record("case-cancelled");
    cancelled_case.status = NotaryCaseStatus::Cancelled;
    let repository = RecordingNotaryCaseRepository::default().with_case(cancelled_case);
    let path_params = BTreeMap::from([
        ("caseId".to_string(), "case-cancelled".to_string()),
        ("partyId".to_string(), "party-1".to_string()),
    ]);

    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.update",
        path_params,
        json!({"name": "Changed party"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("terminal cases must reject party mutations");

    assert_eq!(error.code(), "invalid-state");
    assert!(error.message().contains("terminal"));
    assert!(!repository
        .events()
        .iter()
        .any(|event| event.starts_with("update_party:")));
}

#[tokio::test]
async fn case_file_listing_uses_denormalized_drive_space_type_without_joining() {
    let appbase = RecordingAppbase::default();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default().with_case(NotaryCaseRecord {
        case_id: "case-1".to_string(),
        case_no: "NT-20260610-001".to_string(),
        organization_id: "200001".to_string(),
        title: "电子合同存证办理".to_string(),
        applicant_name: "张三网络科技".to_string(),
        primary_notary_name: Some("李明".to_string()),
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        primary_notary_user_id: Some("1".to_string()),
        status: NotaryCaseStatus::Processing,
        order_id: "order-1".to_string(),
        order_item_id: "item-1".to_string(),
        sku_id: "sku-electronic-contract".to_string(),
        matter_title: "电子合同存证".to_string(),
        fee_amount: "500.00".to_string(),
        currency_code: "CNY".to_string(),
        drive_space_id: "space-notary-200001".to_string(),
        drive_space_type: "notary".to_string(),
        drive_folder_node_id: "folder-case-1".to_string(),
        chain_hash: None,
        remarks: None,
        request_no: "REQ-20260610-001".to_string(),
        idempotency_key: "idem-case-1".to_string(),
        version: 1,
        created_at: "2026-06-10 10:00".to_string(),
        updated_at: "2026-06-10 10:00".to_string(),
    });

    let files = list_case_files(
        &runtime_context(),
        "case-1",
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(files.len(), 1);
    assert_eq!(files[0].node_id, "node-folder-case-1");
    assert_eq!(
        drive.events(),
        vec!["list_nodes:notary:space-notary-200001:folder-case-1::50:"],
    );
}

#[tokio::test]
async fn app_operation_dispatcher_creates_case_and_lists_drive_files() {
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");
    let context = runtime_context();

    let created = handle_notary_app_operation_with_metadata(
        &context,
        "notary.cases.create",
        BTreeMap::new(),
        json!({
            "organizationId": "200001",
            "skuId": "sku-electronic-contract",
            "title": "Electronic contract preservation",
            "applicantName": "Zhang San Network",
            "primaryNotaryMembershipId": "member-notary-1",
            "parties": [
                {
                    "name": "Zhang San",
                    "role": "applicant",
                    "identityNo": "110105199001011234",
                    "phone": "13800138000"
                }
            ]
        }),
        &NotaryOperationMetadata {
            idempotency_key: Some("idem-route-case-1".to_string()),
        },
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
async fn app_operation_dispatcher_does_not_accept_body_idempotency_key() {
    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.create",
        BTreeMap::new(),
        json!({
            "organizationId": "200001",
            "skuId": "sku-electronic-contract",
            "title": "Electronic contract preservation",
            "applicantName": "Zhang San Network",
            "idempotencyKey": "legacy-body-key"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase_with_notary_staff(),
            commerce: &RecordingCommerce::default(),
            drive: &RecordingDrive::default(),
            repository: &RecordingNotaryCaseRepository::default(),
        },
    )
    .await
    .expect_err("body idempotency key must not satisfy the header requirement");

    assert_eq!(error.code(), "validation");
    assert_eq!(error.message(), "Idempotency-Key header is required");
}

#[tokio::test]
async fn app_operation_dispatcher_returns_frontend_case_details_from_notary_and_drive() {
    let case = sample_case_record("case-1");
    let appbase = RecordingAppbase::default();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
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
            actor_user_id: Some("1".to_string()),
            occurred_at: "2026-06-10 10:00".to_string(),
        });

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let detail = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.retrieve",
        path_params,
        serde_json::Value::Null,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        json!({"organizationId": "200001", "status": "PROCESSING", "q": "contract"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(listed["items"][0]["id"], "case-1");
    assert_eq!(listed["pageInfo"]["totalItems"], "1");
    assert_eq!(
        repository.events().last().unwrap(),
        "list_cases:200001:processing::contract:20:"
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

    let appbase = RecordingAppbase::default();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
        .with_case(case)
        .with_case(other_case)
        .with_event(NotaryCaseEventRecord {
            event_id: "event-1".to_string(),
            case_id: "case-1".to_string(),
            event_type: "notary.case.submitted".to_string(),
            event_title: "Case submitted".to_string(),
            actor_user_id: Some("1".to_string()),
            occurred_at: "2026-06-10 10:00".to_string(),
        });

    let listed = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.list",
        BTreeMap::new(),
        json!({
            "organizationId": "200001",
            "status": "PROCESSING",
            "q": "contract",
            "sku_id": "sku-electronic-contract",
            "page_size": 25,
            "cursor": "case-z"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(listed["items"].as_array().unwrap().len(), 1);
    assert_eq!(listed["items"][0]["skuId"], "sku-electronic-contract");
    assert_eq!(
        repository.events().last().unwrap(),
        "list_cases:200001:processing:sku-electronic-contract:contract:25:case-z"
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(files["items"][0]["category"], "identity");
    assert_eq!(
        drive.events().last().unwrap(),
        "list_nodes:notary:space-notary-200001:folder-case-1:identity:25:file-cursor"
    );

    let _events = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.events.list",
        path_params,
        json!({
            "page_size": 1,
            "cursor": "event-0"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(
        repository.events().last().unwrap(),
        "list_events:case-1:1:event-0"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_returns_dashboard_statistics_and_monthly_report() {
    let mut completed_case = sample_case_record("case-2");
    completed_case.status = NotaryCaseStatus::Completed;
    completed_case.case_no = "NT-20260610-002".to_string();
    completed_case.title = "Completed evidence preservation".to_string();
    completed_case.chain_hash = Some("chain-hash-case-2".to_string());

    let mut rejected_case = sample_case_record("case-3");
    rejected_case.status = NotaryCaseStatus::Rejected;
    rejected_case.case_no = "NT-20260610-003".to_string();
    rejected_case.title = "Rejected evidence preservation".to_string();

    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
        .with_case(sample_case_record("case-1"))
        .with_case(completed_case)
        .with_case(rejected_case)
        .with_dashboard_statistics(NotaryDashboardStatisticsAggregate {
            pending_review_count: 2,
            today_completed_count: 4,
            yesterday_completed_count: 6,
            monthly_case_count: 3,
            anomaly_intercepted_count: 1,
            unsynced_completed_count: 0,
        })
        .with_monthly_case_count(3);

    let statistics = handle_notary_app_operation(
        &runtime_context(),
        "notary.dashboard.statistics.retrieve",
        BTreeMap::new(),
        serde_json::Value::Null,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(statistics["pendingReviewQueue"]["count"], 2);
    assert_eq!(
        statistics["pendingReviewQueue"]["estimatedProcessHours"],
        4.0
    );
    assert_eq!(statistics["todayCompleted"]["count"], 4);
    assert_eq!(statistics["todayCompleted"]["comparedToYesterday"], -2);
    assert_eq!(statistics["anomalyIntercepted"]["count"], 1);
    assert_eq!(statistics["monthlyPreservationTotal"]["count"], 3);
    assert_eq!(
        statistics["monthlyPreservationTotal"]["blockchainSyncStatus"],
        "OK"
    );
    assert_eq!(repository.events(), vec!["get_dashboard_statistics:200001"]);

    let report = handle_notary_app_operation(
        &runtime_context(),
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        json!({"organizationId": "200001", "month": "2026-06", "format": "csv"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
    assert_eq!(
        repository.events()[1],
        "count_cases_for_month:200001:2026-06"
    );
}

#[tokio::test]
async fn monthly_report_defaults_to_current_utc_month_and_uses_repository_count() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default().with_monthly_case_count(7);
    let before_month = now_iso8601()[..7].to_string();

    let report = handle_notary_app_operation(
        &runtime_context(),
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        serde_json::Value::Null,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    let after_month = now_iso8601()[..7].to_string();
    let report_month = report["month"].as_str().unwrap();
    assert!(report_month == before_month || report_month == after_month);
    assert_eq!(report["caseCount"], 7);
    assert_eq!(
        repository.events(),
        vec![format!("count_cases_for_month:200001:{report_month}")]
    );
}

#[tokio::test]
async fn monthly_report_rejects_invalid_calendar_month_before_repository_or_drive_calls() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default();

    let error = handle_notary_app_operation(
        &runtime_context(),
        "notary.reports.monthly.retrieve",
        BTreeMap::new(),
        json!({"month": "2026-13", "format": "csv"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .expect_err("invalid calendar month");

    assert_eq!(error.code(), "validation");
    assert_eq!(
        error.message(),
        "month must use YYYY-MM with a calendar month from 01 to 12"
    );
    assert!(repository.is_empty());
    assert!(drive.events().is_empty());
}

#[tokio::test]
async fn app_operation_dispatcher_mutates_case_party_and_drive_file_workflows() {
    let appbase = appbase_with_notary_staff();
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let mut case = sample_case_record("case-1");
    case.status = NotaryCaseStatus::PendingReview;
    let repository = RecordingNotaryCaseRepository::default().with_case(case);

    let mut path_params = BTreeMap::new();
    path_params.insert("caseId".to_string(), "case-1".to_string());
    let accepted = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.acceptances.create",
        path_params.clone(),
        json!({"remarks": "materials accepted"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(accepted["status"], "PROCESSING");
    assert_eq!(accepted["version"], "2");
    assert!(repository
        .events()
        .contains(&"update_case:notary.case.accepted".to_string()));

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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(with_party["name"], "Li Si");
    assert!(repository
        .events()
        .contains(&"append_event:notary.party.created".to_string()));

    let party_id = with_party["id"].as_str().unwrap().to_string();
    let mut invite_params = path_params.clone();
    invite_params.insert("partyId".to_string(), party_id.clone());
    let invite = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.videoInvites.create",
        invite_params,
        json!({"purpose": "identity_verification"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        repository.events().last().unwrap(),
        "append_event:notary.party.video_invite.created"
    );

    let mut signature_invite_params = path_params.clone();
    signature_invite_params.insert("partyId".to_string(), party_id.clone());
    let signature_invite = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.parties.signatureInvites.create",
        signature_invite_params,
        json!({"purpose": "remote_signature"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        repository.events().last().unwrap(),
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
            "partyId": party_id,
            "reviewStatus": "pending"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(file["nodeId"], "drive-node-1");
    assert_eq!(file["driveSpaceType"], "notary");
    assert_eq!(file["parentNodeId"], "folder-case-1");
    assert_eq!(file["materialCode"], "contract.pdf");
    assert_eq!(file["partyId"], party_id);

    let listed_files = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.files.list",
        path_params.clone(),
        json!({"page_size": "20"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(listed_files["items"][0]["materialCode"], "contract.pdf");
    assert_eq!(listed_files["items"][0]["partyId"], party_id);

    let package = handle_notary_app_operation(
        &runtime_context(),
        "notary.cases.downloadPackages.create",
        path_params,
        json!({"packageName": "case-1.zip"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
    completed_case.chain_hash = Some("chain-hash-case-2".to_string());

    let appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-owner".to_string(),
            user_id: "1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "Owner".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary_admin".to_string()],
            positions: vec!["notary director".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "李明".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
        .with_case(sample_case_record("case-1"))
        .with_case(completed_case);

    let opened = handle_notary_backend_operation(
        &runtime_context(),
        "notary.organizationProfiles.create",
        BTreeMap::new(),
        json!({
            "organizationId": "200001",
            "openedByMembershipId": "member-owner"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(opened["driveSpaceType"], "notary");
    assert_eq!(opened["organizationId"], "200001");

    let profiles = handle_notary_backend_operation(
        &runtime_context(),
        "notary.organizationProfiles.list",
        BTreeMap::new(),
        json!({"organizationId": "200001", "pageSize": 20}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(profiles["items"][0]["organizationId"], "200001");
    assert!(repository
        .events()
        .contains(&"list_profiles:200001:20:".to_string()));

    let staff = handle_notary_backend_operation(
        &runtime_context(),
        "notary.staff.list",
        BTreeMap::new(),
        json!({"organizationId": "200001", "staffRole": "notary"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        json!({"organizationId": "200001", "pageSize": 20}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(cases["items"].as_array().unwrap().len(), 2);

    let summary = handle_notary_backend_operation(
        &runtime_context(),
        "notary.reports.caseSummary.retrieve",
        BTreeMap::new(),
        json!({"organizationId": "200001"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
    let appbase = RecordingAppbase::default().with_member(AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    });
    let commerce = RecordingCommerce::default().with_matter(sample_matter_record(
        "sku-electronic-contract",
        "Electronic contract preservation",
    ));
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_profile("200001", "space-notary-200001");

    let access = handle_notary_app_operation(
        &runtime_context(),
        "notary.access.retrieve",
        BTreeMap::new(),
        serde_json::Value::Null,
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        json!({"organizationId": "200001", "q": "contract", "pageSize": 20}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(matters["items"][0]["skuId"], "sku-electronic-contract");
    assert_eq!(matters["items"][0]["spuId"], "spu-sku-electronic-contract");
    assert_eq!(matters["items"][0]["priceAmount"], "500.00");
    assert_eq!(
        commerce.events().last().unwrap(),
        "list_matters:200001:contract::20:0"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_lists_notary_staff_and_assigns_selected_member() {
    let appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "李明".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-assistant-1".to_string(),
            user_id: "user-assistant-1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "Assistant".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["assistant".to_string()],
            positions: vec!["assistant".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-sales-1".to_string(),
            user_id: "user-sales-1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "Sales".to_string(),
            enterprise_verified: true,
            notary_enabled: false,
            roles: vec!["sales".to_string()],
            positions: vec!["sales".to_string()],
            departments: vec!["sales".to_string()],
        });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository =
        RecordingNotaryCaseRepository::default().with_case(sample_case_record("case-1"));

    let staff = handle_notary_app_operation(
        &runtime_context(),
        "notary.staff.list",
        BTreeMap::new(),
        json!({"organizationId": "200001", "staffRole": "notary"}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();

    assert_eq!(assignment["caseId"], "case-1");
    assert_eq!(assignment["organizationMembershipId"], "member-notary-1");
    assert_eq!(assignment["userId"], "1");
    assert_eq!(assignment["assignmentRole"], "primary_notary");
    assert_eq!(
        repository.events(),
        vec![
            "insert_assignment:assignment-case-1-member-notary-1-primary_notary",
            "append_event:notary.case.assignment_created",
        ],
    );
}

#[tokio::test]
async fn backend_operation_dispatcher_manages_profile_matters_and_assignments() {
    let appbase = RecordingAppbase::default()
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-owner".to_string(),
            user_id: "1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "Owner".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary_admin".to_string()],
            positions: vec!["notary director".to_string()],
            departments: vec!["notary-office".to_string()],
        })
        .with_member(AppbaseOrganizationMember {
            membership_id: "member-notary-1".to_string(),
            user_id: "1".to_string(),
            organization_id: "200001".to_string(),
            display_name: "李明".to_string(),
            enterprise_verified: true,
            notary_enabled: true,
            roles: vec!["notary".to_string()],
            positions: vec!["notary".to_string()],
            departments: vec!["notary-office".to_string()],
        });
    let commerce = RecordingCommerce::default();
    let drive = RecordingDrive::default();
    let repository = RecordingNotaryCaseRepository::default()
        .with_profile("200001", "space-notary-200001")
        .with_case(sample_case_record("case-1"));

    let mut profile_path = BTreeMap::new();
    profile_path.insert("organizationProfileId".to_string(), "200001".to_string());
    let updated_profile = handle_notary_backend_operation(
        &admin_runtime_context(),
        "notary.organizationProfiles.update",
        profile_path,
        json!({"status": "suspended", "settings": {"reviewMode": "manual"}}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(updated_profile["organizationId"], "200001");
    assert_eq!(updated_profile["status"], "suspended");

    let created_matter = handle_notary_backend_operation(
        &admin_runtime_context(),
        "notary.matters.create",
        BTreeMap::new(),
        json!({
            "organizationId": "200001",
            "title": "Electronic evidence preservation",
            "description": "Preserve electronic evidence",
            "priceAmount": "600.00",
            "originalPriceAmount": "800.00",
            "currencyCode": "CNY",
            "status": "active",
            "spec": {"materialCodes": ["identity", "evidence"]}
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        json!({"organizationId": "200001", "q": "evidence", "pageSize": 10}),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
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
        &admin_runtime_context(),
        "notary.matters.update",
        matter_path,
        json!({
            "title": "Updated evidence preservation",
            "description": null,
            "originalPriceAmount": null,
            "status": "inactive"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(updated_matter["title"], "Updated evidence preservation");
    assert!(updated_matter["description"].is_null());
    assert!(updated_matter["originalPriceAmount"].is_null());
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert_eq!(assignment["caseId"], "case-1");
    assert_eq!(assignment["organizationMembershipId"], "member-notary-1");
    assert_eq!(assignment["assignmentRole"], "primary_notary");
    assert_eq!(
        repository.events().last().unwrap(),
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
        &NotaryRuntimePorts {
            appbase: &appbase,
            commerce: &commerce,
            drive: &drive,
            repository: &repository,
        },
    )
    .await
    .unwrap();
    assert!(released.is_null());
    assert_eq!(
        repository.events().last().unwrap(),
        "append_event:notary.case.assignment_released"
    );
}

#[tokio::test]
async fn app_operation_dispatcher_rejects_missing_operation_permission() {
    let mut context = runtime_context();
    context.permission_scopes = vec!["notary.cases.read".to_string()];

    let error = handle_notary_app_operation(
        &context,
        "notary.cases.create",
        BTreeMap::new(),
        serde_json::json!({
            "organizationId": "200001",
            "matterSkuId": "sku-electronic-contract"
        }),
        &NotaryRuntimePorts {
            appbase: &appbase_with_notary_staff(),
            commerce: &RecordingCommerce::default(),
            drive: &RecordingDrive::default(),
            repository: &RecordingNotaryCaseRepository::default(),
        },
    )
    .await
    .expect_err("missing permission");

    assert!(error
        .message()
        .contains("missing permission: notary.cases.create"));
}

fn runtime_context() -> NotaryRuntimeContext {
    NotaryRuntimeContext {
        tenant_id: "100001".to_string(),
        organization_id: Some("200001".to_string()),
        user_id: "1".to_string(),
        membership_id: Some("member-notary-1".to_string()),
        session_id: "session-1".to_string(),
        app_id: "sdkwork-im-pc".to_string(),
        permission_scopes: vec!["notary.*".to_string()],
    }
}

fn notary_staff_member() -> AppbaseOrganizationMember {
    AppbaseOrganizationMember {
        membership_id: "member-notary-1".to_string(),
        user_id: "1".to_string(),
        organization_id: "200001".to_string(),
        display_name: "Notary Staff".to_string(),
        enterprise_verified: true,
        notary_enabled: true,
        roles: vec!["notary".to_string()],
        positions: vec!["notary".to_string()],
        departments: vec!["notary-office".to_string()],
    }
}

fn appbase_with_notary_staff() -> RecordingAppbase {
    RecordingAppbase::default().with_member(notary_staff_member())
}

fn admin_runtime_context() -> NotaryRuntimeContext {
    let mut context = runtime_context();
    context.membership_id = Some("member-owner".to_string());
    context
}

fn sample_case_record(case_id: &str) -> NotaryCaseRecord {
    NotaryCaseRecord {
        case_id: case_id.to_string(),
        case_no: "NT-20260610-001".to_string(),
        organization_id: "200001".to_string(),
        title: "Electronic contract preservation".to_string(),
        applicant_name: "Zhang San Network".to_string(),
        primary_notary_membership_id: Some("member-notary-1".to_string()),
        primary_notary_user_id: Some("1".to_string()),
        primary_notary_name: Some("Li Ming".to_string()),
        status: NotaryCaseStatus::Processing,
        order_id: "order-1".to_string(),
        order_item_id: "item-1".to_string(),
        sku_id: "sku-electronic-contract".to_string(),
        matter_title: "Electronic contract preservation".to_string(),
        fee_amount: "500.00".to_string(),
        currency_code: "CNY".to_string(),
        drive_space_id: "space-notary-200001".to_string(),
        drive_space_type: "notary".to_string(),
        drive_folder_node_id: "folder-case-1".to_string(),
        chain_hash: None,
        remarks: Some("priority".to_string()),
        request_no: "REQ-20260610-001".to_string(),
        idempotency_key: "idem-case-1".to_string(),
        version: 1,
        created_at: "2026-06-10 10:00".to_string(),
        updated_at: "2026-06-10 10:00".to_string(),
    }
}

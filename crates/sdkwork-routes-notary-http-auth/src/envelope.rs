use serde_json::Value;
use sdkwork_utils_rust::sdkwork_resource_json;

const LIST_OPERATIONS: &[&str] = &[
    "notary.matters.list",
    "notary.staff.list",
    "notary.cases.list",
    "notary.cases.parties.list",
    "notary.cases.files.list",
    "notary.cases.events.list",
    "notary.organizationProfiles.list",
    "notary.matters.management.list",
    "notary.cases.management.list",
];

const DELETE_NO_CONTENT_OPERATIONS: &[&str] = &[
    "notary.cases.parties.delete",
    "notary.cases.assignments.delete",
];

pub fn is_list_operation(operation_id: &str) -> bool {
    LIST_OPERATIONS.contains(&operation_id)
}

pub fn is_delete_no_content_operation(operation_id: &str) -> bool {
    DELETE_NO_CONTENT_OPERATIONS.contains(&operation_id)
}

pub fn envelope_success_data(operation_id: &str, data: Value) -> Value {
    if is_list_operation(operation_id) || data.get("items").is_some() {
        data
    } else {
        sdkwork_resource_json(data)
    }
}

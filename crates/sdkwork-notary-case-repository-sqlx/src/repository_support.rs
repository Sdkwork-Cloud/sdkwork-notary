use sdkwork_notary_case_contract::NotaryServiceError;

pub fn last4(value: &str) -> String {
    let mut chars = value.chars().rev().take(4).collect::<Vec<_>>();
    chars.reverse();
    chars.into_iter().collect()
}

pub fn mask_phone(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= 7 {
        return "****".to_string();
    }
    let prefix = trimmed.chars().take(3).collect::<String>();
    let suffix = trimmed
        .chars()
        .rev()
        .take(4)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    format!("{prefix}****{suffix}")
}

pub fn store_error(context: &'static str) -> impl FnOnce(sqlx::Error) -> NotaryServiceError {
    move |error| {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.is_unique_violation() {
                return NotaryServiceError::conflict(format!("{context}: duplicate notary record"));
            }
        }
        NotaryServiceError::storage(format!("{context}: {error}"))
    }
}

pub fn event_title(event_type: &str) -> &'static str {
    match event_type {
        "notary.case.submitted" => "Case submitted",
        "notary.case.accepted" => "Case accepted",
        "notary.case.rejected" => "Case rejected",
        "notary.case.completed" => "Case completed",
        "notary.party.video_invite.created" => "Party video verification invite created",
        "notary.party.signature_invite.created" => "Party mobile signature invite created",
        _ => "Notary case event",
    }
}

pub fn validate_profile_status(status: &str) -> Result<(), NotaryServiceError> {
    if matches!(status, "active" | "suspended" | "closed") {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary organization profile status: {status}"
        )))
    }
}

pub fn validate_assignment_role(role: &str) -> Result<(), NotaryServiceError> {
    if matches!(
        role,
        "primary_notary" | "assistant" | "reviewer" | "approver"
    ) {
        Ok(())
    } else {
        Err(NotaryServiceError::validation(format!(
            "unsupported notary case assignment role: {role}"
        )))
    }
}

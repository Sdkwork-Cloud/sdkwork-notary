use axum::http::StatusCode;
use sdkwork_web_core::WebRequestContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryRequestContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub user_id: String,
    pub membership_id: Option<String>,
    pub session_id: String,
    pub app_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryAuthError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
}

impl NotaryAuthError {
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "unauthenticated",
            message: message.into(),
        }
    }
}

pub fn notary_request_context_from_web(
    app_ctx: &WebRequestContext,
) -> Result<NotaryRequestContext, NotaryAuthError> {
    let principal = app_ctx.principal.as_ref().ok_or_else(|| {
        NotaryAuthError::unauthorized("authenticated request context is required")
    })?;

    Ok(NotaryRequestContext {
        tenant_id: principal.tenant_id().to_owned(),
        organization_id: principal.organization_id().map(str::to_owned),
        user_id: principal.user_id().to_owned(),
        membership_id: resolve_membership_id(principal),
        session_id: principal
            .session_id()
            .map(str::to_owned)
            .unwrap_or_else(|| format!("{}:{}", principal.app_id(), principal.user_id())),
        app_id: principal.app_id().to_owned(),
    })
}

fn resolve_membership_id(principal: &sdkwork_web_core::WebRequestPrincipal) -> Option<String> {
    for scope in principal
        .scopes
        .data_scope
        .iter()
        .chain(principal.scopes.permission_scope.iter())
    {
        if let Some(id) = scope.strip_prefix("organization_membership:") {
            if !id.is_empty() {
                return Some(id.to_owned());
            }
        }
        if scope.starts_with("orgmem_") {
            return Some(scope.clone());
        }
    }

    principal
        .organization_id()
        .map(|organization_id| stable_local_membership_id(organization_id, principal.user_id()))
}

fn stable_local_membership_id(organization_id: &str, user_id: &str) -> String {
    let suffix = [organization_id, user_id]
        .iter()
        .map(|part| normalize_identifier(part))
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if suffix.is_empty() {
        "orgmem".to_owned()
    } else {
        format!("orgmem_{suffix}")
    }
}

fn normalize_identifier(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_web_core::{WebLoginScope, WebRequestPrincipal};

    #[test]
    fn resolves_explicit_organization_membership_scope() {
        let principal = WebRequestPrincipal::builder()
            .tenant_id("tenant-1")
            .organization_id(Some("org-1".to_owned()))
            .user_id("user-1")
            .app_id("app-1")
            .data_scope(vec!["organization_membership:mem-123".to_owned()])
            .build();

        assert_eq!(
            resolve_membership_id(&principal),
            Some("mem-123".to_owned())
        );
    }

    #[test]
    fn derives_stable_local_membership_when_organization_present() {
        let principal = WebRequestPrincipal::builder()
            .tenant_id("tenant-1")
            .organization_id(Some("org_notary_dev".to_owned()))
            .login_scope(WebLoginScope::Organization)
            .user_id("contract-test-user-001")
            .app_id("app-1")
            .build();

        assert_eq!(
            resolve_membership_id(&principal),
            Some("orgmem_org_notary_dev_contract_test_user_001".to_owned())
        );
    }

    #[test]
    fn leaves_membership_absent_without_organization_context() {
        let principal = WebRequestPrincipal::builder()
            .tenant_id("tenant-1")
            .user_id("user-1")
            .app_id("app-1")
            .build();

        assert_eq!(resolve_membership_id(&principal), None);
    }

    #[test]
    fn maps_web_request_context_into_notary_context() {
        let app_ctx = crate::test_support::test_web_request_context();
        let context = notary_request_context_from_web(&app_ctx).expect("context");
        assert_eq!(context.tenant_id, crate::test_support::TEST_TENANT_ID);
        assert_eq!(
            context.organization_id.as_deref(),
            Some(crate::test_support::TEST_ORGANIZATION_ID),
        );
        assert_eq!(context.user_id, crate::test_support::TEST_USER_ID);
        assert_eq!(context.session_id, crate::test_support::TEST_SESSION_ID);
        assert_eq!(context.app_id, crate::test_support::TEST_APP_ID);
        assert_eq!(
            context.membership_id,
            Some("orgmem_org_1_user_1".to_owned()),
        );
    }

    #[test]
    fn rejects_missing_principal() {
        let mut app_ctx = crate::test_support::test_web_request_context();
        app_ctx.principal = None;
        let error = notary_request_context_from_web(&app_ctx).expect_err("missing principal");
        assert_eq!(error.code, "unauthenticated");
    }
}

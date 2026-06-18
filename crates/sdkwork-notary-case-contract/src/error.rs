#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NotaryServiceErrorKind {
    Unauthenticated,
    Unauthorized,
    NotFound,
    Conflict,
    InvalidState,
    Validation,
    Transport,
    ProviderUnavailable,
    Storage,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryServiceError {
    kind: NotaryServiceErrorKind,
    message: String,
}

impl NotaryServiceError {
    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Unauthenticated, message)
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Unauthorized, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::NotFound, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Conflict, message)
    }

    pub fn invalid_state(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::InvalidState, message)
    }

    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Validation, message)
    }

    pub fn transport(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Transport, message)
    }

    pub fn provider_unavailable(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::ProviderUnavailable, message)
    }

    pub fn storage(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Storage, message)
    }

    pub fn unknown(message: impl Into<String>) -> Self {
        Self::new(NotaryServiceErrorKind::Unknown, message)
    }

    pub fn code(&self) -> &'static str {
        match self.kind {
            NotaryServiceErrorKind::Unauthenticated => "unauthenticated",
            NotaryServiceErrorKind::Unauthorized => "unauthorized",
            NotaryServiceErrorKind::NotFound => "not-found",
            NotaryServiceErrorKind::Conflict => "conflict",
            NotaryServiceErrorKind::InvalidState => "invalid-state",
            NotaryServiceErrorKind::Validation => "validation",
            NotaryServiceErrorKind::Transport => "transport",
            NotaryServiceErrorKind::ProviderUnavailable => "provider-unavailable",
            NotaryServiceErrorKind::Storage => "storage",
            NotaryServiceErrorKind::Unknown => "unknown",
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(kind: NotaryServiceErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryRuntimeContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub user_id: String,
    pub membership_id: Option<String>,
    pub session_id: String,
    pub app_id: String,
    pub permission_scopes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NotaryCaseStatus {
    PendingReview,
    Processing,
    Completed,
    Rejected,
    Cancelled,
    CreateFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryPartyCommand {
    pub name: String,
    pub party_role: String,
    pub identity_no: String,
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseCommand {
    pub organization_id: String,
    pub sku_id: String,
    pub title: String,
    pub drive_folder_name: Option<String>,
    pub applicant_name: String,
    pub remarks: Option<String>,
    pub primary_notary_membership_id: Option<String>,
    pub idempotency_key: String,
    pub parties: Vec<NotaryPartyCommand>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryCaseRecord {
    pub case_id: String,
    pub case_no: String,
    pub organization_id: String,
    pub title: String,
    pub applicant_name: String,
    pub primary_notary_membership_id: Option<String>,
    pub primary_notary_user_id: Option<String>,
    pub primary_notary_name: Option<String>,
    pub status: NotaryCaseStatus,
    pub order_id: String,
    pub order_item_id: String,
    pub sku_id: String,
    pub matter_title: String,
    pub fee_amount: String,
    pub currency_code: String,
    pub drive_space_id: String,
    pub drive_space_type: String,
    pub drive_folder_node_id: String,
    pub chain_hash: Option<String>,
    pub remarks: Option<String>,
    pub request_no: String,
    pub idempotency_key: String,
    pub created_at: String,
    pub updated_at: String,
}

impl NotaryCaseStatus {
    pub fn as_storage_value(&self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::CreateFailed => "create_failed",
        }
    }

    pub fn as_frontend_value(&self) -> &'static str {
        match self {
            Self::PendingReview => "PENDING_REVIEW",
            Self::Processing => "PROCESSING",
            Self::Completed => "COMPLETED",
            Self::Rejected => "REJECTED",
            Self::Cancelled => "REJECTED",
            Self::CreateFailed => "REJECTED",
        }
    }

    pub fn from_storage_value(value: &str) -> Option<Self> {
        match value {
            "pending_review" => Some(Self::PendingReview),
            "processing" => Some(Self::Processing),
            "completed" => Some(Self::Completed),
            "rejected" => Some(Self::Rejected),
            "cancelled" => Some(Self::Cancelled),
            "create_failed" => Some(Self::CreateFailed),
            _ => None,
        }
    }

    pub fn allows_transition_to(&self, next: &Self) -> bool {
        use NotaryCaseStatus::*;
        if self == next {
            return true;
        }
        matches!(
            (self, next),
            (PendingReview, Processing)
                | (PendingReview, Rejected)
                | (PendingReview, Cancelled)
                | (PendingReview, CreateFailed)
                | (Processing, Completed)
                | (Processing, Rejected)
                | (Processing, Cancelled)
        )
    }

    pub fn allows_acceptance(&self) -> bool {
        matches!(self, Self::PendingReview)
    }

    pub fn allows_rejection(&self) -> bool {
        matches!(self, Self::PendingReview | Self::Processing)
    }

    pub fn allows_completion(&self) -> bool {
        matches!(self, Self::Processing)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Rejected | Self::Cancelled | Self::CreateFailed
        )
    }
}

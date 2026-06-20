use crate::NotaryServiceError;
use sdkwork_utils_rust::is_blank;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotaryServiceContract {
    pub domain: &'static str,
    pub service_name: &'static str,
    pub write_commands: Vec<&'static str>,
    pub read_queries: Vec<&'static str>,
    pub ports: Vec<&'static str>,
    pub requires_idempotency_for_writes: bool,
}

impl NotaryServiceContract {
    pub fn new(
        domain: &'static str,
        service_name: &'static str,
        write_commands: Vec<&'static str>,
        read_queries: Vec<&'static str>,
        ports: Vec<&'static str>,
        requires_idempotency_for_writes: bool,
    ) -> Self {
        Self {
            domain,
            service_name,
            write_commands,
            read_queries,
            ports,
            requires_idempotency_for_writes,
        }
    }

    pub fn validate(&self) -> Result<(), NotaryServiceError> {
        if is_blank(Some(self.domain)) {
            return Err(NotaryServiceError::validation("service domain is required"));
        }
        if !self.service_name.starts_with("notary.") {
            return Err(NotaryServiceError::validation(
                "service_name must start with notary.",
            ));
        }
        if self.write_commands.is_empty() {
            return Err(NotaryServiceError::validation(
                "service contract requires write commands",
            ));
        }
        if self.read_queries.is_empty() {
            return Err(NotaryServiceError::validation(
                "service contract requires read queries",
            ));
        }
        if self.ports.is_empty() {
            return Err(NotaryServiceError::validation(
                "service contract requires ports",
            ));
        }
        if !self.requires_idempotency_for_writes {
            return Err(NotaryServiceError::validation(
                "notary write commands require idempotency",
            ));
        }

        Ok(())
    }
}

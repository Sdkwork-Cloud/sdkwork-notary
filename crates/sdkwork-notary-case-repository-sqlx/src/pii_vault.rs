use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_utils_rust::{base64_decode, base64_encode, sha256_hash};

const VAULT_PREFIX: &str = "notary-vault:v1:";

pub struct PiiVault {
    cipher: Aes256Gcm,
}

impl PiiVault {
    pub fn for_tenant(tenant_id: &str) -> Result<Self, NotaryServiceError> {
        let key_material = match std::env::var("NOTARY_PII_VAULT_KEY") {
            Ok(value) if !value.is_empty() => value,
            Ok(_) | Err(_) if allows_dev_pii_vault_key() => {
                format!("sdkwork-notary-dev:{tenant_id}")
            }
            Ok(_) | Err(_) => {
                return Err(NotaryServiceError::storage(
                    "NOTARY_PII_VAULT_KEY is required outside development and test environments",
                ));
            }
        };
        let mut key = [0u8; 32];
        let bytes = key_material.as_bytes();
        let len = bytes.len().min(32);
        key[..len].copy_from_slice(&bytes[..len]);
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, NotaryServiceError> {
        let nonce_bytes = random_nonce();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
        let mut payload = nonce_bytes.to_vec();
        payload.extend(ciphertext);
        Ok(format!("{VAULT_PREFIX}{}", base64_encode(&payload)))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, NotaryServiceError> {
        let encoded_body = encoded
            .strip_prefix(VAULT_PREFIX)
            .ok_or_else(|| NotaryServiceError::storage("unsupported pii vault payload"))?;
        let payload = base64_decode(encoded_body)
            .ok_or_else(|| NotaryServiceError::storage("invalid pii vault payload"))?;
        if payload.len() < 12 {
            return Err(NotaryServiceError::storage("invalid pii vault payload"));
        }
        let (nonce_bytes, ciphertext) = payload.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
        String::from_utf8(plaintext).map_err(|error| NotaryServiceError::storage(error.to_string()))
    }
}

fn random_nonce() -> [u8; 12] {
    use aes_gcm::aead::rand_core::RngCore;
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

fn allows_dev_pii_vault_key() -> bool {
    notary_runtime_environment_allows_dev_fallback()
}

fn notary_runtime_environment_allows_dev_fallback() -> bool {
    matches!(
        std::env::var("SDKWORK_NOTARY_ENVIRONMENT")
            .or_else(|_| std::env::var("SDKWORK_ENVIRONMENT"))
            .unwrap_or_else(|_| "development".to_owned())
            .to_ascii_lowercase()
            .as_str(),
        "development" | "dev" | "test" | "local"
    )
}

pub fn identity_fingerprint(value: &str) -> String {
    format!("notary-sha256:{}", sha256_hash(value.as_bytes()))
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::{identity_fingerprint, PiiVault};

    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn pii_vault_round_trips_sensitive_values() {
        let _guard = ENV_TEST_LOCK.lock().expect("env test lock");
        let vault = PiiVault::for_tenant("tenant-1").expect("vault");
        let encrypted = vault.encrypt("110101199001011234").expect("encrypt");
        assert!(encrypted.starts_with("notary-vault:v1:"));
        let decrypted = vault.decrypt(&encrypted).expect("decrypt");
        assert_eq!(decrypted, "110101199001011234");
    }

    #[test]
    fn pii_vault_requires_key_outside_dev_environments() {
        let _guard = ENV_TEST_LOCK.lock().expect("env test lock");
        let previous_env = std::env::var("SDKWORK_NOTARY_ENVIRONMENT").ok();
        let previous_key = std::env::var("NOTARY_PII_VAULT_KEY").ok();
        std::env::set_var("SDKWORK_NOTARY_ENVIRONMENT", "production");
        std::env::remove_var("NOTARY_PII_VAULT_KEY");

        match PiiVault::for_tenant("tenant-1") {
            Ok(_) => panic!("production vault must fail without NOTARY_PII_VAULT_KEY"),
            Err(error) => assert!(error.message().contains("NOTARY_PII_VAULT_KEY is required")),
        }

        if let Some(value) = previous_env {
            std::env::set_var("SDKWORK_NOTARY_ENVIRONMENT", value);
        } else {
            std::env::remove_var("SDKWORK_NOTARY_ENVIRONMENT");
        }
        if let Some(value) = previous_key {
            std::env::set_var("NOTARY_PII_VAULT_KEY", value);
        }
    }

    #[test]
    fn identity_fingerprint_is_stable() {
        assert_eq!(
            identity_fingerprint("110101199001011234"),
            identity_fingerprint("110101199001011234")
        );
    }
}

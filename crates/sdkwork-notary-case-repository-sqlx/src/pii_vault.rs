use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use sdkwork_notary_case_contract::NotaryServiceError;

const VAULT_PREFIX: &str = "notary-vault:v1:";

pub struct PiiVault {
    cipher: Aes256Gcm,
}

impl PiiVault {
    pub fn for_tenant(tenant_id: &str) -> Result<Self, NotaryServiceError> {
        let key_material = std::env::var("NOTARY_PII_VAULT_KEY")
            .unwrap_or_else(|_| format!("sdkwork-notary-dev:{tenant_id}"));
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
        Ok(format!("{VAULT_PREFIX}{}", STANDARD.encode(payload)))
    }

    pub fn decrypt(&self, encoded: &str) -> Result<String, NotaryServiceError> {
        let encoded_body = encoded
            .strip_prefix(VAULT_PREFIX)
            .ok_or_else(|| NotaryServiceError::storage("unsupported pii vault payload"))?;
        let payload = STANDARD
            .decode(encoded_body)
            .map_err(|error| NotaryServiceError::storage(error.to_string()))?;
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

pub fn identity_fingerprint(value: &str) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(value.as_bytes());
    format!("notary-sha256:{digest:064x}")
}

#[cfg(test)]
mod tests {
    use super::{identity_fingerprint, PiiVault};

    #[test]
    fn pii_vault_round_trips_sensitive_values() {
        let vault = PiiVault::for_tenant("tenant-1").expect("vault");
        let encrypted = vault.encrypt("110101199001011234").expect("encrypt");
        assert!(encrypted.starts_with("notary-vault:v1:"));
        let decrypted = vault.decrypt(&encrypted).expect("decrypt");
        assert_eq!(decrypted, "110101199001011234");
    }

    #[test]
    fn identity_fingerprint_is_stable() {
        assert_eq!(
            identity_fingerprint("110101199001011234"),
            identity_fingerprint("110101199001011234")
        );
    }
}

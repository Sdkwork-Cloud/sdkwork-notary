use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_utils_rust::{
    base64_decode, base64_encode, DEFAULT_LIST_PAGE_SIZE, MAX_LIST_PAGE_SIZE,
};

const KEYSET_CURSOR_PREFIX: &str = "nsk1:";

/// Clamp list `page_size` to the platform contract (`PAGINATION_SPEC` / `API_SPEC` §14.1).
pub fn validated_list_page_size(page_size: i64) -> i64 {
    page_size.clamp(1, i64::from(MAX_LIST_PAGE_SIZE))
}

pub fn default_list_page_size() -> i64 {
    i64::from(DEFAULT_LIST_PAGE_SIZE)
}

/// Encode an opaque composite keyset cursor for `(sort_value, id)` seek pagination.
pub fn encode_keyset_cursor(sort_value: &str, id: &str) -> String {
    let payload = serde_json::json!({
        "u": sort_value,
        "i": id,
    });
    format!(
        "{KEYSET_CURSOR_PREFIX}{}",
        base64_encode(payload.to_string().as_bytes())
    )
}

/// Decode an opaque composite keyset cursor.
pub fn decode_keyset_cursor(cursor: Option<&str>) -> Result<Option<(String, String)>, NotaryServiceError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let encoded = cursor
        .strip_prefix(KEYSET_CURSOR_PREFIX)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?;
    let bytes = base64_decode(encoded)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?;
    let payload: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| NotaryServiceError::validation("invalid list cursor"))?;
    let sort_value = payload
        .get("u")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?
        .to_string();
    let id = payload
        .get("i")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?
        .to_string();
    Ok(Some((sort_value, id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_keyset_cursor_round_trips() {
        let encoded = encode_keyset_cursor("2026-01-01T00:00:00Z", "case-1");
        let decoded = decode_keyset_cursor(Some(&encoded)).expect("decode cursor");
        assert_eq!(
            decoded,
            Some((
                "2026-01-01T00:00:00Z".to_string(),
                "case-1".to_string()
            ))
        );
    }

    #[test]
    fn validated_list_page_size_clamps_to_platform_max() {
        assert_eq!(validated_list_page_size(0), 1);
        assert_eq!(validated_list_page_size(20), 20);
        assert_eq!(validated_list_page_size(500), i64::from(MAX_LIST_PAGE_SIZE));
    }
}

use sdkwork_notary_case_contract::NotaryServiceError;
use sdkwork_utils_rust::{base64_decode, base64_encode, DEFAULT_LIST_PAGE_SIZE};

const KEYSET_CURSOR_PREFIX: &str = "nsk1:";
const OFFSET_CURSOR_PREFIX: &str = "nof1:";
pub const NOTARY_MAX_LIST_PAGE_SIZE: i64 = 100;

/// Validate list `page_size` against the platform contract (`PAGINATION_SPEC` / `API_SPEC` section 14.1).
pub fn validated_list_page_size(page_size: i64) -> Result<i64, NotaryServiceError> {
    if !(1..=NOTARY_MAX_LIST_PAGE_SIZE).contains(&page_size) {
        return Err(NotaryServiceError::validation(format!(
            "page_size must be between 1 and {NOTARY_MAX_LIST_PAGE_SIZE}"
        )));
    }
    Ok(page_size)
}

pub fn default_list_page_size() -> i64 {
    i64::from(DEFAULT_LIST_PAGE_SIZE)
}

/// Encode an opaque continuation token for an offset-backed dependency.
pub fn encode_offset_cursor(offset: i64) -> String {
    format!(
        "{OFFSET_CURSOR_PREFIX}{}",
        base64_encode(offset.to_string().as_bytes())
    )
}

/// Decode an opaque offset token without exposing numeric cursor semantics to clients.
pub fn decode_offset_cursor(cursor: Option<&str>) -> Result<i64, NotaryServiceError> {
    let Some(cursor) = cursor.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(0);
    };
    let encoded = cursor
        .strip_prefix(OFFSET_CURSOR_PREFIX)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?;
    let bytes = base64_decode(encoded)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?;
    let offset = std::str::from_utf8(&bytes)
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .filter(|value| *value >= 0)
        .ok_or_else(|| NotaryServiceError::validation("invalid list cursor"))?;
    Ok(offset)
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
pub fn decode_keyset_cursor(
    cursor: Option<&str>,
) -> Result<Option<(String, String)>, NotaryServiceError> {
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
            Some(("2026-01-01T00:00:00Z".to_string(), "case-1".to_string()))
        );
    }

    #[test]
    fn offset_cursor_is_opaque_and_rejects_numeric_aliases() {
        let encoded = encode_offset_cursor(40);
        assert!(encoded.starts_with(OFFSET_CURSOR_PREFIX));
        assert_eq!(decode_offset_cursor(Some(&encoded)).unwrap(), 40);
        assert!(decode_offset_cursor(Some("40")).is_err());
        assert!(decode_offset_cursor(Some("nof1:not-base64")).is_err());
    }

    #[test]
    fn validated_list_page_size_rejects_values_outside_notary_contract_bounds() {
        assert!(validated_list_page_size(0).is_err());
        assert_eq!(validated_list_page_size(20).unwrap(), 20);
        assert_eq!(validated_list_page_size(100).unwrap(), 100);
        assert!(validated_list_page_size(101).is_err());
    }
}

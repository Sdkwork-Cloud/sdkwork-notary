use std::collections::BTreeMap;

use crate::response::NotaryRouteError;

const NOTARY_MAX_LIST_PAGE_SIZE: i64 = 100;

const FORBIDDEN_PAGINATION_QUERY_KEYS: &[&str] = &[
    "pageSize",
    "page-size",
    "pagesize",
    "limit",
    "offset",
    "page_no",
    "pageNo",
    "per_page",
    "size",
    "page_token",
    "pageToken",
];

/// Validate canonical cursor pagination before service dispatch.
pub fn validate_list_query(query: &BTreeMap<String, String>) -> Result<(), NotaryRouteError> {
    if let Some(alias) = FORBIDDEN_PAGINATION_QUERY_KEYS
        .iter()
        .copied()
        .find(|key| query.contains_key(*key))
    {
        return Err(NotaryRouteError::invalid_parameter(format!(
            "query parameter `{alias}` is not supported; use `page_size` or `cursor` for pagination"
        )));
    }

    if query.contains_key("page") {
        return Err(NotaryRouteError::invalid_parameter(
            "query parameter `page` is not supported for this cursor-paginated operation",
        ));
    }

    let page_size = parse_integer(
        query,
        "page_size",
        "`page_size` must be an integer between 1 and 100",
    )?;
    if page_size.is_some_and(|value| !(1..=NOTARY_MAX_LIST_PAGE_SIZE).contains(&value)) {
        return Err(NotaryRouteError::invalid_parameter(
            "`page_size` must be between 1 and 100",
        ));
    }

    if let Some(cursor) = query.get("cursor") {
        let cursor = cursor.trim();
        if cursor.is_empty() {
            return Err(NotaryRouteError::invalid_parameter(
                "`cursor` must be a non-empty continuation token",
            ));
        }
        if is_numeric_offset_cursor(cursor) {
            return Err(NotaryRouteError::invalid_parameter(
                "`cursor` must be an opaque continuation token",
            ));
        }
    }

    Ok(())
}

fn is_numeric_offset_cursor(cursor: &str) -> bool {
    cursor.chars().all(|character| character.is_ascii_digit())
        || cursor.split_once(':').is_some_and(|(offset, skip)| {
            !offset.is_empty()
                && !skip.is_empty()
                && offset.chars().all(|character| character.is_ascii_digit())
                && skip.chars().all(|character| character.is_ascii_digit())
        })
}

fn parse_integer(
    query: &BTreeMap<String, String>,
    key: &str,
    message: &'static str,
) -> Result<Option<i64>, NotaryRouteError> {
    query
        .get(key)
        .map(|value| {
            value
                .parse::<i64>()
                .map_err(|_| NotaryRouteError::invalid_parameter(message))
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_utils_rust::SdkWorkResultCode;

    #[test]
    fn rejects_forbidden_pagination_aliases() {
        for alias in FORBIDDEN_PAGINATION_QUERY_KEYS {
            let query = BTreeMap::from([(alias.to_string(), "20".to_string())]);
            let error = validate_list_query(&query).expect_err("alias must be rejected");
            assert_eq!(error.result_code, SdkWorkResultCode::InvalidParameter);
        }
    }

    #[test]
    fn rejects_invalid_ranges_types_and_mode_combinations() {
        for query in [
            BTreeMap::from([("page_size".to_string(), "0".to_string())]),
            BTreeMap::from([("page_size".to_string(), "101".to_string())]),
            BTreeMap::from([("page_size".to_string(), "many".to_string())]),
            BTreeMap::from([("page".to_string(), "0".to_string())]),
            BTreeMap::from([("page".to_string(), "first".to_string())]),
            BTreeMap::from([
                ("page".to_string(), i64::MAX.to_string()),
                ("page_size".to_string(), "100".to_string()),
            ]),
            BTreeMap::from([
                ("page".to_string(), "1".to_string()),
                ("cursor".to_string(), "next-page".to_string()),
            ]),
            BTreeMap::from([("cursor".to_string(), " ".to_string())]),
            BTreeMap::from([("cursor".to_string(), "40".to_string())]),
            BTreeMap::from([("cursor".to_string(), "40:3".to_string())]),
        ] {
            let error = validate_list_query(&query).expect_err("query must be rejected");
            assert_eq!(error.result_code, SdkWorkResultCode::InvalidParameter);
        }
    }

    #[test]
    fn accepts_canonical_pagination_and_domain_filters() {
        let query = BTreeMap::from([
            ("page_size".to_string(), "100".to_string()),
            ("cursor".to_string(), "nsk1:opaque".to_string()),
            ("status".to_string(), "processing".to_string()),
            ("q".to_string(), "contract".to_string()),
        ]);

        validate_list_query(&query).expect("canonical query");
    }
}

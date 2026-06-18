# sdkwork-router-notary-http-auth

Shared HTTP authentication layer for Notary route crates.

## Responsibilities

- Mount dual-token `WebRequestContext` resolution through `sdkwork-web-axum` and `sdkwork-iam-web-adapter`.
- Project `WebRequestContext` into `NotaryRequestContext` for service ports.
- Provide test fixtures for route crate integration tests.

## Usage

Route crates call `with_dual_token_request_context(router, manifest)` from `layer.rs` and map handlers with `notary_request_context_from_web`.

## Membership resolution

`NotaryRequestContext.membership_id` is resolved from explicit scope tokens when present, otherwise derived with the IAM-compatible `orgmem_{organization}_{user}` stable local identifier when an organization context exists.

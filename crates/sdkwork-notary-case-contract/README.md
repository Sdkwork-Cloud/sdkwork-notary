# sdkwork-notary-case-contract

Core domain types for SDKWork Notary.

This crate owns storage-agnostic Notary model types, status conversion, service contract metadata, and typed service errors. It deliberately has no HTTP framework, SDK transport, SQL, Drive provider, or Commerce provider dependency.

It is consumed by `sdkwork-notary-case-service` and future app/backend route crates.

## Verification

```powershell
cargo test -p sdkwork-notary-case-service --target-dir target-codex-test
```

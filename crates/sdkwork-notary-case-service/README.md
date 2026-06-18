# sdkwork-notary-case-service

Runtime orchestration for SDKWork Notary.

This crate implements the first service-use-case boundary for Notary:

- verifies Appbase/IAM organization members before Notary business actions;
- opens Notary business by creating a Drive `space_type='notary'` space before writing the organization profile;
- creates a Notary case by composing Commerce SKU/order/order item creation with Drive case-folder creation;
- lists case files through Drive using denormalized `space_type='notary'`, `space_id`, and `parent_node_id`;
- writes only Notary-owned case, party, profile, assignment, and event facts through a repository port.

The crate uses ports for Appbase, Commerce, Drive, and Notary storage. It does not call raw HTTP, parse auth headers, or write dependency-owned tables.

## Verification

```powershell
cargo test -p sdkwork-notary-case-service --target-dir target-codex-test
```

# SDKWork Local Dictionary

This directory holds repository-local agent skills, plugins, manifests, and workspace metadata for `sdkwork-notary`.

Canonical SDKWork standards remain in `../sdkwork-specs/`. Read `../AGENTS.md` before adding local skills or plugins here.

Authoritative paths:

- `.sdkwork/skills/`: repository-local agent skills
- `.sdkwork/plugins/`: repository-local agent plugins
- `../specs/`, `../apis/`, `../sdks/`, and `../crates/`: contract and implementation ownership

Local-only state belongs under ignored paths listed in `.sdkwork/.gitignore`.

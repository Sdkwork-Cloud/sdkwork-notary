# Applications

`sdkwork-notary` is a **domain library** repository. It does not ship a standalone runnable application root under `apps/`.

Host applications (for example IM PC or a dedicated notary client) wire `sdkwork-router-notary-*` route crates and consume `sdkwork-notary-*` SDK families.

Application identity for catalog and release metadata lives at the repository root in `sdkwork.app.config.json`.

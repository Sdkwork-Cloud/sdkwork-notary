# SDKWork Notary App SDK

User-facing SDK family for Notary workflows.

- API authority: `sdkwork-notary.app`
- API prefix: `/app/v3/api`
- Owner: `sdkwork-notary`
- Dependency SDKs: `sdkwork-iam-app-sdk`, `sdkwork-catalog-app-sdk`, `sdkwork-order-app-sdk`, `sdkwork-drive-app-sdk`
- TypeScript package root: `@sdkwork/notary-app-sdk`
- Package entry: `sdkwork-notary-app-sdk-typescript/src/index.ts`
- Composed facade: `sdkwork-notary-app-sdk-typescript/composed/index.ts`

The generated transport must include only Notary-owned paths. Upload, file listing, node metadata, and download packages are delegated to Drive SDKs through the composed facade. The package root exports the generated client, generated types, and the composed `createNotaryApi` workflow facade for application integration.

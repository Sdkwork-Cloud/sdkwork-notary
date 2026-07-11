# SDKWork Notary Backend SDK

Operator SDK family for Notary management workflows.

- API authority: `sdkwork-notary.backend`
- API prefix: `/backend/v3/api`
- Owner: `sdkwork-notary`
- Dependency SDKs: `sdkwork-iam-backend-sdk`, `sdkwork-drive-backend-sdk`
- TypeScript package root: `@sdkwork/notary-backend-sdk`
- Package entry: `sdkwork-notary-backend-sdk-typescript/src/index.ts`
- Composed facade: `sdkwork-notary-backend-sdk-typescript/composed/index.ts`

Backend SDK consumers manage notary enablement, Merchandise-backed matter definitions, and case workflow. IAM member, role, position, department, Merchandise SPU/SKU, Order checkout/order items, Payment execution, and Drive space/node capabilities remain owner-domain concerns; this SDK exposes only Notary orchestration. The package root exports the generated backend client, generated types, and the composed `createNotaryBackendApi` management facade.

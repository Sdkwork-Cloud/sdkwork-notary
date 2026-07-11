# SDKWork Notary PC Admin Shell

Internal backend-admin shell for the Notary PC application. It owns the `/admin` layout, navigation, permission-aware route composition, and lazy loading of admin capability packages.

The shell does not construct SDK clients or own business services. Runtime setup is supplied by `@sdkwork/notary-pc-admin-core` and business behavior is supplied by capability packages.

- Component contract: `specs/component.spec.json`
- Canonical route and package rules: `../../../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`
- Backend-admin rules: `../../../../../sdkwork-specs/BACKEND_UI_SPEC.md`

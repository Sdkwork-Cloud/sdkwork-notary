import path from 'node:path';
import { fileURLToPath } from 'node:url';
import react from '@vitejs/plugin-react';
import { defineConfig, loadEnv } from 'vite';

const h5Root = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(h5Root, '../..');
const workspaceRoot = path.resolve(repoRoot, '..');
const sdkCommonRoot = path.resolve(
  workspaceRoot,
  'sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);
const utilsRoot = path.resolve(workspaceRoot, 'sdkwork-utils/packages/sdkwork-utils-typescript/src');
const generatedDriveAppSdkEntry = path.resolve(
  workspaceRoot,
  'sdkwork-drive/sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts',
);
const generatedAppbaseAppSdkEntry = path.resolve(
  workspaceRoot,
  'sdkwork-iam/sdks/sdkwork-iam-app-sdk/sdkwork-iam-app-sdk-typescript/generated/server-openapi/src/index.ts',
);

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, h5Root, '');

  return {
    plugins: [react()],
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    resolve: {
      alias: {
        '@sdkwork/utils/string': path.resolve(utilsRoot, 'string.ts'),
        '@sdkwork/utils': path.resolve(utilsRoot, 'index.ts'),
        '@sdkwork/sdk-common': path.resolve(sdkCommonRoot, 'index.ts'),
        '@sdkwork/notary-h5-core': path.resolve(h5Root, 'packages/sdkwork-notary-h5-core/src/index.ts'),
        '@sdkwork/notary-h5-commons': path.resolve(h5Root, 'packages/sdkwork-notary-h5-commons/src/index.ts'),
        '@sdkwork/notary-h5-shell': path.resolve(h5Root, 'packages/sdkwork-notary-h5-shell/src/index.ts'),
        '@sdkwork/notary-h5-notary': path.resolve(h5Root, 'packages/sdkwork-notary-h5-notary/src/index.ts'),
        '@sdkwork/notary-app-sdk': path.resolve(
          repoRoot,
          'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts',
        ),
        '@sdkwork/drive-app-sdk': generatedDriveAppSdkEntry,
        '@sdkwork/iam-app-sdk': generatedAppbaseAppSdkEntry,
      },
    },
    optimizeDeps: {
      exclude: [
        '@sdkwork/notary-app-sdk',
        '@sdkwork/drive-app-sdk',
        '@sdkwork/iam-app-sdk',
        '@sdkwork/sdk-common',
        '@sdkwork/utils',
      ],
    },
    server: {
      port: 5185,
    },
  };
});

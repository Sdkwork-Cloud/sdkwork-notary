import path from 'node:path';
import { fileURLToPath } from 'node:url';
import react from '@vitejs/plugin-react';
import { defineConfig, loadEnv } from 'vite';

const pcRoot = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, pcRoot, '');

  return {
    plugins: [react()],
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    resolve: {
      alias: {
        '@sdkwork/notary-pc-core': path.resolve(pcRoot, 'packages/sdkwork-notary-pc-core/src/index.ts'),
        '@sdkwork/notary-pc-commons': path.resolve(pcRoot, 'packages/sdkwork-notary-pc-commons/src/index.ts'),
        '@sdkwork/notary-pc-shell': path.resolve(pcRoot, 'packages/sdkwork-notary-pc-shell/src/index.ts'),
        '@sdkwork/notary-pc-notary': path.resolve(pcRoot, 'packages/sdkwork-notary-pc-notary/src/index.ts'),
        '@sdkwork/notary-app-sdk': path.resolve(
          pcRoot,
          '../../sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts',
        ),
      },
    },
    optimizeDeps: {
      exclude: ['@sdkwork/notary-app-sdk'],
    },
    server: {
      port: 5285,
    },
  };
});

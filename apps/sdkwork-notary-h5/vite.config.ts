import path from 'node:path';
import { fileURLToPath } from 'node:url';
import react from '@vitejs/plugin-react';
import { defineConfig, loadEnv } from 'vite';

const h5Root = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, h5Root, '');

  return {
    plugins: [react()],
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    resolve: {
      alias: {
        '@sdkwork/notary-h5-core': path.resolve(h5Root, 'packages/sdkwork-notary-h5-core/src/index.ts'),
        '@sdkwork/notary-h5-commons': path.resolve(h5Root, 'packages/sdkwork-notary-h5-commons/src/index.ts'),
        '@sdkwork/notary-h5-shell': path.resolve(h5Root, 'packages/sdkwork-notary-h5-shell/src/index.ts'),
        '@sdkwork/notary-h5-notary': path.resolve(h5Root, 'packages/sdkwork-notary-h5-notary/src/index.ts'),
        '@sdkwork/notary-app-sdk': path.resolve(
          h5Root,
          '../../sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts',
        ),
      },
    },
    server: {
      port: 5185,
    },
  };
});

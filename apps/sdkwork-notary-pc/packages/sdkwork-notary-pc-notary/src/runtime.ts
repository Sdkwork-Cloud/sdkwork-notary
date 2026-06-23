import type { NotaryPcHostAdapter } from '@sdkwork/notary-pc-commons';
import { configureNotaryPcHost } from '@sdkwork/notary-pc-commons';
import type { NotaryPcSdkPorts } from '@sdkwork/notary-pc-core';
import { configureNotaryPcSdkPorts } from '@sdkwork/notary-pc-core';

import { resetNotaryService } from './services/NotaryService';

export interface ConfigureNotaryPcRuntimeOptions {
  host: NotaryPcHostAdapter;
  sdkPorts: NotaryPcSdkPorts;
}

export function configureNotaryPcRuntime(options: ConfigureNotaryPcRuntimeOptions): void {
  configureNotaryPcHost(options.host);
  configureNotaryPcSdkPorts(options.sdkPorts);
  resetNotaryService();
}

export { configureNotaryPcRuntime as configureNotaryPcRuntimeFromNotaryPackage };

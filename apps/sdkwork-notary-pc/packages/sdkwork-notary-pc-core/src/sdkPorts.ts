import type { SdkworkNotaryAppClient } from '@sdkwork/notary-app-sdk';

export interface NotaryPcSdkPorts {
  getNotaryClient: () => SdkworkNotaryAppClient;
  getDriveClient: () => unknown;
  getAppbaseClient: () => unknown;
}

let sdkPorts: NotaryPcSdkPorts | null = null;

export function configureNotaryPcSdkPorts(ports: NotaryPcSdkPorts): void {
  sdkPorts = ports;
}

export function getNotaryPcSdkPorts(): NotaryPcSdkPorts {
  if (!sdkPorts) {
    throw new Error('Notary PC SDK ports are not configured. Call configureNotaryPcSdkPorts first.');
  }
  return sdkPorts;
}

export function getConfiguredNotaryAppSdkClient(): SdkworkNotaryAppClient {
  return getNotaryPcSdkPorts().getNotaryClient();
}

export function getConfiguredDriveAppSdkClient(): unknown {
  return getNotaryPcSdkPorts().getDriveClient();
}

export function getConfiguredAppbaseAppSdkClient(): unknown {
  return getNotaryPcSdkPorts().getAppbaseClient();
}

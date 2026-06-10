import {
  createClient as createGeneratedNotaryBackendClient,
  SdkworkBackendClient,
} from "../generated/server-openapi/src/index";
import type { SdkworkBackendConfig } from "../generated/server-openapi/src/types/common";

export { SdkworkBackendClient, createGeneratedNotaryBackendClient };
export type { SdkworkBackendConfig };
export * from "../generated/server-openapi/src/types";
export * from "../generated/server-openapi/src/api";
export * from "../generated/server-openapi/src/http";
export * from "../generated/server-openapi/src/auth";
export * from "../composed/index";

export type SdkworkNotaryBackendClient = SdkworkBackendClient;

export function createNotaryBackendClient(
  config: SdkworkBackendConfig,
): SdkworkNotaryBackendClient {
  return createGeneratedNotaryBackendClient(config);
}

export function createClient(config: SdkworkBackendConfig): SdkworkNotaryBackendClient {
  return createNotaryBackendClient(config);
}

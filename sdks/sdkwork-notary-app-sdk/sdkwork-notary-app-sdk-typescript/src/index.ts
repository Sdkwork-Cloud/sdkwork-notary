import {
  createClient as createGeneratedNotaryAppClient,
  SdkworkAppClient,
} from "../generated/server-openapi/src/index";
import type { SdkworkAppConfig } from "../generated/server-openapi/src/types/common";

export { SdkworkAppClient, createGeneratedNotaryAppClient };
export type { SdkworkAppConfig };
export * from "../generated/server-openapi/src/types";
export * from "../generated/server-openapi/src/api";
export * from "../generated/server-openapi/src/http";
export * from "../generated/server-openapi/src/auth";
export * from "../composed/index";
export {
  createNotaryApi,
  type AppbaseAppSdkPort,
  type CaseCommandInput,
  type CommerceAppSdkPort,
  type CreateCaseInput,
  type CreateNotaryApiOptions,
  type DriveAppSdkPort,
  type ListCaseFilesInput,
  type ListCasesInput,
  type ListStaffInput,
  type ListCaseEventsInput,
  type GetMonthlyReportInput,
  type NotaryMatterOption,
  type NotaryAppSdkPort,
  type UploadCaseFileInput,
} from "../composed/index";

export type SdkworkNotaryAppClient = SdkworkAppClient;

export function createNotaryAppClient(config: SdkworkAppConfig): SdkworkNotaryAppClient {
  return createGeneratedNotaryAppClient(config);
}

export function createClient(config: SdkworkAppConfig): SdkworkNotaryAppClient {
  return createNotaryAppClient(config);
}

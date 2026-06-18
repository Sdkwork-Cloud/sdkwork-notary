export interface NotaryAppSdkPort {
  access: {
    retrieve(input?: unknown): Promise<unknown>;
  };
  matters: {
    list(input?: unknown): Promise<unknown>;
  };
  staff: {
    list(input?: unknown): Promise<unknown>;
  };
  cases: {
    create(input: unknown, options?: unknown): Promise<unknown>;
    list(input?: unknown): Promise<unknown>;
    retrieve(caseId: string): Promise<unknown>;
    update(caseId: string, input: unknown): Promise<unknown>;
    assignments: {
      create(caseId: string, input: unknown): Promise<unknown>;
    };
    acceptances: {
      create(caseId: string, input: unknown): Promise<unknown>;
    };
    rejections: {
      create(caseId: string, input: unknown): Promise<unknown>;
    };
    completions: {
      create(caseId: string, input: unknown): Promise<unknown>;
    };
    parties: {
      list(caseId: string, input?: unknown): Promise<unknown>;
      create(caseId: string, input: unknown): Promise<unknown>;
      update(caseId: string, partyId: string, input: unknown): Promise<unknown>;
      delete(caseId: string, partyId: string): Promise<void>;
      signatures: {
        create(caseId: string, partyId: string, input: unknown): Promise<unknown>;
      };
      videoInvites: {
        create(caseId: string, partyId: string, input: unknown): Promise<unknown>;
      };
      signatureInvites: {
        create(caseId: string, partyId: string, input: unknown): Promise<unknown>;
      };
    };
    files: {
      list(caseId: string, input?: unknown): Promise<unknown>;
      create(caseId: string, input: unknown): Promise<unknown>;
    };
    downloadPackages: {
      create(caseId: string, input: unknown): Promise<unknown>;
    };
  };
}

export interface DriveAppSdkPort {
  spaces?: {
    create(input: unknown): Promise<unknown>;
  };
  nodes?: {
    create?(input: unknown): Promise<unknown>;
    list?(input: unknown): Promise<unknown>;
    delete?(nodeId: string, input?: unknown): Promise<unknown>;
    downloadUrls?: {
      create(nodeId: string, input?: unknown): Promise<unknown>;
    };
    files?: {
      create(input: unknown): Promise<unknown>;
    };
  };
  drive?: {
    nodes?: {
      delete?(nodeId: string, input?: unknown): Promise<unknown>;
      downloadUrls?: {
        create(nodeId: string, input?: unknown): Promise<unknown>;
      };
      files?: {
        create(input: unknown): Promise<unknown>;
      };
      list?(spaceId: string, input?: unknown): Promise<unknown>;
    };
    downloadUrls?: {
      create(input: unknown): Promise<unknown>;
    };
    trash?: {
      move?(nodeId: string, input?: unknown): Promise<unknown>;
    };
  };
  downloadUrls?: {
    create(input: unknown): Promise<unknown>;
  };
  trash?: {
    move?(nodeId: string, input?: unknown): Promise<unknown>;
  };
  uploader?: {
    upload(input: unknown): Promise<unknown>;
  };
  nodeProperties?: {
    set(nodeId: string, input: unknown): Promise<unknown>;
  };
}

export interface CommerceAppSdkPort {
  checkout?: {
    create(input: unknown): Promise<unknown>;
  };
  orders?: {
    retrieve(orderId: string): Promise<unknown>;
  };
  catalog?: {
    skus?: {
      retrieve(skuId: string): Promise<unknown>;
    };
  };
}

export interface AppbaseAppSdkPort {
  iam?: {
    organizations?: {
      current?: {
        retrieve(input?: unknown): Promise<unknown>;
      };
    };
  };
}

export interface CreateNotaryApiOptions {
  notary: NotaryAppSdkPort;
  drive: DriveAppSdkPort;
  commerce?: CommerceAppSdkPort;
  appbase: AppbaseAppSdkPort;
}

export interface CreateCaseInput {
  skuId: string;
  title: string;
  applicantName: string;
  description?: string;
  remarks?: string;
  parties?: unknown[];
  driveFolderName?: string;
  primaryNotaryMembershipId?: string;
  idempotencyKey?: string;
}

export interface ListStaffInput {
  staffRole?: "notary" | "assistant" | "reviewer" | "approver";
  q?: string;
  pageSize?: number;
  cursor?: string;
}

export interface ListCasesInput {
  status?: string;
  skuId?: string;
  q?: string;
  pageSize?: number;
  cursor?: string;
}

export interface ListCaseFilesInput {
  driveSpaceType?: "notary" | string;
  category?: "identity" | "evidence" | "notary";
  pageSize?: number;
  cursor?: string;
}

export interface AssignCaseInput {
  organizationMembershipId: string;
  assignmentRole: "primary_notary" | "assistant" | "reviewer" | "approver";
}

export interface UploadCaseFileInput {
  caseId: string;
  file: unknown;
  category: "identity" | "evidence" | "notary";
  materialCode?: string;
  partyId?: string;
  source?: string;
}

export interface DeleteCaseFileInput {
  nodeId: string;
  operatorId?: string;
  strategy?: "delete" | "trash";
}

export interface CreateCaseFileDownloadUrlInput {
  nodeId: string;
  requestedTtlSeconds?: number;
  expiresInSeconds?: number;
  disposition?: "inline" | "attachment";
}

export interface AttachPartySignatureInput {
  file?: unknown;
  signatureUrl?: string;
  signatureNodeId?: string;
  driveNodeId?: string;
  nodeId?: string;
  source?: string;
}

export interface CreatePartyVideoInviteInput {
  purpose?: "identity_verification" | "material_confirmation" | "remote_inquiry";
  expiresInSeconds?: number;
}

export interface CreatePartySignatureInviteInput {
  purpose?: "remote_signature" | "onsite_signature_confirmation" | "material_signature";
  expiresInSeconds?: number;
}

export interface CaseCommandInput {
  remarks?: string;
  reason?: string;
  result?: string;
}

export function createNotaryApi({ notary, drive, commerce, appbase }: CreateNotaryApiOptions) {
  async function getAccess() {
    return notary.access.retrieve();
  }

  async function listMatters(input?: unknown) {
    return notary.matters.list(input);
  }

  async function listStaff(input: ListStaffInput = {}) {
    return notary.staff.list({
      ...(input.staffRole ? { staffRole: input.staffRole } : {}),
      ...(input.q ? { q: input.q } : {}),
      ...(input.pageSize ? { pageSize: input.pageSize } : {}),
      ...(input.cursor ? { cursor: input.cursor } : {})
    });
  }

  async function createCase(input: CreateCaseInput) {
    const result = await notary.cases.create(
      {
        skuId: input.skuId,
        title: input.title,
        applicantName: input.applicantName,
        description: input.description,
        remarks: input.remarks,
        parties: input.parties,
        driveFolderName: input.driveFolderName,
        primaryNotaryMembershipId: input.primaryNotaryMembershipId
      },
      input.idempotencyKey ? { idempotencyKey: input.idempotencyKey } : {}
    );

    return result;
  }

  async function listCases(input: ListCasesInput = {}) {
    return notary.cases.list({
      ...(input.status ? { status: input.status } : {}),
      ...(input.skuId ? { skuId: input.skuId } : {}),
      ...(input.q ? { q: input.q } : {}),
      ...(input.pageSize ? { pageSize: input.pageSize } : {}),
      ...(input.cursor ? { cursor: input.cursor } : {})
    });
  }

  async function getCase(caseId: string) {
    return notary.cases.retrieve(caseId);
  }

  async function updateCase(caseId: string, input: unknown) {
    return notary.cases.update(caseId, input);
  }

  async function acceptCase(caseId: string, input: CaseCommandInput = {}) {
    return notary.cases.acceptances.create(caseId, input);
  }

  async function rejectCase(caseId: string, input: CaseCommandInput = {}) {
    return notary.cases.rejections.create(caseId, input);
  }

  async function completeCase(caseId: string, input: CaseCommandInput = {}) {
    return notary.cases.completions.create(caseId, input);
  }

  async function assignCase(caseId: string, input: AssignCaseInput) {
    return notary.cases.assignments.create(caseId, {
      organizationMembershipId: input.organizationMembershipId,
      assignmentRole: input.assignmentRole
    });
  }

  async function addParty(caseId: string, input: unknown) {
    return notary.cases.parties.create(caseId, input);
  }

  async function updateParty(caseId: string, partyId: string, input: unknown) {
    return notary.cases.parties.update(caseId, partyId, input);
  }

  async function deleteParty(caseId: string, partyId: string) {
    return notary.cases.parties.delete(caseId, partyId);
  }

  async function attachPartySignature(
    caseId: string,
    partyId: string,
    input: AttachPartySignatureInput,
  ) {
    let signatureNodeId = stringField(input, ["signatureNodeId", "signature_node_id", "driveNodeId", "nodeId"]);
    if (!signatureNodeId) {
      const uploaded = drive.uploader
        ? await drive.uploader.upload({
            file: resolveSignatureFile(input),
            profile: "image",
            scene: "notary.party_signature",
            source: input.source ?? "sdkwork-notary",
            target: {
              driveSpaceType: "notary",
              appResourceType: "notary_case_party_signature",
              appResourceId: `${caseId}:${partyId}`
            }
          })
        : await createDriveFileNode(drive, {
            nodeType: "file",
            driveSpaceType: "notary",
            appResourceType: "notary_case_party_signature",
            appResourceId: `${caseId}:${partyId}`,
            profile: "image",
            scene: "notary.party_signature",
            file: resolveSignatureFile(input)
          });
      signatureNodeId = resolveDriveUploadNodeId(uploaded);
    }

    return notary.cases.parties.signatures.create(caseId, partyId, {
      signatureNodeId,
      ...(input.signatureUrl ? { signatureUrl: input.signatureUrl } : {})
    });
  }

  async function createPartyVideoInvite(
    caseId: string,
    partyId: string,
    input: CreatePartyVideoInviteInput = {},
  ) {
    return notary.cases.parties.videoInvites.create(caseId, partyId, {
      purpose: input.purpose ?? "identity_verification",
      ...(input.expiresInSeconds ? { expiresInSeconds: input.expiresInSeconds } : {})
    });
  }

  async function createPartySignatureInvite(
    caseId: string,
    partyId: string,
    input: CreatePartySignatureInviteInput = {},
  ) {
    return notary.cases.parties.signatureInvites.create(caseId, partyId, {
      purpose: input.purpose ?? "remote_signature",
      ...(input.expiresInSeconds ? { expiresInSeconds: input.expiresInSeconds } : {})
    });
  }

  async function listCaseFiles(caseId: string, input: ListCaseFilesInput = {}) {
    return notary.cases.files.list(caseId, {
      driveSpaceType: "notary",
      ...(input.category ? { category: input.category } : {}),
      ...(input.pageSize ? { pageSize: input.pageSize } : {}),
      ...(input.cursor ? { cursor: input.cursor } : {})
    });
  }

  async function uploadCaseFile(input: UploadCaseFileInput) {
    const target = input.partyId
      ? {
          driveSpaceType: "notary",
          appResourceType: "notary_case_party_file",
          appResourceId: `${input.caseId}:${input.partyId}`
        }
      : {
          driveSpaceType: "notary",
          appResourceType: "notary_case",
          appResourceId: input.caseId
        };
    const uploaded = drive.uploader
      ? await drive.uploader.upload({
          file: resolveCaseFile(input.file),
          profile: "document",
          scene: "notary.case_file",
          source: input.source ?? "sdkwork-notary",
          target
        })
      : await createDriveFileNode(drive, {
          nodeType: "file",
          driveSpaceType: "notary",
          appResourceType: target.appResourceType,
          appResourceId: target.appResourceId,
          file: resolveCaseFile(input.file)
        });

    const driveNodeId = resolveDriveUploadNodeId(uploaded);
    return notary.cases.files.create(input.caseId, {
      driveNodeId,
      category: input.category,
      materialCode: input.materialCode,
      partyId: input.partyId,
      reviewStatus: "pending"
    });
  }

  async function createDownloadPackage(caseId: string, input: unknown) {
    return notary.cases.downloadPackages.create(caseId, input);
  }

  async function createCaseFileDownloadUrl(
    caseId: string,
    input: CreateCaseFileDownloadUrlInput,
  ) {
    const notaryCaseId = caseId.trim();
    if (!notaryCaseId) {
      throw new Error("caseId is required to create a notary case file download URL");
    }
    const nodeId = input.nodeId?.trim();
    if (!nodeId) {
      throw new Error("nodeId is required to create a notary case file download URL");
    }

    const response = await createDriveNodeDownloadUrl(drive, nodeId, {
      ...(input.requestedTtlSeconds ?? input.expiresInSeconds
        ? { requestedTtlSeconds: input.requestedTtlSeconds ?? input.expiresInSeconds }
        : {})
    });
    return normalizeDownloadUrlResponse(response);
  }

  async function deleteCaseFile(caseId: string, input: DeleteCaseFileInput) {
    const nodeId = input.nodeId?.trim();
    if (!nodeId) {
      throw new Error("nodeId is required to delete a notary case file");
    }

    await deleteDriveNode(drive, {
      nodeId,
      operatorId: input.operatorId,
      strategy: input.strategy ?? "delete"
    });
    return listCaseFiles(caseId);
  }

  return {
    notary,
    drive,
    commerce,
    appbase,
    getAccess,
    listMatters,
    listStaff,
    createCase,
    listCases,
    getCase,
    updateCase,
    acceptCase,
    rejectCase,
    completeCase,
    assignCase,
    addParty,
    updateParty,
    deleteParty,
    attachPartySignature,
    createPartyVideoInvite,
    createPartySignatureInvite,
    listCaseFiles,
    uploadCaseFile,
    createDownloadPackage,
    createCaseFileDownloadUrl,
    deleteCaseFile
  };
}

function asObject(value: unknown): Record<string, unknown> {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

async function createDriveFileNode(drive: DriveAppSdkPort, input: unknown): Promise<unknown> {
  if (drive.drive?.nodes?.files?.create) {
    return drive.drive.nodes.files.create(input);
  }
  if (drive.nodes?.files?.create) {
    return drive.nodes.files.create(input);
  }
  if (drive.nodes?.create) {
    return drive.nodes.create(input);
  }
  throw new Error("Drive file creation capability is required for notary case files");
}

async function deleteDriveNode(
  drive: DriveAppSdkPort,
  input: Required<Pick<DeleteCaseFileInput, "nodeId" | "strategy">> & {
    operatorId?: string;
  },
): Promise<unknown> {
  const params = {
    ...(input.operatorId ? { operatorId: input.operatorId } : {})
  };
  if (input.strategy === "trash") {
    if (drive.drive?.trash?.move) {
      return drive.drive.trash.move(input.nodeId, params);
    }
    if (drive.trash?.move) {
      return drive.trash.move(input.nodeId, params);
    }
  }
  if (drive.drive?.nodes?.delete) {
    return drive.drive.nodes.delete(input.nodeId, params);
  }
  if (drive.nodes?.delete) {
    return drive.nodes.delete(input.nodeId, params);
  }
  if (drive.drive?.trash?.move) {
    return drive.drive.trash.move(input.nodeId, params);
  }
  if (drive.trash?.move) {
    return drive.trash.move(input.nodeId, params);
  }
  throw new Error("Drive node deletion capability is required for notary case files");
}

async function createDriveNodeDownloadUrl(
  drive: DriveAppSdkPort,
  nodeId: string,
  input: { requestedTtlSeconds?: number },
): Promise<unknown> {
  if (drive.drive?.nodes?.downloadUrls?.create) {
    return drive.drive.nodes.downloadUrls.create(nodeId, input);
  }
  if (drive.nodes?.downloadUrls?.create) {
    return drive.nodes.downloadUrls.create(nodeId, input);
  }
  if (drive.drive?.downloadUrls?.create) {
    return drive.drive.downloadUrls.create({ nodeId, ...input });
  }
  if (drive.downloadUrls?.create) {
    return drive.downloadUrls.create({ nodeId, ...input });
  }
  throw new Error("Drive node download URL capability is required for notary case files");
}

function normalizeDownloadUrlResponse(value: unknown): Record<string, unknown> {
  const record = asObject(value);
  const data = asObject(record.data);
  const downloadUrl = stringField(record, ["downloadUrl", "download_url", "url"])
    || stringField(data, ["downloadUrl", "download_url", "url"]);
  const previewUrl = stringField(record, ["previewUrl", "preview_url"])
    || stringField(data, ["previewUrl", "preview_url"]);
  const expiresAt = stringField(record, ["expiresAt", "expires_at"])
    || stringField(data, ["expiresAt", "expires_at"]);
  const output: Record<string, unknown> = { ...record };

  if (downloadUrl) {
    output.downloadUrl = downloadUrl;
  }
  if (!stringField(output, ["url"]) && downloadUrl) {
    output.url = downloadUrl;
  }
  if (previewUrl) {
    output.previewUrl = previewUrl;
  }
  if (expiresAt) {
    output.expiresAt = expiresAt;
  }
  return output;
}

function resolveDriveUploadNodeId(value: unknown): string {
  const driveNodeId = extractId(value);
  if (!driveNodeId) {
    throw new Error("Drive upload did not return a node id for notary case file");
  }
  return driveNodeId;
}

function resolveSignatureFile(input: AttachPartySignatureInput): unknown {
  if (input.file) {
    return input.file;
  }
  if (input.signatureUrl) {
    return dataUrlToFileLike(input.signatureUrl);
  }
  throw new Error("signatureUrl or file is required to attach a notary party signature");
}

function resolveCaseFile(value: unknown): unknown {
  if (typeof value === "string" && value.startsWith("data:")) {
    return dataUrlToFileLike(value, "notary-case-file");
  }
  return value;
}

function dataUrlToFileLike(value: string, fileName = "party-signature.png"): unknown {
  if (!value.startsWith("data:")) {
    return value;
  }
  const match = /^data:([^;,]+)?(?:;[^,]*)?,(.*)$/.exec(value);
  if (!match) {
    return value;
  }
  const mimeType = match[1] || "image/png";
  const payload = match[2] || "";
  const bytes = decodeBase64(payload);
  if (typeof Blob !== "undefined") {
    const blob = new Blob([bytes], { type: mimeType });
    if (typeof File !== "undefined") {
      return new File([blob], fileName, { type: mimeType });
    }
    return blob;
  }
  return value;
}

function decodeBase64(value: string): Uint8Array {
  const decoded = typeof globalThis.atob === "function"
    ? globalThis.atob(value)
    : nodeBufferFromBase64(value);
  const bytes = new Uint8Array(decoded.length);
  for (let index = 0; index < decoded.length; index += 1) {
    bytes[index] = decoded.charCodeAt(index);
  }
  return bytes;
}

function nodeBufferFromBase64(value: string): string {
  const buffer = (globalThis as unknown as {
    Buffer?: {
      from(input: string, encoding: "base64"): {
        toString(encoding: "binary"): string;
      };
    };
  }).Buffer;
  return buffer ? buffer.from(value, "base64").toString("binary") : "";
}

function stringField(value: unknown, names: string[]): string {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    return "";
  }
  const record = value as Record<string, unknown>;
  for (const name of names) {
    const candidate = record[name];
    if (typeof candidate === "string" && candidate.trim()) {
      return candidate.trim();
    }
  }
  return "";
}

function extractId(value: unknown): string {
  if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    for (const key of ["nodeId", "id", "driveNodeId"]) {
      if (typeof record[key] === "string") {
        return record[key];
      }
    }
    for (const key of ["uploadSession", "uploadItem", "node", "file", "data"]) {
      const nested = extractId(record[key]);
      if (nested) {
        return nested;
      }
    }
  }
  return "";
}

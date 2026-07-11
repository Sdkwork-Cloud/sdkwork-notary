import type {
  DriveUploaderClient,
  DriveUploaderRequest,
} from "@sdkwork/drive-app-sdk";

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
    create(input: unknown, options: { idempotencyKey: string }): Promise<unknown>;
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
    events: {
      list(caseId: string, input?: unknown): Promise<unknown>;
    };
  };
  dashboard: {
    statistics: {
      retrieve(input?: unknown): Promise<unknown>;
    };
  };
  reports: {
    monthly: {
      retrieve(input?: unknown): Promise<unknown>;
    };
  };
}

export type NotaryDriveUploadInput = DriveUploaderRequest & {
  uploadProfileCode: NonNullable<DriveUploaderRequest["uploadProfileCode"]>;
  spaceId: string;
  parentNodeId: string;
};

export interface DriveAppSdkPort {
  nodes?: {
    list?(input: unknown): Promise<unknown>;
    delete?(nodeId: string): Promise<unknown>;
    downloadUrls?: {
      retrieve?(nodeId: string, input?: unknown): Promise<unknown>;
      create?(nodeId: string, input?: unknown): Promise<unknown>;
    };
  };
  drive?: {
    nodes?: {
      delete?(nodeId: string): Promise<unknown>;
      downloadUrls?: {
        retrieve?(nodeId: string, input?: unknown): Promise<unknown>;
        create?(nodeId: string, input?: unknown): Promise<unknown>;
      };
      list?(spaceId: string, input?: unknown): Promise<unknown>;
    };
    downloadUrls?: {
      create(input: unknown): Promise<unknown>;
    };
    trash?: {
      create?(nodeId: string, input?: unknown): Promise<unknown>;
    };
  };
  downloadUrls?: {
    create(input: unknown): Promise<unknown>;
  };
  trash?: {
    create?(nodeId: string, input?: unknown): Promise<unknown>;
  };
  uploader?: Pick<DriveUploaderClient, "upload">;
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
  idempotencyKey: string;
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

export interface CaseDriveTargetInput {
  driveSpaceId?: string;
  driveFolderNodeId?: string;
}

export interface UploadCaseFileInput extends CaseDriveTargetInput {
  caseId: string;
  file: unknown;
  category: "identity" | "evidence" | "notary";
  materialCode?: string;
  partyId?: string;
  uploadIntentId?: string;
  taskId?: string;
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

export interface AttachPartySignatureInput extends CaseDriveTargetInput {
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
  version?: string;
}

export interface ListCaseEventsInput {
  pageSize?: number;
  cursor?: string;
}

export interface GetMonthlyReportInput {
  month?: string;
  format?: "pdf" | "excel" | "csv";
}

export interface NotaryMatterOption {
  skuId: string;
  title: string;
  description?: string;
}

export function createNotaryApi({ notary, drive, appbase }: CreateNotaryApiOptions) {
  async function getAccess() {
    return notary.access.retrieve();
  }

  async function getDashboardStatistics() {
    return notary.dashboard.statistics.retrieve();
  }

  async function getMonthlyReport(input: GetMonthlyReportInput = {}) {
    return notary.reports.monthly.retrieve({
      ...(input.month ? { month: input.month } : {}),
      ...(input.format ? { format: input.format } : {}),
    });
  }

  async function listMatters(input?: unknown) {
    return notary.matters.list(input);
  }

  async function listCaseEvents(caseId: string, input: ListCaseEventsInput = {}) {
    return notary.cases.events.list(caseId, {
      ...(input.pageSize ? { pageSize: input.pageSize } : {}),
      ...(input.cursor ? { cursor: input.cursor } : {}),
    });
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
    const idempotencyKey = input.idempotencyKey.trim();
    if (!idempotencyKey) {
      throw new Error("idempotencyKey is required to create a notary case");
    }
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
      { idempotencyKey }
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
    const target = await resolveMutableCaseDriveTarget(notary, caseId, input);
    let signatureNodeId = stringField(input, ["signatureNodeId", "signature_node_id", "driveNodeId", "nodeId"]);
    if (!signatureNodeId) {
      const signatureFile = resolveSignatureFile(input);
      const uploaded = await uploadThroughDrive(drive, {
        file: signatureFile,
        taskId: buildNotaryUploadTaskId({
          caseId,
          partyId,
          materialCode: "signature",
          uploadIntentId: "signature",
          file: signatureFile,
        }),
        appResourceType: "notary_case_party_signature",
        appResourceId: `${caseId}:${partyId}`,
        uploadProfileCode: "image",
        scene: "notary.party_signature",
        source: input.source ?? "sdkwork-notary",
        spaceId: target.spaceId,
        parentNodeId: target.parentNodeId,
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
          appResourceType: "notary_case_party_file",
          appResourceId: `${input.caseId}:${input.partyId}`
        }
      : {
          appResourceType: "notary_case",
          appResourceId: input.caseId
        };
    const caseDriveTarget = await resolveMutableCaseDriveTarget(notary, input.caseId, input);
    const file = resolveCaseFile(input.file);
    const uploaded = await uploadThroughDrive(drive, {
      file,
      taskId: input.taskId ?? buildNotaryUploadTaskId({ ...input, file }),
      appResourceType: target.appResourceType,
      appResourceId: target.appResourceId,
      uploadProfileCode: "document",
      scene: "notary.case_file",
      source: input.source ?? "sdkwork-notary",
      spaceId: caseDriveTarget.spaceId,
      parentNodeId: caseDriveTarget.parentNodeId,
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

    await ensureNotaryCaseFileOwnership(notary, notaryCaseId, nodeId);

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

    await ensureMutableNotaryCase(notary, caseId);
    await ensureNotaryCaseFileOwnership(notary, caseId, nodeId);

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
    appbase,
    getAccess,
    getDashboardStatistics,
    getMonthlyReport,
    listMatters,
    listCaseEvents,
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

function extractListItems(value: unknown): unknown[] {
  if (Array.isArray(value)) {
    return value;
  }
  const record = asObject(value);
  if (Array.isArray(record.items)) {
    return record.items;
  }
  const data = asObject(record.data);
  return Array.isArray(data.items) ? data.items : [];
}

function extractPageInfo(value: unknown): { hasMore: boolean; nextCursor?: string } {
  const record = asObject(value);
  const data = asObject(record.data);
  const pageInfo = asObject(record.pageInfo ?? data.pageInfo);
  const nextCursor = stringField(pageInfo, ["nextCursor", "next_cursor"]);
  return {
    hasMore: typeof pageInfo.hasMore === "boolean" ? pageInfo.hasMore : Boolean(nextCursor),
    ...(nextCursor ? { nextCursor } : {}),
  };
}

async function resolveMutableCaseDriveTarget(
  notary: NotaryAppSdkPort,
  caseId: string,
  input: CaseDriveTargetInput,
): Promise<{ spaceId: string; parentNodeId: string }> {
  const notaryCaseId = caseId.trim();
  if (!notaryCaseId) {
    throw new Error("caseId is required to resolve a notary case Drive upload target");
  }

  const candidates = await retrieveNotaryCaseCandidates(notary, notaryCaseId);
  ensureMutableNotaryCaseStatus(notaryCaseId, candidates);

  const spaceId = firstStringField(candidates, ["driveSpaceId", "drive_space_id"]);
  const parentNodeId = firstStringField(candidates, [
    "driveFolderNodeId",
    "drive_folder_node_id",
  ]);
  const requestedSpaceId = stringField(input, ["driveSpaceId", "drive_space_id"]);
  const requestedParentNodeId = stringField(input, [
    "driveFolderNodeId",
    "drive_folder_node_id",
  ]);
  if (requestedSpaceId && requestedSpaceId !== spaceId) {
    throw new Error(`Notary case ${notaryCaseId} Drive space does not match the case`);
  }
  if (requestedParentNodeId && requestedParentNodeId !== parentNodeId) {
    throw new Error(`Notary case ${notaryCaseId} Drive folder does not match the case`);
  }

  if (!spaceId || !parentNodeId) {
    throw new Error(
      `Notary case ${notaryCaseId} is missing driveSpaceId or driveFolderNodeId for Drive upload`,
    );
  }
  return { spaceId, parentNodeId };
}

async function ensureMutableNotaryCase(notary: NotaryAppSdkPort, caseId: string): Promise<void> {
  const notaryCaseId = caseId.trim();
  if (!notaryCaseId) {
    throw new Error("caseId is required for notary case mutation");
  }
  const candidates = await retrieveNotaryCaseCandidates(notary, notaryCaseId);
  ensureMutableNotaryCaseStatus(notaryCaseId, candidates);
}

async function retrieveNotaryCaseCandidates(
  notary: NotaryAppSdkPort,
  caseId: string,
): Promise<Record<string, unknown>[]> {
  const retrieved = asObject(await notary.cases.retrieve(caseId));
  const data = asObject(retrieved.data);
  return [
    retrieved,
    data,
    asObject(retrieved.item),
    asObject(data.item),
    asObject(retrieved.case),
    asObject(data.case),
  ];
}

function ensureMutableNotaryCaseStatus(
  caseId: string,
  candidates: Record<string, unknown>[],
): void {
  const status = firstStringField(candidates, ["status"]);
  if (!status) {
    throw new Error(`Notary case ${caseId} is missing status for mutation safety`);
  }
  if (["COMPLETED", "REJECTED", "CANCELLED", "CREATE_FAILED"].includes(status.toUpperCase())) {
    throw new Error(`Notary case ${caseId} is terminal and cannot be modified`);
  }
}

async function ensureNotaryCaseFileOwnership(
  notary: NotaryAppSdkPort,
  caseId: string,
  nodeId: string,
): Promise<void> {
  let cursor: string | undefined;
  const visitedCursors = new Set<string>();
  do {
    const response = await notary.cases.files.list(caseId, {
      driveSpaceType: "notary",
      pageSize: 100,
      ...(cursor ? { cursor } : {}),
    });
    if (extractListItems(response).some((item) => extractId(item) === nodeId)) {
      return;
    }
    const pageInfo = extractPageInfo(response);
    if (!pageInfo.hasMore) {
      break;
    }
    const nextCursor = pageInfo.nextCursor;
    if (!nextCursor || visitedCursors.has(nextCursor)) {
      throw new Error(`Notary case ${caseId} file pagination did not advance`);
    }
    visitedCursors.add(nextCursor);
    cursor = nextCursor;
  } while (cursor);

  throw new Error(`Drive node ${nodeId} is not registered to notary case ${caseId}`);
}

async function uploadThroughDrive(
  drive: DriveAppSdkPort,
  input: NotaryDriveUploadInput,
): Promise<unknown> {
  if (!drive.uploader?.upload) {
    throw new Error("Drive uploader capability is required for notary case files");
  }
  return drive.uploader.upload(input);
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
    if (drive.drive?.trash?.create) {
      return drive.drive.trash.create(input.nodeId, params);
    }
    if (drive.trash?.create) {
      return drive.trash.create(input.nodeId, params);
    }
    throw new Error("Drive trash capability is required for notary case files");
  }
  if (input.operatorId) {
    throw new Error("Drive hard delete resolves operator identity from the authenticated context");
  }
  if (drive.drive?.nodes?.delete) {
    return drive.drive.nodes.delete(input.nodeId);
  }
  if (drive.nodes?.delete) {
    return drive.nodes.delete(input.nodeId);
  }
  throw new Error("Drive node deletion capability is required for notary case files");
}

async function createDriveNodeDownloadUrl(
  drive: DriveAppSdkPort,
  nodeId: string,
  input: { requestedTtlSeconds?: number },
): Promise<unknown> {
  if (drive.drive?.nodes?.downloadUrls?.retrieve) {
    return drive.drive.nodes.downloadUrls.retrieve(nodeId, input);
  }
  if (drive.nodes?.downloadUrls?.retrieve) {
    return drive.nodes.downloadUrls.retrieve(nodeId, input);
  }
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

function resolveSignatureFile(input: AttachPartySignatureInput): DriveUploaderRequest["file"] {
  if (input.file) {
    return requireDriveUploaderFile(input.file, "notary party signature");
  }
  if (input.signatureUrl) {
    return requireDriveUploaderFile(
      dataUrlToFileLike(input.signatureUrl),
      "notary party signature",
    );
  }
  throw new Error("signatureUrl or file is required to attach a notary party signature");
}

function resolveCaseFile(value: unknown): DriveUploaderRequest["file"] {
  if (typeof value === "string" && value.startsWith("data:")) {
    return requireDriveUploaderFile(
      dataUrlToFileLike(value, "notary-case-file"),
      "notary case file",
    );
  }
  return requireDriveUploaderFile(value, "notary case file");
}

function buildNotaryUploadTaskId(input: {
  caseId: string;
  partyId?: string;
  materialCode?: string;
  uploadIntentId?: string;
  file: DriveUploaderRequest["file"];
}): string {
  const file = input.file as DriveUploaderRequest["file"] & {
    lastModified?: number;
  };
  const fileName = file.name || "file";
  const size = Number.isFinite(file.size) ? String(file.size) : "0";
  const contentType = file.type || "application/octet-stream";
  const lastModified = Number.isFinite(file.lastModified) ? String(file.lastModified) : "0";
  const intent = input.uploadIntentId || input.materialCode || fileName;
  return [
    "notary",
    "case-file",
    input.caseId,
    input.partyId || "case",
    intent,
    fileName,
    size,
    contentType,
    lastModified,
  ]
    .map(sanitizeTaskIdPart)
    .filter(Boolean)
    .join("-")
    .slice(0, 240);
}

function sanitizeTaskIdPart(value: string): string {
  return value
    .trim()
    .replace(/[^A-Za-z0-9._-]+/g, "-")
    .replace(/-{2,}/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 48);
}

function requireDriveUploaderFile(
  value: unknown,
  resourceName: string,
): DriveUploaderRequest["file"] {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`${resourceName} must be a Drive uploader compatible file`);
  }
  const candidate = value as {
    size?: unknown;
    slice?: unknown;
  };
  if (
    typeof candidate.size !== "number"
    || !Number.isFinite(candidate.size)
    || candidate.size < 0
    || typeof candidate.slice !== "function"
  ) {
    throw new Error(`${resourceName} must be a Drive uploader compatible file`);
  }
  return value as DriveUploaderRequest["file"];
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
    const blob = new Blob([toArrayBuffer(bytes)], { type: mimeType });
    if (typeof File !== "undefined") {
      return new File([blob], fileName, { type: mimeType });
    }
    return blob;
  }
  return value;
}

function toArrayBuffer(bytes: Uint8Array): ArrayBuffer {
  const buffer = bytes.buffer;
  if (buffer instanceof ArrayBuffer) {
    return buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength);
  }
  const copy = new Uint8Array(bytes.byteLength);
  copy.set(bytes);
  return copy.buffer;
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

function firstStringField(values: unknown[], names: string[]): string {
  for (const value of values) {
    const result = stringField(value, names);
    if (result) {
      return result;
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

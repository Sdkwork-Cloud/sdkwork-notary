import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chatPcRoot, chatPcTest } from "./helpers/chat-pc-root.mjs";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");
const notaryServicePath = "packages/sdkwork-im-pc-notary/src/services/NotaryService.ts";

function readChatPcText(relativePath) {
  return readFileSync(path.join(chatPcRoot, relativePath), "utf8");
}

function readNotaryServiceSource() {
  return readChatPcText(notaryServicePath);
}

function methodBody(source, methodName) {
  const match = new RegExp(`async\\s+${methodName}\\s*\\(`).exec(source);
  assert(match, `${methodName} must exist`);
  const start = match.index;
  const next = source.indexOf("\n    async ", start + 1);
  return next >= 0 ? source.slice(start, next) : source.slice(start);
}

function functionBody(source, functionName) {
  const match = new RegExp(`function\\s+${functionName}\\s*\\(`).exec(source);
  assert(match, `${functionName} must exist`);
  const start = match.index;
  const next = source.indexOf("\nfunction ", start + 1);
  return next >= 0 ? source.slice(start, next) : source.slice(start);
}

chatPcTest("real chat-pc notary service preserves the existing UI service shape", () => {
  const source = readNotaryServiceSource();
  for (const method of [
    "getTasks",
    "getTaskById",
    "getStaff",
    "createTask",
    "updateTaskStatus",
    "updateTask",
    "addParty",
    "addDocument",
    "listPartyDocuments",
    "uploadPartyDocument",
    "createVideoInvite",
    "createSignatureInvite",
    "downloadDocuments",
    "getDocumentUrl",
    "getPartyIdentityMediaUrls",
    "deleteTask",
    "removeDocument",
  ]) {
    assert.match(source, new RegExp(`async\\s+${method}\\s*\\(`), `${method} must be implemented`);
  }

  for (const token of [
    "NotaryTask",
    "Party",
    "NotaryDocument",
    "createNotaryApi",
    "getNotaryAppSdkClient",
    "getDriveAppSdkClient",
    "getAppbaseAppSdkClient",
    "driveSpaceType: 'notary'",
  ]) {
    assert(source.includes(token), `${notaryServicePath} must include ${token}`);
  }

  assert.doesNotMatch(
    source,
    /fetch\(|axios|Authorization|Access-Token|MockNotaryService|mockTasks|picsum\.photos/,
    "real notary service must not bypass SDKs, assemble auth headers, or keep mock data",
  );
});

chatPcTest("real chat-pc notary service maps generated SDK case models to existing task view models", () => {
  const source = readNotaryServiceSource();

  for (const field of [
    "orderId",
    "orderItemId",
    "skuId",
    "driveSpaceId",
    "driveFolderNodeId",
    "documents",
    "timeline",
    "PENDING_REVIEW",
    "PROCESSING",
    "COMPLETED",
    "REJECTED",
  ]) {
    assert(source.includes(field), `service must map ${field}`);
  }

  for (const sdkCall of [
    "notaryApi.listCases",
    "notaryApi.listStaff",
    "notaryApi.assignCase",
    "notaryApi.createCase",
    "notaryApi.uploadCaseFile",
    "notaryApi.createDownloadPackage",
    "notaryApi.createPartyVideoInvite",
    "notaryApi.createPartySignatureInvite",
  ]) {
    assert(source.includes(sdkCall), `service must call ${sdkCall}`);
  }
});

chatPcTest("real chat-pc notary service keeps operational case id separate from display case number", () => {
  const source = readNotaryServiceSource();

  assert(
    source.includes("id: stringValue(record.caseId ?? record.id)"),
    "task.id must use the backend case id used by notary app-api routes",
  );
  assert(source.includes("caseNo: optionalString(record.caseNo)"));
  assert(source.includes("caseId: optionalString(record.caseId ?? record.id)"));
  assert.doesNotMatch(
    source,
    /id:\s*stringValue\(record\.caseNo\s*\?\?\s*record\.caseId\s*\?\?\s*record\.id\)/,
    "task.id must not prefer display case number over operational case id",
  );
});

chatPcTest("real chat-pc notary service creates cases with staff assignment and creation uploads", () => {
  const source = readNotaryServiceSource();
  const createTaskBody = methodBody(source, "createTask");

  for (const token of [
    "documents: data.documents ?? []",
    "primaryNotaryMembershipId: resolvePrimaryNotaryMembershipId(data)",
    "await syncCaseAssignments(createdTask.id, data)",
    "await syncInitialPartySignatures(createdTask.id, data.parties ?? [])",
    "await syncInitialPartyIdentityMedia(createdTask.id, data.parties ?? [])",
    "const createdTaskForDocumentUpload = documents.some(hasDocumentPartyId)",
    "const createdPartyIdByClientPartyId = mapCreatedPartyIds(data.parties, createdTaskForDocumentUpload.parties ?? [])",
    "partyId: resolveCreationDocumentPartyId(document, createdPartyIdByClientPartyId)",
    "notaryApi.uploadCaseFile",
  ]) {
    assert(createTaskBody.includes(token), `createTask must include ${token}`);
  }

  const syncBody = functionBody(source, "syncCaseAssignments");
  assert.match(syncBody, /notaryApi\.assignCase\(taskId,\s*\{/);
  assert.match(syncBody, /assignmentRole:\s*["']primary_notary["']/);
  assert.match(syncBody, /organizationMembershipId:\s*primaryNotaryMembershipId/);
  assert.doesNotMatch(syncBody, /fetch\(|axios|Authorization|Access-Token/);

  const resolveBody = functionBody(source, "resolvePrimaryNotaryMembershipId");
  assert.match(resolveBody, /primaryNotaryMembershipId/);
  assert.match(resolveBody, /notaryMembershipId/);
  assert.match(resolveBody, /selectedNotaryStaff/);
});

chatPcTest("real chat-pc notary service loads staff, filters cases by SKU, and forwards pagination", () => {
  const source = readNotaryServiceSource();

  const staffBody = methodBody(source, "getStaff");
  assert.match(staffBody, /notaryApi\.listStaff\(\{/);
  assert.match(staffBody, /staffRole:\s*filters\.staffRole/);
  assert.match(staffBody, /q:\s*filters\.searchTerm/);
  assert.match(staffBody, /pageSize:\s*filters\.pageSize/);
  assert.match(staffBody, /cursor:\s*filters\.cursor/);
  assert.match(staffBody, /extractItems/);
  assert.doesNotMatch(staffBody, /fetch\(|axios|Authorization|Access-Token|MockNotaryService|mockTasks/);

  const queryBody = functionBody(source, "resolveListCaseQuery");
  for (const token of [
    "FILTER_SKU_IDS_BY_TYPE",
    "ELECTRONIC",
    "IPR",
    "EVIDENCE",
    "VALID_CASE_STATUSES",
    "pageSize: filters?.pageSize ?? 50",
    "cursor: filters?.cursor",
    "skuId",
  ]) {
    assert(source.includes(token) || queryBody.includes(token), `${notaryServicePath} must include ${token}`);
  }
  assert.doesNotMatch(
    source,
    /status:\s*filters\?\.status\s*&&\s*filters\.status\s*!==\s*["']ALL["']/,
    "service must not pass business type filter keys as notary status",
  );

  for (const [matterName, skuId] of [
    ["Electronic Contract Preservation", "sku-notary-electronic-contract"],
    ["Intellectual Property Confirmation", "sku-notary-ipr"],
    ["Electronic Evidence Preservation", "sku-notary-evidence"],
    ["Trade Secret Confirmation", "sku-notary-trade-secret"],
    ["Lottery Process Notarization", "sku-notary-lottery"],
    ["Will Notarization", "sku-notary-will"],
  ]) {
    assert(
      source.includes(`"${matterName}": "${skuId}"`) || source.includes(`'${matterName}': '${skuId}'`),
      `${notaryServicePath} must map ${matterName} to ${skuId}`,
    );
  }
});

chatPcTest("real chat-pc notary service keeps document operations inside generated SDK facades", () => {
  const source = readNotaryServiceSource();

  const downloadBody = methodBody(source, "downloadDocuments");
  assert.match(downloadBody, /createDownloadPackage\(taskId,\s*\{\s*\}\)/);
  assert.doesNotMatch(downloadBody, /\b(driveSpaceType|mode|documentName)\b/);

  const documentUrlBody = methodBody(source, "getDocumentUrl");
  assert.match(documentUrlBody, /listCaseFiles\(taskId,\s*driveListScope\)/);
  assert.match(documentUrlBody, /createCaseFileDownloadUrl\(taskId,\s*\{/);
  assert.match(documentUrlBody, /nodeId/);
  assert.doesNotMatch(documentUrlBody, /createDownloadPackage|picsum\.photos/);

  const removeDocumentBody = methodBody(source, "removeDocument");
  assert.match(removeDocumentBody, /notaryApi\.deleteCaseFile/);
  assert.match(removeDocumentBody, /return loadTask\(taskId\)/);
  assert.doesNotMatch(removeDocumentBody, /createDownloadPackage|fetch\(|axios|Authorization|Access-Token/);
});

chatPcTest("real chat-pc notary service persists party identity media and party Drive files", () => {
  const source = readNotaryServiceSource();

  for (const token of [
    "getPartyIdentityMediaUrls",
    "syncPartyIdentityMedia",
    "extractPartyIdentityDocuments",
    "identity_front",
    "identity_back",
    "face_capture",
  ]) {
    assert(source.includes(token), `${notaryServicePath} must include ${token}`);
  }

  const identityUrlBody = methodBody(source, "getPartyIdentityMediaUrls");
  assert.match(identityUrlBody, /listCaseFiles\(taskId,\s*\{\s*\.\.\.driveListScope,\s*category:\s*["']identity["']\s*\}\)/);
  assert.match(identityUrlBody, /partyId/);
  assert.match(identityUrlBody, /createCaseFileDownloadUrl/);
  assert.match(identityUrlBody, /identityFrontUrl/);
  assert.match(identityUrlBody, /identityBackUrl/);
  assert.match(identityUrlBody, /faceImageUrl/);
  assert.doesNotMatch(identityUrlBody, /picsum\.photos/);

  const listBody = methodBody(source, "listPartyDocuments");
  assert.match(listBody, /listCaseFiles\(taskId,\s*driveListScope\)/);
  assert.match(listBody, /partyId/);
  assert.doesNotMatch(listBody, /fetch\(|axios|Authorization|Access-Token/);

  const uploadBody = methodBody(source, "uploadPartyDocument");
  assert.match(uploadBody, /uploadCaseFile\(\{/);
  assert.match(uploadBody, /caseId:\s*taskId/);
  assert.match(uploadBody, /category:\s*["']evidence["']/);
  assert.match(uploadBody, /partyId/);
  assert.match(uploadBody, /materialCode:\s*resolveFileMaterialCode\(file\)/);
  assert.match(uploadBody, /return loadTask\(taskId\)/);
  assert.doesNotMatch(uploadBody, /fetch\(|axios|Authorization|Access-Token/);
});

chatPcTest("real chat-pc notary service creates party video and signature invites through app SDK", () => {
  const source = readNotaryServiceSource();

  const videoBody = methodBody(source, "createVideoInvite");
  assert.match(videoBody, /notaryApi\.createPartyVideoInvite\(taskId,\s*partyId,\s*\{/);
  assert.match(videoBody, /purpose:\s*["']identity_verification["']/);
  assert.match(videoBody, /conversationId/);
  assert.match(videoBody, /inviteUrl/);
  assert.doesNotMatch(videoBody, /fetch\(|axios|Authorization|Access-Token/);

  const signatureInviteBody = methodBody(source, "createSignatureInvite");
  assert.match(signatureInviteBody, /notaryApi\.createPartySignatureInvite\(taskId,\s*partyId,\s*\{/);
  assert.match(signatureInviteBody, /purpose:\s*["']remote_signature["']/);
  assert.match(signatureInviteBody, /inviteUrl/);
  assert.match(signatureInviteBody, /signingUrl/);
  assert.doesNotMatch(signatureInviteBody, /fetch\(|axios|Authorization|Access-Token/);
});

chatPcTest("real chat-pc notary service synchronizes party edits and signatures through generated resources", () => {
  const source = readNotaryServiceSource();

  const updateBody = methodBody(source, "updateTask");
  assert.match(updateBody, /syncParties\(taskId,\s*updates\.parties\)/);
  assert.doesNotMatch(updateBody, /\bparties\s*:/);

  for (const token of [
    "notaryApi.updateParty",
    "notaryApi.addParty",
    "notaryApi.deleteParty",
    "syncPartySignature",
    "notaryApi.attachPartySignature",
    "signatureUrl: party.signatureUrl",
    "syncInitialPartySignatures(createdTask.id, data.parties ?? [])",
  ]) {
    assert(source.includes(token), `${notaryServicePath} must include ${token}`);
  }
  assert.match(methodBody(source, "addParty"), /syncPartySignature/);
  assert.doesNotMatch(functionBody(source, "mapPartyToUpdateRequest"), /signatureUrl/);
  assert.doesNotMatch(functionBody(source, "mapPartyToCreateRequest"), /signatureUrl/);
});

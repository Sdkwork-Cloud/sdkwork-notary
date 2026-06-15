import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");
const chatPcRoot = process.env.SDKWORK_CHAT_PC_ROOT
  ? path.resolve(process.env.SDKWORK_CHAT_PC_ROOT)
  : path.resolve(workspaceRoot, "..", "sdkwork-im", "apps", "sdkwork-chat-pc");

function readText(root, relativePath) {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(root, relativePath) {
  return JSON.parse(readText(root, relativePath));
}

function exists(root, relativePath) {
  return existsSync(path.join(root, relativePath));
}

test("notary workspace no longer owns a sdkwork-chat-pc integration fork", () => {
  assert(
    !exists(workspaceRoot, "integrations/sdkwork-chat-pc"),
    "sdkwork-notary must not keep the deleted sdkwork-chat-pc integration fork",
  );
});

test("real sdkwork-chat-pc app root wires the notary app SDK through core bootstrap", () => {
  assert(exists(chatPcRoot, "package.json"), `${chatPcRoot} must be the real sdkwork-chat-pc app root`);

  const workspace = readText(chatPcRoot, "pnpm-workspace.yaml");
  assert(
    workspace.includes("../../../sdkwork-notary/sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript"),
    "real chat-pc workspace must include the sdkwork-notary app SDK workspace",
  );

  const packageJson = readJson(chatPcRoot, "package.json");
  assert.equal(packageJson.dependencies?.["@sdkwork/notary-app-sdk"], "workspace:*");
  assert.equal(packageJson.pnpm?.overrides?.["@sdkwork/notary-app-sdk"], "workspace:*");

  const tsconfig = readJson(chatPcRoot, "tsconfig.json");
  assert.deepEqual(tsconfig.compilerOptions?.paths?.["@sdkwork/notary-app-sdk"], [
    "../../../sdkwork-notary/sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts",
  ]);

  const viteConfig = readText(chatPcRoot, "vite.config.ts");
  for (const token of [
    "generatedNotaryAppSdkEntry",
    "dependencyRoot('sdkwork-notary')",
    "{ find: '@sdkwork/notary-app-sdk', replacement: generatedNotaryAppSdkEntry }",
  ]) {
    assert(viteConfig.includes(token), `vite.config.ts must include ${token}`);
  }
  assert.match(
    viteConfig,
    /exclude:\s*\[[\s\S]*'@sdkwork\/notary-app-sdk'/,
    "vite.config.ts must exclude the notary app SDK from optimizeDeps prebundling",
  );

  const coreClient = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-core/src/sdk/notaryAppSdkClient.ts",
  );
  for (const token of [
    "createNotaryAppClient",
    "resolveAppSdkBaseUrl",
    "createSdkworkChatRequestContextInterceptors",
    "getSdkworkChatGlobalTokenManager",
    "tokenManager: getSdkworkChatGlobalTokenManager()",
    "getNotaryAppSdkClientWithSession",
    "resetNotaryAppSdkClient",
  ]) {
    assert(coreClient.includes(token), `notaryAppSdkClient.ts must include ${token}`);
  }
  for (const forbidden of ["fetch(", "axios", "Authorization", "Access-Token"]) {
    assert(!coreClient.includes(forbidden), `notaryAppSdkClient.ts must not include ${forbidden}`);
  }

  const coreIndex = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-core/src/index.ts");
  assert(coreIndex.includes("export * from './sdk/notaryAppSdkClient';"));
});

test("real sdkwork-clawchat-pc-notary service uses generated SDK clients instead of mock data", () => {
  const service = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-notary/src/services/NotaryService.ts",
  );

  for (const token of [
    "createChatPcNotaryService",
    "createNotaryApi",
    "getNotaryAppSdkClient",
    "getDriveAppSdkClient",
    "getAppbaseAppSdkClient",
    "resolveAppSdkTenantId",
    "notaryService",
    "driveSpaceType: 'notary'",
    "resolveListCaseQuery",
    "listCaseFiles",
    "createDownloadPackage",
    "createCaseFileDownloadUrl",
    "createPartyVideoInvite",
    "createPartySignatureInvite",
    "deleteCaseFile",
  ]) {
    assert(service.includes(token), `NotaryService.ts must include ${token}`);
  }

  for (const forbidden of [
    "MockNotaryService",
    "mockTasks",
    "delay(",
    "fetch(",
    "axios",
    "Authorization",
    "Access-Token",
    "picsum.photos",
  ]) {
    assert(!service.includes(forbidden), `NotaryService.ts must not include ${forbidden}`);
  }
});

test("real notary UI drives documents, signatures, video, and staff selection through the service", () => {
  const createTaskView = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-notary/src/CreateNotaryTaskView.tsx",
  );
  for (const token of [
    "file: File",
    "documents: attachments.map",
    "category: 'evidence'",
    "partyId: attachment.partyId",
    "notaryStaffMembers",
    "notaryService.getStaff",
    "selectedNotaryStaff",
    "primaryNotaryMembershipId",
  ]) {
    assert(createTaskView.includes(token), `CreateNotaryTaskView.tsx must include ${token}`);
  }

  const notaryView = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-notary/src/index.tsx");
  for (const token of [
    "notaryService.downloadDocuments",
    "notaryService.getDocumentUrl",
    "notaryService.getPartyIdentityMediaUrls",
    "notaryService.listPartyDocuments",
    "notaryService.uploadPartyDocument",
    "notaryService.createVideoInvite",
    "notaryService.createSignatureInvite",
    "partyDriveDocuments",
    "partyIdentityMediaUrls",
    "activeSignInviteUrl",
    "mobileSignatureUrl={activeSignInviteUrl}",
    "EMPTY_NOTARY_PRINT_IMAGE_URL",
  ]) {
    assert(notaryView.includes(token), `index.tsx must include ${token}`);
  }
  for (const forbidden of [
    "new Blob([JSON.stringify(task.documents",
    "picsum.photos/seed/notary_docs",
    "picsum.photos/seed/doc",
    "picsum.photos/seed/id_",
    "picsum.photos/seed/live_",
  ]) {
    assert(!notaryView.includes(forbidden), `index.tsx must not include ${forbidden}`);
  }

  const drawer = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-notary/src/PartyDrawer.tsx");
  for (const token of [
    "idFrontFile",
    "idBackFile",
    "faceImageDataUrl",
    "identityFrontFile",
    "identityBackFile",
  ]) {
    assert(drawer.includes(token), `PartyDrawer.tsx must include ${token}`);
  }

  const signaturePad = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-notary/src/SignaturePad.tsx");
  assert(signaturePad.includes("mobileSignatureUrl?: string"));
  assert(signaturePad.includes("<SignaturePadMobileQR signatureUrl={mobileSignatureUrl} />"));

  const signatureQr = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-notary/src/components/SignaturePadMobileQR.tsx",
  );
  assert(signatureQr.includes("react-qr-code"));
  assert(signatureQr.includes("navigator.clipboard.writeText(signatureUrl)"));
  assert(signatureQr.includes("<QRCode value={signatureUrl}"));
});

test("real chat shell exposes notary entries while notary workflows stay SDK-backed", () => {
  const accessService = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-chat/src/services/NotaryAccessService.ts",
  );
  for (const token of [
    "getNotaryAppSdkClient",
    "notary.access.retrieve()",
    "notaryBusinessEnabled === true",
    "organizationVerified === true",
    "canUseNotary",
    "subscribe",
  ]) {
    assert(accessService.includes(token), `NotaryAccessService.ts must include ${token}`);
  }
  assert(!accessService.includes("canShowNotaryMenu"));
  assert(!/\bvisible:\s*boolean/u.test(accessService));
  assert(!/\bvisible:\s*false/u.test(accessService));

  const sidebar = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-chat/src/components/Sidebar.tsx");
  assert(sidebar.includes("ALWAYS_CONFIGURABLE_MODULES.has(m)"));
  assert(sidebar.includes('modId === "notary"'));
  assert(!sidebar.includes("notaryAccessService.canShowNotaryMenu"));
  assert(!sidebar.includes("filterNotaryModules"));
  assert(!sidebar.includes('modId === "notary" && canShowNotaryMenu'));

  const settings = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-chat/src/components/SettingsModal.tsx",
  );
  assert(settings.includes("ALWAYS_CONFIGURABLE_MODULES"));
  assert(settings.includes("ALWAYS_CONFIGURABLE_MODULES.has(mod.id)"));
  assert(!settings.includes("notaryAccessService.canShowNotaryMenu"));
  assert(!settings.includes("filterNotaryModules"));

  const settingsService = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-chat/src/services/SettingsService.ts",
  );
  assert(settingsService.includes('export const ALWAYS_CONFIGURABLE_MODULES = new Set(["notary"]);'));

  const workspaceService = readText(
    chatPcRoot,
    "packages/sdkwork-clawchat-pc-workspace/src/services/WorkspaceService.ts",
  );
  assert(workspaceService.includes("id: 'notary'"));
  assert(workspaceService.includes("nameKey: 'apps.notary'"));
  assert(workspaceService.includes("iconName: 'ShieldCheck'"));

  const workspaceView = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-workspace/src/index.tsx");
  assert(workspaceView.includes("onAppSelect(app.id)"));
  assert(!workspaceService.includes("notaryAccessService"));
  assert(!workspaceView.includes("notaryAccessService"));
  assert(!workspaceService.includes("canUseNotary"));
  assert(!workspaceView.includes("canUseNotary"));
  assert(!workspaceService.includes("canShowNotaryMenu"));
  assert(!workspaceView.includes("canShowNotaryMenu"));

  const chatLayout = readText(chatPcRoot, "packages/sdkwork-clawchat-pc-chat/src/pages/ChatLayout.tsx");
  assert(chatLayout.includes("handleTabChange"));
  assert(chatLayout.includes('appId === "notary") setActiveTab("notary")'));
  assert(chatLayout.includes('case "notary":'));
  assert(chatLayout.includes("return <NotaryView />;"));
  assert(!chatLayout.includes("notaryAccessService.canUseNotary"));
  assert(!chatLayout.includes("notaryAccess?.canUseNotary"));
});

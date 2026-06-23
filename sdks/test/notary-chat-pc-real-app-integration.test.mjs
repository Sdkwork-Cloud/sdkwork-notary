import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { imPcRoot, imPcTest, notaryPcRoot, workspaceRoot } from "./helpers/chat-pc-root.mjs";

function readText(root, relativePath) {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(root, relativePath) {
  return JSON.parse(readText(root, relativePath));
}

function exists(root, relativePath) {
  return existsSync(path.join(root, relativePath));
}

test("notary workspace owns sdkwork-notary-pc application root", () => {
  assert(exists(workspaceRoot, "apps/sdkwork-notary-pc/package.json"));
  assert(exists(notaryPcRoot, "packages/sdkwork-notary-pc-notary/src/services/NotaryService.ts"));
});

test("notary workspace no longer owns a sdkwork-chat-pc integration fork", () => {
  assert(
    !exists(workspaceRoot, "integrations/sdkwork-chat-pc"),
    "sdkwork-notary must not keep the deleted sdkwork-chat-pc integration fork",
  );
});

imPcTest("real sdkwork-im-pc app root wires the notary app SDK through core bootstrap", () => {
  assert(exists(imPcRoot, "package.json"), `${imPcRoot} must be the real sdkwork-im-pc app root`);

  const workspace = readText(imPcRoot, "pnpm-workspace.yaml");
  assert(
    workspace.includes("../../../sdkwork-notary/sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript"),
    "real IM PC workspace must include the sdkwork-notary app SDK workspace",
  );
  assert(
    workspace.includes("sdkwork-notary-pc-notary"),
    "real IM PC workspace must include sdkwork-notary-pc-notary package",
  );

  assert.equal(
    exists(imPcRoot, "packages/sdkwork-im-pc-notary"),
    false,
    "sdkwork-im-pc must not keep a local notary capability package",
  );

  const packageJson = readJson(imPcRoot, "package.json");
  assert.equal(packageJson.dependencies?.["@sdkwork/notary-app-sdk"], "workspace:*");

  const tsconfig = readJson(imPcRoot, "tsconfig.json");
  assert.deepEqual(tsconfig.compilerOptions?.paths?.["@sdkwork/notary-app-sdk"], [
    "../../../sdkwork-notary/sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts",
  ]);

  const viteConfig = readText(imPcRoot, "vite.config.ts");
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
    imPcRoot,
    "packages/sdkwork-im-pc-core/src/sdk/notaryAppSdkClient.ts",
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

  const coreIndex = readText(imPcRoot, "packages/sdkwork-im-pc-core/src/index.ts");
  assert(coreIndex.includes("export * from './sdk/notaryAppSdkClient';"));
  assert(coreIndex.includes("export * from './sdk/notaryPcIntegration';"));

  const notaryIntegration = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-core/src/sdk/notaryPcIntegration.ts",
  );
  assert(notaryIntegration.includes("bootstrapNotaryPcForIm"));
  assert(notaryIntegration.includes("configureNotaryPcRuntime"));
});

imPcTest("real sdkwork-notary-pc-notary service uses generated SDK clients instead of mock data", () => {
  const service = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/services/NotaryService.ts",
  );

  for (const token of [
    "createNotaryPcService",
    "createNotaryApi",
    "getConfiguredNotaryAppSdkClient",
    "getConfiguredDriveAppSdkClient",
    "getConfiguredAppbaseAppSdkClient",
    "driveSpaceType: 'notary'",
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

imPcTest("real notary UI drives documents, signatures, video, and staff selection through the service", () => {
  const createTaskView = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/CreateNotaryTaskView.tsx",
  );
  for (const token of [
    "notaryStaffMembers",
    "notaryService.getStaff",
    "notaryService.getMatters",
    "selectedNotaryStaff",
    "primaryNotaryMembershipId",
    "BusinessTypeSelector",
    "PartyBindingStep",
    "WizardHeader",
    "WizardStepper",
    "WizardNavigation",
    "NotaryPickerDrawer",
    "TaskDetailsForm",
    "ConfirmationSummary",
    "PartyDriveModal",
    "VideoCallQROverlay",
    "useLocalAttachments",
    "validateWizardStep",
    "selectBusinessTypeFirst",
  ]) {
    assert(createTaskView.includes(token), `CreateNotaryTaskView.tsx must include ${token}`);
  }

  const localAttachmentsHook = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/hooks/useLocalAttachments.ts",
  );
  assert(localAttachmentsHook.includes("file: File"), "useLocalAttachments.ts must include file: File");

  const videoCallQr = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/create/VideoCallQROverlay.tsx",
  );
  for (const token of ["react-qr-code", "inviteUrl", "QRCode value={inviteUrl}"]) {
    assert(videoCallQr.includes(token), `VideoCallQROverlay.tsx must include ${token}`);
  }

  const notaryPickerDrawer = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/create/NotaryPickerDrawer.tsx",
  );
  for (const token of ["searchTerm", "filteredStaff"]) {
    assert(notaryPickerDrawer.includes(token), `NotaryPickerDrawer.tsx must include ${token}`);
  }

  const partyListTab = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/PartyListTab.tsx",
  );
  assert(partyListTab.includes("expandedPartyMediaLoading"), "PartyListTab.tsx must include expandedPartyMediaLoading");
  for (const token of ["expandedPartyMediaUrls", "identityFrontUrl", "identityBackUrl", "faceImageUrl"]) {
    assert(partyListTab.includes(token), `PartyListTab.tsx must include ${token}`);
  }

  const createTaskSubmitBody = createTaskView.slice(createTaskView.indexOf('handleSubmit'));
  for (const token of [
    "documents: attachments.map",
    "category: 'evidence'",
    "partyId: attachment.partyId",
    "notaryService.createTask",
  ]) {
    assert(createTaskSubmitBody.includes(token), `CreateNotaryTaskView.tsx submit flow must include ${token}`);
  }

  const notaryView = readText(notaryPcRoot, "packages/sdkwork-notary-pc-notary/src/NotaryView.tsx");
  for (const token of [
    "notaryService.downloadDocuments",
    "notaryService.getDocumentUrl",
    "notaryService.getPartyIdentityMediaUrls",
    "notaryService.listPartyDocuments",
    "notaryService.uploadPartyDocument",
    "notaryService.createVideoInvite",
    "notaryService.createSignatureInvite",
    "notaryService.getDashboardStatistics",
    "notaryService.getMonthlyReport",
    "notaryService.getTaskById",
    "handleSelectTask",
    "matters={matters}",
    "partyDriveDocuments",
    "partyIdentityMediaUrls",
    "expandedPartyMediaUrls",
    "isNotaryAssigned",
    "VideoCallQROverlay",
    "activeVideoQrParty",
    "activeSignInviteUrl",
    "mobileSignatureUrl={activeSignInviteUrl}",
    "PrintOverlay",
    "NotaryHeader",
    "DetailPane",
    "PartyDriveModal",
    "statusFilter",
  ]) {
    assert(notaryView.includes(token), `NotaryView.tsx must include ${token}`);
  }

  const filterBar = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/NotaryFilterBar.tsx",
  );
  for (const token of ["matters", "mattersLoading", "LEGACY_TYPE_FILTERS"]) {
    assert(filterBar.includes(token), `NotaryFilterBar.tsx must include ${token}`);
  }
  assert(!filterBar.includes("advancedFilter"), "NotaryFilterBar.tsx must not include dead advanced filter button");

  const printPartyPage = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/PrintPartyPage.tsx",
  );
  assert(printPartyPage.includes("EMPTY_NOTARY_PRINT_IMAGE_URL"),
    "PrintPartyPage.tsx must include EMPTY_NOTARY_PRINT_IMAGE_URL",
  );
  assert(printPartyPage.includes("referenceThresholdValue"), "PrintPartyPage.tsx must use i18n reference threshold");

  const printOverlay = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/PrintOverlay.tsx",
  );
  assert(printOverlay.includes("loadingMedia"), "PrintOverlay.tsx must show loading state while print media loads");
  assert(notaryView.includes("printMediaLoading"), "NotaryView.tsx must track print media loading state");
  assert(notaryView.includes("assignNotary"), "NotaryView.tsx must support notary reassignment through the service");
  assert(notaryView.includes("debouncedSearchTerm"), "NotaryView.tsx must debounce list search input");
  assert(notaryView.includes("NotaryPickerDrawer"), "NotaryView.tsx must include NotaryPickerDrawer for detail reassignment");
  assert(notaryView.includes("cancelConfirm"), "NotaryView.tsx must confirm before cancelling a case");
  for (const forbidden of [
    "new Blob([JSON.stringify(task.documents",
    "picsum.photos/seed/notary_docs",
    "picsum.photos/seed/doc",
    "picsum.photos/seed/id_",
    "picsum.photos/seed/live_",
  ]) {
    assert(!notaryView.includes(forbidden), `NotaryView.tsx must not include ${forbidden}`);
  }

  const taskBaseInfo = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/TaskBaseInfo.tsx",
  );
  assert(taskBaseInfo.includes("getNotaryTaskDisplayNo"), "TaskBaseInfo.tsx must include getNotaryTaskDisplayNo");
  assert(taskBaseInfo.includes("changeNotary"), "TaskBaseInfo.tsx must support notary reassignment action");

  const detailPane = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/DetailPane.tsx",
  );
  assert(detailPane.includes("TimelineTab"), "DetailPane.tsx must include TimelineTab");
  assert(detailPane.includes("loading"), "DetailPane.tsx must support detail loading overlay");

  const materialsTab = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/list/MaterialsTab.tsx",
  );
  assert(materialsTab.includes("otherMaterials"), "MaterialsTab.tsx must include uncategorized otherMaterials section");
  assert(materialsTab.includes("KNOWN_CATEGORIES"), "MaterialsTab.tsx must group documents by known categories");

  assert(notaryView.includes("openPrintTask"), "NotaryView.tsx must fetch full task detail before printing");
  assert(notaryView.includes("detailLoading"), "NotaryView.tsx must track detail loading state");

  const partyDrawer = readText(notaryPcRoot, "packages/sdkwork-notary-pc-notary/src/PartyDrawer.tsx");
  assert(partyDrawer.includes("generateClientId"), "PartyDrawer.tsx must include generateClientId");
  for (const token of [
    "IdentityVerification",
    "BasicInfoForm",
    "AuxiliaryMaterials",
    "PartyDrawerFooter",
    "FaceCaptureOverlay",
    "useCameraCapture",
    "useLocalAttachments",
    "idFrontFile",
    "idBackFile",
    "faceImageDataUrl",
  ]) {
    assert(partyDrawer.includes(token), `PartyDrawer.tsx must include ${token}`);
  }

  const signaturePad = readText(notaryPcRoot, "packages/sdkwork-notary-pc-notary/src/SignaturePad.tsx");
  assert(signaturePad.includes("mobileSignatureUrl?: string"));
  assert(signaturePad.includes("<SignaturePadMobileQR signatureUrl={mobileSignatureUrl} />"));

  const signatureQr = readText(
    notaryPcRoot,
    "packages/sdkwork-notary-pc-notary/src/components/SignaturePadMobileQR.tsx",
  );
  assert(signatureQr.includes("react-qr-code"));
  assert(signatureQr.includes("navigator.clipboard.writeText(signatureUrl)"));
  assert(signatureQr.includes("<QRCode value={signatureUrl}"));
});

imPcTest("real chat shell exposes notary entries while notary workflows stay SDK-backed", () => {
  const accessService = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-chat/src/services/NotaryAccessService.ts",
  );
  for (const token of [
    "createNotaryAccessService",
    "getNotaryAppSdkClient",
    "notaryAccessService",
    "createImNotaryAccessService",
  ]) {
    assert(accessService.includes(token), `NotaryAccessService.ts must include ${token}`);
  }
  assert(!accessService.includes("canShowNotaryMenu"));

  const sidebar = readText(imPcRoot, "packages/sdkwork-im-pc-chat/src/components/Sidebar.tsx");
  assert(sidebar.includes("ALWAYS_CONFIGURABLE_MODULES"));
  assert(sidebar.includes('modId === "notary"'));
  assert(!sidebar.includes("notaryAccessService.canShowNotaryMenu"));
  assert(!sidebar.includes("filterNotaryModules"));

  const settings = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-chat/src/components/SettingsModal.tsx",
  );
  assert(settings.includes("ALWAYS_CONFIGURABLE_MODULES"));
  assert(settings.includes('(ALWAYS_CONFIGURABLE_MODULES as ReadonlySet<string>).has(mod.id)'));
  assert(!settings.includes("notaryAccessService.canShowNotaryMenu"));
  assert(!settings.includes("filterNotaryModules"));

  const settingsService = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-chat/src/services/SettingsService.ts",
  );
  assert(settingsService.includes('from "@sdkwork/im-pc-shell/moduleRegistry"'));
  assert(settingsService.includes("ALWAYS_CONFIGURABLE_MODULES"));

  const workspaceService = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-workspace/src/services/WorkspaceService.ts",
  );
  assert(workspaceService.includes("id: 'notary'"));
  assert(workspaceService.includes("nameKey: 'apps.notary'"));
  assert(workspaceService.includes("iconName: 'ShieldCheck'"));

  const workspaceView = readText(imPcRoot, "packages/sdkwork-im-pc-workspace/src/index.tsx");
  assert(workspaceView.includes("onAppSelect"));

  const capabilitySurface = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-chat/src/surfaces/CapabilityModuleSurface.tsx",
  );
  assert(capabilitySurface.includes('case "notary":'));
  assert(capabilitySurface.includes("LazyCapabilityModuleRenderer"));

  const shellLoaders = readText(
    imPcRoot,
    "packages/sdkwork-im-pc-shell/src/capabilityModuleLoaders.ts",
  );
  assert(shellLoaders.includes("notary: () => import('@sdkwork/notary-pc-notary')"));
  assert(shellLoaders.includes("module.NotaryView"));
});

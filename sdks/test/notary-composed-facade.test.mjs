import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");
import { imPcRoot, imPcTest, notaryPcRoot } from "./helpers/im-pc-root.mjs";

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function readChatPcText(relativePath) {
  return readFileSync(path.join(imPcRoot, relativePath), "utf8");
}

test("backend composed matter management delegates only through Notary owner orchestration", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/composed/index.ts",
  );
  const source = readFileSync(modulePath, "utf8");
  const { createNotaryBackendApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryBackendApi({
    notary: {
      matters: {
        create: async (body, options) => {
          calls.push({ method: "matters.create", body, options });
          return { skuId: "sku-1", ...body };
        },
        update: async () => ({}),
        management: {
          list: async (input) => {
            calls.push({ method: "matters.management.list", input });
            return { items: [], pageInfo: { mode: "cursor", pageSize: 20, hasMore: false } };
          },
        },
      },
      organizationProfiles: {
        list: async () => ({}),
        create: async () => ({}),
        retrieve: async () => ({}),
        update: async () => ({}),
      },
      staff: { list: async () => ({}) },
      cases: {
        management: { list: async () => ({}), retrieve: async () => ({}) },
        assignments: { create: async () => ({}), delete: async () => undefined },
      },
    },
    appbase: {},
    drive: {},
  });

  await api.createMatter({
    organizationId: "org-1",
    title: "Evidence preservation",
    priceAmount: "99.00",
    originalPriceAmount: "129.00",
    currencyCode: "CNY",
    status: "active",
    idempotencyKey: "matter-intent-1",
  });
  await api.listMatters({ pageSize: 20, q: "evidence" });

  assert.deepEqual(calls, [
    {
      method: "matters.create",
      body: {
        organizationId: "org-1",
        title: "Evidence preservation",
        description: undefined,
        priceAmount: "99.00",
        originalPriceAmount: "129.00",
        currencyCode: "CNY",
        status: "active",
        spec: undefined,
      },
      options: { idempotencyKey: "matter-intent-1" },
    },
    {
      method: "matters.management.list",
      input: { pageSize: 20, q: "evidence" },
    },
  ]);
  assert(!source.includes("CommerceBackendSdkPort"));
  assert(!source.includes("commerce,"));
  assert(!source.includes("productType: \"notary\""));
  assert(!source.includes("skuPolicy: \"one_spu_one_sku\""));
});

test("backend composed staff list uses generated camelCase parameter names", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-backend-sdk/sdkwork-notary-backend-sdk-typescript/composed/index.ts",
  );
  const { createNotaryBackendApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryBackendApi({
    notary: {
      organizationProfiles: {
        list: async () => ({}), create: async () => ({}), retrieve: async () => ({}), update: async () => ({}),
      },
      matters: {
        create: async () => ({}), update: async () => ({}), management: { list: async () => ({}) },
      },
      staff: {
        list: async (input) => {
          calls.push(input);
          return { items: [], pageInfo: { mode: "cursor", pageSize: 20, hasMore: false } };
        },
      },
      cases: {
        management: { list: async () => ({}), retrieve: async () => ({}) },
        assignments: { create: async () => ({}), delete: async () => undefined },
      },
    },
    appbase: {},
    drive: {},
  });

  await api.listStaffMembers({
    organizationId: "org-1",
    staffRole: "notary",
    q: "Li",
    pageSize: 50,
    cursor: "cursor-2",
  });

  assert.deepEqual(calls, [{
    organizationId: "org-1",
    staffRole: "notary",
    q: "Li",
    pageSize: 50,
    cursor: "cursor-2",
  }]);
});

test("app composed notary API exposes high-level workflow methods for IM PC integration", () => {
  const source = readText(
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );

  for (const method of [
    "getDashboardStatistics",
    "getMonthlyReport",
    "listCaseEvents",
    "listStaff",
    "updateCase",
    "acceptCase",
    "rejectCase",
    "completeCase",
    "assignCase",
    "addParty",
    "updateParty",
    "deleteParty",
    "attachPartySignature",
    "createPartyVideoInvite",
    "createPartySignatureInvite",
    "createCaseFileDownloadUrl",
    "deleteCaseFile",
  ]) {
    assert.match(
      source,
      new RegExp(`async\\s+(?:function\\s+)?${method}\\s*\\(`),
      `${method} must exist`,
    );
    assert.match(source, new RegExp(`\\b${method}\\b`), `${method} must be exported`);
  }

  assert(source.includes("notary.dashboard.statistics.retrieve"));
  assert(source.includes("notary.reports.monthly.retrieve"));
  assert(source.includes("notary.cases.events.list"));
  assert(source.includes("notary.staff.list"));
  assert(source.includes("notary.cases.assignments.create"));
  assert(source.includes("notary.cases.acceptances.create"));
  assert(source.includes("notary.cases.rejections.create"));
  assert(source.includes("notary.cases.completions.create"));
  assert(source.includes("notary.cases.parties.create"));
  assert(source.includes("notary.cases.parties.videoInvites.create"));
  assert(source.includes("notary.cases.parties.signatureInvites.create"));
  assert(source.includes("driveSpaceType: \"notary\""));
  assert(source.includes("deleteDriveNode"));
  assert(source.includes("ensureMutableNotaryCase"));
});

test("app composed notary API exposes typed paginated list inputs", () => {
  const source = readText(
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );

  assert.match(
    source,
    /export interface ListCasesInput \{[\s\S]*status\?: string;[\s\S]*skuId\?: string;[\s\S]*q\?: string;[\s\S]*pageSize\?: number;[\s\S]*cursor\?: string;[\s\S]*\}/,
    "listCases must expose the generated SDK pagination and SKU filter as a typed composed facade input",
  );
  assert.match(
    source,
    /async function listCases\(input: ListCasesInput = \{\}\)/,
    "listCases must use the typed ListCasesInput boundary instead of unknown",
  );
  assert.match(
    source,
    /export interface ListCaseFilesInput \{[\s\S]*category\?: "identity" \| "evidence" \| "notary";[\s\S]*pageSize\?: number;[\s\S]*cursor\?: string;[\s\S]*\}/,
    "listCaseFiles must expose the generated SDK pagination and category filter as a typed composed facade input",
  );
  assert.match(
    source,
    /async function listCaseFiles\(caseId: string, input: ListCaseFilesInput = \{\}\)/,
    "listCaseFiles must use the typed ListCaseFilesInput boundary instead of unknown",
  );
});

test("app composed listStaff delegates to app notary staff resource", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      staff: {
        list: async (params) => {
          calls.push({ method: "notary.staff.list", params });
          return {
            items: [
              {
                membershipId: "member-notary-1",
                userId: "user-notary-1",
                displayName: "Li Ming",
                status: "active",
                roles: ["notary"],
                positions: ["notary"],
                departments: ["notary-office"],
                notaryStaffRole: "notary",
              },
            ],
            pageInfo: { hasMore: false },
          };
        },
      },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({ status: "PROCESSING" }),
        update: async () => ({}),
        assignments: { create: async () => ({}) },
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {},
    appbase: {},
  });

  const result = await api.listStaff({ staffRole: "notary", q: "Li" });

  assert.deepEqual(calls, [
    {
      method: "notary.staff.list",
      params: { staffRole: "notary", q: "Li" },
    },
  ]);
  assert.equal(result.items[0].membershipId, "member-notary-1");
  assert.equal(result.items[0].notaryStaffRole, "notary");
});

test("app composed assignCase delegates to app notary assignment resource", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      staff: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({ status: "PROCESSING" }),
        update: async () => ({}),
        assignments: {
          create: async (caseId, body) => {
            calls.push({ method: "notary.assignments.create", caseId, body });
            return {
              id: "assignment-1",
              caseId,
              organizationMembershipId: body.organizationMembershipId,
              userId: "user-notary-1",
              assignmentRole: body.assignmentRole,
              status: "active",
              assignedAt: "2026-06-10T10:30:00Z",
            };
          },
        },
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {},
    appbase: {},
  });

  const result = await api.assignCase("case-1", {
    organizationMembershipId: "member-notary-1",
    assignmentRole: "primary_notary",
  });

  assert.deepEqual(calls, [
    {
      method: "notary.assignments.create",
      caseId: "case-1",
      body: {
        organizationMembershipId: "member-notary-1",
        assignmentRole: "primary_notary",
      },
    },
  ]);
  assert.equal(result.id, "assignment-1");
  assert.equal(result.organizationMembershipId, "member-notary-1");
  assert.equal(result.assignmentRole, "primary_notary");
});

imPcTest("real IM PC notary service consumes the composed workflow facade instead of raw resources", () => {
  const source = readFileSync(
    path.join(notaryPcRoot, "packages/sdkwork-notary-pc-notary/src/services/NotaryService.ts"),
    "utf8",
  );

  for (const method of [
    "notaryApi.acceptCase",
    "notaryApi.rejectCase",
    "notaryApi.completeCase",
    "notaryApi.updateCase",
    "notaryApi.addParty",
  ]) {
    assert(source.includes(method), `adapter must call ${method}`);
  }

  assert(!source.includes("notaryApi.notary.cases."), "adapter must not bypass composed facade");
  assert(!source.includes("fetch("));
  assert(!source.includes("axios"));
  assert(!source.includes("Authorization"));
});

test("app composed createCase always passes generated SDK params object", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async (body, params) => {
          calls.push({ body, params });
          if (!params || typeof params !== "object") {
            throw new Error("generated cases.create params object is required");
          }
          return {
            caseId: "case-1",
            caseNo: "N-1",
            orderId: "order-1",
            orderItemId: "item-1",
            skuId: body.skuId,
            driveSpaceId: "space-1",
            driveSpaceType: "notary",
            driveFolderNodeId: "folder-1",
          };
        },
        list: async () => ({ items: [] }),
        retrieve: async () => ({}),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {},
    appbase: {},
  });

  await api.createCase({
    skuId: "sku-notary-evidence",
    title: "Electronic Evidence Preservation",
    applicantName: "Applicant",
    idempotencyKey: "case-intent-1",
  });

  assert.equal(calls.length, 1);
  assert.deepEqual(calls[0].params, { idempotencyKey: "case-intent-1" });
});

test("app composed deleteCaseFile delegates to Drive node deletion and refreshes through notary file list", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  let fileListCall = 0;
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({ status: "PROCESSING" }),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async (caseId, params) => {
            calls.push({ method: "notary.files.list", caseId, params });
            fileListCall += 1;
            return fileListCall === 1
              ? {
                  items: [{ nodeId: "node-delete", name: "delete.pdf" }],
                  pageInfo: { hasMore: false },
                }
              : { items: [{ nodeId: "node-kept", name: "kept.pdf" }] };
          },
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {
      drive: {
        nodes: {
          delete: async (nodeId) => {
            calls.push({ method: "drive.nodes.delete", nodeId });
            return { deleted: true };
          },
        },
      },
    },
    appbase: {},
  });

  const result = await api.deleteCaseFile("case-1", {
    nodeId: "node-delete",
  });

  assert.deepEqual(calls, [
    {
      method: "notary.files.list",
      caseId: "case-1",
      params: { driveSpaceType: "notary", pageSize: 100 },
    },
    {
      method: "drive.nodes.delete",
      nodeId: "node-delete",
    },
    {
      method: "notary.files.list",
      caseId: "case-1",
      params: { driveSpaceType: "notary" },
    },
  ]);
  assert.deepEqual(result, { items: [{ nodeId: "node-kept", name: "kept.pdf" }] });
});

test("app composed attachPartySignature uploads signature image through Drive and binds returned node id", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({
          status: "PROCESSING",
          driveSpaceId: "space-case-1",
          driveFolderNodeId: "folder-case-1",
        }),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: {
            create: async (caseId, partyId, body) => {
              calls.push({ method: "notary.signatures.create", caseId, partyId, body });
              return { partyId, signatureNodeId: body.signatureNodeId };
            },
          },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {
      uploader: {
        upload: async (input) => {
          calls.push({ method: "drive.uploader.upload", input });
          return { uploadItem: { nodeId: "signature-node-1" } };
        },
      },
    },
    appbase: {},
  });

  const result = await api.attachPartySignature("case-1", "party-1", {
    signatureUrl: "data:image/png;base64,AAAA",
    source: "sdkwork-im-pc",
  });

  assert.equal(calls[0].method, "drive.uploader.upload");
  assert.match(calls[0].input.taskId, /^notary-case-file-case-1-party-1-signature-/);
  assert.deepEqual(Object.keys(calls[0].input).sort(), [
    "appResourceId",
    "appResourceType",
    "file",
    "parentNodeId",
    "scene",
    "source",
    "spaceId",
    "taskId",
    "uploadProfileCode",
  ].sort());
  assert.deepEqual({ ...calls[0].input, file: "<file>", taskId: "<taskId>" }, {
    file: "<file>",
    appResourceType: "notary_case_party_signature",
    appResourceId: "case-1:party-1",
    uploadProfileCode: "image",
    scene: "notary.party_signature",
    source: "sdkwork-im-pc",
    spaceId: "space-case-1",
    parentNodeId: "folder-case-1",
    taskId: "<taskId>",
  });
  assert.deepEqual(calls[1], {
    method: "notary.signatures.create",
    caseId: "case-1",
    partyId: "party-1",
    body: {
      signatureNodeId: "signature-node-1",
      signatureUrl: "data:image/png;base64,AAAA",
    },
  });
  assert.deepEqual(result, { partyId: "party-1", signatureNodeId: "signature-node-1" });
});

test("app composed createPartyVideoInvite delegates to generated notary video invite resource", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({}),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: {
            create: async (caseId, partyId, body) => {
              calls.push({ method: "notary.videoInvites.create", caseId, partyId, body });
              return {
                inviteId: "notary-video-case-1-party-1",
                caseId,
                partyId,
                conversationId: "notary-case-1-party-1-video",
                inviteUrl: "sdkwork://notary/video?inviteId=notary-video-case-1-party-1",
                expiresAt: "2026-06-10T10:10:00Z",
              };
            },
          },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {},
    appbase: {},
  });

  const result = await api.createPartyVideoInvite("case-1", "party-1", {
    purpose: "identity_verification",
  });

  assert.deepEqual(calls, [
    {
      method: "notary.videoInvites.create",
      caseId: "case-1",
      partyId: "party-1",
      body: { purpose: "identity_verification" },
    },
  ]);
  assert.equal(result.conversationId, "notary-case-1-party-1-video");
  assert.equal(result.caseId, "case-1");
  assert.equal(result.partyId, "party-1");
});

test("app composed createPartySignatureInvite delegates to generated notary signature invite resource", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({}),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: {
            create: async (caseId, partyId, body) => {
              calls.push({ method: "notary.signatureInvites.create", caseId, partyId, body });
              return {
                inviteId: "signature-invite-case-1-party-1",
                caseId,
                partyId,
                inviteUrl: "sdkwork://notary/signature?inviteId=signature-invite-case-1-party-1",
                signingUrl: "sdkwork://notary/signature?inviteId=signature-invite-case-1-party-1",
                expiresAt: "2026-06-10T10:10:00Z",
                driveSpaceType: "notary",
              };
            },
          },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {},
    appbase: {},
  });

  const result = await api.createPartySignatureInvite("case-1", "party-1", {
    purpose: "remote_signature",
  });

  assert.deepEqual(calls, [
    {
      method: "notary.signatureInvites.create",
      caseId: "case-1",
      partyId: "party-1",
      body: { purpose: "remote_signature" },
    },
  ]);
  assert.equal(result.inviteUrl, "sdkwork://notary/signature?inviteId=signature-invite-case-1-party-1");
  assert.equal(result.signingUrl, "sdkwork://notary/signature?inviteId=signature-invite-case-1-party-1");
  assert.equal(result.caseId, "case-1");
  assert.equal(result.partyId, "party-1");
});

test("app composed uploadCaseFile keeps party identity files scoped to notary case and party metadata", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({
          status: "PROCESSING",
          driveSpaceId: "space-explicit-case-1",
          driveFolderNodeId: "folder-explicit-case-1",
        }),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [] }),
          create: async (caseId, body) => {
            calls.push({ method: "notary.files.create", caseId, body });
            return { nodeId: body.driveNodeId, partyId: body.partyId, materialCode: body.materialCode };
          },
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {
      uploader: {
        upload: async (input) => {
          calls.push({ method: "drive.uploader.upload", input });
          return { uploadItem: { nodeId: "identity-front-node-1" } };
        },
      },
    },
    appbase: {},
  });

  const result = await api.uploadCaseFile({
    caseId: "case-1",
    partyId: "party-1",
    file: new Blob(["identity-front"], { type: "image/png" }),
    category: "identity",
    materialCode: "identity_front",
    source: "sdkwork-im-pc",
    driveSpaceId: "space-explicit-case-1",
    driveFolderNodeId: "folder-explicit-case-1",
  });

  assert.equal(calls[0].method, "drive.uploader.upload");
  assert.match(calls[0].input.taskId, /^notary-case-file-case-1-party-1-identity_front-/);
  assert.deepEqual(Object.keys(calls[0].input).sort(), [
    "appResourceId",
    "appResourceType",
    "file",
    "parentNodeId",
    "scene",
    "source",
    "spaceId",
    "taskId",
    "uploadProfileCode",
  ].sort());
  assert.deepEqual({ ...calls[0].input, file: "<file>", taskId: "<taskId>" }, {
    file: "<file>",
    appResourceType: "notary_case_party_file",
    appResourceId: "case-1:party-1",
    uploadProfileCode: "document",
    scene: "notary.case_file",
    source: "sdkwork-im-pc",
    spaceId: "space-explicit-case-1",
    parentNodeId: "folder-explicit-case-1",
    taskId: "<taskId>",
  });
  assert.deepEqual(calls[1], {
    method: "notary.files.create",
    caseId: "case-1",
    body: {
      driveNodeId: "identity-front-node-1",
      category: "identity",
      materialCode: "identity_front",
      partyId: "party-1",
      reviewStatus: "pending",
    },
  });
  assert.deepEqual(result, {
    nodeId: "identity-front-node-1",
    partyId: "party-1",
    materialCode: "identity_front",
  });
});

test("app composed uploadCaseFile fails closed when the notary case has no Drive target", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  let uploadCalled = false;
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async () => ({ id: "case-missing-target", status: "PROCESSING" }),
        files: { create: async () => ({}) },
      },
    },
    drive: {
      uploader: {
        upload: async () => {
          uploadCalled = true;
          return {};
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    api.uploadCaseFile({
      caseId: "case-missing-target",
      file: new Blob(["evidence"], { type: "application/pdf" }),
      category: "evidence",
    }),
    /missing driveSpaceId or driveFolderNodeId/,
  );
  assert.equal(uploadCalled, false);
});

test("app composed case file mutations reject terminal cases before Drive side effects", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  let uploadCalled = false;
  let deleteCalled = false;
  let registerCalled = false;
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async () => ({
          id: "case-cancelled",
          status: "CANCELLED",
          driveSpaceId: "space-case-cancelled",
          driveFolderNodeId: "folder-case-cancelled",
        }),
        files: {
          list: async () => ({ items: [] }),
          create: async () => {
            registerCalled = true;
            return {};
          },
        },
      },
    },
    drive: {
      uploader: {
        upload: async () => {
          uploadCalled = true;
          return {};
        },
      },
      nodes: {
        delete: async () => {
          deleteCalled = true;
          return {};
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    api.uploadCaseFile({
      caseId: "case-cancelled",
      file: new Blob(["evidence"], { type: "application/pdf" }),
      category: "evidence",
    }),
    /terminal and cannot be modified/,
  );
  await assert.rejects(
    api.deleteCaseFile("case-cancelled", { nodeId: "node-1" }),
    /terminal and cannot be modified/,
  );
  assert.equal(uploadCalled, false);
  assert.equal(deleteCalled, false);
  assert.equal(registerCalled, false);
});

test("app composed uploadCaseFile rejects Drive targets that do not belong to the case", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  let uploadCalled = false;
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async () => ({
          status: "PROCESSING",
          driveSpaceId: "space-case-1",
          driveFolderNodeId: "folder-case-1",
        }),
        files: { create: async () => ({}) },
      },
    },
    drive: {
      uploader: {
        upload: async () => {
          uploadCalled = true;
          return {};
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    api.uploadCaseFile({
      caseId: "case-1",
      file: new Blob(["evidence"], { type: "application/pdf" }),
      category: "evidence",
      driveSpaceId: "space-other-case",
      driveFolderNodeId: "folder-case-1",
    }),
    /Drive space does not match the case/,
  );
  assert.equal(uploadCalled, false);
});

test("app composed uploadCaseFile never falls back to direct Drive node creation", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  let directNodeCreationCalled = false;
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async () => ({
          status: "PROCESSING",
          driveSpaceId: "space-case-1",
          driveFolderNodeId: "folder-case-1",
        }),
        files: { create: async () => ({}) },
      },
    },
    drive: {
      nodes: {
        files: {
          create: async () => {
            directNodeCreationCalled = true;
            return { nodeId: "bypassed-node" };
          },
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    api.uploadCaseFile({
      caseId: "case-1",
      file: new Blob(["evidence"], { type: "application/pdf" }),
      category: "evidence",
      driveSpaceId: "space-case-1",
      driveFolderNodeId: "folder-case-1",
    }),
    /Drive uploader capability is required/,
  );
  assert.equal(directNodeCreationCalled, false);
});

test("app composed createCaseFileDownloadUrl delegates to Drive node download URL SDK", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const calls = [];
  const api = createNotaryApi({
    notary: {
      access: { retrieve: async () => ({}) },
      matters: { list: async () => ({ items: [] }) },
      cases: {
        create: async () => ({}),
        list: async () => ({ items: [] }),
        retrieve: async () => ({ status: "PROCESSING" }),
        update: async () => ({}),
        acceptances: { create: async () => ({}) },
        rejections: { create: async () => ({}) },
        completions: { create: async () => ({}) },
        parties: {
          list: async () => ({ items: [] }),
          create: async () => ({}),
          update: async () => ({}),
          delete: async () => undefined,
          signatures: { create: async () => ({}) },
          videoInvites: { create: async () => ({}) },
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async () => ({ items: [{ nodeId: "node-1" }] }),
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {
      drive: {
        nodes: {
          downloadUrls: {
            retrieve: async (nodeId, params) => {
              calls.push({ method: "drive.nodes.downloadUrls.retrieve", nodeId, params });
              return {
                downloadUrl: `https://download.example/${nodeId}`,
                expiresAt: "2026-06-10T10:00:00Z",
              };
            },
          },
        },
      },
    },
    appbase: {},
  });

  const result = await api.createCaseFileDownloadUrl("case-1", {
    nodeId: "node-1",
    requestedTtlSeconds: 300,
  });

  assert.deepEqual(calls, [
    {
      method: "drive.nodes.downloadUrls.retrieve",
      nodeId: "node-1",
      params: {
        requestedTtlSeconds: 300,
      },
    },
  ]);
  assert.deepEqual(result, {
    downloadUrl: "https://download.example/node-1",
    url: "https://download.example/node-1",
    expiresAt: "2026-06-10T10:00:00Z",
  });
});

test("app composed deleteCaseFile uses Drive trash.create and never falls back to hard delete", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const trashCalls = [];
  let hardDeleteCalled = false;
  const notary = {
    cases: {
      retrieve: async () => ({ status: "PROCESSING" }),
      files: {
        list: async () => ({ items: [{ nodeId: "node-trash" }] }),
      },
    },
  };
  const api = createNotaryApi({
    notary,
    drive: {
      drive: {
        trash: {
          create: async (nodeId, body) => {
            trashCalls.push({ nodeId, body });
            return { nodeId, trashed: true };
          },
        },
        nodes: {
          delete: async () => {
            hardDeleteCalled = true;
          },
        },
      },
    },
    appbase: {},
  });

  await api.deleteCaseFile("case-1", {
    nodeId: "node-trash",
    operatorId: "operator-1",
    strategy: "trash",
  });

  assert.deepEqual(trashCalls, [
    {
      nodeId: "node-trash",
      body: { operatorId: "operator-1" },
    },
  ]);
  assert.equal(hardDeleteCalled, false);

  const apiWithoutTrash = createNotaryApi({
    notary,
    drive: {
      drive: {
        nodes: {
          delete: async () => {
            hardDeleteCalled = true;
          },
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    apiWithoutTrash.deleteCaseFile("case-1", {
      nodeId: "node-trash",
      strategy: "trash",
    }),
    /Drive trash capability is required/,
  );
  assert.equal(hardDeleteCalled, false);
});

test("app composed uploadCaseFile isolates task ids by case and keeps retry intent stable", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const uploads = [];
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async (caseId) => ({
          status: "PROCESSING",
          driveSpaceId: `space-${caseId}`,
          driveFolderNodeId: `folder-${caseId}`,
        }),
        files: {
          create: async (caseId, body) => ({ caseId, nodeId: body.driveNodeId }),
        },
      },
    },
    drive: {
      uploader: {
        upload: async (input) => {
          uploads.push(input);
          return { uploadItem: { nodeId: `node-${uploads.length}` } };
        },
      },
    },
    appbase: {},
  });
  const file = new Blob(["same-file"], { type: "application/pdf" });
  const commonInput = {
    file,
    category: "evidence",
    materialCode: "evidence_document",
    uploadIntentId: "wizard-attachment-1",
  };

  await api.uploadCaseFile({ caseId: "case-a", ...commonInput });
  await api.uploadCaseFile({ caseId: "case-a", ...commonInput });
  await api.uploadCaseFile({ caseId: "case-b", ...commonInput });

  assert.equal(uploads[0].taskId, uploads[1].taskId);
  assert.notEqual(uploads[0].taskId, uploads[2].taskId);
  assert.match(uploads[0].taskId, /case-a-case-wizard-attachment-1/);
  assert.match(uploads[2].taskId, /case-b-case-wizard-attachment-1/);
});

test("app composed case file ownership follows cursor pagination before Drive access", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  const fileListCalls = [];
  const downloadCalls = [];
  const api = createNotaryApi({
    notary: {
      cases: {
        files: {
          list: async (caseId, params) => {
            fileListCalls.push({ caseId, params });
            if (!params.cursor) {
              return {
                data: {
                  items: [{ driveNodeId: "node-page-1" }],
                  pageInfo: { hasMore: true, nextCursor: "cursor-2" },
                },
              };
            }
            return {
              data: {
                items: [{ driveNodeId: "node-page-2" }],
                pageInfo: { hasMore: false },
              },
            };
          },
        },
      },
    },
    drive: {
      drive: {
        nodes: {
          downloadUrls: {
            retrieve: async (nodeId, params) => {
              downloadCalls.push({ nodeId, params });
              return { downloadUrl: `https://download.example/${nodeId}` };
            },
          },
        },
      },
    },
    appbase: {},
  });

  const result = await api.createCaseFileDownloadUrl("case-1", {
    nodeId: "node-page-2",
    requestedTtlSeconds: 120,
  });

  assert.deepEqual(fileListCalls, [
    {
      caseId: "case-1",
      params: { driveSpaceType: "notary", pageSize: 100 },
    },
    {
      caseId: "case-1",
      params: { driveSpaceType: "notary", pageSize: 100, cursor: "cursor-2" },
    },
  ]);
  assert.deepEqual(downloadCalls, [
    { nodeId: "node-page-2", params: { requestedTtlSeconds: 120 } },
  ]);
  assert.equal(result.downloadUrl, "https://download.example/node-page-2");
});

test("app composed rejects download and delete for nodes outside the notary case", async () => {
  const modulePath = path.join(
    workspaceRoot,
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );
  const { createNotaryApi } = await import(pathToFileURL(modulePath).href);
  let downloadCalled = false;
  let deleteCalled = false;
  const api = createNotaryApi({
    notary: {
      cases: {
        retrieve: async () => ({ status: "PROCESSING" }),
        files: {
          list: async () => ({
            items: [{ driveNodeId: "owned-node" }],
            pageInfo: { hasMore: false },
          }),
        },
      },
    },
    drive: {
      drive: {
        nodes: {
          downloadUrls: {
            retrieve: async () => {
              downloadCalled = true;
              return {};
            },
          },
          delete: async () => {
            deleteCalled = true;
          },
        },
      },
    },
    appbase: {},
  });

  await assert.rejects(
    api.createCaseFileDownloadUrl("case-1", { nodeId: "foreign-node" }),
    /Drive node foreign-node is not registered to notary case case-1/,
  );
  await assert.rejects(
    api.deleteCaseFile("case-1", { nodeId: "foreign-node" }),
    /Drive node foreign-node is not registered to notary case case-1/,
  );
  assert.equal(downloadCalled, false);
  assert.equal(deleteCalled, false);
});

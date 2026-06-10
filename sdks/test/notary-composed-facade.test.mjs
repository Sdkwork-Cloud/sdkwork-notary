import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath, pathToFileURL } from "node:url";

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, "..", "..");
const chatPcRoot = process.env.SDKWORK_CHAT_PC_ROOT
  ? path.resolve(process.env.SDKWORK_CHAT_PC_ROOT)
  : path.resolve(workspaceRoot, "..", "craw-chat", "apps", "sdkwork-chat-pc");

function readText(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), "utf8");
}

function readChatPcText(relativePath) {
  return readFileSync(path.join(chatPcRoot, relativePath), "utf8");
}

test("app composed notary API exposes high-level workflow methods for chat-pc integration", () => {
  const source = readText(
    "sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/composed/index.ts",
  );

  for (const method of [
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
        retrieve: async () => ({}),
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
        retrieve: async () => ({}),
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

test("real chat-pc notary service consumes the composed workflow facade instead of raw resources", () => {
  const source = readChatPcText("packages/sdkwork-clawchat-pc-notary/src/services/NotaryService.ts");

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
  });

  assert.equal(calls.length, 1);
  assert.deepEqual(calls[0].params, {});
});

test("app composed deleteCaseFile delegates to Drive node deletion and refreshes through notary file list", async () => {
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
          signatureInvites: { create: async () => ({}) },
        },
        files: {
          list: async (caseId, params) => {
            calls.push({ method: "notary.files.list", caseId, params });
            return { items: [{ nodeId: "node-kept", name: "kept.pdf" }] };
          },
          create: async () => ({}),
        },
        downloadPackages: { create: async () => ({}) },
      },
    },
    drive: {
      drive: {
        nodes: {
          delete: async (nodeId, params) => {
            calls.push({ method: "drive.nodes.delete", nodeId, params });
            return { deleted: true };
          },
        },
      },
    },
    appbase: {},
  });

  const result = await api.deleteCaseFile("case-1", {
    nodeId: "node-delete",
    tenantId: "tenant-1",
  });

  assert.deepEqual(calls, [
    {
      method: "drive.nodes.delete",
      nodeId: "node-delete",
      params: { tenantId: "tenant-1" },
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
          return { nodeId: "signature-node-1" };
        },
      },
    },
    appbase: {},
  });

  const result = await api.attachPartySignature("case-1", "party-1", {
    signatureUrl: "data:image/png;base64,AAAA",
    source: "sdkwork-chat-pc",
  });

  assert.equal(calls[0].method, "drive.uploader.upload");
  assert.equal(calls[0].input.profile, "image");
  assert.equal(calls[0].input.scene, "notary.party_signature");
  assert.deepEqual(calls[0].input.target, {
    driveSpaceType: "notary",
    appResourceType: "notary_case_party_signature",
    appResourceId: "case-1:party-1",
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
          return { nodeId: "identity-front-node-1" };
        },
      },
    },
    appbase: {},
  });

  const result = await api.uploadCaseFile({
    caseId: "case-1",
    partyId: "party-1",
    file: { name: "id-front.png" },
    category: "identity",
    materialCode: "identity_front",
    source: "sdkwork-chat-pc",
  });

  assert.equal(calls[0].method, "drive.uploader.upload");
  assert.equal(calls[0].input.profile, "document");
  assert.equal(calls[0].input.scene, "notary.case_file");
  assert.deepEqual(calls[0].input.target, {
    driveSpaceType: "notary",
    appResourceType: "notary_case_party_file",
    appResourceId: "case-1:party-1",
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
    drive: {
      drive: {
        nodes: {
          downloadUrls: {
            create: async (nodeId, params) => {
              calls.push({ method: "drive.nodes.downloadUrls.create", nodeId, params });
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
    tenantId: "tenant-1",
    requestedTtlSeconds: 300,
  });

  assert.deepEqual(calls, [
    {
      method: "drive.nodes.downloadUrls.create",
      nodeId: "node-1",
      params: {
        tenantId: "tenant-1",
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

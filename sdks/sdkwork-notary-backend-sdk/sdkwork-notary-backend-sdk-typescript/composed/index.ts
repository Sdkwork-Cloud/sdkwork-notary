export interface NotaryBackendSdkPort {
  organizationProfiles: {
    list(input?: unknown): Promise<unknown>;
    create(input: unknown, options?: unknown): Promise<unknown>;
    retrieve(organizationProfileId: string): Promise<unknown>;
    update(organizationProfileId: string, input: unknown): Promise<unknown>;
  };
  matters: {
    create(input: unknown, options?: unknown): Promise<unknown>;
    update(skuId: string, input: unknown): Promise<unknown>;
    management: {
      list(input?: unknown): Promise<unknown>;
    };
  };
  staff: {
    list(input?: unknown): Promise<unknown>;
  };
  cases: {
    management: {
      list(input?: unknown): Promise<unknown>;
      retrieve(caseId: string): Promise<unknown>;
    };
    assignments: {
      create(caseId: string, input: unknown): Promise<unknown>;
      delete(caseId: string, assignmentId: string): Promise<void>;
    };
  };
}

export interface AppbaseBackendSdkPort {
  iam?: {
    organizations?: {
      retrieve(organizationId: string): Promise<unknown>;
    };
    members?: {
      list(input?: unknown): Promise<unknown>;
    };
    roles?: {
      list(input?: unknown): Promise<unknown>;
    };
    positions?: {
      list(input?: unknown): Promise<unknown>;
    };
    departments?: {
      list(input?: unknown): Promise<unknown>;
    };
  };
}

export interface CommerceBackendSdkPort {
  catalog?: {
    products?: {
      create(input: unknown): Promise<unknown>;
      update(productId: string, input: unknown): Promise<unknown>;
    };
    skus?: {
      create(input: unknown): Promise<unknown>;
      update(skuId: string, input: unknown): Promise<unknown>;
    };
  };
}

export interface DriveBackendSdkPort {
  spaces?: {
    create(input: unknown): Promise<unknown>;
    list(input?: unknown): Promise<unknown>;
  };
  nodes?: {
    list(input: unknown): Promise<unknown>;
  };
}

export interface CreateNotaryBackendApiOptions {
  notary: NotaryBackendSdkPort;
  appbase: AppbaseBackendSdkPort;
  commerce: CommerceBackendSdkPort;
  drive: DriveBackendSdkPort;
}

export interface CreateMatterSkuInput {
  title: string;
  priceAmount: string;
  currencyCode: string;
  organizationId?: string;
  description?: string;
  originalPriceAmount?: string;
  spec?: Record<string, unknown>;
  status?: "draft" | "active" | "inactive";
  idempotencyKey?: string;
}

export interface ListStaffMembersInput {
  organizationId?: string;
  q?: string;
  staffRole?: "notary" | "assistant" | "reviewer" | "approver";
  pageSize?: number;
  cursor?: string;
}

export function createNotaryBackendApi({
  notary,
  appbase,
  commerce,
  drive
}: CreateNotaryBackendApiOptions) {
  async function openNotaryBusiness(input: {
    organizationId: string;
    displayName?: string;
    settings?: Record<string, unknown>;
    idempotencyKey?: string;
  }) {
    return notary.organizationProfiles.create(
      {
        organizationId: input.organizationId,
        displayName: input.displayName,
        settings: {
          ...input.settings,
          driveSpaceType: "notary"
        }
      },
      input.idempotencyKey ? { idempotencyKey: input.idempotencyKey } : undefined
    );
  }

  async function createMatterSku(input: CreateMatterSkuInput) {
    return notary.matters.create(
      {
        organizationId: input.organizationId,
        title: input.title,
        description: input.description,
        priceAmount: input.priceAmount,
        originalPriceAmount: input.originalPriceAmount,
        currencyCode: input.currencyCode,
        status: input.status ?? "active",
        spec: {
          ...input.spec,
          productType: "notary",
          skuPolicy: "one_spu_one_sku"
        }
      },
      input.idempotencyKey ? { idempotencyKey: input.idempotencyKey } : undefined
    );
  }

  async function listMatterSkus(input?: unknown) {
    return notary.matters.management.list(input);
  }

  async function listStaffMembers(input?: ListStaffMembersInput) {
    return notary.staff.list({
      organization_id: input?.organizationId,
      q: input?.q,
      staff_role: input?.staffRole,
      page_size: input?.pageSize,
      cursor: input?.cursor
    });
  }

  async function assignCase(caseId: string, organizationMembershipId: string, assignmentRole: string) {
    return notary.cases.assignments.create(caseId, {
      organizationMembershipId,
      assignmentRole
    });
  }

  async function listCaseFilesByFolder(input: {
    driveSpaceId: string;
    driveFolderNodeId: string;
    pageSize?: number;
    cursor?: string;
  }) {
    if (!drive.nodes) {
      return { items: [], pageInfo: { hasMore: false } };
    }
    return drive.nodes.list({
      driveSpaceType: "notary",
      spaceType: "notary",
      spaceId: input.driveSpaceId,
      parentNodeId: input.driveFolderNodeId,
      page_size: input.pageSize,
      cursor: input.cursor
    });
  }

  return {
    notary,
    appbase,
    commerce,
    drive,
    openNotaryBusiness,
    createMatterSku,
    listMatterSkus,
    listStaffMembers,
    assignCase,
    listCaseFilesByFolder
  };
}

import { appApiPath } from './paths';
import type { ApiRequestOptions, HttpClient } from '../http/client';

import type { CompleteNotaryCaseRequest, CreateCaseAssignmentRequest, CreateNotaryCaseFileRequest, CreateNotaryCaseRequest, CreateNotaryDownloadPackageRequest, CreateNotaryPartyRequest, CreatePartySignatureInviteRequest, CreatePartySignatureRequest, CreatePartyVideoInviteRequest, MonthlyReport, NotaryAccess, NotaryCase, NotaryCaseAssignment, NotaryCaseCommandRequest, NotaryCasePage, NotaryCaseStatus, NotaryDocument, NotaryDocumentCategory, NotaryDocumentList, NotaryDownloadPackage, NotaryMatterPage, NotaryPartyList, NotaryStaffMemberPage, NotaryStatistics, Party, PartySignatureInvite, PartyVideoInvite, RejectNotaryCaseRequest, TimelineEventList, UpdateNotaryCaseRequest, UpdateNotaryPartyRequest } from '../types';


export interface NotaryReportsMonthlyRetrieveParams {
  month?: string;
  format?: 'pdf' | 'excel' | 'csv';
}

export class NotaryReportsMonthlyApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Download monthly business report */
  async retrieve(params?: NotaryReportsMonthlyRetrieveParams, requestOptions?: ApiRequestOptions): Promise<MonthlyReport> {
    const query = buildQueryString([
      { name: 'month', value: params?.month, style: 'form', explode: true, allowReserved: false },
      { name: 'format', value: params?.format, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<MonthlyReport>(appendQueryString(appApiPath(`/notary/reports/monthly`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryReportsApi {
  public readonly monthly: NotaryReportsMonthlyApi;

  constructor(client: HttpClient) {
    this.monthly = new NotaryReportsMonthlyApi(client);
  }

}

export class NotaryDashboardStatisticsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve notary dashboard statistics */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<NotaryStatistics> {
    return this.client.request<NotaryStatistics>(appApiPath(`/notary/dashboard/statistics`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryDashboardApi {
  public readonly statistics: NotaryDashboardStatisticsApi;

  constructor(client: HttpClient) {
    this.statistics = new NotaryDashboardStatisticsApi(client);
  }

}

export interface NotaryStaffListParams {
  pageSize?: number;
  cursor?: string;
  q?: string;
  staffRole?: 'notary' | 'assistant' | 'reviewer' | 'approver';
}

export class NotaryStaffApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List IAM organization members enabled for notary business */
  async list(params?: NotaryStaffListParams, requestOptions?: ApiRequestOptions): Promise<NotaryStaffMemberPage> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'staff_role', value: params?.staffRole, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<NotaryStaffMemberPage>(appendQueryString(appApiPath(`/notary/staff`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class NotaryCasesAssignmentsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Assign an IAM organization member to a notary case */
  async create(caseId: string, body: CreateCaseAssignmentRequest, requestOptions?: ApiRequestOptions): Promise<NotaryCaseAssignment> {
    return this.client.request<NotaryCaseAssignment>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/assignments`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface NotaryCasesEventsListParams {
  pageSize?: number;
  cursor?: string;
}

export class NotaryCasesEventsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List case timeline events */
  async list(caseId: string, params?: NotaryCasesEventsListParams, requestOptions?: ApiRequestOptions): Promise<TimelineEventList> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<TimelineEventList>(appendQueryString(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/events`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class NotaryCasesDownloadPackagesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a Drive-backed download package for case files */
  async create(caseId: string, body: CreateNotaryDownloadPackageRequest, requestOptions?: ApiRequestOptions): Promise<NotaryDownloadPackage> {
    return this.client.request<NotaryDownloadPackage>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/download_packages`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface NotaryCasesFilesListParams {
  category?: NotaryDocumentCategory;
  pageSize?: number;
  cursor?: string;
}

export class NotaryCasesFilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List Drive-backed files in a notary case folder */
  async list(caseId: string, params?: NotaryCasesFilesListParams, requestOptions?: ApiRequestOptions): Promise<NotaryDocumentList> {
    const query = buildQueryString([
      { name: 'category', value: params?.category, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<NotaryDocumentList>(appendQueryString(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/files`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Attach a Drive node to a notary case through Drive node properties */
  async create(caseId: string, body: CreateNotaryCaseFileRequest, requestOptions?: ApiRequestOptions): Promise<NotaryDocument> {
    return this.client.request<NotaryDocument>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/files`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesPartiesSignatureInvitesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a mobile signature invite for a notary party */
  async create(caseId: string, partyId: string, body: CreatePartySignatureInviteRequest, requestOptions?: ApiRequestOptions): Promise<PartySignatureInvite> {
    return this.client.request<PartySignatureInvite>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties/${serializePathParameter(partyId, { name: 'partyId', style: 'simple', explode: false })}/signature_invites`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesPartiesVideoInvitesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Create a video verification invite for a notary party */
  async create(caseId: string, partyId: string, body: CreatePartyVideoInviteRequest, requestOptions?: ApiRequestOptions): Promise<PartyVideoInvite> {
    return this.client.request<PartyVideoInvite>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties/${serializePathParameter(partyId, { name: 'partyId', style: 'simple', explode: false })}/video_invites`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesPartiesSignaturesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Attach a Drive-backed signature node to a party */
  async create(caseId: string, partyId: string, body: CreatePartySignatureRequest, requestOptions?: ApiRequestOptions): Promise<Party> {
    return this.client.request<Party>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties/${serializePathParameter(partyId, { name: 'partyId', style: 'simple', explode: false })}/signatures`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesPartiesApi {
  private client: HttpClient;
  public readonly signatures: NotaryCasesPartiesSignaturesApi;
  public readonly videoInvites: NotaryCasesPartiesVideoInvitesApi;
  public readonly signatureInvites: NotaryCasesPartiesSignatureInvitesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.signatures = new NotaryCasesPartiesSignaturesApi(client);
    this.videoInvites = new NotaryCasesPartiesVideoInvitesApi(client);
    this.signatureInvites = new NotaryCasesPartiesSignatureInvitesApi(client);
  }


/** List parties for a case */
  async list(caseId: string, requestOptions?: ApiRequestOptions): Promise<NotaryPartyList> {
    return this.client.request<NotaryPartyList>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'data' });
  }

/** Create a party for a case */
  async create(caseId: string, body: CreateNotaryPartyRequest, requestOptions?: ApiRequestOptions): Promise<Party> {
    return this.client.request<Party>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Update party information */
  async update(caseId: string, partyId: string, body: UpdateNotaryPartyRequest, requestOptions?: ApiRequestOptions): Promise<Party> {
    return this.client.request<Party>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties/${serializePathParameter(partyId, { name: 'partyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }

/** Remove a party from a case */
  async delete(caseId: string, partyId: string, requestOptions?: ApiRequestOptions): Promise<void> {
    return this.client.request<void>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/parties/${serializePathParameter(partyId, { name: 'partyId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'DELETE' as any });
  }
}

export class NotaryCasesCompletionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Complete a notary case */
  async create(caseId: string, body: CompleteNotaryCaseRequest, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/completions`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesRejectionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Reject a notary case */
  async create(caseId: string, body: RejectNotaryCaseRequest, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/rejections`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryCasesAcceptancesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Accept a paid, non-terminal notary case */
  async create(caseId: string, body: NotaryCaseCommandRequest, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/acceptances`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface NotaryCasesListParams {
  pageSize?: number;
  cursor?: string;
  q?: string;
  status?: NotaryCaseStatus;
  skuId?: string;
}

export interface NotaryCasesCreateParams {
  idempotencyKey: string;
}

export class NotaryCasesApi {
  private client: HttpClient;
  public readonly acceptances: NotaryCasesAcceptancesApi;
  public readonly rejections: NotaryCasesRejectionsApi;
  public readonly completions: NotaryCasesCompletionsApi;
  public readonly parties: NotaryCasesPartiesApi;
  public readonly files: NotaryCasesFilesApi;
  public readonly downloadPackages: NotaryCasesDownloadPackagesApi;
  public readonly events: NotaryCasesEventsApi;
  public readonly assignments: NotaryCasesAssignmentsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.acceptances = new NotaryCasesAcceptancesApi(client);
    this.rejections = new NotaryCasesRejectionsApi(client);
    this.completions = new NotaryCasesCompletionsApi(client);
    this.parties = new NotaryCasesPartiesApi(client);
    this.files = new NotaryCasesFilesApi(client);
    this.downloadPackages = new NotaryCasesDownloadPackagesApi(client);
    this.events = new NotaryCasesEventsApi(client);
    this.assignments = new NotaryCasesAssignmentsApi(client);
  }


/** List notary cases */
  async list(params?: NotaryCasesListParams, requestOptions?: ApiRequestOptions): Promise<NotaryCasePage> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'sku_id', value: params?.skuId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<NotaryCasePage>(appendQueryString(appApiPath(`/notary/cases`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }

/** Create a notary case through Order checkout and a Drive notary folder */
  async create(body: CreateNotaryCaseRequest, params: NotaryCasesCreateParams, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'POST' as any, body, contentType: 'application/json', ...(requestHeaders !== undefined ? { headers: requestHeaders } : {}), sdkworkUnwrapKind: 'item' });
  }

/** Retrieve a notary case */
  async retrieve(caseId: string, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }

/** Update mutable notary case fields */
  async update(caseId: string, body: UpdateNotaryCaseRequest, requestOptions?: ApiRequestOptions): Promise<NotaryCase> {
    return this.client.request<NotaryCase>(appApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'PATCH' as any, body, contentType: 'application/json', sdkworkUnwrapKind: 'item' });
  }
}

export interface NotaryMattersListParams {
  pageSize?: number;
  cursor?: string;
  q?: string;
}

export class NotaryMattersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List active SKU-backed notary matters */
  async list(params?: NotaryMattersListParams, requestOptions?: ApiRequestOptions): Promise<NotaryMatterPage> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.request<NotaryMatterPage>(appendQueryString(appApiPath(`/notary/matters`), query), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'page' });
  }
}

export class NotaryAccessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve current notary business access */
  async retrieve(requestOptions?: ApiRequestOptions): Promise<NotaryAccess> {
    return this.client.request<NotaryAccess>(appApiPath(`/notary/access`), { ...(requestOptions?.signal !== undefined ? { signal: requestOptions.signal } : {}), ...(requestOptions?.timeout !== undefined ? { timeout: requestOptions.timeout } : {}), method: 'GET' as any, sdkworkUnwrapKind: 'item' });
  }
}

export class NotaryApi {
  public readonly access: NotaryAccessApi;
  public readonly matters: NotaryMattersApi;
  public readonly cases: NotaryCasesApi;
  public readonly staff: NotaryStaffApi;
  public readonly dashboard: NotaryDashboardApi;
  public readonly reports: NotaryReportsApi;

  constructor(client: HttpClient) {
    this.access = new NotaryAccessApi(client);
    this.matters = new NotaryMattersApi(client);
    this.cases = new NotaryCasesApi(client);
    this.staff = new NotaryStaffApi(client);
    this.dashboard = new NotaryDashboardApi(client);
    this.reports = new NotaryReportsApi(client);
  }

}

export function createNotaryApi(client: HttpClient): NotaryApi {
  return new NotaryApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
function buildRequestHeaders(
  headers: Record<string, HeaderParameterSpec | undefined>,
  cookies: Record<string, HeaderParameterSpec | undefined> = {},
): Record<string, string> | undefined {
  const requestHeaders: Record<string, string> = {};

  for (const [name, parameter] of Object.entries(headers)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      requestHeaders[name] = serialized;
    }
  }

  const cookieHeader = buildCookieHeader(cookies);
  if (cookieHeader) {
    requestHeaders.Cookie = requestHeaders.Cookie
      ? `${requestHeaders.Cookie}; ${cookieHeader}`
      : cookieHeader;
  }

  return Object.keys(requestHeaders).length > 0 ? requestHeaders : undefined;
}

interface HeaderParameterSpec {
  value: unknown;
  style: string;
  explode: boolean;
  contentType?: string;
}

function buildCookieHeader(cookies: Record<string, HeaderParameterSpec | undefined>): string | undefined {
  const pairs: string[] = [];
  for (const [name, parameter] of Object.entries(cookies)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      pairs.push(`${encodeURIComponent(name)}=${encodeURIComponent(serialized)}`);
    }
  }
  return pairs.length > 0 ? pairs.join('; ') : undefined;
}

function serializeParameterValue(parameter: HeaderParameterSpec | undefined): string | undefined {
  const value = parameter?.value;
  if (value === undefined || value === null) {
    return undefined;
  }
  if (parameter?.contentType) {
    return JSON.stringify(value);
  }
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (Array.isArray(value)) {
    return value.map((item) => serializeHeaderPrimitive(item)).join(',');
  }
  if (typeof value === 'object' && value !== null) {
    return serializeHeaderObject(value as Record<string, unknown>, parameter?.explode === true);
  }
  return serializeHeaderPrimitive(value);
}

function serializeHeaderObject(value: Record<string, unknown>, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (explode) {
    return entries.map(([key, entryValue]) => `${key}=${serializeHeaderPrimitive(entryValue)}`).join(',');
  }
  return entries.flatMap(([key, entryValue]) => [key, serializeHeaderPrimitive(entryValue)]).join(',');
}

function serializeHeaderPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  return String(value);
}

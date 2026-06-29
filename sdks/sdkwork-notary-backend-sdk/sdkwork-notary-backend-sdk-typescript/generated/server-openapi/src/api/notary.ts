import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { CreateCaseAssignmentRequest, CreateNotaryMatterRequest, CreateNotaryOrganizationProfileRequest, NotaryCasesAssignmentsCreateResponse201, NotaryCasesManagementListResponse, NotaryCasesManagementRetrieveResponse, NotaryCaseStatus, NotaryMattersCreateResponse201, NotaryMattersManagementListResponse, NotaryOrganizationProfilesCreateResponse201, NotaryOrganizationProfilesListResponse, NotaryReportsCaseSummaryRetrieveResponse, NotaryStaffListResponse, UpdateNotaryMatterRequest, UpdateNotaryOrganizationProfileRequest } from '../types';


export interface NotaryReportsCaseSummaryRetrieveParams {
  organizationId?: string;
  createdAfter?: string;
  createdBefore?: string;
}

export class NotaryReportsCaseSummaryApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve notary case summary report */
  async retrieve(params?: NotaryReportsCaseSummaryRetrieveParams): Promise<NotaryReportsCaseSummaryRetrieveResponse> {
    const query = buildQueryString([
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'created_after', value: params?.createdAfter, style: 'form', explode: true, allowReserved: false },
      { name: 'created_before', value: params?.createdBefore, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<NotaryReportsCaseSummaryRetrieveResponse>(appendQueryString(backendApiPath(`/notary/reports/case_summary`), query));
  }
}

export class NotaryReportsApi {
  private client: HttpClient;
  public readonly caseSummary: NotaryReportsCaseSummaryApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.caseSummary = new NotaryReportsCaseSummaryApi(client);
  }

}

export interface NotaryStaffListParams {
  organizationId?: string;
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
  async list(params?: NotaryStaffListParams): Promise<NotaryStaffListResponse> {
    const query = buildQueryString([
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'staff_role', value: params?.staffRole, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<NotaryStaffListResponse>(appendQueryString(backendApiPath(`/notary/staff`), query));
  }
}

export class NotaryCasesAssignmentsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Assign an IAM organization member to a notary case */
  async create(caseId: string, body: CreateCaseAssignmentRequest): Promise<NotaryCasesAssignmentsCreateResponse201> {
    return this.client.post<NotaryCasesAssignmentsCreateResponse201>(backendApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/assignments`), body, undefined, undefined, 'application/json');
  }

/** Release a case assignment */
  async delete(caseId: string, assignmentId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}/assignments/${serializePathParameter(assignmentId, { name: 'assignmentId', style: 'simple', explode: false })}`));
  }
}

export interface NotaryCasesManagementListParams {
  pageSize?: number;
  cursor?: string;
  q?: string;
  organizationId?: string;
  status?: NotaryCaseStatus;
}

export class NotaryCasesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List notary cases for operators */
  async list(params?: NotaryCasesManagementListParams): Promise<NotaryCasesManagementListResponse> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<NotaryCasesManagementListResponse>(appendQueryString(backendApiPath(`/notary/cases`), query));
  }

/** Retrieve operator notary case detail */
  async retrieve(caseId: string): Promise<NotaryCasesManagementRetrieveResponse> {
    return this.client.get<NotaryCasesManagementRetrieveResponse>(backendApiPath(`/notary/cases/${serializePathParameter(caseId, { name: 'caseId', style: 'simple', explode: false })}`));
  }
}

export class NotaryCasesApi {
  private client: HttpClient;
  public readonly management: NotaryCasesManagementApi;
  public readonly assignments: NotaryCasesAssignmentsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new NotaryCasesManagementApi(client);
    this.assignments = new NotaryCasesAssignmentsApi(client);
  }

}

export interface NotaryMattersManagementListParams {
  pageSize?: number;
  cursor?: string;
  q?: string;
  organizationId?: string;
}

export class NotaryMattersManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List SKU-backed notary matters */
  async list(params?: NotaryMattersManagementListParams): Promise<NotaryMattersManagementListResponse> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<NotaryMattersManagementListResponse>(appendQueryString(backendApiPath(`/notary/matters`), query));
  }
}

export interface NotaryMattersCreateParams {
  idempotencyKey: string;
}

export class NotaryMattersApi {
  private client: HttpClient;
  public readonly management: NotaryMattersManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new NotaryMattersManagementApi(client);
  }


/** Create a notary matter by creating one Commerce SPU with one SKU */
  async create(body: CreateNotaryMatterRequest, params: NotaryMattersCreateParams): Promise<NotaryMattersCreateResponse201> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<NotaryMattersCreateResponse201>(backendApiPath(`/notary/matters`), body, undefined, requestHeaders, 'application/json');
  }

/** Update a SKU-backed notary matter */
  async update(skuId: string, body: UpdateNotaryMatterRequest): Promise<NotaryMattersCreateResponse201> {
    return this.client.patch<NotaryMattersCreateResponse201>(backendApiPath(`/notary/matters/${serializePathParameter(skuId, { name: 'skuId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface NotaryOrganizationProfilesListParams {
  pageSize?: number;
  cursor?: string;
  organizationId?: string;
}

export interface NotaryOrganizationProfilesCreateParams {
  idempotencyKey: string;
}

export class NotaryOrganizationProfilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List notary organization profiles */
  async list(params?: NotaryOrganizationProfilesListParams): Promise<NotaryOrganizationProfilesListResponse> {
    const query = buildQueryString([
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'organization_id', value: params?.organizationId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<NotaryOrganizationProfilesListResponse>(appendQueryString(backendApiPath(`/notary/organization_profiles`), query));
  }

/** Open notary business for an enterprise-verified organization */
  async create(body: CreateNotaryOrganizationProfileRequest, params: NotaryOrganizationProfilesCreateParams): Promise<NotaryOrganizationProfilesCreateResponse201> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<NotaryOrganizationProfilesCreateResponse201>(backendApiPath(`/notary/organization_profiles`), body, undefined, requestHeaders, 'application/json');
  }

/** Retrieve a notary organization profile */
  async retrieve(organizationProfileId: string): Promise<NotaryOrganizationProfilesCreateResponse201> {
    return this.client.get<NotaryOrganizationProfilesCreateResponse201>(backendApiPath(`/notary/organization_profiles/${serializePathParameter(organizationProfileId, { name: 'organizationProfileId', style: 'simple', explode: false })}`));
  }

/** Update notary organization profile settings or status */
  async update(organizationProfileId: string, body: UpdateNotaryOrganizationProfileRequest): Promise<NotaryOrganizationProfilesCreateResponse201> {
    return this.client.patch<NotaryOrganizationProfilesCreateResponse201>(backendApiPath(`/notary/organization_profiles/${serializePathParameter(organizationProfileId, { name: 'organizationProfileId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class NotaryApi {
  private client: HttpClient;
  public readonly organizationProfiles: NotaryOrganizationProfilesApi;
  public readonly matters: NotaryMattersApi;
  public readonly cases: NotaryCasesApi;
  public readonly staff: NotaryStaffApi;
  public readonly reports: NotaryReportsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.organizationProfiles = new NotaryOrganizationProfilesApi(client);
    this.matters = new NotaryMattersApi(client);
    this.cases = new NotaryCasesApi(client);
    this.staff = new NotaryStaffApi(client);
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

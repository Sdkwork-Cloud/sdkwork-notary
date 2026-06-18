export interface MonthlyReport {
  /** URL to download the report file */
  downloadUrl: string;
  /** Unique report identifier */
  reportId: string;
  /** Month the report covers in YYYY-MM format */
  month?: string;
  /** Report file format */
  format?: 'pdf' | 'excel' | 'csv';
  /** When the report was generated */
  generatedAt?: string;
  /** When the download URL expires */
  expiresAt?: string;
  /** Report file size in bytes */
  fileSize?: number;
  /** Number of cases included in the report */
  caseCount?: number;
}

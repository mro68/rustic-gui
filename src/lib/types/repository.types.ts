/**
 * Platzhalter-Typ für Repository
 * Wird bei Backend-Integration konkretisiert.
 */
export interface Repository {
  id: string;
  name: string;
  path: string;
  type?: string;
  snapshot_count?: number;
  total_size?: string;
  last_backup?: string;
  file_count?: number;
  progress?: number;
  progress_label?: string;
  [key: string]: unknown;
}

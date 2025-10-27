/**
 * Platzhalter-Typ für BackupJob
 * Wird bei Backend-Integration konkretisiert.
 */
export interface BackupJob {
  id: string;
  name: string;
  repositoryId: string;
  [key: string]: unknown;
}

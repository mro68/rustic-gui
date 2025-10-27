/**
 * Platzhalter-Typ für Snapshot
 * Wird bei Backend-Integration konkretisiert.
 */
export interface Snapshot {
  id: string;
  date: string;
  repositoryId?: string;
  [key: string]: unknown;
}

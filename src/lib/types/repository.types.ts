/**
 * Platzhalter-Typ für Repository
 * Wird bei Backend-Integration konkretisiert.
 */
export interface Repository {
  id: string;
  name: string;
  path: string;
  type?: string;
  [key: string]: unknown;
}

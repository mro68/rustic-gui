/**
 * Snapshot-spezifische Type Definitions.
 *
 * @deprecated Diese Typen sind Legacy. Verwende stattdessen `SnapshotDto` aus `index.ts`.
 *
 * @module types/snapshot
 */

/**
 * Platzhalter-Typ für Snapshot.
 *
 * @deprecated Wird bei Backend-Integration durch `SnapshotDto` ersetzt.
 */
export interface Snapshot {
  /** Eindeutige Snapshot-ID */
  id: string;
  /** Erstellungsdatum */
  date: string;
  /** Zugehöriges Repository */
  repositoryId?: string;
  /** Zusätzliche Felder */
  [key: string]: unknown;
}

/**
 * Repository-spezifische Type Definitions.
 *
 * @deprecated Diese Typen sind Legacy. Verwende stattdessen `RepositoryDto` aus `index.ts`.
 *
 * @module types/repository
 */

/**
 * Platzhalter-Typ für Repository.
 *
 * @deprecated Wird bei Backend-Integration durch `RepositoryDto` ersetzt.
 */
export interface Repository {
  /** Eindeutige ID */
  id: string;
  /** Repository-Name */
  name: string;
  /** Repository-Pfad */
  path: string;
  /** Backend-Typ */
  type?: string;
  /** Anzahl Snapshots */
  snapshot_count?: number;
  /** Gesamtgröße */
  total_size?: string;
  /** Letztes Backup */
  last_backup?: string;
  /** Anzahl Dateien */
  file_count?: number;
  /** Progress (0-100) */
  progress?: number;
  /** Progress-Label */
  progress_label?: string;
  /** Zusätzliche Felder */
  [key: string]: unknown;
}

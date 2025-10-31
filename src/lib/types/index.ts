/**
 * TypeScript Type Definitions für Rustic GUI.
 *
 * Diese Interfaces entsprechen den Rust DTOs in `src-tauri/src/types.rs`.
 * Alle Zeitstempel sind ISO 8601 Strings.
 *
 * @module types
 */

/**
 * Repository Data Transfer Object.
 *
 * Repräsentiert ein rustic Repository mit Metadaten.
 */
export interface RepositoryDto {
  /** Eindeutige Repository-ID */
  id: string;
  /** Anzeigename des Repositories */
  name: string;
  /** Pfad zum Repository (lokal oder Remote-URL) */
  path: string;
  /** Backend-Typ (Local, SFTP, S3, etc.) */
  repository_type: RepositoryType;
  /** Aktueller Gesundheitsstatus */
  status: RepositoryStatus;
  /** Anzahl der Snapshots */
  snapshot_count: number;
  /** Gesamtgröße in Bytes */
  total_size: number;
  /** Letzter Zugriff (ISO 8601) */
  last_accessed?: string;
  /** Erstellungsdatum (ISO 8601) */
  created_at: string;
}

/**
 * Repository-Backend-Typ.
 */
export type RepositoryType = 'Local' | 'Sftp' | 'S3' | 'Rest' | 'Rclone';

/**
 * Repository-Gesundheitsstatus.
 */
export type RepositoryStatus = 'Healthy' | 'Warning' | 'Unavailable' | 'Locked';

/**
 * Snapshot Data Transfer Object.
 *
 * Repräsentiert einen einzelnen Backup-Snapshot.
 */
export interface SnapshotDto {
  /** Eindeutige Snapshot-ID */
  id: string;
  /** Erstellungszeit (ISO 8601) */
  time: string;
  /** Hostname des Backup-Clients */
  hostname: string;
  /** Snapshot-Tags für Organisation */
  tags: string[];
  /** Gesicherte Pfade */
  paths: string[];
  /** Anzahl der Dateien */
  file_count: number;
  /** Gesamtgröße in Bytes */
  total_size: number;
  /** Zugehöriges Repository */
  repository_id: string;
  /** Benutzername (optional) */
  username?: string;
  /** Erweiterte Zusammenfassung (optional) */
  summary?: SnapshotSummary;
}

/**
 * Erweiterte Snapshot-Statistiken.
 */
export interface SnapshotSummary {
  /** Anzahl Dateien */
  files_count?: number;
  /** Anzahl Verzeichnisse */
  dirs_count?: number;
  /** Datengröße in Bytes */
  data_size?: number;
}

/**
 * Backup-Job Data Transfer Object.
 *
 * Definiert einen geplanten oder manuellen Backup-Job.
 */
export interface BackupJobDto {
  /** Eindeutige Job-ID */
  id: string;
  /** Anzeigename des Jobs */
  name: string;
  /** Ziel-Repository-ID */
  repository_id: string;
  /** Zu sichernde Quellpfade */
  source_paths: string[];
  /** Job-Tags für Organisation */
  tags: string[];
  /** Cron-Ausdruck für Scheduling (optional) */
  schedule?: string;
  /** Job aktiviert/deaktiviert */
  enabled: boolean;
  /** Letzte Ausführung (ISO 8601) */
  last_run?: string;
  /** Nächste geplante Ausführung (ISO 8601) */
  next_run?: string;
  /** Retention-Policy für alte Snapshots */
  retention?: RetentionPolicy;
}

/**
 * Retention-Policy für Snapshot-Verwaltung.
 *
 * Definiert wie viele Snapshots behalten werden sollen.
 */
export interface RetentionPolicy {
  /** Behalte letzte N Snapshots */
  keep_last?: number;
  /** Behalte tägliche Snapshots (Tage) */
  keep_daily?: number;
  /** Behalte wöchentliche Snapshots (Wochen) */
  keep_weekly?: number;
  /** Behalte monatliche Snapshots (Monate) */
  keep_monthly?: number;
  /** Behalte jährliche Snapshots (Jahre) */
  keep_yearly?: number;
}

/**
 * Restore-Optionen für Wiederherstellung.
 *
 * Konfiguriert wie Dateien aus einem Snapshot wiederhergestellt werden.
 */
export interface RestoreOptionsDto {
  /** Quell-Snapshot-ID */
  snapshot_id: string;
  /** Zielverzeichnis für Wiederherstellung */
  target_path: string;
  /** Spezifische Pfade (leer = alles) */
  paths: string[];
  /** Existierende Dateien überschreiben */
  overwrite: boolean;
  /** Dateiberechtigungen wiederherstellen */
  restore_permissions: boolean;
  /** Zeitstempel wiederherstellen */
  restore_timestamps: boolean;
  /** Dry-Run-Modus (keine Änderungen) */
  dry_run: boolean;
}

/**
 * Generische API-Response.
 *
 * @template T - Typ der Antwortdaten
 */
export interface ApiResponse<T> {
  /** Erfolg oder Fehler */
  success: boolean;
  /** Antwortdaten (bei Erfolg) */
  data?: T;
  /** Fehlermeldung (bei Fehler) */
  error?: string;
}

/**
 * Paginierte API-Response.
 *
 * @template T - Typ der Item-Daten
 */
export interface PaginatedResponse<T> {
  /** Items der aktuellen Seite */
  items: T[];
  /** Gesamtanzahl aller Items */
  total: number;
  /** Aktuelle Seitennummer (1-basiert) */
  page: number;
  /** Items pro Seite */
  page_size: number;
}

/**
 * Basis-Progress-Informationen.
 */
export interface ProgressInfo {
  /** Aktueller Fortschritt */
  current: number;
  /** Gesamt-Anzahl */
  total: number;
  /** Status-Nachricht */
  message: string;
  /** Fortschritt in Prozent (0-100) */
  percentage: number;
}

/**
 * Backup-spezifische Progress-Informationen.
 */
export interface BackupProgress extends ProgressInfo {
  /** Anzahl verarbeiteter Dateien */
  files_processed: number;
  /** Verarbeitete Bytes */
  bytes_processed: number;
  /** Gesamt-Bytes (optional) */
  total_bytes?: number;
  /** Aktuell verarbeitete Datei */
  current_file?: string;
  /** Geschätzte Restzeit in Sekunden */
  estimated_time_remaining?: number;
}

/**
 * Restore-spezifische Progress-Informationen.
 */
export interface RestoreProgress extends ProgressInfo {
  /** Anzahl wiederhergestellter Dateien */
  files_restored: number;
  /** Wiederhergestellte Bytes */
  bytes_restored: number;
  /** Aktuell wiederhergestellte Datei */
  current_file?: string;
}

/**
 * File-Tree-Node für Restore-Browser.
 *
 * Repräsentiert eine Datei oder ein Verzeichnis in einem Snapshot.
 */
export interface FileTreeNode {
  /** Datei-/Verzeichnisname */
  name: string;
  /** Vollständiger Pfad */
  path: string;
  /** Ist Verzeichnis */
  is_directory: boolean;
  /** Dateigröße in Bytes (optional) */
  size?: number;
  /** Letztes Änderungsdatum (ISO 8601) */
  modified?: string;
  /** Untergeordnete Elemente (bei Verzeichnissen) */
  children?: FileTreeNode[];
}

/**
 * Diff-Ergebnis beim Snapshot-Vergleich.
 */
export interface DiffResultDto {
  /** Hinzugefügte Dateien */
  added: string[];
  /** Entfernte Dateien */
  removed: string[];
  /** Geänderte Dateien */
  modified: string[];
  /** Diff-Statistiken */
  stats: DiffStats;
}

/**
 * Statistiken zum Snapshot-Vergleich.
 */
export interface DiffStats {
  /** Anzahl hinzugefügter Dateien */
  added_count: number;
  /** Anzahl entfernter Dateien */
  removed_count: number;
  /** Anzahl geänderter Dateien */
  modified_count: number;
  /** Größenänderung in Bytes (kann negativ sein) */
  total_size_change: number;
}

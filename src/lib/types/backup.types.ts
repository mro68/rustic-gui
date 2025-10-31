/**
 * Backup-Job Type Definitions.
 *
 * @deprecated Diese Typen sind Legacy. Verwende stattdessen `BackupJobDto` aus `index.ts`.
 *
 * @module types/backup
 */

/**
 * Backup-Job-Schedule-Konfiguration.
 *
 * @deprecated Verwende stattdessen `BackupJobDto.schedule` (Cron-String).
 */
export interface BackupJobSchedule {
  /** Schedule-Typ */
  type: 'manual' | 'daily' | 'weekly' | 'monthly' | 'cron';
  /** Uhrzeit (HH:MM) */
  time?: string;
  /** Wochentage (für weekly) */
  days?: string[];
  /** Cron-Ausdruck (für cron) */
  cron?: string;
}

/**
 * Backup-Job-Retention-Policy.
 *
 * @deprecated Verwende stattdessen `RetentionPolicy` aus `index.ts`.
 */
export interface BackupJobRetention {
  /** Behalte letzte N Snapshots */
  keepLast: number;
  /** Behalte stündliche Snapshots */
  keepHourly: number;
  /** Behalte tägliche Snapshots */
  keepDaily: number;
  /** Behalte wöchentliche Snapshots */
  keepWeekly: number;
  /** Behalte monatliche Snapshots */
  keepMonthly: number;
  /** Behalte jährliche Snapshots */
  keepYearly: number;
}

/**
 * Backup-Job-Definition.
 *
 * @deprecated Verwende stattdessen `BackupJobDto` aus `index.ts`.
 */
export interface BackupJob {
  /** Eindeutige Job-ID */
  id: string;
  /** Job-Name */
  name: string;
  /** Ziel-Repository-ID */
  repositoryId: string;
  /** Job-Tags */
  tags?: string[];
  /** Zu sichernde Pfade */
  backupPaths?: string[];
  /** Ausschlussmuster */
  excludePatterns?: string[];
  /** Schedule-Konfiguration */
  schedule?: BackupJobSchedule;
  /** Retention-Policy */
  retention?: BackupJobRetention;
  /** Erstellungsdatum (ISO 8601) */
  createdAt?: string;
  /** Letztes Update (ISO 8601) */
  updatedAt?: string;
  /** Letzte Ausführung (ISO 8601) */
  lastRun?: string;
  /** Job-Status */
  status?: 'idle' | 'running' | 'error';
}

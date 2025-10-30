/**
 * Backup-Job Typ-Definitionen
 */

export interface BackupJobSchedule {
  type: 'manual' | 'daily' | 'weekly' | 'monthly' | 'cron';
  time?: string;
  days?: string[];
  cron?: string;
}

export interface BackupJobRetention {
  keepLast: number;
  keepHourly: number;
  keepDaily: number;
  keepWeekly: number;
  keepMonthly: number;
  keepYearly: number;
}

export interface BackupJob {
  id: string;
  name: string;
  repositoryId: string;
  tags?: string[];
  backupPaths?: string[];
  excludePatterns?: string[];
  schedule?: BackupJobSchedule;
  retention?: BackupJobRetention;
  createdAt?: string;
  updatedAt?: string;
  lastRun?: string;
  status?: 'idle' | 'running' | 'error';
}

import type { BackupJobDto } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Backup-Job-Kommandos (Tauri Backend)
 *
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * - listBackupJobs
 * - getBackupJob
 * - createBackupJob
 * - updateBackupJob
 * - deleteBackupJob
 *
 * Backend-Commands: commands/backup.rs (alle implementiert)
 */

/**
 * Listet alle Backup-Jobs auf.
 *
 * @returns Promise mit Array von Backup-Job-DTOs
 * @throws Error wenn Abfrage fehlschlägt
 */
export async function listBackupJobs(): Promise<BackupJobDto[]> {
  return await invoke<BackupJobDto[]>('list_backup_jobs');
}

/**
 * Holt Details zu einem spezifischen Backup-Job.
 *
 * @param id - Backup-Job-ID
 * @returns Promise mit Backup-Job-DTO
 * @throws Error wenn Job nicht gefunden oder Abfrage fehlschlägt
 */
export async function getBackupJob(id: string): Promise<BackupJobDto> {
  return await invoke<BackupJobDto>('get_backup_job', { id });
}

/**
 * Erstellt einen neuen Backup-Job.
 *
 * @param job - Backup-Job-Konfiguration (partielle DTO)
 * @returns Promise mit generierter Job-ID
 * @throws Error wenn Erstellung fehlschlägt (z.B. ungültige Config)
 */
export async function createBackupJob(job: Partial<BackupJobDto>): Promise<string> {
  return await invoke<string>('create_backup_job', { job });
}

/**
 * Aktualisiert einen existierenden Backup-Job.
 *
 * @param job - Vollständige Backup-Job-Konfiguration
 * @returns Promise (void)
 * @throws Error wenn Job nicht existiert oder Update fehlschlägt
 */
export async function updateBackupJob(job: BackupJobDto): Promise<void> {
  await invoke('update_backup_job', { job });
}

/**
 * Löscht einen Backup-Job.
 *
 * @param id - Backup-Job-ID
 * @returns Promise (void)
 * @throws Error wenn Job nicht existiert oder Löschung fehlschlägt
 */
export async function deleteBackupJob(id: string): Promise<void> {
  await invoke('delete_backup_job', { id });
}

/**
 * Plant einen Backup-Job mit Cron-Expression.
 *
 * @param jobId - Backup-Job-ID
 * @param cronExpression - Cron-Expression (z.B. "0 2 * * *" für täglich 2 Uhr)
 * @returns Promise (void)
 * @throws Error wenn Job nicht existiert oder Cron-Expression ungültig ist
 */
export async function scheduleBackup(jobId: string, cronExpression: string): Promise<void> {
  await invoke('schedule_backup', { jobId, cronExpression });
}

/**
 * Entfernt die Planung eines Backup-Jobs.
 *
 * @param jobId - Backup-Job-ID
 * @returns Promise (void)
 * @throws Error wenn Job nicht geplant ist
 */
export async function unscheduleBackup(jobId: string): Promise<void> {
  await invoke('unschedule_backup', { jobId });
}

/**
 * Listet alle geplanten Backup-Jobs auf.
 *
 * @returns Promise mit Array von Job-IDs
 * @throws Error wenn Abfrage fehlschlägt
 */
export async function listScheduledBackups(): Promise<string[]> {
  return await invoke<string[]>('list_scheduled_backups');
}

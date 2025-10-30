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
 * Listet alle Backup-Jobs auf
 */
export async function listBackupJobs(): Promise<BackupJobDto[]> {
  return await invoke<BackupJobDto[]>('list_backup_jobs');
}

/**
 * Holt Details zu einem spezifischen Backup-Job
 */
export async function getBackupJob(id: string): Promise<BackupJobDto> {
  return await invoke<BackupJobDto>('get_backup_job', { id });
}

/**
 * Erstellt einen neuen Backup-Job
 */
export async function createBackupJob(job: Partial<BackupJobDto>): Promise<string> {
  return await invoke<string>('create_backup_job', { job });
}

/**
 * Aktualisiert einen existierenden Backup-Job
 */
export async function updateBackupJob(job: BackupJobDto): Promise<void> {
  await invoke('update_backup_job', { job });
}

/**
 * Löscht einen Backup-Job
 */
export async function deleteBackupJob(id: string): Promise<void> {
  await invoke('delete_backup_job', { id });
}

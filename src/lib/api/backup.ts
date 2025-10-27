import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Backup-Kommandos (Tauri Backend)
 *
 * - runBackup
 * - cancelBackup
 */

export async function runBackup(jobId: string): Promise<void> {
  await invoke('run_backup', { jobId });
}

export async function cancelBackup(jobId: string): Promise<void> {
  await invoke('cancel_backup', { jobId });
}

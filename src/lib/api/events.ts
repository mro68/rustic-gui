import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';
import type { BackupProgress } from '../types';

/**
 * Event-Listener-Setup für Backup-Progress, Completed, Failed
 *
 * Usage:
 *   const unlisten = await onBackupProgress((data) => { ... });
 *   await unlisten(); // zum Entfernen
 */

export async function onBackupProgress(
  callback: (_data: BackupProgress) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-progress', (event) => callback(event.payload as BackupProgress));
}

export async function onBackupCompleted(
  callback: (_data: { jobId: string; success: boolean }) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-completed', (event) =>
    callback(event.payload as { jobId: string; success: boolean })
  );
}

export async function onBackupFailed(
  callback: (_data: { jobId: string; error: string }) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-failed', (event) =>
    callback(event.payload as { jobId: string; error: string })
  );
}

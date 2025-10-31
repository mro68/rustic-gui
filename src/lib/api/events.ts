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

/**
 * Hört auf Backup-Progress-Events.
 *
 * @param callback - Callback für Progress-Updates (bytes_done, total_bytes, files_done, etc.)
 * @returns Promise mit Unlisten-Funktion zum Cleanup
 * @throws Error wenn Event-Listener nicht registriert werden kann
 *
 * @example
 * ```typescript
 * const unlisten = await onBackupProgress((data) => {
 *   console.log(`Fortschritt: ${data.bytes_done}/${data.total_bytes}`);
 * });
 * // Später aufräumen:
 * await unlisten();
 * ```
 */
export async function onBackupProgress(
  callback: (_data: BackupProgress) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-progress', (event) => callback(event.payload as BackupProgress));
}

/**
 * Hört auf Backup-Completed-Events.
 *
 * @param callback - Callback für erfolgreichen Backup-Abschluss (jobId, success)
 * @returns Promise mit Unlisten-Funktion zum Cleanup
 * @throws Error wenn Event-Listener nicht registriert werden kann
 */
export async function onBackupCompleted(
  callback: (_data: { jobId: string; success: boolean }) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-completed', (event) =>
    callback(event.payload as { jobId: string; success: boolean })
  );
}

/**
 * Hört auf Backup-Failed-Events.
 *
 * @param callback - Callback für fehlgeschlagenen Backup (jobId, error)
 * @returns Promise mit Unlisten-Funktion zum Cleanup
 * @throws Error wenn Event-Listener nicht registriert werden kann
 */
export async function onBackupFailed(
  callback: (_data: { jobId: string; error: string }) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  return await listen('backup-failed', (event) =>
    callback(event.payload as { jobId: string; error: string })
  );
}

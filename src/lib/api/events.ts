import type { UnlistenFn } from '@tauri-apps/api/event';
import { listen } from '@tauri-apps/api/event';

/**
 * Event-Listener-Setup für Backup-Progress, Completed, Failed
 *
 * Usage:
 *   const unlisten = await onBackupProgress((data) => { ... });
 *   await unlisten(); // zum Entfernen
 */

export async function onBackupProgress(callback: (data: any) => void): Promise<UnlistenFn> {
  return await listen('backup-progress', (event) => callback(event.payload));
}

export async function onBackupCompleted(callback: (data: any) => void): Promise<UnlistenFn> {
  return await listen('backup-completed', (event) => callback(event.payload));
}

export async function onBackupFailed(callback: (data: any) => void): Promise<UnlistenFn> {
  return await listen('backup-failed', (event) => callback(event.payload));
}

import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Backup-Kommandos (Tauri Backend)
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * Referenz: TODO.md Zeile 208-214
 *
 * Commands:
 * - runBackup (TODO.md Zeile 190-192) ✅ IMPLEMENTIERT (simuliert in lib.rs:121)
 * - cancelBackup (TODO.md Zeile 193-194) ✅ IMPLEMENTIERT (lib.rs:37)
 *
 * Backend-Events:
 * - backup-progress (BackupEvent in lib.rs:111-117)
 * - backup-completed
 * - backup-failed
 */

export async function runBackup(jobId: string): Promise<void> {
  await invoke('run_backup', { jobId });
}

export async function cancelBackup(jobId: string): Promise<void> {
  await invoke('cancel_backup', { jobId });
}

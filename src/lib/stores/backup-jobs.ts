import type { BackupJobDto } from '$lib/types';
import { writable } from 'svelte/store';
import * as api from '$lib/api/backup-jobs';

/**
 * Store für Backup-Job-Management.
 * TODO.md: Phase 2 - Stores mit Daten-Loading ✅ KOMPLETT
 * Referenz: TODO.md Zeile 219, 320-325
 * 
 * Backend-Commands:
 * - list_backup_jobs: src-tauri/src/commands/backup.rs:289
 * - create_backup_job: src-tauri/src/commands/backup.rs:21
 * - update_backup_job: src-tauri/src/commands/backup.rs:122
 * - delete_backup_job: src-tauri/src/commands/backup.rs:217
 * - get_backup_job: src-tauri/src/commands/backup.rs:255
 * 
 * API-Wrapper: src/lib/api/backup-jobs.ts
 * 
 * Verwendung:
 * - src/lib/components/pages/BackupJobs.svelte (loadJobs in onMount)
 * 
 * Features:
 * - jobs: Liste aller Backup-Jobs
 * - runningJobId: ID des aktuell laufenden Jobs (oder null)
 * - loading: Ladezustand
 * - error: Fehlertext
 */

const _jobs = writable<BackupJobDto[]>([]);
const _runningJobId = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

export const jobs = { 
  subscribe: _jobs.subscribe,
  set: _jobs.set,
  update: _jobs.update
};
export const runningJobId = { subscribe: _runningJobId.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

/**
 * Setzt die Backup-Job-Liste im Store.
 *
 * @param list - Array von Backup-Job-DTOs
 */
export function setJobs(list: BackupJobDto[]): void {
  _jobs.set(list);
}

/**
 * Setzt die ID des aktuell laufenden Backup-Jobs.
 *
 * @param id - Job-ID (oder null wenn kein Job läuft)
 */
export function setRunningJobId(id: string | null): void {
  _runningJobId.set(id);
}

/**
 * Setzt den Ladezustand.
 *
 * @param val - true = Laden aktiv, false = Laden abgeschlossen
 */
export function setLoading(val: boolean): void {
  _loading.set(val);
}

/**
 * Setzt eine Fehlermeldung.
 *
 * @param msg - Fehlertext (oder null zum Löschen)
 */
export function setError(msg: string | null): void {
  _error.set(msg);
}

/**
 * Lädt alle Backup-Jobs vom Backend.
 *
 * Setzt loading=true, ruft API ab, aktualisiert Store.
 * Bei Fehler wird error-Store gesetzt.
 *
 * @returns Promise (void)
 * @throws Error wenn Backend-Abruf fehlschlägt (wird in error-Store gespeichert)
 */
export async function loadJobs(): Promise<void> {
  setLoading(true);
  setError(null);
  
  try {
    const jobList = await api.listBackupJobs();
    _jobs.set(jobList);
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : 'Fehler beim Laden der Backup-Jobs';
    setError(errorMsg);
    console.error('loadJobs error:', err);
  } finally {
    setLoading(false);
  }
}

/**
 * Setzt den Backup-Job-Store auf Initialzustand zurück.
 *
 * Löscht alle Jobs, laufenden Job, Loading-State und Fehler.
 */
export function resetJobs(): void {
  _jobs.set([]);
  _runningJobId.set(null);
  _loading.set(false);
  _error.set(null);
}

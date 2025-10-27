import type { BackupJob } from '$lib/types/backup.types';
import { writable } from 'svelte/store';

/**
 * Store für Backup-Job-Management.
 * - jobs: Liste aller Backup-Jobs
 * - runningJobId: ID des aktuell laufenden Jobs (oder null)
 * - loading: Ladezustand
 * - error: Fehlertext
 */

const _jobs = writable<BackupJob[]>([]);
const _runningJobId = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

export const jobs = { subscribe: _jobs.subscribe };
export const runningJobId = { subscribe: _runningJobId.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

// Actions
export function setJobs(list: BackupJob[]): void {
  _jobs.set(list);
}

export function setRunningJobId(id: string | null): void {
  _runningJobId.set(id);
}

export function setLoading(val: boolean): void {
  _loading.set(val);
}

export function setError(msg: string | null): void {
  _error.set(msg);
}

export function resetJobs(): void {
  _jobs.set([]);
  _runningJobId.set(null);
  _loading.set(false);
  _error.set(null);
}

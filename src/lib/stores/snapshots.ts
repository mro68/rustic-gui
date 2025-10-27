import type { Snapshot } from '$lib/types/snapshot.types';
import { writable } from 'svelte/store';

/**
 * Store für Snapshot-Management.
 * - snapshots: Liste aller Snapshots
 * - filter: Filter-Objekt (optional)
 * - sort: Sortierkriterium (optional)
 * - loading: Ladezustand
 * - error: Fehlertext
 */

type SnapshotFilter = {
  tag?: string;
  dateFrom?: Date;
  dateTo?: Date;
  hostname?: string;
} | null;

const _snapshots = writable<Snapshot[]>([]);
const _filter = writable<SnapshotFilter>(null);
const _sort = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

export const snapshots = { subscribe: _snapshots.subscribe };
export const filter = { subscribe: _filter.subscribe };
export const sort = { subscribe: _sort.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

// Actions
export function setSnapshots(list: Snapshot[]): void {
  _snapshots.set(list);
}

export function setFilter(f: SnapshotFilter): void {
  _filter.set(f);
}

export function setSort(s: string | null): void {
  _sort.set(s);
}

export function setLoading(val: boolean): void {
  _loading.set(val);
}

export function setError(msg: string | null): void {
  _error.set(msg);
}

export function resetSnapshots(): void {
  _snapshots.set([]);
  _filter.set(null);
  _sort.set(null);
  _loading.set(false);
  _error.set(null);
}

/* eslint-env browser, es2021 */
import { listSnapshots as apiListSnapshots } from '$lib/api/snapshots';
import type { SnapshotDto } from '$lib/types/index';
import { get, writable } from 'svelte/store';

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

const _snapshots = writable<SnapshotDto[]>([]);
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
export function setSnapshots(list: SnapshotDto[]): void {
  _snapshots.set(list);
}

export function addSnapshots(newSnapshots: SnapshotDto[]): void {
  _snapshots.update((current) => {
    // Vermeide Duplikate basierend auf ID
    const existingIds = new Set(current.map((s) => s.id));
    const uniqueNew = newSnapshots.filter((s) => !existingIds.has(s.id));
    return [...current, ...uniqueNew];
  });
}

export function removeSnapshot(snapshotId: string): void {
  _snapshots.update((current) => current.filter((s) => s.id !== snapshotId));
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

// Async Actions
export async function loadSnapshots(repositoryId?: string): Promise<void> {
  setLoading(true);
  setError(null);

  try {
    const snapshotList = await apiListSnapshots(repositoryId);
    if (repositoryId) {
      // Wenn eine spezifische Repository geladen wird, füge hinzu statt zu ersetzen
      addSnapshots(snapshotList);
    } else {
      // Wenn alle Snapshots geladen werden, ersetze die Liste
      setSnapshots(snapshotList);
    }
  } catch (error) {
    console.error('Failed to load snapshots:', error);
    setError(error instanceof Error ? error.message : 'Failed to load snapshots');
  } finally {
    setLoading(false);
  }
}

export async function refreshSnapshots(): Promise<void> {
  const currentSnapshots = get(_snapshots);
  const repositoryIds = [...new Set(currentSnapshots.map((s) => s.repository_id))];

  setLoading(true);
  setError(null);

  try {
    // Lade Snapshots für alle bekannten Repositories neu
    const allSnapshots: SnapshotDto[] = [];
    for (const repoId of repositoryIds) {
      const repoSnapshots = await apiListSnapshots(repoId);
      allSnapshots.push(...repoSnapshots);
    }
    setSnapshots(allSnapshots);
  } catch (error) {
    console.error('Failed to refresh snapshots:', error);
    setError(error instanceof Error ? error.message : 'Failed to refresh snapshots');
  } finally {
    setLoading(false);
  }
}

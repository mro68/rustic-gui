/* eslint-env browser, es2021 */
/**
 * snapshots.ts - Store für Snapshot-Management
 *
 * TODO.md: Phase 2 - Stores mit Daten-Loading ✅ KOMPLETT
 * Referenz: TODO.md Zeile 53, 116-119, 320-325
 *
 * Backend-Commands:
 * - list_snapshots_command: src-tauri/src/lib.rs:96
 * - get_snapshot_command: src-tauri/src/lib.rs:84
 * - delete_snapshot_command: src-tauri/src/lib.rs:73
 *
 * API-Wrapper: src/lib/api/snapshots.ts
 *
 * Verwendung:
 * - src/lib/components/pages/Snapshots.svelte (refreshSnapshots in onMount)
 * - src/lib/components/pages/DashboardPage.svelte
 *
 * Features:
 * - Snapshot-Liste mit Filter-Unterstützung
 * - Loading/Error States
 * - loadSnapshots() lädt Snapshots vom Backend
 */

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

/**
 * Setzt die Snapshot-Liste im Store (ersetzt alte Liste).
 *
 * @param list - Array von Snapshot-DTOs
 */
export function setSnapshots(list: SnapshotDto[]): void {
  _snapshots.set(list);
}

/**
 * Fügt neue Snapshots hinzu (vermeidet Duplikate basierend auf ID).
 *
 * @param newSnapshots - Array von Snapshot-DTOs zum Hinzufügen
 */
export function addSnapshots(newSnapshots: SnapshotDto[]): void {
  _snapshots.update((current) => {
    // Vermeide Duplikate basierend auf ID
    const existingIds = new Set(current.map((s) => s.id));
    const uniqueNew = newSnapshots.filter((s) => !existingIds.has(s.id));
    return [...current, ...uniqueNew];
  });
}

/**
 * Entfernt einen Snapshot aus dem Store.
 *
 * @param snapshotId - ID des zu entfernenden Snapshots
 */
export function removeSnapshot(snapshotId: string): void {
  _snapshots.update((current) => current.filter((s) => s.id !== snapshotId));
}

/**
 * Setzt den Snapshot-Filter.
 *
 * @param f - Filter-Objekt (tag, dateFrom, dateTo, hostname) oder null
 */
export function setFilter(f: SnapshotFilter): void {
  _filter.set(f);
}

/**
 * Setzt das Sortierkriterium.
 *
 * @param s - Sortierkriterium (z.B. "date", "size") oder null
 */
export function setSort(s: string | null): void {
  _sort.set(s);
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
 * Setzt den Snapshot-Store auf Initialzustand zurück.
 *
 * Löscht alle Snapshots, Filter, Sort, Loading-State und Fehler.
 */
export function resetSnapshots(): void {
  _snapshots.set([]);
  _filter.set(null);
  _sort.set(null);
  _loading.set(false);
  _error.set(null);
}

/**
 * Lädt Snapshots vom Backend (optional für ein spezifisches Repository).
 *
 * @param repositoryId - Repository-ID (optional, listet alle wenn nicht angegeben)
 * @returns Promise (void)
 * @throws Error wenn Backend-Abruf fehlschlägt (wird in error-Store gespeichert)
 */
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

/**
 * Lädt alle Snapshots aller bekannten Repositories neu.
 *
 * Extrahiert Repository-IDs aus aktuellen Snapshots und lädt diese neu.
 *
 * @returns Promise (void)
 * @throws Error wenn Backend-Abruf fehlschlägt (wird in error-Store gespeichert)
 */
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

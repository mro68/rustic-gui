import type { DiffResultDto, SnapshotDto } from '$lib/types/index';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Snapshot-Kommandos (Tauri Backend)
 *
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * Referenz: TODO.md Zeile 211, 316-317
 *
 * Backend-Implementation:
 * - listSnapshots → src-tauri/src/lib.rs:96 (list_snapshots_command)
 * - getSnapshotInfo → src-tauri/src/lib.rs:84 (get_snapshot_command)
 * - deleteSnapshot → src-tauri/src/lib.rs:73 (delete_snapshot_command)
 * - compareSnapshots → ⏳ auskommentiert in lib.rs:422, stub in commands/snapshot.rs:26
 *
 * ⚠️ Hinweis: Backend-Implementations sind teilweise Stubs (siehe TODOs in Rust-Code)
 *
 * Verwendung in Frontend:
 * - src/lib/stores/snapshots.ts (loadSnapshots)
 * - src/lib/components/pages/Snapshots.svelte
 * - src/lib/components/dialogs/CompareSnapshotsDialog.svelte
 */

/**
 * Listet alle Snapshots auf (optional für ein spezifisches Repository).
 *
 * @param repositoryId - Repository-ID (optional, listet alle wenn nicht angegeben)
 * @returns Promise mit Array von Snapshot-DTOs
 * @throws Error wenn Abfrage fehlschlägt
 */
export async function listSnapshots(repositoryId?: string): Promise<SnapshotDto[]> {
  return await invoke<SnapshotDto[]>('list_snapshots_command', { repositoryId });
}

/**
 * Holt detaillierte Informationen zu einem Snapshot.
 *
 * @param id - Snapshot-ID
 * @returns Promise mit Snapshot-DTO
 * @throws Error wenn Snapshot nicht gefunden oder Abfrage fehlschlägt
 */
export async function getSnapshotInfo(id: string): Promise<SnapshotDto> {
  return await invoke<SnapshotDto>('get_snapshot_command', { id });
}

/**
 * Löscht einen Snapshot.
 * Task 4.1: Snapshot-Deletion
 *
 * @param snapshotId - Snapshot-ID
 * @returns Promise (void)
 * @throws Error wenn Snapshot nicht existiert oder Löschung fehlschlägt
 */
export async function deleteSnapshot(snapshotId: string): Promise<void> {
  await invoke('delete_snapshot_command', { snapshotId });
}

/**
 * Vergleicht zwei Snapshots und gibt die Unterschiede zurück.
 *
 * @param snapshotIdA - ID des ersten Snapshots
 * @param snapshotIdB - ID des zweiten Snapshots
 * @param password - Repository-Passwort (benötigt für Indexierung)
 * @returns Promise mit Diff-Ergebnis (hinzugefügt/entfernt/geändert)
 * @throws Error wenn Snapshots nicht gefunden oder Vergleich fehlschlägt
 */
export async function compareSnapshots(
  snapshotIdA: string,
  snapshotIdB: string,
  password: string
): Promise<DiffResultDto> {
  return await invoke<DiffResultDto>('compare_snapshots', { snapshotIdA, snapshotIdB, password });
}

/**
 * Fügt Tags zu einem Snapshot hinzu.
 *
 * @param snapshotId - Snapshot-ID
 * @param tags - Array von Tags zum Hinzufügen
 * @returns Promise (void)
 * @throws Error wenn Snapshot nicht existiert oder Operation fehlschlägt
 */
export async function addSnapshotTags(snapshotId: string, tags: string[]): Promise<void> {
  await invoke('add_snapshot_tags', { snapshotId, tags });
}

/**
 * Entfernt Tags von einem Snapshot.
 *
 * @param snapshotId - Snapshot-ID
 * @param tags - Array von Tags zum Entfernen
 * @returns Promise (void)
 * @throws Error wenn Snapshot nicht existiert oder Operation fehlschlägt
 */
export async function removeSnapshotTags(snapshotId: string, tags: string[]): Promise<void> {
  await invoke('remove_snapshot_tags', { snapshotId, tags });
}

/**
 * Snapshot-Filter-Optionen
 */
export interface SnapshotFilter {
  /** Filter nach Tags (OR-Logic: wenn irgendeiner passt) */
  tags?: string[];
  /** Filter nach Hostname */
  hostname?: string;
  /** Filter nach Zeitraum (von) - ISO 8601 String */
  timeFrom?: string;
  /** Filter nach Zeitraum (bis) - ISO 8601 String */
  timeTo?: string;
}

/**
 * Listet Snapshots mit optionalen Filtern.
 *
 * @param repositoryPath - Repository-Pfad
 * @param password - Repository-Passwort
 * @param filter - Optionale Filter-Optionen
 * @returns Promise mit gefilterten Snapshots
 * @throws Error wenn Abfrage fehlschlägt
 */
export async function listSnapshotsFiltered(
  repositoryPath: string,
  password: string,
  filter?: SnapshotFilter
): Promise<SnapshotDto[]> {
  return await invoke<SnapshotDto[]>('list_snapshots_filtered', {
    repositoryPath,
    password,
    filter,
  });
}

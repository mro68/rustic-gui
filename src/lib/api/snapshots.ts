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
 *
 * @param id - Snapshot-ID
 * @returns Promise (void)
 * @throws Error wenn Snapshot nicht existiert oder Löschung fehlschlägt
 */
export async function deleteSnapshot(id: string): Promise<void> {
  await invoke('delete_snapshot_command', { id });
}

/**
 * Vergleicht zwei Snapshots und gibt die Unterschiede zurück.
 *
 * @param idA - ID des ersten Snapshots
 * @param idB - ID des zweiten Snapshots
 * @returns Promise mit Diff-Ergebnis (hinzugefügt/entfernt/geändert)
 * @throws Error wenn Snapshots nicht gefunden oder Vergleich fehlschlägt
 *
 * ⏳ TODO: Backend-Command noch nicht registriert (lib.rs:422 auskommentiert)
 * Frontend: CompareSnapshotsDialog.svelte wartet auf diese Implementation
 */
export async function compareSnapshots(idA: string, idB: string): Promise<DiffResultDto> {
  return await invoke<DiffResultDto>('compare_snapshots', { idA, idB });
}

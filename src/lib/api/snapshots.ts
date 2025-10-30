import type { SnapshotDto, DiffResultDto } from '$lib/types/index';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Snapshot-Kommandos (Tauri Backend)
 *
 * - listSnapshots
 * - getSnapshotInfo
 * - deleteSnapshot
 * - compareSnapshots
 */

export async function listSnapshots(repositoryId?: string): Promise<SnapshotDto[]> {
  return await invoke<SnapshotDto[]>('list_snapshots', { repositoryId });
}

export async function getSnapshotInfo(id: string): Promise<SnapshotDto> {
  return await invoke<SnapshotDto>('get_snapshot_info', { id });
}

export async function deleteSnapshot(id: string): Promise<void> {
  await invoke('delete_snapshot', { id });
}

/**
 * Vergleicht zwei Snapshots und gibt die Unterschiede zurück
 */
export async function compareSnapshots(idA: string, idB: string): Promise<DiffResultDto> {
  return await invoke<DiffResultDto>('compare_snapshots', { idA, idB });
}

import type { SnapshotDto } from '$lib/types/index';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Snapshot-Kommandos (Tauri Backend)
 *
 * - listSnapshots
 * - getSnapshotInfo
 * - deleteSnapshot
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

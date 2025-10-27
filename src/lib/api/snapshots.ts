import type { Snapshot } from '$lib/types/snapshot.types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Snapshot-Kommandos (Tauri Backend)
 *
 * - listSnapshots
 * - getSnapshotInfo
 * - deleteSnapshot
 */

export async function listSnapshots(): Promise<Snapshot[]> {
  return await invoke<Snapshot[]>('list_snapshots');
}

export async function getSnapshotInfo(id: string): Promise<Snapshot> {
  return await invoke<Snapshot>('get_snapshot_info', { id });
}

export async function deleteSnapshot(id: string): Promise<void> {
  await invoke('delete_snapshot', { id });
}

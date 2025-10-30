// src/lib/api/restore.ts

import type { FileTreeNode, RestoreOptionsDto, RestoreProgress } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

/**
 * API-Wrapper für Restore-Operationen.
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * Referenz: TODO.md Zeile 208-214
 *
 * Kapselt alle Tauri-Commands und Events für Datei-Wiederherstellung.
 * 
 * Commands:
 * - getFileTree (TODO.md Zeile 198) ✅ IMPLEMENTIERT (lib.rs:312, stub in rustic/restore.rs:181)
 * - restoreFiles (TODO.md Zeile 195-197) ✅ IMPLEMENTIERT (lib.rs:324, simuliert mit Events)
 * 
 * Backend-Events:
 * - restore-progress (RestoreEvent in lib.rs:16-24)
 * - restore-completed
 * - restore-failed
 */

/**
 * Lädt die Dateistruktur eines Snapshots.
 *
 * @param repositoryPath - Pfad zum Repository
 * @param password - Repository-Passwort
 * @param snapshotId - ID des Snapshots
 * @param path - Optionaler Pfad innerhalb des Snapshots (für Lazy-Loading)
 * @returns Promise mit FileTreeNode
 */
export async function getFileTree(
  repositoryPath: string,
  password: string,
  snapshotId: string,
  path?: string
): Promise<FileTreeNode> {
  return await invoke<FileTreeNode>('get_file_tree_command', {
    repositoryPath,
    password,
    snapshotId,
    path,
  });
}

/**
 * Stellt Dateien aus einem Snapshot wieder her.
 *
 * @param repositoryPath - Pfad zum Repository
 * @param password - Repository-Passwort
 * @param snapshotId - ID des Snapshots
 * @param files - Liste der wiederherzustellenden Dateien/Pfade
 * @param targetPath - Zielverzeichnis für die Wiederherstellung
 * @param options - Restore-Optionen
 * @returns Promise mit void (Erfolg)
 */
export async function restoreFiles(
  repositoryPath: string,
  password: string,
  snapshotId: string,
  files: string[],
  targetPath: string,
  options: Omit<RestoreOptionsDto, 'snapshot_id' | 'target_path' | 'paths'>
): Promise<void> {
  const restoreOptions: RestoreOptionsDto = {
    snapshot_id: snapshotId,
    target_path: targetPath,
    paths: files,
    ...options,
  };

  return await invoke('restore_files_command', {
    repositoryPath,
    password,
    snapshotId,
    files,
    targetPath,
    options: restoreOptions,
  });
}

/**
 * Hört auf Restore-Progress-Events.
 *
 * @param jobId - Job-ID für die Progress-Events (falls verfügbar)
 * @param callback - Callback für Progress-Updates
 * @returns Unlisten-Funktion zum Cleanup
 */
export async function onRestoreProgress(
  jobId: string | null,
  callback: (progress: RestoreProgress) => void // eslint-disable-line no-unused-vars
): Promise<UnlistenFn> {
  const eventName = jobId ? `restore-progress-${jobId}` : 'restore-progress';
  return await listen<RestoreProgress>(eventName, (event) => {
    callback(event.payload);
  });
}

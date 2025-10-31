/**
 * API-Wrapper für Keychain/Passwort-Management
 *
 * Kommuniziert mit Backend-Commands für sichere Passwort-Speicherung.
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Speichert ein Repository-Passwort im System-Keychain.
 * @param repoId - Repository-ID
 * @param password - Das zu speichernde Passwort
 */
export async function storeRepositoryPassword(
  repoId: string,
  password: string
): Promise<void> {
  await invoke('store_repository_password', { repoId, password });
}

/**
 * Lädt ein Repository-Passwort aus dem System-Keychain.
 * @param repoId - Repository-ID
 * @returns Das gespeicherte Passwort
 */
export async function getRepositoryPassword(repoId: string): Promise<string> {
  return await invoke('get_repository_password', { repoId });
}

/**
 * Löscht ein Repository-Passwort aus dem System-Keychain.
 * @param repoId - Repository-ID
 */
export async function deleteRepositoryPassword(repoId: string): Promise<void> {
  await invoke('delete_repository_password', { repoId });
}

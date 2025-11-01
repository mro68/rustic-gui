/**
 * API-Wrapper für Keychain/Passwort-Management
 *
 * Kommuniziert mit Backend-Commands für sichere Passwort-Speicherung.
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Speichert ein Repository-Passwort im System-Keychain.
 *
 * @param repoId - Repository-ID
 * @param password - Das zu speichernde Passwort
 * @returns Promise (void)
 * @throws Error wenn Keychain-Zugriff fehlschlägt oder Passwort nicht gespeichert werden kann
 */
/**
 * Speichert ein Repository-Passwort im System-Keychain.
 *
 * Nutzt den nativen Credential-Store des Betriebssystems:
 * - Linux: GNOME Keyring / KWallet
 * - Windows: Windows Credential Manager
 * - macOS: Keychain Access
 *
 * @param repoId - Eindeutige Repository-ID
 * @param password - Zu speicherndes Passwort
 * @returns Promise<void>
 * @throws Error wenn Keychain-Zugriff fehlschlägt
 *
 * @example
 * ```typescript
 * await storeRepositoryPassword('repo-123', 'my-password');
 * ```
 */
export async function storeRepositoryPassword(repoId: string, password: string): Promise<void> {
  await invoke('store_repository_password', { repoId, password });
}

/**
 * Lädt ein Repository-Passwort aus dem System-Keychain.
 *
 * @param repoId - Repository-ID
 * @returns Promise mit dem gespeicherten Passwort
 * @throws Error wenn Keychain-Zugriff fehlschlägt oder Passwort nicht gefunden
 */
/**
 * Holt ein gespeichertes Repository-Passwort aus dem System-Keychain.
 *
 * @param repoId - Repository-ID
 * @returns Promise<string> - Das gespeicherte Passwort
 * @throws Error wenn Passwort nicht gefunden oder Keychain-Zugriff fehlschlägt
 *
 * @example
 * ```typescript
 * try {
 *   const password = await getRepositoryPassword('repo-123');
 *   console.log('Passwort gefunden');
 * } catch (error) {
 *   console.error('Passwort nicht im Keychain gespeichert');
 * }
 * ```
 */
export async function getRepositoryPassword(repoId: string): Promise<string> {
  return await invoke('get_repository_password', { repoId });
}

/**
 * Löscht ein Repository-Passwort aus dem System-Keychain.
 *
 * @param repoId - Repository-ID
 * @returns Promise (void)
 * @throws Error wenn Keychain-Zugriff fehlschlägt oder Passwort nicht gelöscht werden kann
 */
export async function deleteRepositoryPassword(repoId: string): Promise<void> {
  await invoke('delete_repository_password', { repoId });
}

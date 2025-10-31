import type { RepositoryDto } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

/**
 * API-Wrapper für Repository-Kommandos (Tauri Backend)
 *
 * TODO.md: Phase 2 - API-Wrapper ✅ KOMPLETT
 * - initRepository
 * - openRepository
 * - checkRepository
 * - listRepositories
 * - deleteRepository
 * - pruneRepository
 * - changePassword
 *
 * Backend-Commands:
 * - lib.rs (init, open, check_repository_v1)
 * - commands/repository.rs (list, delete, check, prune, change_password)
 *
 * ⚠️ Hinweis: Viele Backend-Implementations sind Stubs (siehe TODO-Kommentare in Rust-Code)
 */

/**
 * Initialisiert ein neues rustic Repository.
 *
 * @param path - Pfad zum Repository (lokal oder Remote-URL)
 * @param password - Verschlüsselungspasswort
 * @param backendType - Backend-Typ ('local', 'sftp', 's3', etc.)
 * @param backendOptions - Backend-spezifische Optionen (optional)
 * @returns Promise mit Repository-DTO
 * @throws Error wenn Initialisierung fehlschlägt
 */
export async function initRepository(
  path: string,
  password: string,
  backendType: string,
  backendOptions?: Record<string, unknown>
): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('init_repository', {
    path,
    password,
    backendType,
    backendOptions,
  });
}

/**
 * Öffnet ein bestehendes Repository.
 *
 * @param path - Pfad zum Repository
 * @param password - Entschlüsselungspasswort
 * @returns Promise mit Repository-DTO
 * @throws Error wenn Repository nicht geöffnet werden kann
 */
export async function openRepository(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('open_repository', { path, password });
}

/**
 * Prüft Repository-Integrität (v1 - mit Pfad und Passwort).
 *
 * @param path - Pfad zum Repository
 * @param password - Entschlüsselungspasswort
 * @returns Promise mit Repository-DTO
 * @throws Error wenn Check fehlschlägt
 */
export async function checkRepositoryV1(path: string, password: string): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('check_repository_v1', { path, password });
}

/**
 * Prüft Repository-Integrität (v2 - verwendet gespeicherte Credentials).
 *
 * @param id - Repository-ID
 * @param readData - Vollständige Datenprüfung (langsamer, aber gründlicher)
 * @returns Promise mit Check-Ergebnis
 * @throws Error wenn Check fehlschlägt
 */
export async function checkRepository(id: string, readData: boolean = false): Promise<string> {
  return await invoke<string>('check_repository', { id, readData });
}

/**
 * Listet alle Repositories auf.
 *
 * @returns Promise mit Array von Repository-DTOs
 * @throws Error wenn Abfrage fehlschlägt
 */
export async function listRepositories(): Promise<RepositoryDto[]> {
  return await invoke<RepositoryDto[]>('list_repositories');
}

/**
 * Löscht ein Repository.
 *
 * @param id - Repository-ID
 * @param deleteData - Auch Backup-Daten löschen (nicht nur Config)
 * @returns Promise (void)
 * @throws Error wenn Löschung fehlschlägt
 */
export async function deleteRepository(id: string, deleteData: boolean): Promise<void> {
  await invoke('delete_repository', { id, deleteData });
}

/**
 * Führt Prune-Operation aus (entfernt unnötige Pack-Dateien).
 *
 * @param id - Repository-ID
 * @returns Promise mit Prune-Ergebnis
 * @throws Error wenn Prune fehlschlägt
 */
export async function pruneRepository(id: string): Promise<string> {
  return await invoke<string>('prune_repository', { id });
}

/**
 * Ändert das Repository-Passwort.
 *
 * @param id - Repository-ID
 * @param oldPass - Aktuelles Passwort
 * @param newPass - Neues Passwort
 * @returns Promise (void)
 * @throws Error wenn Passwortänderung fehlschlägt
 */
export async function changePassword(id: string, oldPass: string, newPass: string): Promise<void> {
  await invoke('change_password', { id, oldPass, newPass });
}

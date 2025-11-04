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
/**
 * Initialisiert ein neues rustic Repository.
 *
 * Erstellt ein Repository mit den angegebenen Parametern,
 * speichert das Passwort im System-Keychain und fügt es
 * zur App-Konfiguration hinzu.
 *
 * @param path - Repository-Pfad (lokal oder remote URL)
 * @param password - Verschlüsselungspasswort (min. 8 Zeichen empfohlen)
 * @param backend_type - Backend-Typ ('local', 's3', 'sftp', 'rest', 'rclone')
 * @param backend_options - Optional: Backend-spezifische Optionen (JSON)
 * @returns Promise mit RepositoryDto (ID, Name, Path, etc.)
 * @throws Error wenn Repository-Initialisierung fehlschlägt
 *
 * @example
 * ```typescript
 * // Lokales Repository
 * const repo = await initRepository(
 *   '/backup/my-repo',
 *   'my-secure-password',
 *   'local'
 * );
 *
 * // S3 Repository
 * const s3Repo = await initRepository(
 *   's3:my-bucket/backups',
 *   'password',
 *   's3',
 *   {
 *     region: 'eu-central-1',
 *     access_key_id: 'AKIAIOSFODNN7EXAMPLE',
 *     secret_access_key: 'wJalrX...'
 *   }
 * );
 * ```
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
 * Wechselt das aktive Repository im Backend-State.
 *
 * @param repositoryId - Repository-ID aus der Konfiguration
 * @param password - Passwort zum Entsperren des Repositories
 * @returns Promise mit aktualisiertem Repository-DTO
 * @throws Error wenn der Wechsel scheitert
 */
export async function switchRepository(
  repositoryId: string,
  password: string
): Promise<RepositoryDto> {
  return await invoke<RepositoryDto>('switch_repository', { repositoryId, password });
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
 * Prüft Repository-Integrität (v2 - über Repository-ID im State).
 *
 * Nutzt das bereits geöffnete Repository aus dem Backend-State.
 * Führt Konsistenzprüfung aller Snapshots und Trees durch.
 *
 * @param repositoryId - Repository-ID aus Config
 * @param trustCache - Cache vertrauen (schneller, weniger sicher)
 * @param readData - Pack-Dateien lesen und verifizieren (gründlich, langsamer)
 * @returns Promise mit Check-Ergebnis (errors/warnings/is_ok)
 * @throws Error wenn Check fehlschlägt
 *
 * @example
 * ```typescript
 * // Schneller Check (mit Cache)
 * const result = await checkRepository('repo-123', true, false);
 * if (!result.is_ok) {
 *   console.error('Errors:', result.errors);
 * }
 *
 * // Gründlicher Check (ohne Cache, mit Daten-Verifikation)
 * const fullCheck = await checkRepository('repo-123', false, true);
 * ```
 */
export interface CheckResultDto {
  errors: string[];
  warnings: string[];
  is_ok: boolean;
}

export async function checkRepository(
  repositoryId: string,
  trustCache: boolean = true,
  readData: boolean = false
): Promise<CheckResultDto> {
  return await invoke<CheckResultDto>('check_repository', {
    repositoryId,
    trustCache,
    readData,
  });
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
 * Entfernt ein Repository aus der Konfiguration (ohne Daten zu löschen).
 *
 * Diese Funktion entfernt nur den Eintrag aus der Config-Datei.
 * Das Repository selbst (Ordner/Daten) bleibt unverändert.
 *
 * @param id - Repository-ID
 * @returns Promise (void)
 * @throws Error wenn Entfernen fehlschlägt
 *
 * @example
 * ```typescript
 * // Repository aus der Liste entfernen (Daten bleiben erhalten)
 * await removeRepositoryFromConfig('repo-123');
 * ```
 */
export async function removeRepositoryFromConfig(id: string): Promise<void> {
  await invoke('remove_repository_from_config', { id });
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
 * Nutzt das geöffnete Repository aus dem Backend-State.
 * Erstellt Prune-Plan und führt ihn aus (2-stufig).
 *
 * @param repositoryId - Repository-ID aus Config
 * @param dryRun - Simulation ohne tatsächliches Löschen
 * @returns Promise mit Prune-Statistiken
 * @throws Error wenn Prune fehlschlägt
 *
 * @example
 * ```typescript
 * // Dry-Run (Simulation)
 * const stats = await pruneRepository('repo-123', true);
 * console.log(`Würde ${stats.packs_removed} Packs entfernen`);
 *
 * // Echtes Prune
 * const result = await pruneRepository('repo-123', false);
 * console.log(`${result.packs_removed} Packs entfernt, ${result.size_removed} Bytes freigegeben`);
 * ```
 */
export interface PruneResultDto {
  packs_removed: number;
  packs_kept: number;
  packs_recovered: number;
  size_removed: number;
  size_kept: number;
  size_recovered: number;
  dry_run: boolean;
}

export async function pruneRepository(
  repositoryId: string,
  dryRun: boolean = false
): Promise<PruneResultDto> {
  return await invoke<PruneResultDto>('prune_repository', {
    repositoryId,
    dryRun,
  });
}

/**
 * Ändert das Repository-Passwort.
 *
 * Erstellt einen neuen Key und aktualisiert den Keychain.
 * Der alte Key bleibt erhalten (rustic erlaubt mehrere Keys).
 *
 * @param repositoryId - Repository-ID aus Config
 * @param oldPassword - Aktuelles Passwort
 * @param newPassword - Neues Passwort
 * @returns Promise (void)
 * @throws Error wenn Passwortänderung fehlschlägt
 *
 * @example
 * ```typescript
 * try {
 *   await changePassword('repo-123', 'old-secret', 'new-secret');
 *   console.log('Passwort erfolgreich geändert');
 * } catch (err) {
 *   console.error('Passwortänderung fehlgeschlagen:', err);
 * }
 * ```
 */
export async function changePassword(
  repositoryId: string,
  oldPassword: string,
  newPassword: string
): Promise<void> {
  await invoke('change_password', {
    repositoryId,
    oldPassword,
    newPassword,
  });
}

/**
 * Holt detaillierte Statistiken für ein Repository.
 * M4.3: Repository-Statistiken
 *
 * @param id - Repository-ID
 * @returns Promise mit Repository-Statistiken
 * @throws Error wenn Abruf fehlschlägt
 */
export async function getRepositoryStats(id: string): Promise<RepositoryStats> {
  return await invoke<RepositoryStats>('get_repository_stats', { id });
}

/**
 * Repository-Statistiken Interface
 */
export interface RepositoryStats {
  snapshot_count: number;
  index_count: number;
  pack_count: number;
  total_size: number;
  data_size: number;
  compression_ratio: number;
  deduplication_ratio: number;
  unique_blobs: number;
}

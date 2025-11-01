import * as api from '$lib/api/repositories';
import type { RepositoryDto } from '$lib/types';
import { derived, writable } from 'svelte/store';

/**
 * Store für Repository-Verwaltung.
 * TODO.md: Phase 2 - Stores mit Daten-Loading ✅ KOMPLETT
 * Referenz: TODO.md Zeile 215-221, 320-325
 *
 * Backend-Commands:
 * - list_repositories: src-tauri/src/commands/repository.rs:7
 * - init_repository: src-tauri/src/lib.rs:180 (simuliert)
 * - open_repository: src-tauri/src/lib.rs:191 (simuliert)
 * - delete_repository: src-tauri/src/commands/repository.rs:41
 *
 * API-Wrapper: src/lib/api/repositories.ts
 *
 * Verwendung:
 * - src/lib/components/pages/Repositories.svelte (loadRepositories in onMount)
 * - src/lib/components/pages/DashboardPage.svelte (refreshRepos in onMount)
 *
 * Features:
 * - repositories: Liste aller Repositories
 * - activeRepositoryId: ID des aktuell ausgewählten Repos
 * - loading: Ladezustand
 * - error: Fehlertext
 */

const _repositories = writable<RepositoryDto[]>([]);
const _activeRepositoryId = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

export const repositories = {
  subscribe: _repositories.subscribe,
  set: _repositories.set,
  update: _repositories.update,
};
export const activeRepositoryId = { subscribe: _activeRepositoryId.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

// Derived Store: Aktives Repository-Objekt
export const activeRepository = derived(
  [_repositories, _activeRepositoryId],
  ([$repositories, $activeRepositoryId]) =>
    $repositories.find((r) => r.id === $activeRepositoryId) ?? null
);

/**
 * Setzt die Repository-Liste im Store.
 *
 * @param list - Array von Repository-DTOs
 */
export function setRepositories(list: RepositoryDto[]): void {
  _repositories.set(list);
}

/**
 * Setzt das aktive Repository.
 *
 * @param id - Repository-ID (oder null zum Deselektieren)
 */
export function setActiveRepository(id: string | null): void {
  _activeRepositoryId.set(id);
}

/**
 * Setzt den Ladezustand.
 *
 * @param val - true = Laden aktiv, false = Laden abgeschlossen
 */
export function setLoading(val: boolean): void {
  _loading.set(val);
}

/**
 * Setzt eine Fehlermeldung.
 *
 * @param msg - Fehlertext (oder null zum Löschen)
 */
export function setError(msg: string | null): void {
  _error.set(msg);
}

/**
 * Wechselt das aktive Repository über das Backend und aktualisiert den Store.
 *
 * @param repositoryId - Repository-ID die aktiviert werden soll
 * @param password - Passwort zum Entsperren bzw. Cachen
 * @returns Repository-DTO bei Erfolg oder null bei Fehler
 */
export async function switchActiveRepository(
  repositoryId: string,
  password: string
): Promise<RepositoryDto | null> {
  setLoading(true);
  setError(null);

  try {
    const repository = await api.switchRepository(repositoryId, password);

    _repositories.update((repos) =>
      repos.map((entry) => (entry.id === repository.id ? repository : entry))
    );

    setActiveRepository(repositoryId);
    return repository;
  } catch (err) {
    const message =
      err instanceof Error ? err.message : 'Repository konnte nicht gewechselt werden';
    setError(message);
    console.error('switchActiveRepository error:', err);
    return null;
  } finally {
    setLoading(false);
  }
}

/**
 * Lädt alle Repositories vom Backend.
 *
 * Setzt loading=true, ruft API ab, aktualisiert Store.
 * Bei Fehler wird error-Store gesetzt.
 *
 * @returns Promise (void)
 * @throws Error wenn Backend-Abruf fehlschlägt (wird in error-Store gespeichert)
 */
export async function loadRepositories(): Promise<void> {
  setLoading(true);
  setError(null);

  try {
    const repos = await api.listRepositories();
    _repositories.set(repos);
  } catch (err) {
    const errorMsg = err instanceof Error ? err.message : 'Fehler beim Laden der Repositories';
    setError(errorMsg);
    console.error('loadRepositories error:', err);
  } finally {
    setLoading(false);
  }
}

/**
 * Setzt den Repository-Store auf Initialzustand zurück.
 *
 * Löscht alle Repositories, aktive Selection, Loading-State und Fehler.
 */
export function resetRepositories(): void {
  _repositories.set([]);
  _activeRepositoryId.set(null);
  _loading.set(false);
  _error.set(null);
}

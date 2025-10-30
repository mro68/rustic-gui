import type { RepositoryDto } from '$lib/types';
import { derived, writable } from 'svelte/store';
import * as api from '$lib/api/repositories';

/**
 * Store für Repository-Verwaltung.
 * - repositories: Liste aller Repositories
 * - activeRepositoryId: ID des aktuell ausgewählten Repos
 * - loading: Ladezustand
 * - error: Fehlertext
 */

const _repositories = writable<RepositoryDto[]>([]);
const _activeRepositoryId = writable<string | null>(null);
const _loading = writable(false);
const _error = writable<string | null>(null);

export const repositories = { subscribe: _repositories.subscribe };
export const activeRepositoryId = { subscribe: _activeRepositoryId.subscribe };
export const loading = { subscribe: _loading.subscribe };
export const error = { subscribe: _error.subscribe };

// Derived Store: Aktives Repository-Objekt
export const activeRepository = derived(
  [_repositories, _activeRepositoryId],
  ([$repositories, $activeRepositoryId]) =>
    $repositories.find((r) => r.id === $activeRepositoryId) ?? null
);

// Actions
export function setRepositories(list: RepositoryDto[]): void {
  _repositories.set(list);
}

export function setActiveRepository(id: string | null): void {
  _activeRepositoryId.set(id);
}

export function setLoading(val: boolean): void {
  _loading.set(val);
}

export function setError(msg: string | null): void {
  _error.set(msg);
}

/**
 * Lädt alle Repositories vom Backend
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

export function resetRepositories(): void {
  _repositories.set([]);
  _activeRepositoryId.set(null);
  _loading.set(false);
  _error.set(null);
}

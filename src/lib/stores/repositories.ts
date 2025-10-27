import type { Repository } from '$lib/types/repository.types';
import { derived, writable } from 'svelte/store';

/**
 * Store für Repository-Verwaltung.
 * - repositories: Liste aller Repositories
 * - activeRepositoryId: ID des aktuell ausgewählten Repos
 * - loading: Ladezustand
 * - error: Fehlertext
 */

const _repositories = writable<Repository[]>([]);
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
    $repositories.find(r => r.id === $activeRepositoryId) ?? null
);

// Actions
export function setRepositories(list: Repository[]) {
  _repositories.set(list);
}

export function setActiveRepository(id: string | null) {
  _activeRepositoryId.set(id);
}

export function setLoading(val: boolean) {
  _loading.set(val);
}

export function setError(msg: string | null) {
  _error.set(msg);
}

export function resetRepositories() {
  _repositories.set([]);
  _activeRepositoryId.set(null);
  _loading.set(false);
  _error.set(null);
}

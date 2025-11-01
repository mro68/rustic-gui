import { beforeEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import * as api from '$lib/api/repositories';
import {
  activeRepository,
  activeRepositoryId,
  error,
  loadRepositories,
  loading,
  repositories,
  resetRepositories,
  setActiveRepository,
  setError,
  setLoading,
  setRepositories,
} from './repositories';
import type { RepositoryDto, RepositoryStatus, RepositoryType } from '$lib/types';

// Mock API
vi.mock('$lib/api/repositories');

describe('repositories store', () => {
  beforeEach(() => {
    // Reset store state
    resetRepositories();
    vi.clearAllMocks();
  });

  describe('Basic Store Operations', () => {
    it('startet mit leerem Array', () => {
      const repos = get(repositories);
      expect(repos).toEqual([]);
    });

    it('startet ohne aktives Repository', () => {
      const activeId = get(activeRepositoryId);
      expect(activeId).toBeNull();
    });

    it('startet nicht im Ladezustand', () => {
      const isLoading = get(loading);
      expect(isLoading).toBe(false);
    });

    it('startet ohne Fehler', () => {
      const err = get(error);
      expect(err).toBeNull();
    });
  });

  describe('setRepositories', () => {
    it('setzt Repository-Liste', () => {
      const mockRepos: RepositoryDto[] = [
        createMockRepo('1', 'Repo 1'),
        createMockRepo('2', 'Repo 2'),
      ];

      setRepositories(mockRepos);

      const repos = get(repositories);
      expect(repos).toEqual(mockRepos);
      expect(repos).toHaveLength(2);
    });

    it('überschreibt vorherige Repository-Liste', () => {
      const first = [createMockRepo('1', 'First')];
      const second = [createMockRepo('2', 'Second')];

      setRepositories(first);
      setRepositories(second);

      const repos = get(repositories);
      expect(repos).toEqual(second);
      expect(repos).toHaveLength(1);
    });
  });

  describe('setActiveRepository', () => {
    it('setzt aktives Repository', () => {
      setActiveRepository('repo-123');

      const activeId = get(activeRepositoryId);
      expect(activeId).toBe('repo-123');
    });

    it('kann aktives Repository deselektieren', () => {
      setActiveRepository('repo-123');
      setActiveRepository(null);

      const activeId = get(activeRepositoryId);
      expect(activeId).toBeNull();
    });
  });

  describe('activeRepository derived store', () => {
    it('gibt null zurück wenn kein Repository aktiv', () => {
      const mockRepos = [createMockRepo('1', 'Repo 1')];
      setRepositories(mockRepos);

      const active = get(activeRepository);
      expect(active).toBeNull();
    });

    it('gibt aktives Repository zurück', () => {
      const mockRepos = [createMockRepo('1', 'Repo 1'), createMockRepo('2', 'Repo 2')];
      setRepositories(mockRepos);
      setActiveRepository('2');

      const active = get(activeRepository);
      expect(active).not.toBeNull();
      expect(active?.id).toBe('2');
      expect(active?.name).toBe('Repo 2');
    });

    it('gibt null zurück wenn ID nicht existiert', () => {
      const mockRepos = [createMockRepo('1', 'Repo 1')];
      setRepositories(mockRepos);
      setActiveRepository('nonexistent');

      const active = get(activeRepository);
      expect(active).toBeNull();
    });

    it('aktualisiert sich wenn Repositories geändert werden', () => {
      const mockRepos = [createMockRepo('1', 'Repo 1')];
      setRepositories(mockRepos);
      setActiveRepository('1');

      let active = get(activeRepository);
      expect(active?.name).toBe('Repo 1');

      // Repository-Liste ändern
      const updatedRepos = [createMockRepo('1', 'Updated Repo 1')];
      setRepositories(updatedRepos);

      active = get(activeRepository);
      expect(active?.name).toBe('Updated Repo 1');
    });
  });

  describe('setLoading', () => {
    it('setzt Ladezustand auf true', () => {
      setLoading(true);

      const isLoading = get(loading);
      expect(isLoading).toBe(true);
    });

    it('setzt Ladezustand auf false', () => {
      setLoading(true);
      setLoading(false);

      const isLoading = get(loading);
      expect(isLoading).toBe(false);
    });
  });

  describe('setError', () => {
    it('setzt Fehler-Message', () => {
      setError('Test error message');

      const err = get(error);
      expect(err).toBe('Test error message');
    });

    it('kann Fehler löschen', () => {
      setError('Error');
      setError(null);

      const err = get(error);
      expect(err).toBeNull();
    });
  });

  describe('loadRepositories', () => {
    it('lädt Repositories vom Backend', async () => {
      const mockRepos = [createMockRepo('1', 'Repo 1'), createMockRepo('2', 'Repo 2')];

      vi.mocked(api.listRepositories).mockResolvedValue(mockRepos);

      await loadRepositories();

      const repos = get(repositories);
      expect(repos).toEqual(mockRepos);
      expect(vi.mocked(api.listRepositories)).toHaveBeenCalledOnce();
    });

    it('setzt Ladezustand während des Ladens', async () => {
      let loadingDuringCall = false;

      vi.mocked(api.listRepositories).mockImplementation(async () => {
        loadingDuringCall = get(loading);
        return [];
      });

      await loadRepositories();

      expect(loadingDuringCall).toBe(true);
      expect(get(loading)).toBe(false); // Nach Abschluss wieder false
    });

    it('löscht Fehler bei erfolgreichem Laden', async () => {
      setError('Previous error');
      vi.mocked(api.listRepositories).mockResolvedValue([]);

      await loadRepositories();

      expect(get(error)).toBeNull();
    });

    it('setzt Fehler bei Backend-Fehler', async () => {
      vi.mocked(api.listRepositories).mockRejectedValue(new Error('Backend error'));

      await loadRepositories();

      expect(get(error)).toBe('Backend error');
    });

    it('setzt Loading auf false auch bei Fehler', async () => {
      vi.mocked(api.listRepositories).mockRejectedValue(new Error('Error'));

      await loadRepositories();

      expect(get(loading)).toBe(false);
    });
  });

  describe('reset', () => {
    it('setzt alle Stores zurück', () => {
      // Setup: Stores mit Daten füllen
      setRepositories([createMockRepo('1', 'Test')]);
      setActiveRepository('1');
      setLoading(true);
      setError('Error');

      // Reset
      resetRepositories();

      // Verify: Alles zurückgesetzt
      expect(get(repositories)).toEqual([]);
      expect(get(activeRepositoryId)).toBeNull();
      expect(get(loading)).toBe(false);
      expect(get(error)).toBeNull();
    });
  });
});

// Helper Functions

function createMockRepo(id: string, name: string): RepositoryDto {
  return {
    id,
    name,
    path: `/path/to/${id}`,
    repository_type: 'Local' as RepositoryType,
    status: 'Healthy' as RepositoryStatus,
    snapshot_count: 0,
    total_size: 0,
    last_accessed: null,
    created_at: new Date().toISOString(),
  };
}

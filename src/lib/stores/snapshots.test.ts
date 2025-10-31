import { beforeEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import * as api from '$lib/api/snapshots';
import {
  addSnapshots,
  error,
  filter,
  loadSnapshots,
  loading,
  removeSnapshot,
  reset,
  setError,
  setFilter,
  setLoading,
  setSnapshots,
  setSort,
  snapshots,
  sort,
} from './snapshots';
import type { SnapshotDto } from '$lib/types';

// Mock API
vi.mock('$lib/api/snapshots');

describe('snapshots store', () => {
  beforeEach(() => {
    reset();
    vi.clearAllMocks();
  });

  describe('Basic Store Operations', () => {
    it('startet mit leerem Array', () => {
      const snaps = get(snapshots);
      expect(snaps).toEqual([]);
    });

    it('startet ohne Filter', () => {
      const f = get(filter);
      expect(f).toBeNull();
    });

    it('startet ohne Sort', () => {
      const s = get(sort);
      expect(s).toBeNull();
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

  describe('setSnapshots', () => {
    it('setzt Snapshot-Liste', () => {
      const mockSnaps = [createMockSnapshot('1'), createMockSnapshot('2')];

      setSnapshots(mockSnaps);

      const snaps = get(snapshots);
      expect(snaps).toEqual(mockSnaps);
      expect(snaps).toHaveLength(2);
    });

    it('überschreibt vorherige Liste', () => {
      const first = [createMockSnapshot('1')];
      const second = [createMockSnapshot('2')];

      setSnapshots(first);
      setSnapshots(second);

      const snaps = get(snapshots);
      expect(snaps).toEqual(second);
      expect(snaps).toHaveLength(1);
    });
  });

  describe('addSnapshots', () => {
    it('fügt neue Snapshots hinzu', () => {
      const initial = [createMockSnapshot('1')];
      const toAdd = [createMockSnapshot('2'), createMockSnapshot('3')];

      setSnapshots(initial);
      addSnapshots(toAdd);

      const snaps = get(snapshots);
      expect(snaps).toHaveLength(3);
      expect(snaps.map((s) => s.id)).toEqual(['1', '2', '3']);
    });

    it('vermeidet Duplikate basierend auf ID', () => {
      const initial = [createMockSnapshot('1'), createMockSnapshot('2')];
      const toAdd = [createMockSnapshot('2'), createMockSnapshot('3')]; // '2' ist Duplikat

      setSnapshots(initial);
      addSnapshots(toAdd);

      const snaps = get(snapshots);
      expect(snaps).toHaveLength(3); // Nicht 4!
      expect(snaps.map((s) => s.id)).toEqual(['1', '2', '3']);
    });

    it('funktioniert mit leerem Array', () => {
      addSnapshots([createMockSnapshot('1')]);

      const snaps = get(snapshots);
      expect(snaps).toHaveLength(1);
    });
  });

  describe('removeSnapshot', () => {
    it('entfernt Snapshot anhand ID', () => {
      const mockSnaps = [createMockSnapshot('1'), createMockSnapshot('2'), createMockSnapshot('3')];
      setSnapshots(mockSnaps);

      removeSnapshot('2');

      const snaps = get(snapshots);
      expect(snaps).toHaveLength(2);
      expect(snaps.map((s) => s.id)).toEqual(['1', '3']);
    });

    it('macht nichts wenn ID nicht existiert', () => {
      const mockSnaps = [createMockSnapshot('1')];
      setSnapshots(mockSnaps);

      removeSnapshot('nonexistent');

      const snaps = get(snapshots);
      expect(snaps).toHaveLength(1);
      expect(snaps[0].id).toBe('1');
    });
  });

  describe('setFilter', () => {
    it('setzt Filter', () => {
      const mockFilter = { tag: 'daily', hostname: 'server1' };

      setFilter(mockFilter);

      const f = get(filter);
      expect(f).toEqual(mockFilter);
    });

    it('kann Filter löschen', () => {
      setFilter({ tag: 'daily' });
      setFilter(null);

      const f = get(filter);
      expect(f).toBeNull();
    });
  });

  describe('setSort', () => {
    it('setzt Sort-Kriterium', () => {
      setSort('time');

      const s = get(sort);
      expect(s).toBe('time');
    });

    it('kann Sort löschen', () => {
      setSort('time');
      setSort(null);

      const s = get(sort);
      expect(s).toBeNull();
    });
  });

  describe('setLoading', () => {
    it('setzt Ladezustand', () => {
      setLoading(true);

      const isLoading = get(loading);
      expect(isLoading).toBe(true);
    });
  });

  describe('setError', () => {
    it('setzt Fehler', () => {
      setError('Test error');

      const err = get(error);
      expect(err).toBe('Test error');
    });

    it('kann Fehler löschen', () => {
      setError('Error');
      setError(null);

      const err = get(error);
      expect(err).toBeNull();
    });
  });

  describe('loadSnapshots', () => {
    it('lädt Snapshots vom Backend', async () => {
      const mockSnaps = [createMockSnapshot('1'), createMockSnapshot('2')];

      vi.mocked(api.listSnapshots).mockResolvedValue(mockSnaps);

      await loadSnapshots();

      const snaps = get(snapshots);
      expect(snaps).toEqual(mockSnaps);
      expect(vi.mocked(api.listSnapshots)).toHaveBeenCalledOnce();
    });

    it('setzt Ladezustand während des Ladens', async () => {
      let loadingDuringCall = false;

      vi.mocked(api.listSnapshots).mockImplementation(async () => {
        loadingDuringCall = get(loading);
        return [];
      });

      await loadSnapshots();

      expect(loadingDuringCall).toBe(true);
      expect(get(loading)).toBe(false);
    });

    it('löscht Fehler bei erfolgreichem Laden', async () => {
      setError('Previous error');
      vi.mocked(api.listSnapshots).mockResolvedValue([]);

      await loadSnapshots();

      expect(get(error)).toBeNull();
    });

    it('setzt Fehler bei Backend-Fehler', async () => {
      vi.mocked(api.listSnapshots).mockRejectedValue(new Error('Backend error'));

      await loadSnapshots();

      expect(get(error)).toBe('Backend error');
    });

    it('übergibt Filter an API', async () => {
      const mockFilter = { tag: 'daily', hostname: 'server1' };
      setFilter(mockFilter);

      vi.mocked(api.listSnapshots).mockResolvedValue([]);

      await loadSnapshots();

      expect(vi.mocked(api.listSnapshots)).toHaveBeenCalledWith(mockFilter);
    });
  });

  describe('reset', () => {
    it('setzt alle Stores zurück', () => {
      // Setup
      setSnapshots([createMockSnapshot('1')]);
      setFilter({ tag: 'daily' });
      setSort('time');
      setLoading(true);
      setError('Error');

      // Reset
      reset();

      // Verify
      expect(get(snapshots)).toEqual([]);
      expect(get(filter)).toBeNull();
      expect(get(sort)).toBeNull();
      expect(get(loading)).toBe(false);
      expect(get(error)).toBeNull();
    });
  });
});

// Helper Functions

function createMockSnapshot(id: string): SnapshotDto {
  return {
    id,
    time: new Date().toISOString(),
    hostname: 'test-host',
    username: 'test-user',
    paths: ['/test/path'],
    tags: [],
    summary: {
      files_new: 0,
      files_changed: 0,
      files_unmodified: 0,
      data_added: 0,
      total_files_processed: 0,
      total_bytes_processed: 0,
    },
  };
}

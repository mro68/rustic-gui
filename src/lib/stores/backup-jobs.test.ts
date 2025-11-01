import { beforeEach, describe, expect, it, vi } from 'vitest';
import { get } from 'svelte/store';
import * as api from '$lib/api/backup-jobs';
import {
  addJob,
  error,
  jobs,
  loadJobs,
  loading,
  removeJob,
  resetJobs,
  setError,
  setJobs,
  setLoading,
  updateJob,
} from './backup-jobs';
import type { BackupJobDto } from '$lib/types';

// Mock API
vi.mock('$lib/api/backup-jobs');

describe('backup-jobs store', () => {
  beforeEach(() => {
    resetJobs();
    vi.clearAllMocks();
  });

  describe('Basic Store Operations', () => {
    it('startet mit leerem Array', () => {
      const jobList = get(jobs);
      expect(jobList).toEqual([]);
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

  describe('setJobs', () => {
    it('setzt Job-Liste', () => {
      const mockJobs = [createMockJob('1', 'Job 1'), createMockJob('2', 'Job 2')];

      setJobs(mockJobs);

      const jobList = get(jobs);
      expect(jobList).toEqual(mockJobs);
      expect(jobList).toHaveLength(2);
    });

    it('überschreibt vorherige Liste', () => {
      const first = [createMockJob('1', 'First')];
      const second = [createMockJob('2', 'Second')];

      setJobs(first);
      setJobs(second);

      const jobList = get(jobs);
      expect(jobList).toEqual(second);
      expect(jobList).toHaveLength(1);
    });
  });

  // TODO: Diese Funktionen (addJob, updateJob, removeJob) existieren nicht im Store
  // Tests übersprungen bis Implementierung erfolgt
  describe('addJob', () => {
    it('fügt neuen Job hinzu', () => {
      const existing = [createMockJob('1', 'Existing')];
      const newJob = createMockJob('2', 'New');

      setJobs(existing);
      addJob(newJob);

      const jobList = get(jobs);
      expect(jobList).toHaveLength(2);
      expect(jobList.map((j) => j.id)).toEqual(['1', '2']);
    });

    it('fügt Job zu leerer Liste hinzu', () => {
      const newJob = createMockJob('1', 'First');

      addJob(newJob);

      const jobList = get(jobs);
      expect(jobList).toHaveLength(1);
      expect(jobList[0]).toEqual(newJob);
    });
  });

  describe('updateJob', () => {
    it('aktualisiert existierenden Job', () => {
      const initialJobs = [createMockJob('1', 'Original'), createMockJob('2', 'Other')];
      setJobs(initialJobs);

      const updated = createMockJob('1', 'Updated');
      updateJob(updated);

      const jobList = get(jobs);
      expect(jobList).toHaveLength(2);
      expect(jobList.find((j) => j.id === '1')?.name).toBe('Updated');
      expect(jobList.find((j) => j.id === '2')?.name).toBe('Other');
    });

    it('macht nichts wenn Job nicht existiert', () => {
      const initialJobs = [createMockJob('1', 'Existing')];
      setJobs(initialJobs);

      const nonExistent = createMockJob('999', 'Nonexistent');
      updateJob(nonExistent);

      const jobList = get(jobs);
      expect(jobList).toHaveLength(1);
      expect(jobList[0].name).toBe('Existing');
    });
  });

  describe('removeJob', () => {
    it('entfernt Job anhand ID', () => {
      const initialJobs = [
        createMockJob('1', 'Job 1'),
        createMockJob('2', 'Job 2'),
        createMockJob('3', 'Job 3'),
      ];
      setJobs(initialJobs);

      removeJob('2');

      const jobList = get(jobs);
      expect(jobList).toHaveLength(2);
      expect(jobList.map((j) => j.id)).toEqual(['1', '3']);
    });

    it('macht nichts wenn ID nicht existiert', () => {
      const initialJobs = [createMockJob('1', 'Job 1')];
      setJobs(initialJobs);

      removeJob('nonexistent');

      const jobList = get(jobs);
      expect(jobList).toHaveLength(1);
    });
  });

  describe('setLoading', () => {
    it('setzt Ladezustand', () => {
      setLoading(true);

      const isLoading = get(loading);
      expect(isLoading).toBe(true);
    });

    it('kann Ladezustand zurücksetzen', () => {
      setLoading(true);
      setLoading(false);

      const isLoading = get(loading);
      expect(isLoading).toBe(false);
    });
  });

  describe('setError', () => {
    it('setzt Fehler-Message', () => {
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

  describe('loadJobs', () => {
    it('lädt Jobs vom Backend', async () => {
      const mockJobs = [createMockJob('1', 'Job 1'), createMockJob('2', 'Job 2')];

      vi.mocked(api.listBackupJobs).mockResolvedValue(mockJobs);

      await loadJobs();

      const jobList = get(jobs);
      expect(jobList).toEqual(mockJobs);
      expect(vi.mocked(api.listBackupJobs)).toHaveBeenCalledOnce();
    });

    it('setzt Ladezustand während des Ladens', async () => {
      let loadingDuringCall = false;

      vi.mocked(api.listBackupJobs).mockImplementation(async () => {
        loadingDuringCall = get(loading);
        return [];
      });

      await loadJobs();

      expect(loadingDuringCall).toBe(true);
      expect(get(loading)).toBe(false);
    });

    it('löscht Fehler bei erfolgreichem Laden', async () => {
      setError('Previous error');
      vi.mocked(api.listBackupJobs).mockResolvedValue([]);

      await loadJobs();

      expect(get(error)).toBeNull();
    });

    it('setzt Fehler bei Backend-Fehler', async () => {
      vi.mocked(api.listBackupJobs).mockRejectedValue(new Error('Backend error'));

      await loadJobs();

      expect(get(error)).toBe('Backend error');
    });

    it('setzt Loading auf false auch bei Fehler', async () => {
      vi.mocked(api.listBackupJobs).mockRejectedValue(new Error('Error'));

      await loadJobs();

      expect(get(loading)).toBe(false);
    });
  });

  describe('resetJobs', () => {
    it('setzt alle Stores zurück', () => {
      // Setup
      setJobs([createMockJob('1', 'Test')]);
      setLoading(true);
      setError('Error');

      // Reset
      resetJobs();

      // Verify
      expect(get(jobs)).toEqual([]);
      expect(get(loading)).toBe(false);
      expect(get(error)).toBeNull();
    });
  });
});

// Helper Functions

function createMockJob(id: string, name: string): BackupJobDto {
  return {
    id,
    name,
    repository_id: 'repo-1',
    source_paths: ['/test/path'],
    tags: [],
    schedule: null,
    enabled: true,
    last_run: null,
    next_run: null,
    created_at: new Date().toISOString(),
  };
}

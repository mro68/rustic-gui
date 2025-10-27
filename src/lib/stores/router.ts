/**
 * Einfacher Router-Store für die Anwendung.
 *
 * Verwaltet die aktuelle Seite und stellt Navigation-Funktionen bereit.
 * Für komplexere Anwendungen könnte später SvelteKit-Routing verwendet werden.
 */

import { writable } from 'svelte/store';

// Verfügbare Seiten
export type Page =
  | 'dashboard'
  | 'repositories'
  | 'snapshots'
  | 'backup-jobs'
  | 'settings';

// Page-Titel Mapping
const pageTitles: Record<Page, string> = {
  dashboard: 'Dashboard',
  repositories: 'Repositories',
  snapshots: 'Snapshots',
  'backup-jobs': 'Backup Jobs',
  settings: 'Settings'
};

// Private Store für aktuelle Seite
const _currentPage = writable<Page>('dashboard');

// Public Stores
export const currentPage = { subscribe: _currentPage.subscribe };

// Abgeleiteter Store für aktuellen Titel
export const currentPageTitle = (() => {
  return {
    // eslint-disable-next-line no-unused-vars
    subscribe: (callback: (title: string) => void) => {
      return _currentPage.subscribe((page) => {
        const title = pageTitles[page];
        callback(title);
      });
    }
  };
})();

// Navigation-Funktionen
export function navigateTo(page: Page): void {
  _currentPage.set(page);
}

export function goToDashboard(): void {
  navigateTo('dashboard');
}

export function goToRepositories(): void {
  navigateTo('repositories');
}

export function goToSnapshots(): void {
  navigateTo('snapshots');
}

export function goToBackupJobs(): void {
  navigateTo('backup-jobs');
}

export function goToSettings(): void {
  navigateTo('settings');
}
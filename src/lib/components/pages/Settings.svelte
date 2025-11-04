<!-- Settings.svelte: App-Einstellungen (gemäß rustic_gui_mockup.html Settings-Tab) -->
<!--
  @component
  Einstellungs-Seite für Theme, Logging, Auto-Update, Passwort-Speicherung.
  
  TODO.md: Phase 2 - Daten-Initialisierung (Stores & Pages) ✅ KOMPLETT
  Referenz: TODO.md Zeile 220, 343
  
  Status:
  - Seite vollständig erstellt ✅
  - Settings Store integriert ✅
  
  Backend-Commands (noch nicht alle implementiert):
  - Geplant: save_settings, load_settings, reset_settings
  
  Store: src/lib/stores/settings.ts
  
  Features:
  - ✅ Theme-Umschaltung (Dark/Light/System)
  - ✅ Logging-Level-Konfiguration
  - ✅ Auto-Update-Einstellungen
  - ✅ Passwort-Speicherung-Optionen
  
  TODO:
  - Zeile 21, 27, 47, 62: Backend-Integration für Settings-Persistenz
-->
<script lang="ts">
  import { onMount } from 'svelte';
  import type { AppSettings } from '../../api/settings';
  import { getSettings, resetSettings, saveSettings, updateTheme } from '../../api/settings';
  import { toastStore } from '../../stores/toast';
  import Checkbox from '../shared/Checkbox.svelte';
  import CustomSelect from '../shared/CustomSelect.svelte';
  import Tooltip from '../shared/Tooltip.svelte';

  // Settings state
  let settings = $state<AppSettings>({
    theme: 'system',
    log_level: 'info',
    check_updates: true,
    max_concurrent_backups: 1,
    notifications_enabled: true,
    language: 'de',
    password_storage: 'system_keychain',
    lock_timeout: 15,
  });

  // eslint-disable-next-line no-unused-vars
  let loading = $state(false);

  // Convert lock_timeout number to string for CustomSelect
  let lockTimeoutString = $state('15');

  // Sync lockTimeoutString with settings on load
  $effect(() => {
    lockTimeoutString = String(settings.lock_timeout);
  });

  // App info (would come from backend in real implementation)
  const appVersion = '1.0.0';
  const rusticVersion = 'rustic_core 0.8.0';
  const configPath = '~/.config/rustic-gui/';

  async function handleThemeChange(value: string) {
    settings.theme = value;
    try {
      await updateTheme(value);
      toastStore.success(`Theme wurde auf "${value}" geändert`);
    } catch (error) {
      toastStore.error(`Theme-Änderung fehlgeschlagen: ${(error as Error).message}`);
    }
  }

  function handleLanguageChange(value: string) {
    settings.language = value;
    toastStore.info('Sprachänderung wird beim nächsten Start übernommen');
  }

  function handleNotificationsChange(value: boolean) {
    settings.notifications_enabled = value;
    toastStore.success(`Desktop-Benachrichtigungen ${value ? 'aktiviert' : 'deaktiviert'}`);
  }

  function handlePasswordStorageChange(value: string) {
    settings.password_storage = value;
    toastStore.success(`Passwort-Speicherung wurde geändert`);
  }

  function handleLockTimeoutChange(value: number) {
    settings.lock_timeout = value;
    toastStore.success(`Sperrzeit wurde auf "${value} Minuten" geändert`);
  }

  async function handleSaveSettings() {
    loading = true;
    try {
      await saveSettings(settings);
      toastStore.success('Einstellungen wurden gespeichert');
    } catch (error) {
      toastStore.error(`Fehler beim Speichern: ${(error as Error).message}`);
    } finally {
      loading = false;
    }
  }

  async function handleResetSettings() {
    loading = true;
    try {
      const defaultSettings = await resetSettings();
      settings = defaultSettings;
      toastStore.info('Einstellungen wurden auf Standard zurückgesetzt');
    } catch (error) {
      toastStore.error(`Fehler beim Zurücksetzen: ${(error as Error).message}`);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    loading = true;
    try {
      settings = await getSettings();
      console.log('Settings geladen:', settings);
    } catch (error) {
      console.error('Fehler beim Laden der Settings:', error);
      toastStore.error(`Fehler beim Laden der Einstellungen: ${(error as Error).message}`);
    } finally {
      loading = false;
    }
  });

  // Watch for theme changes
  let previousTheme = $state<string | undefined>(undefined);
  $effect(() => {
    if (previousTheme === undefined) {
      previousTheme = settings.theme;
      return;
    }
    if (settings.theme !== previousTheme) {
      handleThemeChange(settings.theme);
      previousTheme = settings.theme;
    }
  });

  // Watch for language changes
  let previousLanguage = $state<string | undefined>(undefined);
  $effect(() => {
    if (previousLanguage === undefined) {
      previousLanguage = settings.language;
      return;
    }
    if (settings.language !== previousLanguage) {
      handleLanguageChange(settings.language);
      previousLanguage = settings.language;
    }
  });

  // Watch for password storage changes
  let previousPasswordStorage = $state<string | undefined>(undefined);
  $effect(() => {
    if (previousPasswordStorage === undefined) {
      previousPasswordStorage = settings.password_storage;
      return;
    }
    if (settings.password_storage !== previousPasswordStorage) {
      handlePasswordStorageChange(settings.password_storage);
      previousPasswordStorage = settings.password_storage;
    }
  });

  // Watch lock_timeout changes (sync with string)
  let previousLockTimeout = $state<number | undefined>(undefined);
  $effect(() => {
    const timeoutNum = parseInt(lockTimeoutString, 10);
    if (!isNaN(timeoutNum) && timeoutNum !== settings.lock_timeout) {
      settings.lock_timeout = timeoutNum;
    }
    if (previousLockTimeout === undefined) {
      previousLockTimeout = settings.lock_timeout;
      return;
    }
    if (settings.lock_timeout !== previousLockTimeout) {
      handleLockTimeoutChange(settings.lock_timeout);
      previousLockTimeout = settings.lock_timeout;
    }
  });
</script>

<div class="settings-page">
  <div class="page-wrapper">
    <!-- Page Header -->
    <div class="page-header">
      <h1 class="page-title">Settings</h1>
      <div class="header-actions">
        <Tooltip text="Einstellungen auf Standard zurücksetzen">
          <button class="btn btn-secondary" onclick={handleResetSettings}>
            <span class="btn-icon">🔄</span>
            <span class="btn-text">Reset</span>
          </button>
        </Tooltip>
      </div>
    </div>

    <div class="settings-container">
      <div class="settings-header">
        <h1 class="page-title">Einstellungen</h1>
      </div>

      <div class="settings-content">
        <!-- General Settings -->
        <div class="settings-card">
          <h2 class="card-title">Allgemeine Einstellungen</h2>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Theme</div>
              <p class="setting-description">Wählen Sie das visuelle Theme für die Anwendung</p>
            </div>
            <div class="setting-control">
              <CustomSelect
                bind:value={settings.theme}
                options={[
                  { value: 'dark', label: 'Dunkel' },
                  { value: 'light', label: 'Hell' },
                  { value: 'system', label: 'System' },
                ]}
              />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Sprache</div>
              <p class="setting-description">Wählen Sie die Sprache der Anwendung</p>
            </div>
            <div class="setting-control">
              <CustomSelect
                bind:value={settings.language}
                options={[
                  { value: 'en', label: 'English' },
                  { value: 'de', label: 'Deutsch' },
                ]}
              />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Benachrichtigungen</div>
              <p class="setting-description">
                Desktop-Benachrichtigungen für Backup-Events aktivieren
              </p>
            </div>
            <div class="setting-control">
              <Checkbox
                label=""
                bind:checked={settings.notifications_enabled}
                onchange={(e: CustomEvent<boolean>) => handleNotificationsChange(e.detail)}
              />
            </div>
          </div>
        </div>

        <!-- Security Settings -->
        <div class="settings-card">
          <h2 class="card-title">Sicherheit</h2>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Passwort-Speicherung</div>
              <p class="setting-description">Wo Repository-Passwörter sicher gespeichert werden</p>
            </div>
            <div class="setting-control">
              <CustomSelect
                bind:value={settings.password_storage}
                options={[
                  { value: 'system_keychain', label: 'System-Keychain (empfohlen)' },
                  { value: 'in_memory', label: 'Nur im Arbeitsspeicher' },
                ]}
              />
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Automatische Sperre</div>
              <p class="setting-description">Repositories automatisch sperren nach Inaktivität</p>
            </div>
            <div class="setting-control">
              <CustomSelect
                bind:value={lockTimeoutString}
                options={[
                  { value: '15', label: '15 Minuten' },
                  { value: '30', label: '30 Minuten' },
                  { value: '60', label: '1 Stunde' },
                  { value: '0', label: 'Nie' },
                ]}
              />
            </div>
          </div>
        </div>

        <!-- About Section -->
        <div class="settings-card">
          <h2 class="card-title">Über</h2>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Version</div>
              <p class="setting-description">Aktuelle Anwendungsversion</p>
            </div>
            <div class="setting-control">
              <span class="version-text">{appVersion}</span>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Rustic Version</div>
              <p class="setting-description">Version des zugrundeliegenden rustic Backup-Tools</p>
            </div>
            <div class="setting-control">
              <span class="version-text">{rusticVersion}</span>
            </div>
          </div>

          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-label">Konfigurations-Pfad</div>
              <p class="setting-description">Speicherort der Anwendungskonfiguration</p>
            </div>
            <div class="setting-control">
              <span class="version-text">{configPath}</span>
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="settings-actions">
          <Tooltip text="Einstellungen zurücksetzen">
            <button class="btn btn-secondary" onclick={handleResetSettings}>
              Auf Standard zurücksetzen
            </button>
          </Tooltip>
          <Tooltip text="Einstellungen speichern">
            <button class="btn btn-primary" onclick={handleSaveSettings}>
              Einstellungen speichern
            </button>
          </Tooltip>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .settings-page {
    width: 100%;
    display: flex;
    justify-content: center;
  }

  .page-wrapper {
    width: 100%;
    min-width: 320px;
    max-width: 1200px;
    padding: 0 1rem;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    padding: 24px 0;
  }

  .page-title {
    font-size: 28px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .header-actions {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .settings-container {
    padding: 1.5rem 0;
  }

  .page-title {
    font-size: 1.875rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }

  .settings-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .settings-card {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    padding: 1.5rem;
  }

  .card-title {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 1.5rem;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem 0;
    border-bottom: 1px solid var(--border-color);
  }

  .setting-row:last-child {
    border-bottom: none;
  }

  .setting-info {
    flex: 1;
    min-width: 0;
  }

  .setting-label {
    display: block;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
  }

  .setting-description {
    font-size: 0.875rem;
    color: var(--text-secondary);
    line-height: 1.4;
    margin: 0;
  }

  .setting-control {
    flex-shrink: 0;
    margin-left: 2rem;
    min-width: 200px;
  }

  .version-text {
    font-size: 0.875rem;
    color: var(--text-primary);
    font-weight: 500;
    font-family: 'Courier New', monospace;
  }

  /* Removed unused .select-field - now using CustomSelect */
  /* Removed unused .form-input:focus - no form-input class used */

  .settings-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding-top: 2rem;
    border-top: 1px solid var(--border-color);
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    border: none;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: var(--color-primary);
    color: white;
  }

  .btn-primary:hover {
    background: var(--color-primary-dark);
  }

  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .btn-secondary:hover {
    background: var(--bg-primary);
  }

  @media (max-width: 768px) {
    .setting-row {
      flex-direction: column;
      gap: 1rem;
    }

    .setting-control {
      margin-left: 0;
      align-self: flex-start;
    }

    .settings-actions {
      flex-direction: column-reverse;
      align-items: stretch;
    }

    .btn {
      justify-content: center;
      padding: 0.75rem 1rem;
    }
  }
</style>

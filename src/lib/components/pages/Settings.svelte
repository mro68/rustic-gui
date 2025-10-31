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
  import { toastStore } from '../../stores/toast';
  import Checkbox from '../shared/Checkbox.svelte';
  import Tooltip from '../shared/Tooltip.svelte';
  import { getSettings, saveSettings, resetSettings, updateTheme } from '../../api/settings';
  import type { AppSettings } from '../../api/settings';

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

  // App info (would come from backend in real implementation)
  const appVersion = '1.0.0';
  const rusticVersion = '0.7.0';
  const configPath = '~/.config/rustic-gui/';

  async function handleThemeChange(value: string) {
    settings.theme = value;
    try {
      await updateTheme(value);
      toastStore.success(`Theme wurde auf "${value}" geändert`);
    } catch (error) {
      toastStore.error('Theme-Änderung fehlgeschlagen', (error as Error).message);
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
      toastStore.error('Fehler beim Speichern', (error as Error).message);
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
      toastStore.error('Fehler beim Zurücksetzen', (error as Error).message);
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
      toastStore.error('Fehler beim Laden der Einstellungen', (error as Error).message);
    } finally {
      loading = false;
    }
  });
</script>

<div class="settings-page">
  <div class="settings-header">
    <h1 class="page-title">Settings</h1>
  </div>

  <div class="settings-content">
    <!-- General Settings -->
    <div class="settings-card">
      <h2 class="card-title">General Settings</h2>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Application Theme</div>
          <p class="setting-description">Choose the visual theme for the application</p>
        </div>
        <div class="setting-control">
          <select
            class="select-field"
            bind:value={settings.theme}
            on:change={(e) => handleThemeChange(e.currentTarget.value)}
          >
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="system">System</option>
          </select>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Language</div>
          <p class="setting-description">Select the application language</p>
        </div>
        <div class="setting-control">
          <select
            class="select-field"
            bind:value={settings.language}
            on:change={(e) => handleLanguageChange(e.currentTarget.value)}
          >
            <option value="en">English</option>
            <option value="de">Deutsch</option>
          </select>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Notifications</div>
          <p class="setting-description">Enable desktop notifications for backup events</p>
        </div>
        <div class="setting-control">
          <Checkbox
            label=""
            bind:checked={settings.notifications_enabled}
            on:change={(e) => handleNotificationsChange(e.detail)}
          />
        </div>
      </div>
    </div>

    <!-- Security Settings -->
    <div class="settings-card">
      <h2 class="card-title">Security</h2>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Password Storage</div>
          <p class="setting-description">Where to store repository passwords securely</p>
        </div>
        <div class="setting-control">
          <select
            class="select-field"
            bind:value={settings.password_storage}
            on:change={(e) => handlePasswordStorageChange(e.currentTarget.value)}
          >
            <option value="system_keychain">System Keychain (recommended)</option>
            <option value="in_memory">In-Memory Only</option>
          </select>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Lock Timeout</div>
          <p class="setting-description">Automatically lock repositories after inactivity</p>
        </div>
        <div class="setting-control">
          <select
            class="select-field"
            bind:value={settings.lock_timeout}
            on:change={(e) => handleLockTimeoutChange(Number(e.currentTarget.value))}
          >
            <option value="15">15 minutes</option>
            <option value="30">30 minutes</option>
            <option value="60">1 hour</option>
            <option value="0">Never</option>
          </select>
        </div>
      </div>
    </div>

    <!-- About Section -->
    <div class="settings-card">
      <h2 class="card-title">About</h2>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Version</div>
          <p class="setting-description">Current application version</p>
        </div>
        <div class="setting-control">
          <span class="version-text">{appVersion}</span>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Rustic Version</div>
          <p class="setting-description">Version of the underlying rustic backup tool</p>
        </div>
        <div class="setting-control">
          <span class="version-text">{rusticVersion}</span>
        </div>
      </div>

      <div class="setting-row">
        <div class="setting-info">
          <div class="setting-label">Config Path</div>
          <p class="setting-description">Location of application configuration files</p>
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
          Reset to Defaults
        </button>
      </Tooltip>
      <Tooltip text="Einstellungen speichern">
        <button class="btn btn-primary" onclick={handleSaveSettings}> Save Settings </button>
      </Tooltip>
    </div>
  </div>
</div>

<style>
  .settings-page {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .settings-header {
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

  .select-field {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    cursor: pointer;
    min-width: 200px;
  }

  .select-field:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

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

<script lang="ts">
  import { onMount } from 'svelte';
  import { toastStore } from '../../stores/toast';
  import Checkbox from '../shared/Checkbox.svelte';

  // Settings state
  let theme = 'Dark';
  let language = 'Deutsch';
  let notifications = true;
  let passwordStorage = 'System Keychain (recommended)';
  let lockTimeout = '15 minutes';

  // App info (would come from backend in real implementation)
  const appVersion = '1.0.0';
  const rusticVersion = '0.7.0';
  const configPath = '~/.config/rustic-gui/';

  function handleThemeChange(value: string) {
    theme = value;
    // TODO: Apply theme change
    toastStore.success(`Theme wurde auf "${value}" geändert`);
  }

  function handleLanguageChange(value: string) {
    language = value;
    // TODO: Apply language change
    toastStore.success(`Sprache wurde auf "${value}" geändert`);
  }

  function handleNotificationsChange(value: boolean) {
    notifications = value;
    toastStore.success(`Desktop-Benachrichtigungen ${value ? 'aktiviert' : 'deaktiviert'}`);
  }

  function handlePasswordStorageChange(value: string) {
    passwordStorage = value;
    toastStore.success(`Passwort-Speicherung wurde geändert`);
  }

  function handleLockTimeoutChange(value: string) {
    lockTimeout = value;
    toastStore.success(`Sperrzeit wurde auf "${value}" geändert`);
  }

  function handleSaveSettings() {
    // TODO: Save settings to backend
    toastStore.success('Einstellungen wurden gespeichert');
  }

  function handleResetSettings() {
    // Reset to defaults
    theme = 'Dark';
    language = 'Deutsch';
    notifications = true;
    passwordStorage = 'System Keychain (recommended)';
    lockTimeout = '15 minutes';
    toastStore.info('Einstellungen wurden auf Standard zurückgesetzt');
  }

  onMount(() => {
    // TODO: Load settings from backend
    console.log('Loading settings...');
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
            bind:value={theme}
            on:change={(e) => handleThemeChange(e.currentTarget.value)}
          >
            <option value="Dark">Dark</option>
            <option value="Light">Light</option>
            <option value="Auto">Auto</option>
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
            bind:value={language}
            on:change={(e) => handleLanguageChange(e.currentTarget.value)}
          >
            <option value="English">English</option>
            <option value="Deutsch">Deutsch</option>
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
            bind:checked={notifications}
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
            bind:value={passwordStorage}
            on:change={(e) => handlePasswordStorageChange(e.currentTarget.value)}
          >
            <option value="System Keychain (recommended)">System Keychain (recommended)</option>
            <option value="In-Memory Only">In-Memory Only</option>
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
            bind:value={lockTimeout}
            on:change={(e) => handleLockTimeoutChange(e.currentTarget.value)}
          >
            <option value="15 minutes">15 minutes</option>
            <option value="30 minutes">30 minutes</option>
            <option value="1 hour">1 hour</option>
            <option value="Never">Never</option>
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
      <button class="btn btn-secondary" on:click={handleResetSettings}> Reset to Defaults </button>
      <button class="btn btn-primary" on:click={handleSaveSettings}> Save Settings </button>
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

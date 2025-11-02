<!-- AddRepositoryDialog.svelte: Dialog zum Hinzufügen eines neuen Repositories -->
<!--
  TODO.md: Phase 2 - Dialog-Workflow: Repository ✅ KOMPLETT
  Referenz: TODO.md Zeile 245-246
  
  Status:
  - handleSubmit an api.initRepository angebunden ✅ (TODO.md Zeile 245)
  - "Durchsuchen"-Button implementiert ✅ (TODO.md Zeile 246)
  
  Backend-Commands:
  - init_repository: src-tauri/src/lib.rs:180 (simuliert in rustic/repository.rs:32)
  
  API-Wrapper: src/lib/api/repositories.ts:initRepository
  
  Verwendung:
  - src/lib/components/pages/Repositories.svelte
  - src/lib/components/pages/DashboardPage.svelte
-->
<script lang="ts">
  /**
   * Dialog zum Hinzufügen/Initialisieren eines neuen Repositories.
   *
   * Unterstützt alle Repository-Typen via LocationPickerDialog:
   * - Lokal (Verzeichnis)
   * - Netzwerk (SFTP, SMB, NFS, WebDAV)
   * - Cloud (S3, B2, Azure, GCS, Wasabi, MinIO, Rclone)
   *
   * Features:
   * - Repository-Initialisierung
   * - Passwort-Setup mit Keychain
   * - Location Picker Integration
   *
   * @component
   *
   * @example
   * ```svelte
   * <AddRepositoryDialog
   *   on:repo-added={handleRepoAdded}
   * />
   * ```
   */
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';
  import Toast from '../shared/Toast.svelte';
  import LocationPickerDialog from './LocationPickerDialog.svelte';

  const dispatch = createEventDispatcher();

  // Props
  let { open = $bindable(true) } = $props();

  // Form state
  let repositoryName = $state<string>('');
  let repositoryType = $state<string>('local');
  let repositoryPath = $state<string>('');
  let password = $state<string>('');
  let storePassword = $state(true);
  let backendOptions = $state<string>('');
  // let locationConfig: any = undefined; // Unused, commented out

  // UI state
  let isSubmitting = $state(false);
  let showToast = $state(false);
  let toastMessage = $state('');
  let toastType = $state<'success' | 'error' | 'info'>('info');
  let showLocationPicker = $state(false);

  // ✅ Enhanced location picker (TODO.md Zeile 246, uses LocationPickerDialog.svelte)
  function openLocationPicker() {
    showLocationPicker = true;
  }

  function handleLocationSelect(event: CustomEvent<{ path: string; type: string; config?: any }>) {
    const { path, type } = event.detail;
    repositoryPath = path;
    repositoryType = type;
    // const { config } = event.detail; // TODO: Use config if needed for backend options
    showLocationPicker = false;
  }

  function handleLocationCancel() {
    showLocationPicker = false;
  }

  // Repository types (now handled by LocationPicker)
  const repositoryTypes = [
    { value: 'local', label: 'Lokal', icon: '💻', description: 'Lokales Dateisystem' },
    { value: 'network', label: 'Netzwerk', icon: '🌐', description: 'SFTP, SMB, NFS, WebDAV' },
    { value: 'cloud', label: 'Cloud', icon: '☁️', description: 'S3, Azure, GCS, etc.' },
  ];

  let selectedType = repositoryTypes[0];

  function selectRepositoryType(typeValue: string) {
    selectedType = repositoryTypes.find((t) => t.value === typeValue) || repositoryTypes[0];
    repositoryType = typeValue;
  }

  async function handleSubmit() {
    if (!repositoryName.trim()) {
      showToastMessage('Repository-Name ist erforderlich', 'error');
      return;
    }

    if (!repositoryPath.trim()) {
      showToastMessage('Repository-Pfad ist erforderlich', 'error');
      return;
    }

    isSubmitting = true;

    try {
      const { initRepository } = await import('$lib/api/repositories');

      // Initialize repository
      const backendOpts = backendOptions.trim() ? JSON.parse(backendOptions.trim()) : undefined;
      const repo = await initRepository(
        repositoryPath.trim(),
        password,
        repositoryType,
        backendOpts
      );

      // Store password in keychain if requested
      if (storePassword && password) {
        try {
          const { invoke } = await import('@tauri-apps/api/core');
          await invoke('store_repository_password', {
            repoId: repo.id,
            password: password,
          });
        } catch (error) {
          console.warn('Failed to store password in keychain:', error);
        }
      }

      dispatch('create', {
        name: repositoryName.trim(),
        type: repositoryType,
        path: repositoryPath.trim(),
        password: storePassword ? password : '',
        storePassword,
        backendOptions: backendOptions.trim(),
      });

      showToastMessage('Repository erfolgreich hinzugefügt', 'success');

      // Reset form
      repositoryName = '';
      repositoryPath = '';
      password = '';
      backendOptions = '';
      selectedType = repositoryTypes[0];
      repositoryType = 'local';
    } catch (error) {
      showToastMessage(`Fehler beim Erstellen des Repositories: ${error}`, 'error');
    } finally {
      isSubmitting = false;
    }
  }

  function showToastMessage(message: string, type: 'success' | 'error' | 'info') {
    toastMessage = message;
    toastType = type;
    showToast = true;
    setTimeout(() => (showToast = false), 3000);
  }

  function handleClose() {
    open = false;
    dispatch('close');
  }
</script>

<Modal bind:open on:close={handleClose}>
  {#snippet header()}
    <h2>Repository hinzufügen</h2>
  {/snippet}
  <div class="add-repo-dialog">
    <!-- Repository Type Selector -->
    <div class="form-group">
      <div id="repo-type-label" class="form-label">Repository-Typ</div>
      <div class="repo-type-grid" role="group" aria-labelledby="repo-type-label">
        {#each repositoryTypes as type}
          <div
            class="repo-type-card {selectedType.value === type.value ? 'selected' : ''}"
            onclick={() => selectRepositoryType(type.value)}
            role="button"
            tabindex="0"
            onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && selectRepositoryType(type.value)}
          >
            <div class="repo-type-icon">{type.icon}</div>
            <div class="repo-type-name">{type.label}</div>
            <div class="repo-type-desc">{type.description}</div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Repository Details -->
    <div class="form-group">
      <label class="form-label" for="repo-name">Repository-Name</label>
      <Input
        id="repo-name"
        bind:value={repositoryName}
        placeholder="z.B. Mein Backup Repository"
        required
      />
    </div>

    <div class="form-group">
      <label class="form-label" for="repo-path"> Repository-Speicherort </label>
      <div class="input-with-button">
        <Input
          id="repo-path"
          bind:value={repositoryPath}
          placeholder="Pfad auswählen..."
          required
          readonly
        />
        <Button variant="secondary" size="sm" onclick={openLocationPicker}>
          📁 Speicherort wählen
        </Button>
      </div>
      {#if repositoryType && repositoryType !== 'local'}
        <div class="form-help">
          Typ: {repositoryType.toUpperCase()} • {repositoryPath}
        </div>
      {/if}
    </div>

    <!-- Password -->
    <div class="form-group">
      <label class="form-label" for="repo-password">Passwort</label>
      <Input
        id="repo-password"
        type="password"
        bind:value={password}
        placeholder="Repository-Passwort eingeben"
      />
      <div class="form-help">
        Das Passwort wird für die Verschlüsselung verwendet. Leer lassen für unverschlüsselte
        Repositories.
      </div>
    </div>

    <!-- Store Password -->
    <div class="form-group">
      <Checkbox
        id="store-password"
        bind:checked={storePassword}
        label="Passwort im Keychain speichern"
      />
      <div class="form-help">
        Speichert das Passwort sicher im System-Keychain für automatische Backups.
      </div>
    </div>

    <!-- Backend Options (for advanced users) -->
    {#if ['s3', 'rest', 'rclone'].includes(selectedType.value)}
      <div class="form-group">
        <label class="form-label" for="backend-options">Backend-Optionen (JSON)</label>
        <textarea
          id="backend-options"
          class="form-textarea"
          bind:value={backendOptions}
          placeholder={'{"key": "value"}'}
          rows="3"
        ></textarea>
        <div class="form-help">Zusätzliche Konfigurationsoptionen als JSON-Objekt.</div>
      </div>
    {/if}

    <!-- Info Box -->
    <div class="info-box">
      <strong>Hinweis:</strong> Nach dem Erstellen des Repositories können Sie es initialisieren und
      Ihren ersten Backup-Job anlegen.
    </div>
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={handleClose}>Abbrechen</Button>
    <Button variant="primary" onclick={handleSubmit} disabled={isSubmitting}>
      {#if isSubmitting}
        Erstelle Repository...
      {:else}
        Repository hinzufügen
      {/if}
    </Button>
  {/snippet}
</Modal>

{#if showToast}
  <Toast type={toastType} message={toastMessage} onClose={() => (showToast = false)} />
{/if}

<!-- Location Picker Dialog -->
<LocationPickerDialog
  bind:isOpen={showLocationPicker}
  mode="init"
  title="Repository-Speicherort auswählen"
  on:select={handleLocationSelect}
  on:cancel={handleLocationCancel}
/>

<style>
  .add-repo-dialog {
    max-width: 600px;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-label {
    display: block;
    font-size: 13px;
    font-weight: 500;
    color: #e4e4e7;
    margin-bottom: 8px;
  }

  .form-help {
    font-size: 12px;
    color: #71717a;
    margin-top: 4px;
  }

  .repo-type-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 12px;
    margin-top: 8px;
  }

  .repo-type-card {
    background: #22273a;
    border: 2px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: center;
  }

  .repo-type-card:hover {
    background: #2d3348;
    border-color: #3b82f6;
  }

  .repo-type-card.selected {
    background: #1e3a5f;
    border-color: #3b82f6;
  }

  .repo-type-icon {
    font-size: 24px;
    margin-bottom: 8px;
  }

  .repo-type-name {
    font-size: 14px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 4px;
  }

  .repo-type-desc {
    font-size: 12px;
    color: #71717a;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
  }

  .input-with-button :global(.input) {
    flex: 1;
  }

  .form-textarea {
    width: 100%;
    padding: 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    font-family: 'Courier New', monospace;
    resize: vertical;
  }

  .form-textarea:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .info-box {
    background: #1e3a5f;
    border: 1px solid #3b82f6;
    border-radius: 6px;
    padding: 12px;
    font-size: 13px;
    color: #e4e4e7;
  }

  .info-box strong {
    color: #3b82f6;
  }
</style>

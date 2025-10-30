<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';
  import Toast from '../shared/Toast.svelte';

  const dispatch = createEventDispatcher();

  // Form state
  let repositoryName = '';
  let repositoryType = 'local';
  let repositoryPath = '';
  let password = '';
  let storePassword = true;
  let backendOptions = '';

  // UI state
  let isSubmitting = false;
  let showToast = false;
  let toastMessage = '';
  let toastType: 'success' | 'error' | 'info' = 'info';

  // Repository types from mockup
  const repositoryTypes = [
    { value: 'local', label: 'Lokal', icon: '💻', description: 'Lokales Dateisystem' },
    { value: 'sftp', label: 'SFTP', icon: '🌐', description: 'Secure FTP Server' },
    { value: 's3', label: 'S3', icon: '☁️', description: 'Amazon S3 oder kompatibel' },
    { value: 'rest', label: 'REST', icon: '🔗', description: 'REST-API Backend' },
    { value: 'rclone', label: 'Rclone', icon: '🚀', description: 'Rclone Remote' },
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
      // TODO: Implement actual repository creation
      // For now, just dispatch the event
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
    dispatch('close');
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository hinzufügen</div>
  <div class="add-repo-dialog">
    <!-- Repository Type Selector -->
    <div class="form-group">
      <div id="repo-type-label" class="form-label">Repository-Typ</div>
      <div class="repo-type-grid" role="group" aria-labelledby="repo-type-label">
        {#each repositoryTypes as type}
          <div
            class="repo-type-card {selectedType.value === type.value ? 'selected' : ''}"
            on:click={() => selectRepositoryType(type.value)}
            role="button"
            tabindex="0"
            on:keydown={(e) => e.key === 'Enter' && selectRepositoryType(type.value)}
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
      <label class="form-label" for="repo-path">
        {#if selectedType.value === 'local'}
          Lokaler Pfad
        {:else if selectedType.value === 'sftp'}
          SFTP URL
        {:else if selectedType.value === 's3'}
          S3 Bucket URL
        {:else if selectedType.value === 'rest'}
          REST API URL
        {:else}
          Rclone Remote
        {/if}
      </label>
      <div class="input-with-button">
        <Input
          id="repo-path"
          bind:value={repositoryPath}
          placeholder={selectedType.value === 'local'
            ? '/pfad/zum/repository'
            : selectedType.value === 'sftp'
              ? 'sftp://user@host/path'
              : selectedType.value === 's3'
                ? 's3://bucket/path'
                : selectedType.value === 'rest'
                  ? 'https://api.example.com/repo'
                  : 'remote:path'}
          required
        />
        {#if selectedType.value === 'local'}
          <Button
            variant="secondary"
            size="sm"
            on:click={() => {
              /* TODO: File browser */
            }}
          >
            Durchsuchen
          </Button>
        {/if}
      </div>
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

  <div slot="footer">
    <Button variant="secondary" on:click={handleClose}>Abbrechen</Button>
    <Button variant="primary" on:click={handleSubmit} disabled={isSubmitting}>
      {#if isSubmitting}
        Erstelle Repository...
      {:else}
        Repository hinzufügen
      {/if}
    </Button>
  </div>
</Modal>

{#if showToast}
  <Toast type={toastType} message={toastMessage} onClose={() => (showToast = false)} />
{/if}

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

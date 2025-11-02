<!-- CredentialPromptModal.svelte: Credential Save Prompt (M2 Task 2.3.3) -->
<script lang="ts">
  import Button from '../../shared/Button.svelte';
  import Modal from '../../shared/Modal.svelte';

  interface CredentialPromptModalProps {
    /** Sichtbarkeit */
    open?: boolean;
    /** Callback bei Speichern */
    onSave: (saveCredentials: boolean, saveFavorite: boolean) => void;
    /** Callback bei Abbruch */
    onCancel: () => void;
  }

  let { open = $bindable(false), onSave, onCancel }: CredentialPromptModalProps = $props();

  let saveCredentialsChecked = $state(true);
  let saveFavoriteChecked = $state(true);

  function handleSave() {
    onSave(saveCredentialsChecked, saveFavoriteChecked);
    open = false;
  }

  function handleCancel() {
    onCancel();
    open = false;
  }
</script>

<Modal bind:open>
  {#snippet header()}
    <h2>Zugangsdaten speichern?</h2>
  {/snippet}

  <div class="credential-prompt">
    <p class="prompt-message">
      Die Verbindung war erfolgreich! Möchten Sie die Zugangsdaten sicher im System-Keychain
      speichern?
    </p>

    <div class="prompt-options">
      <label class="checkbox-label">
        <input type="checkbox" bind:checked={saveCredentialsChecked} />
        <span>Zugangsdaten im Keychain speichern</span>
      </label>

      <label class="checkbox-label">
        <input type="checkbox" bind:checked={saveFavoriteChecked} />
        <span>Als Favorit speichern</span>
      </label>
    </div>

    <div class="prompt-info">
      <p style="font-size: 12px; color: var(--text-secondary); margin-top: 16px;">
        🔒 Zugangsdaten werden verschlüsselt im System-Keychain gespeichert. Sie werden nicht in der
        Konfigurationsdatei abgelegt.
      </p>
    </div>
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={handleCancel}>Nicht speichern</Button>
    <Button variant="primary" onclick={handleSave}>Speichern</Button>
  {/snippet}
</Modal>

<style>
  .credential-prompt {
    padding: 16px 0;
  }

  .prompt-message {
    margin-bottom: 20px;
    line-height: 1.5;
  }

  .prompt-options {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    padding: 8px;
    border-radius: 4px;
    transition: background 0.2s;
  }

  .checkbox-label:hover {
    background: rgba(59, 130, 246, 0.05);
  }

  .checkbox-label input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .checkbox-label span {
    font-size: 14px;
  }

  .prompt-info {
    margin-top: 16px;
    padding: 12px;
    background: rgba(59, 130, 246, 0.05);
    border-radius: 6px;
  }
</style>

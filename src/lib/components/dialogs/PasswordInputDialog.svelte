<!--
  PasswordInputDialog.svelte
  -------------------------
  Dialog zur Passwort-Eingabe wenn Passwort nicht in Config gespeichert ist.
  
  Verwendung:
  - Manuelle Backups ohne gespeichertes Passwort
  - Import von Configs ohne Passwort-Feld
-->
<script lang="ts">
  /**
   * Einfacher Passwort-Input-Dialog.
   *
   * Wird angezeigt wenn:
   * - Job kein gespeichertes Passwort hat
   * - Repository-Unlock erforderlich
   *
   * @component
   *
   * @example
   * ```svelte
   * <PasswordInputDialog
   *   bind:open={showPasswordDialog}
   *   bind:password={enteredPassword}
   *   title="Passwort für Backup eingeben"
   *   onConfirm={handlePasswordConfirm}
   * />
   * ```
   */
  import Modal from '../shared/Modal.svelte';

  interface PasswordInputDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Dialog-Titel */
    title?: string;
    /** Beschreibung/Hinweis */
    description?: string;
    /** Eingegebenes Passwort (bindable) */
    password?: string;
    /** Callback bei Confirm */
    onConfirm?: ((password: string) => void) | undefined;
    /** Callback bei Cancel */
    onCancel?: (() => void) | undefined;
  }

  let {
    open = $bindable(false),
    title = 'Passwort eingeben',
    description = 'Bitte geben Sie das Repository-Passwort ein.',
    password = $bindable(''),
    onConfirm = undefined,
    onCancel = undefined,
  }: PasswordInputDialogProps = $props();

  let showPassword = $state(false);

  function handleConfirm() {
    if (!password.trim()) {
      return;
    }
    if (onConfirm) {
      onConfirm(password);
    }
    open = false;
  }

  function handleCancel() {
    password = '';
    if (onCancel) {
      onCancel();
    }
    open = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      handleConfirm();
    }
  }
</script>

<Modal bind:open>
  {#snippet header()}
    {title}
  {/snippet}

  <div class="password-dialog">
    <p class="description">{description}</p>

    <div class="input-group">
      <label for="password-input" class="label">Passwort</label>
      <div class="password-wrapper">
        <input
          id="password-input"
          type={showPassword ? 'text' : 'password'}
          bind:value={password}
          onkeydown={handleKeydown}
          placeholder="Repository-Passwort"
          class="input"
          autocomplete="off"
          autofocus
        />
        <button
          type="button"
          class="toggle-password"
          onclick={() => (showPassword = !showPassword)}
          aria-label={showPassword ? 'Passwort verbergen' : 'Passwort anzeigen'}
        >
          {showPassword ? '👁️' : '👁️‍🗨️'}
        </button>
      </div>
    </div>

    <div class="actions">
      <button type="button" class="btn btn-secondary" onclick={handleCancel}>Abbrechen</button>
      <button
        type="button"
        class="btn btn-primary"
        onclick={handleConfirm}
        disabled={!password.trim()}
      >
        Bestätigen
      </button>
    </div>
  </div>
</Modal>

<style>
  .password-dialog {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .description {
    color: #a1a1aa;
    font-size: 14px;
    margin: 0;
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .label {
    font-size: 13px;
    font-weight: 500;
    color: #e4e4e7;
  }

  .password-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .input {
    width: 100%;
    padding: 10px 40px 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    transition: border-color 0.2s;
  }

  .input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .input::placeholder {
    color: #71717a;
  }

  .toggle-password {
    position: absolute;
    right: 8px;
    background: transparent;
    border: none;
    cursor: pointer;
    font-size: 18px;
    padding: 4px 8px;
    color: #71717a;
    transition: color 0.2s;
  }

  .toggle-password:hover {
    color: #e4e4e7;
  }

  .actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .btn {
    padding: 10px 20px;
    border-radius: 6px;
    border: none;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-primary {
    background: #3b82f6;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background: #2d3348;
    color: #e4e4e7;
    border: 1px solid #3e4457;
  }

  .btn-secondary:hover {
    background: #3e4457;
  }
</style>

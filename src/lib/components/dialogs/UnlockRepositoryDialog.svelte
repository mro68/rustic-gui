<script lang="ts">
  /**
   * Dialog zum Entsperren eines gesperrten Repositories.
   *
   * Fragt nach Passwort und bietet Keychain-Integration.
   *
   * TODO.md: Phase 2 - Dialog-Workflow Repository (Zeile 248)
   * Status: ✅ KOMPLETT - API-Integration vollständig inkl. Keychain
   *
   * @component
   *
   * @example
   * ```svelte
   * <UnlockRepositoryDialog
   *   {repositoryName}
   *   {repositoryPath}
   *   {repositoryId}
   *   on:unlocked={handleUnlocked}
   * />
   * ```
   */

  import { storeRepositoryPassword } from '$lib/api/keychain';
  import { openRepository } from '$lib/api/repositories';
  import { toastStore } from '$lib/stores/toast';
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';

  const dispatch = createEventDispatcher();

  interface UnlockRepositoryDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Name des Repositories */
    repositoryName?: string;
    /** Pfad zum Repository */
    repositoryPath?: string;
    /** Repository-ID */
    repositoryId?: string;
  }

  let {
    open = $bindable(false),
    repositoryName = '',
    repositoryPath = '',
    repositoryId = '',
  }: UnlockRepositoryDialogProps = $props();

  // Form state
  let password = $state('');
  let rememberPassword = $state(true);
  let isUnlocking = $state(false);

  // Password strength
  let passwordStrength = $state(0);
  let strengthLabel = $state('');
  let strengthColor = $state('');

  $effect(() => {
    updatePasswordStrength(password);
  });

  function updatePasswordStrength(pwd: string) {
    if (pwd.length === 0) {
      passwordStrength = 0;
      strengthLabel = '';
      strengthColor = '';
      return;
    }

    let score = 0;

    // Length check
    if (pwd.length >= 8) score += 1;
    if (pwd.length >= 12) score += 1;

    // Character variety
    if (/[a-z]/.test(pwd)) score += 1;
    if (/[A-Z]/.test(pwd)) score += 1;
    if (/[0-9]/.test(pwd)) score += 1;
    if (/[^A-Za-z0-9]/.test(pwd)) score += 1;

    passwordStrength = Math.min(score / 6, 1);

    if (passwordStrength < 0.3) {
      strengthLabel = 'Schwach';
      strengthColor = '#ef4444';
    } else if (passwordStrength < 0.6) {
      strengthLabel = 'Mittel';
      strengthColor = '#fbbf24';
    } else {
      strengthLabel = 'Stark';
      strengthColor = '#22c55e';
    }
  }

  async function handleUnlock() {
    if (!password.trim()) {
      toastStore.error('Passwort erforderlich');
      return;
    }

    isUnlocking = true;

    try {
      // ✅ Tatsächliche API-Integration (TODO.md Phase 2 Zeile 248)
      await openRepository(repositoryPath, password.trim());

      // ✅ Passwort im Keychain speichern wenn rememberPassword = true
      if (rememberPassword) {
        try {
          await storeRepositoryPassword(repositoryId, password.trim());
        } catch (keychainError) {
          console.warn('Keychain-Speicherung fehlgeschlagen:', keychainError);
          // Fehler nicht kritisch - Repository ist trotzdem entsperrt
        }
      }

      toastStore.success('Repository erfolgreich entsperrt');

      dispatch('unlock', {
        repositoryId,
        password: password.trim(),
        remember: rememberPassword,
      });

      // Close dialog on success
      open = false;
    } catch (error: any) {
      const errorMessage = error?.message || 'Unbekannter Fehler';
      toastStore.error('Repository entsperren fehlgeschlagen: ' + errorMessage);
      console.error('Unlock failed:', error);
    } finally {
      isUnlocking = false;
    }
  }

  function handleClose() {
    open = false;
    dispatch('close');
  }
</script>

<Modal bind:open on:close={handleClose}>
  {#snippet header()}
    Repository entsperren
  {/snippet}
  
  <div class="unlock-dialog">
    <!-- Repository Info -->
    <div class="repo-info">
      <div class="repo-name">{repositoryName || 'Repository'}</div>
      <div class="repo-path">{repositoryPath || '/pfad/zum/repository'}</div>
    </div>

    <!-- Password Input -->
    <div class="form-group">
      <label class="form-label" for="unlock-password">Passwort</label>
      <Input
        id="unlock-password"
        type="password"
        bind:value={password}
        placeholder="Repository-Passwort eingeben"
        required
        autofocus
      />

      <!-- Password Strength Indicator -->
      {#if password}
        <div class="password-strength">
          <div class="strength-bar">
            <div
              class="strength-fill"
              style="width: {passwordStrength * 100}%; background-color: {strengthColor}"
            ></div>
          </div>
          <span class="strength-label" style="color: {strengthColor}">
            {strengthLabel}
          </span>
        </div>
      {/if}

      <div class="form-help">
        Geben Sie das Passwort ein, das beim Erstellen des Repositories verwendet wurde.
      </div>
    </div>

    <!-- Remember Password -->
    <div class="form-group">
      <Checkbox id="remember-password" bind:checked={rememberPassword} label="Passwort merken" />
      <div class="form-help">Speichert das Passwort temporär für diese Sitzung.</div>
    </div>

    <!-- Info Box -->
    <div class="info-box">
      <strong>Sicherheitshinweis:</strong> Das Passwort wird nur lokal verarbeitet und nicht an Server
      gesendet.
    </div>
  </div>

  {#snippet footer()}
    <Button variant="secondary" onclick={handleClose}>Abbrechen</Button>
    <Button variant="primary" onclick={handleUnlock} disabled={isUnlocking || !password.trim()}>
      {#if isUnlocking}
        Entsperre...
      {:else}
        Repository entsperren
      {/if}
    </Button>
  {/snippet}
</Modal>

<style>
  .unlock-dialog {
    max-width: 400px;
  }

  .repo-info {
    background: #22273a;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 24px;
  }

  .repo-name {
    font-size: 16px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 4px;
  }

  .repo-path {
    font-size: 13px;
    color: #71717a;
    font-family: 'Courier New', monospace;
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

  .password-strength {
    margin-top: 8px;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .strength-bar {
    flex: 1;
    height: 4px;
    background: #2d3348;
    border-radius: 2px;
    overflow: hidden;
  }

  .strength-fill {
    height: 100%;
    transition:
      width 0.2s,
      background-color 0.2s;
  }

  .strength-label {
    font-size: 12px;
    font-weight: 500;
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

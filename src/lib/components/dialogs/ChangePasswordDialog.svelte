<script lang="ts">
  /**
   * ChangePasswordDialog.svelte
   *
   * TODO.md: Phase 2 - Dialog-Workflow Repository (Zeile 251)
   * Status: ✅ KOMPLETT - API-Integration vollständig inkl. Keychain
   *
   * Backend-Command: src-tauri/src/commands/repository.rs:151 (change_password)
   * API-Wrapper: src/lib/api/repositories.ts:57 (changePassword)
   * Keychain: src/lib/api/keychain.ts (storeRepositoryPassword)
   *
   * Implementierung:
   * - ✅ API-Integration mit changePassword
   * - ✅ Error-Handling mit Toasts
   * - ✅ Success-Toast bei erfolgreichem Passwort-Wechsel
   * - ✅ Passwort-Stärke-Anzeige
   * - ✅ Keychain-Integration für savePassword (2025-10-31)
   */

  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';
  import { toastStore } from '$lib/stores/toast';
  import { changePassword } from '$lib/api/repositories';
  import { storeRepositoryPassword } from '$lib/api/keychain';

  const dispatch = createEventDispatcher();

  export let repositoryId: string = '';

  let currentPassword = '';
  let newPassword = '';
  let confirmPassword = '';
  let savePassword = true;
  let isLoading = false;

  let passwordStrength = {
    score: 0,
    label: 'Sehr schwach',
    color: '#ef4444',
    width: '20%',
  };

  $: updatePasswordStrength(newPassword);

  function updatePasswordStrength(password: string) {
    let score = 0;

    // Length check
    if (password.length >= 8) score += 1;
    if (password.length >= 12) score += 1;

    // Character variety
    if (/[a-z]/.test(password)) score += 1;
    if (/[A-Z]/.test(password)) score += 1;
    if (/[0-9]/.test(password)) score += 1;
    if (/[^A-Za-z0-9]/.test(password)) score += 1;

    // Update display
    switch (score) {
      case 0:
      case 1:
        passwordStrength = { score, label: 'Sehr schwach', color: '#ef4444', width: '20%' };
        break;
      case 2:
      case 3:
        passwordStrength = { score, label: 'Schwach', color: '#fbbf24', width: '40%' };
        break;
      case 4:
      case 5:
        passwordStrength = { score, label: 'Mittel', color: '#f59e0b', width: '60%' };
        break;
      case 6:
      case 7:
        passwordStrength = { score, label: 'Stark', color: '#22c55e', width: '80%' };
        break;
      default:
        passwordStrength = { score, label: 'Sehr stark', color: '#16a34a', width: '100%' };
    }
  }

  function validateForm(): boolean {
    if (!currentPassword.trim()) {
      return false;
    }
    if (!newPassword.trim()) {
      return false;
    }
    if (newPassword !== confirmPassword) {
      return false;
    }
    if (passwordStrength.score < 3) {
      return false;
    }
    return true;
  }

  async function handleSubmit() {
    if (!validateForm()) {
      toastStore.error('Bitte alle Felder korrekt ausfüllen');
      return;
    }

    isLoading = true;
    try {
      // ✅ Tatsächliche API-Integration (TODO.md Phase 2 Zeile 251)
      await changePassword(repositoryId, currentPassword, newPassword);

      // ✅ Passwort im Keychain aktualisieren wenn savePassword = true
      if (savePassword) {
        try {
          await storeRepositoryPassword(repositoryId, newPassword);
        } catch (keychainError) {
          console.warn('Keychain-Aktualisierung fehlgeschlagen:', keychainError);
          // Fehler nicht kritisch - Passwort wurde trotzdem geändert
        }
      }

      toastStore.success('Passwort erfolgreich geändert');

      dispatch('password-changed', {
        repositoryId,
      });

      dispatch('close');
    } catch (error: any) {
      const errorMessage = error?.message || 'Unbekannter Fehler';
      toastStore.error('Passwort-Änderung fehlgeschlagen: ' + errorMessage);
      console.error('Password change failed:', error);
    } finally {
      isLoading = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }

  function resetForm() {
    currentPassword = '';
    newPassword = '';
    confirmPassword = '';
    savePassword = true;
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository-Passwort ändern</div>
  <div class="change-password-form">
    <div class="form-section">
      <h3>Aktuelles Passwort</h3>
      <Input
        type="password"
        placeholder="Aktuelles Repository-Passwort eingeben"
        bind:value={currentPassword}
        required
      />
    </div>

    <div class="form-section">
      <h3>Neues Passwort</h3>
      <Input
        type="password"
        placeholder="Neues Passwort eingeben"
        bind:value={newPassword}
        required
      />

      {#if newPassword}
        <div class="password-strength">
          <div class="strength-bar">
            <div
              class="strength-fill"
              style="width: {passwordStrength.width}; background-color: {passwordStrength.color}"
            ></div>
          </div>
          <span class="strength-label" style="color: {passwordStrength.color}">
            {passwordStrength.label}
          </span>
        </div>
      {/if}
    </div>

    <div class="form-section">
      <h3>Passwort bestätigen</h3>
      <Input
        type="password"
        placeholder="Neues Passwort wiederholen"
        bind:value={confirmPassword}
        required
      />

      {#if confirmPassword && newPassword !== confirmPassword}
        <div class="error-message">Passwörter stimmen nicht überein</div>
      {/if}
    </div>

    <div class="form-section">
      <Checkbox label="Neues Passwort im Keychain speichern" bind:checked={savePassword} />
    </div>

    <div class="security-info">
      <div class="info-icon">🔒</div>
      <div class="info-text">
        <strong>Sicherheitshinweis:</strong> Verwenden Sie ein starkes Passwort mit mindestens 12 Zeichen,
        Groß- und Kleinbuchstaben, Zahlen und Sonderzeichen. Das Passwort schützt Ihre Backup-Daten.
      </div>
    </div>
  </div>

  <div slot="footer">
    <Button variant="secondary" on:click={handleClose} disabled={isLoading}>Abbrechen</Button>
    <Button variant="primary" on:click={handleSubmit} disabled={!validateForm() || isLoading}>
      {#if isLoading}
        Ändere Passwort...
      {:else}
        Passwort ändern
      {/if}
    </Button>
  </div>
</Modal>

<style>
  .change-password-form {
    max-width: 500px;
  }

  .form-section {
    margin-bottom: 24px;
  }

  .form-section h3 {
    font-size: 14px;
    font-weight: 600;
    color: #e4e4e7;
    margin-bottom: 8px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
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
    transition: all 0.2s ease;
  }

  .strength-label {
    font-size: 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .error-message {
    margin-top: 8px;
    color: #ef4444;
    font-size: 12px;
    font-weight: 500;
  }

  .security-info {
    background: #22273a;
    border: 1px solid #2d3348;
    border-radius: 8px;
    padding: 16px;
    display: flex;
    gap: 12px;
    align-items: flex-start;
  }

  .info-icon {
    font-size: 20px;
    flex-shrink: 0;
  }

  .info-text {
    font-size: 14px;
    color: #e4e4e7;
    line-height: 1.5;
  }

  .info-text strong {
    color: #3b82f6;
  }

  @media (max-width: 480px) {
    .security-info {
      flex-direction: column;
      text-align: center;
    }
  }
</style>

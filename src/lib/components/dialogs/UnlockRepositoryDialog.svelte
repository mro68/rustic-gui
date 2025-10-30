<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Checkbox from '../shared/Checkbox.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';

  const dispatch = createEventDispatcher();

  export let repositoryName = '';
  export let repositoryPath = '';

  // Form state
  let password = '';
  let rememberPassword = true;
  let isUnlocking = false;

  // Password strength
  let passwordStrength = 0;
  let strengthLabel = '';
  let strengthColor = '';

  $: updatePasswordStrength(password);

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
      // TODO: Show error toast
      return;
    }

    isUnlocking = true;

    try {
      // TODO: Implement actual unlock logic
      dispatch('unlock', {
        password: password.trim(),
        remember: rememberPassword,
      });

      // Close dialog on success
      dispatch('close');
    } catch (error) {
      // TODO: Show error toast
      console.error('Unlock failed:', error);
    } finally {
      isUnlocking = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }
</script>

<Modal on:close={handleClose}>
  <div slot="header">Repository entsperren</div>
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

  <div slot="footer">
    <Button variant="secondary" on:click={handleClose}>Abbrechen</Button>
    <Button variant="primary" on:click={handleUnlock} disabled={isUnlocking || !password.trim()}>
      {#if isUnlocking}
        Entsperre...
      {:else}
        Repository entsperren
      {/if}
    </Button>
  </div>
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

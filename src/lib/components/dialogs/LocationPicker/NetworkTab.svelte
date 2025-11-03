<!-- NetworkTab.svelte: Network (SFTP, SMB, NFS, WebDAV) Configuration -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Button from '../../shared/Button.svelte';
  import CustomSelect from '../../shared/CustomSelect.svelte';
  import Input from '../../shared/Input.svelte';

  interface NetworkTabProps {
    /** Netzwerk-Protokoll */
    networkProtocol?: 'sftp' | 'smb' | 'nfs' | 'webdav';
    /** Hostname */
    networkHost?: string;
    /** Port */
    networkPort?: string;
    /** Benutzername */
    networkUsername?: string;
    /** Passwort */
    networkPassword?: string;
    /** Authentifizierungs-Methode */
    networkAuth?: string;
    /** Remote-Pfad */
    networkPath?: string;
  }

  let {
    networkProtocol = $bindable('sftp'),
    networkHost = $bindable(''),
    networkPort = $bindable('22'),
    networkUsername = $bindable(''),
    networkPassword = $bindable(''),
    networkAuth = $bindable('password'),
    networkPath = $bindable(''),
  }: NetworkTabProps = $props();

  // Connection test state
  let testing = $state(false);
  let testResult: { success: boolean; message: string; latency_ms?: number } | null = $state(null);

  /**
   * Testet die Verbindung zum konfigurierten Backend.
   * M2 Task 2.3.1: Connection-Test-Button implementiert
   */
  async function testConnection() {
    testing = true;
    testResult = null;

    try {
      const backendType = 'rclone';
      const backendOptions = {
        remote_name: `rustic_sftp_${networkHost.replace(/\./g, '_')}`,
        provider: networkProtocol,
        path: networkPath,
        options: {
          host: networkHost,
          port: networkPort,
          user: networkUsername,
          pass: networkPassword,
        },
      };

      const result = await invoke<{
        success: boolean;
        message: string;
        latency_ms?: number;
      }>('test_repository_connection', {
        backendType,
        backendOptions,
      });

      testResult = result;
    } catch (error: any) {
      testResult = {
        success: false,
        message: error?.message || String(error),
      };
    } finally {
      testing = false;
    }
  }

  // Update network port when protocol changes
  $effect(() => {
    if (networkProtocol === 'sftp') {
      networkPort = '22';
    } else if (networkProtocol === 'smb') {
      networkPort = '445';
    } else if (networkProtocol === 'nfs') {
      networkPort = '2049';
    } else if (networkProtocol === 'webdav') {
      networkPort = '443';
    }
  });
</script>

<div class="tab-panel">
  <div class="info-box">🌐 Mit Remote-Repositories über SFTP, SMB, NFS oder WebDAV verbinden</div>

  <div class="form-group">
    <span class="form-label">Protokoll</span>
    <CustomSelect
      bind:value={networkProtocol}
      options={[
        { value: 'sftp', label: 'SFTP (SSH File Transfer)' },
        { value: 'smb', label: 'SMB/CIFS (Windows Share)' },
        { value: 'nfs', label: 'NFS (Network File System)' },
        { value: 'webdav', label: 'WebDAV' },
      ]}
    />
  </div>

  <div class="form-row">
    <div class="form-group">
      <span class="form-label">Host</span>
      <Input bind:value={networkHost} placeholder="backup.example.com" />
    </div>
    <div class="form-group" style="max-width: 120px;">
      <span class="form-label">Port</span>
      <Input type="number" bind:value={networkPort} />
    </div>
  </div>

  <div class="form-row">
    <div class="form-group">
      <span class="form-label">Benutzername</span>
      <Input bind:value={networkUsername} placeholder="user" />
    </div>
    <div class="form-group">
      <span class="form-label">Authentifizierung</span>
      <CustomSelect
        bind:value={networkAuth}
        options={[
          { value: 'password', label: 'Passwort' },
          { value: 'key', label: 'SSH Key' },
        ]}
      />
    </div>
  </div>

  <div class="form-group">
    <span class="form-label">Remote-Pfad</span>
    <Input bind:value={networkPath} placeholder="/pfad/zum/repository" />
    <div class="form-help">Pfad auf dem Remote-Server</div>
  </div>

  {#if networkHost}
    <div class="info-box success">
      ✅ Vorschau: <strong
        >{networkProtocol}://{networkUsername}@{networkHost}:{networkPort}{networkPath}</strong
      >
    </div>
  {/if}

  <!-- M2 Task 2.3.1: Connection Test UI for Network -->
  {#if networkHost && networkUsername && networkProtocol === 'sftp'}
    <div class="connection-test-section">
      <Button variant="secondary" size="sm" onclick={testConnection} disabled={testing}>
        {#if testing}
          🔄 Teste Verbindung...
        {:else}
          🔌 Verbindung testen
        {/if}
      </Button>

      {#if testResult}
        <div
          class="test-result"
          class:success={testResult.success}
          class:error={!testResult.success}
        >
          {#if testResult.success}
            <span class="result-icon">✅</span>
            <span class="result-message">{testResult.message}</span>
            {#if testResult.latency_ms}
              <span class="result-latency">({testResult.latency_ms}ms)</span>
            {/if}
          {:else}
            <span class="result-icon">❌</span>
            <span class="result-message">{testResult.message}</span>
          {/if}
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .tab-panel {
    animation: fadeIn 0.2s ease-in;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .info-box {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 8px;
    padding: 12px 16px;
    margin-bottom: 16px;
    font-size: 14px;
  }

  .info-box.success {
    background: rgba(34, 197, 94, 0.1);
    border-color: rgba(34, 197, 94, 0.3);
    color: var(--color-success);
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-label {
    display: block;
    margin-bottom: 6px;
    font-weight: 500;
    font-size: 14px;
  }

  .form-help {
    margin-top: 4px;
    font-size: 12px;
    color: var(--text-secondary);
  }

  .form-row {
    display: flex;
    gap: 12px;
  }

  .form-row .form-group {
    flex: 1;
  }

  /* Connection Test Section - M2 Task 2.3.1 */
  .connection-test-section {
    margin-top: 16px;
    padding: 12px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-secondary);
  }

  .test-result {
    margin-top: 12px;
    padding: 10px 12px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
  }

  .test-result.success {
    background: rgba(34, 197, 94, 0.1);
    border: 1px solid rgba(34, 197, 94, 0.3);
    color: #22c55e;
  }

  .test-result.error {
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #ef4444;
  }

  .result-icon {
    font-size: 16px;
  }

  .result-message {
    flex: 1;
  }

  .result-latency {
    font-size: 12px;
    opacity: 0.8;
  }
</style>

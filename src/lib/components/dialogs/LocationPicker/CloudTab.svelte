<!-- CloudTab.svelte: Cloud Provider Selection & Configuration -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Button from '../../shared/Button.svelte';
  import Input from '../../shared/Input.svelte';

  interface CloudTabProps {
    /** Ausgewählter Cloud-Provider */
    selectedCloudProvider?: string;
    /** Cloud-Endpoint */
    cloudEndpoint?: string;
    /** Bucket/Container-Name */
    cloudBucket?: string;
    /** Access Key */
    cloudAccessKey?: string;
    /** Secret Key */
    cloudSecretKey?: string;
    /** Region */
    cloudRegion?: string;
    /** Callback bei Provider-Auswahl */
    onProviderSelect?: (providerId: string) => void;
  }

  let {
    selectedCloudProvider = $bindable('s3'),
    cloudEndpoint = $bindable(''),
    cloudBucket = $bindable(''),
    cloudAccessKey = $bindable(''),
    cloudSecretKey = $bindable(''),
    cloudRegion = $bindable(''),
    onProviderSelect,
  }: CloudTabProps = $props();

  // Connection test state
  let testing = $state(false);
  let testResult: { success: boolean; message: string; latency_ms?: number } | null = $state(null);

  // Cloud providers from mockup
  const cloudProviders = [
    { id: 's3', name: 'Amazon S3', icon: '📦', description: 'AWS Object Storage' },
    { id: 'b2', name: 'Backblaze B2', icon: '☁️', description: 'Affordable Cloud Storage' },
    { id: 'azure', name: 'Azure Blob', icon: '🔷', description: 'Microsoft Cloud Storage' },
    { id: 'gcs', name: 'Google Cloud', icon: '🌐', description: 'GCS Object Storage' },
    { id: 'wasabi', name: 'Wasabi', icon: '💚', description: 'Hot Cloud Storage' },
    { id: 'minio', name: 'MinIO', icon: '🪣', description: 'Self-hosted S3-compatible' },
    { id: 'rclone', name: 'Rclone', icon: '🔗', description: '70+ Cloud Providers' },
  ];

  function selectCloudProvider(providerId: string) {
    selectedCloudProvider = providerId;
    if (onProviderSelect) {
      onProviderSelect(providerId);
    }
  }

  /**
   * Testet die Verbindung zum konfigurierten Cloud-Backend.
   * M2 Task 2.3.1: Connection-Test-Button implementiert
   */
  async function testConnection() {
    testing = true;
    testResult = null;

    try {
      // Map cloud provider names to backend types
      const providerMap: Record<string, string> = {
        s3: 's3',
        b2: 'b2',
        azure: 'azblob',
        gcs: 'gcs',
        wasabi: 's3', // Wasabi uses S3 protocol
        minio: 's3', // MinIO uses S3 protocol
        rclone: 'rclone',
      };

      const backendType = providerMap[selectedCloudProvider] || 's3';
      let backendOptions: any;

      if (selectedCloudProvider === 'rclone') {
        backendOptions = {
          remote_name: `rustic_${cloudBucket}`,
          provider: 'custom',
          path: '/',
          options: {
            endpoint: cloudEndpoint,
          },
        };
      } else {
        backendOptions = {
          provider: backendType,
          endpoint: cloudBucket || cloudEndpoint,
          access_key: cloudAccessKey,
          secret_key: cloudSecretKey,
          region: cloudRegion || undefined,
          endpoint_url:
            selectedCloudProvider === 'wasabi' || selectedCloudProvider === 'minio'
              ? cloudEndpoint
              : undefined,
        };
      }

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
</script>

<div class="tab-panel">
  <div class="info-box">☁️ Wählen Sie Ihren Cloud-Storage-Anbieter für Repository-Zugriff</div>

  <div class="cloud-grid">
    {#each cloudProviders as provider}
      <button
        class="cloud-card"
        class:selected={selectedCloudProvider === provider.id}
        onclick={() => selectCloudProvider(provider.id)}
      >
        <div class="cloud-icon">{provider.icon}</div>
        <div class="cloud-name">{provider.name}</div>
        <div class="cloud-desc">{provider.description}</div>
      </button>
    {/each}
  </div>

  {#if selectedCloudProvider}
    <div class="form-group" style="margin-top: 20px;">
      <span class="form-label">Endpoint / Region</span>
      <Input
        bind:value={cloudEndpoint}
        placeholder={selectedCloudProvider === 's3'
          ? 's3.amazonaws.com oder s3.eu-central-1.amazonaws.com'
          : 'Endpoint-URL'}
      />
    </div>

    <div class="form-group">
      <span class="form-label">Bucket / Container</span>
      <Input bind:value={cloudBucket} placeholder="my-backup-bucket" />
    </div>

    <div class="form-row">
      <div class="form-group">
        <span class="form-label">Access Key / Client ID</span>
        <Input bind:value={cloudAccessKey} placeholder="AKIAIOSFODNN7EXAMPLE" />
      </div>
      <div class="form-group">
        <span class="form-label">Secret Key / Client Secret</span>
        <Input
          type="password"
          bind:value={cloudSecretKey}
          placeholder="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
        />
      </div>
    </div>

    {#if cloudBucket}
      <div class="info-box success">
        ✅ Vorschau: <strong>{selectedCloudProvider}:{cloudEndpoint}/{cloudBucket}</strong>
      </div>
    {/if}

    <!-- M2 Task 2.3.1: Connection Test UI -->
    {#if cloudAccessKey && cloudSecretKey}
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

  .cloud-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 12px;
    margin-bottom: 20px;
  }

  .cloud-card {
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    border-radius: 12px;
    padding: 20px;
    cursor: pointer;
    text-align: center;
    transition: all 0.2s;
  }

  .cloud-card:hover {
    border-color: var(--color-primary);
    background: var(--bg-tertiary);
  }

  .cloud-card.selected {
    border-color: var(--color-primary);
    background: rgba(59, 130, 246, 0.1);
  }

  .cloud-icon {
    font-size: 32px;
    margin-bottom: 8px;
  }

  .cloud-name {
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 4px;
  }

  .cloud-desc {
    font-size: 12px;
    color: var(--text-secondary);
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

<!-- LocationPickerDialog.svelte: Unified Location Picker for Repository Paths -->
<!--
  Mockup-Referenz: docs/mockups/rustic_location_picker.html
  
  Features:
  - 📁 Local File/Directory Browser mit Breadcrumb-Navigation
  - 🌐 Network Tab (SFTP, SMB, NFS, WebDAV)
  - ☁️ Cloud Provider Selection (S3, B2, Azure, GCS, Wasabi, MinIO, Rclone)
  - 🕐 Recent Locations Tab
  
  Events:
  - select: { path: string, type: 'local' | 'network' | 'cloud', config?: any }
  - cancel: void
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Modal from '../shared/Modal.svelte';
  import Button from '../shared/Button.svelte';
  import Input from '../shared/Input.svelte';
  import Select from '../shared/Select.svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  const dispatch = createEventDispatcher<{
    select: { path: string; type: string; config?: any };
    cancel: void;
  }>();

  // Props
  export let isOpen = false;
  export let mode: 'init' | 'open' = 'init';
  export let title = 'Repository-Speicherort auswählen';

  // Tab state
  let activeTab: 'local' | 'network' | 'cloud' | 'recent' = 'local';

  // Local tab state
  let selectedPath = '';
  let currentPath = '';
  let newFolderName = '';
  let fileItems: Array<{
    name: string;
    path: string;
    isDirectory: boolean;
    size?: string;
  }> = [];

  // Network tab state
  let networkProtocol: 'sftp' | 'smb' | 'nfs' | 'webdav' = 'sftp';
  let networkHost = '';
  let networkPort = '22';
  let networkUsername = '';
  let networkAuth = 'password';
  let networkPassword = '';
  let networkPath = '';

  // Cloud tab state
  let selectedCloudProvider = 's3';
  let cloudEndpoint = '';
  let cloudBucket = '';
  let cloudAccessKey = '';
  let cloudSecretKey = '';
  let cloudRegion = '';

  // Recent locations (would come from config)
  let recentLocations = [
    {
      path: '/home/user/backup',
      type: 'Local',
      icon: '💾',
      lastUsed: 'Today 14:23',
    },
    {
      path: 'sftp://backup.example.com/prod',
      type: 'SFTP',
      icon: '🌐',
      lastUsed: 'Yesterday 02:00',
    },
    {
      path: 's3:s3.amazonaws.com/my-backup',
      type: 'Amazon S3',
      icon: '☁️',
      lastUsed: 'Oct 10',
    },
  ];

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

  async function browseLocalDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Verzeichnis auswählen',
      });

      if (selected && typeof selected === 'string') {
        selectedPath = selected;
        currentPath = selected;
      }
    } catch (error) {
      console.error('File browser error:', error);
    }
  }

  async function browseLocalFile() {
    try {
      const selected = await open({
        directory: false,
        multiple: false,
        title: 'Datei auswählen',
      });

      if (selected && typeof selected === 'string') {
        selectedPath = selected;
      }
    } catch (error) {
      console.error('File browser error:', error);
    }
  }

  function selectTab(tab: typeof activeTab) {
    activeTab = tab;
  }

  function selectCloudProvider(providerId: string) {
    selectedCloudProvider = providerId;
  }

  function selectRecentLocation(location: (typeof recentLocations)[0]) {
    if (location.type === 'Local') {
      activeTab = 'local';
      selectedPath = location.path;
    } else if (location.type === 'SFTP') {
      activeTab = 'network';
      networkProtocol = 'sftp';
      // Parse SFTP URL
      const url = location.path.replace('sftp://', '');
      const parts = url.split('/');
      networkHost = parts[0];
      networkPath = '/' + parts.slice(1).join('/');
    } else if (location.type.includes('S3')) {
      activeTab = 'cloud';
      selectedCloudProvider = 's3';
    }
  }

  function handleSelect() {
    if (activeTab === 'local') {
      let finalPath = selectedPath;
      if (newFolderName.trim()) {
        finalPath = `${selectedPath}/${newFolderName.trim()}`;
      }
      dispatch('select', {
        path: finalPath,
        type: 'local',
      });
    } else if (activeTab === 'network') {
      const url = `${networkProtocol}://${networkUsername}@${networkHost}:${networkPort}${networkPath}`;
      dispatch('select', {
        path: url,
        type: 'network',
        config: {
          protocol: networkProtocol,
          host: networkHost,
          port: networkPort,
          username: networkUsername,
          auth: networkAuth,
          remotePath: networkPath,
        },
      });
    } else if (activeTab === 'cloud') {
      const path = `${selectedCloudProvider}:${cloudEndpoint}/${cloudBucket}`;
      dispatch('select', {
        path,
        type: 'cloud',
        config: {
          provider: selectedCloudProvider,
          endpoint: cloudEndpoint,
          bucket: cloudBucket,
          accessKey: cloudAccessKey,
          secretKey: cloudSecretKey,
          region: cloudRegion,
        },
      });
    }
    isOpen = false;
  }

  function handleCancel() {
    dispatch('cancel');
    isOpen = false;
  }

  // Update network port when protocol changes
  $: if (networkProtocol === 'sftp') {
    networkPort = '22';
  } else if (networkProtocol === 'smb') {
    networkPort = '445';
  } else if (networkProtocol === 'nfs') {
    networkPort = '2049';
  } else if (networkProtocol === 'webdav') {
    networkPort = '443';
  }
</script>

<Modal bind:isOpen {title} size="large">
  <div class="location-picker">
    <!-- Tabs -->
    <div class="location-tabs">
      <button
        class="location-tab"
        class:active={activeTab === 'local'}
        on:click={() => selectTab('local')}
      >
        <span class="tab-icon">💻</span>
        Lokal
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'network'}
        on:click={() => selectTab('network')}
      >
        <span class="tab-icon">🌐</span>
        Netzwerk
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'cloud'}
        on:click={() => selectTab('cloud')}
      >
        <span class="tab-icon">☁️</span>
        Cloud
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'recent'}
        on:click={() => selectTab('recent')}
      >
        <span class="tab-icon">🕐</span>
        Zuletzt verwendet
      </button>
    </div>

    <!-- Local Tab -->
    {#if activeTab === 'local'}
      <div class="tab-panel">
        <div class="info-box">
          💾 Wählen Sie ein lokales Verzeichnis für Ihr Repository
        </div>

        <div class="browser-toolbar">
          <Button variant="secondary" size="small" on:click={browseLocalDirectory}>
            📁 Verzeichnis wählen
          </Button>
          {#if mode === 'open'}
            <Button variant="secondary" size="small" on:click={browseLocalFile}>
              📄 Datei wählen
            </Button>
          {/if}
        </div>

        {#if selectedPath}
          <div class="form-group">
            <label class="form-label">Ausgewählter Pfad</label>
            <Input bind:value={selectedPath} placeholder="/pfad/zum/repository" />
          </div>
        {/if}

        {#if mode === 'init'}
          <div class="form-group">
            <label class="form-label">Neuer Ordner-Name (optional)</label>
            <Input
              bind:value={newFolderName}
              placeholder="z.B. mein-neues-repo"
            />
            <div class="form-help">
              Leer lassen um ausgewähltes Verzeichnis zu verwenden, oder Namen eingeben um neuen
              Ordner zu erstellen
            </div>
          </div>
        {/if}

        {#if selectedPath}
          <div class="info-box success">
            ✅ Ausgewählt: <strong>{selectedPath}</strong>
            {#if newFolderName.trim()}
              / <strong>{newFolderName.trim()}</strong>
            {/if}
          </div>
        {/if}
      </div>
    {/if}

    <!-- Network Tab -->
    {#if activeTab === 'network'}
      <div class="tab-panel">
        <div class="info-box">
          🌐 Mit Remote-Repositories über SFTP, SMB, NFS oder WebDAV verbinden
        </div>

        <div class="form-group">
          <label class="form-label">Protokoll</label>
          <Select bind:value={networkProtocol}>
            <option value="sftp">SFTP (SSH File Transfer)</option>
            <option value="smb">SMB/CIFS (Windows Share)</option>
            <option value="nfs">NFS (Network File System)</option>
            <option value="webdav">WebDAV</option>
          </Select>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label class="form-label">Host</label>
            <Input bind:value={networkHost} placeholder="backup.example.com" />
          </div>
          <div class="form-group" style="max-width: 120px;">
            <label class="form-label">Port</label>
            <Input type="number" bind:value={networkPort} />
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label class="form-label">Benutzername</label>
            <Input bind:value={networkUsername} placeholder="backup-user" />
          </div>
          <div class="form-group">
            <label class="form-label">Authentifizierung</label>
            <Select bind:value={networkAuth}>
              <option value="password">Passwort</option>
              <option value="key">SSH Key</option>
            </Select>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">Remote-Pfad</label>
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
      </div>
    {/if}

    <!-- Cloud Tab -->
    {#if activeTab === 'cloud'}
      <div class="tab-panel">
        <div class="info-box">
          ☁️ Wählen Sie Ihren Cloud-Storage-Anbieter für Repository-Zugriff
        </div>

        <div class="cloud-grid">
          {#each cloudProviders as provider}
            <button
              class="cloud-card"
              class:selected={selectedCloudProvider === provider.id}
              on:click={() => selectCloudProvider(provider.id)}
            >
              <div class="cloud-icon">{provider.icon}</div>
              <div class="cloud-name">{provider.name}</div>
              <div class="cloud-desc">{provider.description}</div>
            </button>
          {/each}
        </div>

        {#if selectedCloudProvider}
          <div class="form-group" style="margin-top: 20px;">
            <label class="form-label">Endpoint / Region</label>
            <Input
              bind:value={cloudEndpoint}
              placeholder={selectedCloudProvider === 's3'
                ? 's3.amazonaws.com oder s3.eu-central-1.amazonaws.com'
                : 'Endpoint-URL'}
            />
          </div>

          <div class="form-group">
            <label class="form-label">Bucket / Container</label>
            <Input bind:value={cloudBucket} placeholder="my-backup-bucket" />
          </div>

          <div class="form-row">
            <div class="form-group">
              <label class="form-label">Access Key / Client ID</label>
              <Input bind:value={cloudAccessKey} placeholder="AKIAIOSFODNN7EXAMPLE" />
            </div>
            <div class="form-group">
              <label class="form-label">Secret Key / Client Secret</label>
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
        {/if}
      </div>
    {/if}

    <!-- Recent Tab -->
    {#if activeTab === 'recent'}
      <div class="tab-panel">
        <div class="info-box">
          🕐 Zuletzt verwendete Repository-Speicherorte
        </div>

        <div class="recent-list">
          {#each recentLocations as location}
            <button class="recent-item" on:click={() => selectRecentLocation(location)}>
              <span class="recent-icon">{location.icon}</span>
              <div class="recent-info">
                <div class="recent-path">{location.path}</div>
                <div class="recent-type">{location.type} • Zuletzt: {location.lastUsed}</div>
              </div>
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <svelte:fragment slot="footer">
    <div style="flex: 1; font-size: 13px; color: var(--text-secondary);">
      {#if activeTab === 'local' && selectedPath}
        Ausgewählt: <span style="color: var(--text-primary);">{selectedPath}</span>
      {:else if activeTab === 'network' && networkHost}
        Ausgewählt: <span style="color: var(--text-primary);"
          >{networkProtocol}://{networkHost}{networkPath}</span
        >
      {:else if activeTab === 'cloud' && cloudBucket}
        Ausgewählt: <span style="color: var(--text-primary);"
          >{selectedCloudProvider}:{cloudBucket}</span
        >
      {:else}
        Bitte wählen Sie einen Speicherort
      {/if}
    </div>
    <Button variant="secondary" on:click={handleCancel}>Abbrechen</Button>
    <Button
      variant="primary"
      on:click={handleSelect}
      disabled={!selectedPath && !networkHost && !cloudBucket}
    >
      Speicherort wählen
    </Button>
  </svelte:fragment>
</Modal>

<style>
  .location-picker {
    min-height: 400px;
  }

  /* Tabs */
  .location-tabs {
    display: flex;
    gap: 8px;
    margin-bottom: 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .location-tab {
    padding: 12px 20px;
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 14px;
    border: none;
    background: none;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .location-tab:hover {
    color: var(--text-primary);
  }

  .location-tab.active {
    color: var(--text-primary);
    border-bottom-color: var(--color-primary);
  }

  .tab-icon {
    font-size: 18px;
  }

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

  /* Info Box */
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

  /* Form Elements */
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

  /* Browser Toolbar */
  .browser-toolbar {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
  }

  /* Cloud Provider Grid */
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

  /* Recent List */
  .recent-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .recent-item {
    padding: 12px 16px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 12px;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-secondary);
    transition: all 0.2s;
    text-align: left;
  }

  .recent-item:hover {
    background: var(--bg-tertiary);
    border-color: var(--color-primary);
  }

  .recent-icon {
    font-size: 24px;
  }

  .recent-info {
    flex: 1;
  }

  .recent-path {
    font-size: 14px;
    font-weight: 500;
    font-family: 'Courier New', monospace;
    margin-bottom: 4px;
  }

  .recent-type {
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>

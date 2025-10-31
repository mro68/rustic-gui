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
  /**
   * Unified Location Picker für Repository-Pfade.
   *
   * 4-Tab-Interface (gemäß rustic_location_picker.html):
   * - Local: File/Directory Browser
   * - Network: SFTP, SMB, NFS, WebDAV
   * - Cloud: S3, B2, Azure, GCS, Wasabi, MinIO, Rclone
   * - Recent: Zuletzt verwendete Locations
   *
   * @component
   *
   * @example
   * ```svelte
   * <LocationPickerDialog
   *   bind:isOpen={showPicker}
   *   mode="init"
   *   title="Repository-Speicherort auswählen"
   *   on:select={handleLocationSelect}
   *   on:cancel={handleCancel}
   * />
   * ```
   */
  import { open as openDialog } from '@tauri-apps/api/dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher, onMount } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Input from '../shared/Input.svelte';
  import Modal from '../shared/Modal.svelte';
  import Select from '../shared/Select.svelte';

  const dispatch = createEventDispatcher<{
    select: { path: string; type: string; config?: any };
    cancel: void;
  }>();

  interface LocationPickerDialogProps {
    /** Steuert Sichtbarkeit */
    isOpen?: boolean;
    /** Modus: 'init' für neues Repo, 'open' für bestehendes */
    mode?: 'init' | 'open';
    /** Dialog-Titel */
    title?: string;
  }

  let {
    isOpen = $bindable(false),
    mode = 'init',
    title = 'Repository-Speicherort auswählen',
  }: LocationPickerDialogProps = $props();

  // Tab state
  let activeTab: 'local' | 'network' | 'cloud' | 'recent' = 'local';

  // Local tab state
  let selectedPath = '';
  // eslint-disable-next-line no-unused-vars
  let currentPath = ''; // TODO: Implement file browser navigation
  let newFolderName = '';
  // eslint-disable-next-line no-unused-vars
  let fileItems: Array<{
    name: string;
    path: string;
    isDirectory: boolean;
    size?: string;
  }> = []; // TODO: Populate with file browser results

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

  // Connection test state (M2 Task 2.3.1)
  let testing = false;
  let testResult: { success: boolean; message: string; latency_ms?: number } | null = null;

  // Credential prompt state (M2 Task 2.3.3)
  let showCredentialPrompt = false;
  let saveCredentialsChecked = true;
  let saveFavoriteChecked = true;

  // Recent locations (M2 Task 2.3.2: Now loaded from backend)
  let recentLocations: Array<{
    id: string;
    name: string;
    path: string;
    type: string;
    icon: string;
    lastUsed: string;
  }> = [];

  // Load favorites on mount
  onMount(async () => {
    await loadFavorites();
  });

  /**
   * Lädt favorisierte Locations vom Backend.
   * M2 Task 2.3.2: Favoriten-Management
   */
  async function loadFavorites() {
    try {
      const favorites = await invoke<
        Array<{
          id: string;
          name: string;
          path: string;
          location_type: string;
          config: any;
          created_at: string;
          last_used?: string;
        }>
      >('list_favorite_locations');

      recentLocations = favorites.map((fav) => ({
        id: fav.id,
        name: fav.name,
        path: fav.path,
        type: getLocationTypeLabel(fav.location_type),
        icon: getLocationIcon(fav.location_type),
        lastUsed: fav.last_used ? formatLastUsed(fav.last_used) : formatLastUsed(fav.created_at),
      }));
    } catch (error) {
      console.error('Fehler beim Laden der Favoriten:', error);
      recentLocations = [];
    }
  }

  /**
   * Gibt ein Label für den Location-Typ zurück.
   */
  function getLocationTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      local: 'Local',
      sftp: 'SFTP',
      s3: 'Amazon S3',
      azure: 'Azure Blob',
      gcs: 'Google Cloud',
      rclone: 'Rclone',
    };
    return labels[type] || type;
  }

  /**
   * Gibt ein Icon für den Location-Typ zurück.
   */
  function getLocationIcon(type: string): string {
    const icons: Record<string, string> = {
      local: '💾',
      sftp: '🌐',
      s3: '☁️',
      azure: '🔷',
      gcs: '🌐',
      rclone: '🔗',
    };
    return icons[type] || '📁';
  }

  /**
   * Formatiert einen ISO-Timestamp für Anzeige.
   */
  function formatLastUsed(isoString: string): string {
    const date = new Date(isoString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Gerade eben';
    if (diffMins < 60) return `Vor ${diffMins} Min`;
    if (diffHours < 24) return `Vor ${diffHours} Std`;
    if (diffDays === 0) return 'Heute';
    if (diffDays === 1) return 'Gestern';
    if (diffDays < 7) return `Vor ${diffDays} Tagen`;

    return date.toLocaleDateString('de-DE', {
      month: 'short',
      day: 'numeric',
    });
  }

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
      const selected = await openDialog({
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
      const selected = await openDialog({
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
    // Markiere als verwendet
    invoke('update_favorite_last_used', { id: location.id }).catch((error) => {
      console.error('Fehler beim Aktualisieren des Favoriten:', error);
    });

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
    } else if (location.type.includes('S3') || location.type.includes('Amazon')) {
      activeTab = 'cloud';
      selectedCloudProvider = 's3';
    } else if (location.type.includes('Azure')) {
      activeTab = 'cloud';
      selectedCloudProvider = 'azure';
    } else if (location.type.includes('Google')) {
      activeTab = 'cloud';
      selectedCloudProvider = 'gcs';
    }
  }

  /**
   * Speichert aktuelle Konfiguration als Favorit.
   * M2 Task 2.3.2: Favoriten-Management
   */
  async function saveCurrentAsFavorite() {
    let name = '';
    let path = '';
    let locationType = '';
    let config: any = null;

    if (activeTab === 'local') {
      name = `Local: ${selectedPath.split('/').pop() || 'Backup'}`;
      path = selectedPath;
      locationType = 'local';
    } else if (activeTab === 'network') {
      name = `${networkProtocol.toUpperCase()}: ${networkHost}`;
      path = `${networkProtocol}://${networkUsername}@${networkHost}:${networkPort}${networkPath}`;
      locationType = 'sftp';
      config = {
        protocol: networkProtocol,
        host: networkHost,
        port: networkPort,
        username: networkUsername,
        remotePath: networkPath,
      };
    } else if (activeTab === 'cloud') {
      name = `${cloudProviders.find((p) => p.id === selectedCloudProvider)?.name}: ${cloudBucket}`;
      path = `${selectedCloudProvider}:${cloudEndpoint}/${cloudBucket}`;
      locationType = selectedCloudProvider;
      config = {
        provider: selectedCloudProvider,
        endpoint: cloudEndpoint,
        bucket: cloudBucket,
        region: cloudRegion,
      };
    }

    if (!name || !path) {
      console.error('Ungültige Favoriten-Daten');
      return;
    }

    try {
      await invoke('save_favorite_location', {
        name,
        path,
        locationType,
        config,
      });

      // Favoriten neu laden
      await loadFavorites();

      console.log('Als Favorit gespeichert!');
    } catch (error) {
      console.error('Fehler beim Speichern des Favoriten:', error);
      console.warn('Fehler beim Speichern des Favoriten: ' + error);
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

  /**
   * Testet die Verbindung zum konfigurierten Backend.
   * M2 Task 2.3.1: Connection-Test-Button implementiert
   */
  async function testConnection() {
    testing = true;
    testResult = null;

    try {
      let backendType: string;
      let backendOptions: any;

      if (activeTab === 'cloud') {
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

        backendType = providerMap[selectedCloudProvider] || 's3';

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
      } else if (activeTab === 'network') {
        backendType = 'rclone';
        backendOptions = {
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
      } else if (activeTab === 'local') {
        backendType = 'local';
        backendOptions = {
          path: selectedPath,
        };
      } else {
        throw new Error('Ungültiger Tab für Connection-Test');
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

      // M2 Task 2.3.3: Zeige Credential-Prompt bei erfolgreichem Test
      if (result.success && (activeTab === 'cloud' || activeTab === 'network')) {
        // Zeige Prompt nur wenn Credentials vorhanden sind
        const hasCredentials =
          (activeTab === 'cloud' && cloudAccessKey && cloudSecretKey) ||
          (activeTab === 'network' && networkPassword);

        if (hasCredentials) {
          showCredentialPrompt = true;
        }
      }
    } catch (error: any) {
      testResult = {
        success: false,
        message: error?.message || String(error),
      };
    } finally {
      testing = false;
    }
  }

  /**
   * Speichert Credentials nach erfolgreichem Connection-Test.
   * M2 Task 2.3.3: Credential-Prompt-Integration
   */
  async function handleCredentialPrompt(save: boolean) {
    showCredentialPrompt = false;

    if (!save) {
      return;
    }

    try {
      // Generiere eine temporäre Repo-ID (wird später beim Init ersetzt)
      const tempRepoId = `temp_${Date.now()}`;

      if (activeTab === 'cloud' && saveCredentialsChecked) {
        // Speichere Cloud-Credentials in Keychain
        await invoke('save_cloud_credentials', {
          repoId: tempRepoId,
          provider: selectedCloudProvider,
          accessKey: cloudAccessKey,
          secretKey: cloudSecretKey,
        });

        console.log('Cloud-Credentials gespeichert im Keychain');
      } else if (activeTab === 'network' && saveCredentialsChecked) {
        // Speichere Network-Credentials in Keychain
        await invoke('save_cloud_credentials', {
          repoId: tempRepoId,
          provider: `sftp_${networkHost}`,
          accessKey: networkUsername,
          secretKey: networkPassword,
        });

        console.log('Network-Credentials gespeichert im Keychain');
      }

      // Optional: Als Favorit speichern
      if (saveFavoriteChecked) {
        await saveCurrentAsFavorite();
      }

      console.log('Zugangsdaten sicher gespeichert!');
    } catch (error) {
      console.error('Fehler beim Speichern der Credentials:', error);
      console.warn('Fehler beim Speichern: ' + error);
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

<Modal bind:open={isOpen} {title} size="large">
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
        <div class="info-box">💾 Wählen Sie ein lokales Verzeichnis für Ihr Repository</div>

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
            <Input bind:value={newFolderName} placeholder="z.B. mein-neues-repo" />
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

        <!-- M2 Task 2.3.1: Connection Test UI for Network -->
        {#if networkHost && networkUsername && networkProtocol === 'sftp'}
          <div class="connection-test-section">
            <Button
              variant="secondary"
              size="small"
              on:click={testConnection}
              disabled={testing}
            >
              {#if testing}
                🔄 Teste Verbindung...
              {:else}
                🔌 Verbindung testen
              {/if}
            </Button>

            {#if testResult}
              <div class="test-result" class:success={testResult.success} class:error={!testResult.success}>
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

          <!-- M2 Task 2.3.1: Connection Test UI -->
          {#if cloudAccessKey && cloudSecretKey}
            <div class="connection-test-section">
              <Button
                variant="secondary"
                size="small"
                on:click={testConnection}
                disabled={testing}
              >
                {#if testing}
                  🔄 Teste Verbindung...
                {:else}
                  🔌 Verbindung testen
                {/if}
              </Button>

              {#if testResult}
                <div class="test-result" class:success={testResult.success} class:error={!testResult.success}>
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
    {/if}

    <!-- Recent Tab -->
    {#if activeTab === 'recent'}
      <div class="tab-panel">
        <div class="info-box">🕐 Zuletzt verwendete Repository-Speicherorte</div>

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

    <!-- M2 Task 2.3.2: Save as Favorite Button -->
    {#if activeTab !== 'recent' && (selectedPath || networkHost || cloudBucket)}
      <Button variant="secondary" size="small" on:click={saveCurrentAsFavorite}>
        ⭐ Als Favorit speichern
      </Button>
    {/if}

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

<!-- M2 Task 2.3.3: Credential-Prompt Dialog -->
{#if showCredentialPrompt}
  <Modal isOpen={true} title="Zugangsdaten speichern?" size="small">
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
          🔒 Zugangsdaten werden verschlüsselt im System-Keychain gespeichert. Sie werden nicht in
          der Konfigurationsdatei abgelegt.
        </p>
      </div>
    </div>

    <svelte:fragment slot="footer">
      <Button variant="secondary" on:click={() => handleCredentialPrompt(false)}>
        Nicht speichern
      </Button>
      <Button variant="primary" on:click={() => handleCredentialPrompt(true)}>
        Speichern
      </Button>
    </svelte:fragment>
  </Modal>
{/if}

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

  /* Credential Prompt (M2 Task 2.3.3) */
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

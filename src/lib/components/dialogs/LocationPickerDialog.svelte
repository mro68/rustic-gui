<!-- LocationPickerDialog.svelte: Unified Location Picker for Repository Paths -->
<!--
  Mockup-Referenz: docs/mockups/rustic_location_picker.html
  
  Modular refactored - Orchestrates 5 sub-components:
  - LocalTab: Local file/directory selection
  - NetworkTab: SFTP, SMB, NFS, WebDAV configuration
  - CloudTab: Cloud provider selection (S3, B2, Azure, GCS, etc.)
  - RecentTab: Recent locations list
  - CredentialPromptModal: Credential save prompt after successful test
  
  Events:
  - select: { path: string, type: 'local' | 'network' | 'cloud', config?: any }
  - cancel: void
-->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher, onMount } from 'svelte';
  import Button from '../shared/Button.svelte';
  import Modal from '../shared/Modal.svelte';
  import CloudTab from './LocationPicker/CloudTab.svelte';
  import CredentialPromptModal from './LocationPicker/CredentialPromptModal.svelte';
  import LocalTab from './LocationPicker/LocalTab.svelte';
  import NetworkTab from './LocationPicker/NetworkTab.svelte';
  import RecentTab from './LocationPicker/RecentTab.svelte';

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
  let activeTab: 'local' | 'network' | 'cloud' | 'recent' = $state('local');

  // Local tab state
  let selectedPath = $state('');
  let newFolderName = $state('');

  // Network tab state
  let networkProtocol: 'sftp' | 'smb' | 'nfs' | 'webdav' = $state('sftp');
  let networkHost = $state('');
  let networkPort = $state('22');
  let networkUsername = $state('');
  let networkPassword = $state('');
  let networkAuth = $state('password');
  let networkPath = $state('');

  // Cloud tab state
  let selectedCloudProvider = $state('s3');
  let cloudEndpoint = $state('');
  let cloudBucket = $state('');
  let cloudAccessKey = $state('');
  let cloudSecretKey = $state('');
  let cloudRegion = $state('');

  // Credential prompt state (M2 Task 2.3.3)
  let showCredentialPrompt = $state(false);

  // Recent locations (M2 Task 2.3.2: Loaded from backend)
  let recentLocations: Array<{
    id: string;
    name: string;
    path: string;
    type: string;
    icon: string;
    lastUsed: string;
  }> = $state([]);

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

  function selectTab(tab: typeof activeTab) {
    activeTab = tab;
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
      name = `${selectedCloudProvider.toUpperCase()}: ${cloudBucket}`;
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
   * Speichert Credentials nach erfolgreichem Connection-Test.
   * M2 Task 2.3.3: Credential-Prompt-Integration
   */
  async function handleCredentialSave(saveCredentials: boolean, saveFavorite: boolean) {
    if (!saveCredentials && !saveFavorite) {
      return;
    }

    try {
      // Generiere eine temporäre Repo-ID (wird später beim Init ersetzt)
      const tempRepoId = `temp_${Date.now()}`;

      if (activeTab === 'cloud' && saveCredentials) {
        // Speichere Cloud-Credentials in Keychain
        await invoke('save_cloud_credentials', {
          repoId: tempRepoId,
          provider: selectedCloudProvider,
          accessKey: cloudAccessKey,
          secretKey: cloudSecretKey,
        });

        console.log('Cloud-Credentials gespeichert im Keychain');
      } else if (activeTab === 'network' && saveCredentials) {
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
      if (saveFavorite) {
        await saveCurrentAsFavorite();
      }

      console.log('Zugangsdaten sicher gespeichert!');
    } catch (error) {
      console.error('Fehler beim Speichern der Credentials:', error);
      console.warn('Fehler beim Speichern: ' + error);
    }
  }

  function handleCredentialCancel() {
    showCredentialPrompt = false;
  }

  // Derived state for footer preview text
  const previewText = $derived.by(() => {
    if (activeTab === 'local' && selectedPath) {
      return `Ausgewählt: ${selectedPath}`;
    } else if (activeTab === 'network' && networkHost) {
      return `Ausgewählt: ${networkProtocol}://${networkHost}${networkPath}`;
    } else if (activeTab === 'cloud' && cloudBucket) {
      return `Ausgewählt: ${selectedCloudProvider}:${cloudBucket}`;
    }
    return 'Bitte wählen Sie einen Speicherort';
  });

  // Derived state for Select button enabled
  const canSelect = $derived.by(() => {
    return Boolean(selectedPath || networkHost || cloudBucket);
  });
</script>

<Modal bind:open={isOpen}>
  {#snippet header()}
    {title}
  {/snippet}

  <div class="location-picker">
    <!-- Tabs -->
    <div class="location-tabs">
      <button
        class="location-tab"
        class:active={activeTab === 'local'}
        onclick={() => selectTab('local')}
      >
        <span class="tab-icon">💻</span>
        Lokal
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'network'}
        onclick={() => selectTab('network')}
      >
        <span class="tab-icon">🌐</span>
        Netzwerk
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'cloud'}
        onclick={() => selectTab('cloud')}
      >
        <span class="tab-icon">☁️</span>
        Cloud
      </button>
      <button
        class="location-tab"
        class:active={activeTab === 'recent'}
        onclick={() => selectTab('recent')}
      >
        <span class="tab-icon">🕐</span>
        Zuletzt verwendet
      </button>
    </div>

    <!-- Tab Panels -->
    {#if activeTab === 'local'}
      <LocalTab {mode} bind:selectedPath bind:newFolderName />
    {:else if activeTab === 'network'}
      <NetworkTab
        bind:networkProtocol
        bind:networkHost
        bind:networkPort
        bind:networkUsername
        bind:networkPassword
        bind:networkAuth
        bind:networkPath
      />
    {:else if activeTab === 'cloud'}
      <CloudTab
        bind:selectedCloudProvider
        bind:cloudEndpoint
        bind:cloudBucket
        bind:cloudAccessKey
        bind:cloudSecretKey
        bind:cloudRegion
      />
    {:else if activeTab === 'recent'}
      <RecentTab {recentLocations} onSelect={selectRecentLocation} />
    {/if}
  </div>

  {#snippet footer()}
    <div style="flex: 1; font-size: 13px; color: var(--text-secondary);">
      <span style="color: var(--text-primary);">{previewText}</span>
    </div>

    <!-- M2 Task 2.3.2: Save as Favorite Button -->
    {#if activeTab !== 'recent' && canSelect}
      <Button variant="secondary" size="sm" onclick={saveCurrentAsFavorite}>
        ⭐ Als Favorit speichern
      </Button>
    {/if}

    <Button variant="secondary" onclick={handleCancel}>Abbrechen</Button>
    <Button variant="primary" onclick={handleSelect} disabled={!canSelect}>
      Speicherort wählen
    </Button>
  {/snippet}
</Modal>

<!-- M2 Task 2.3.3: Credential-Prompt Dialog -->
<CredentialPromptModal
  bind:open={showCredentialPrompt}
  onSave={handleCredentialSave}
  onCancel={handleCredentialCancel}
/>

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
</style>

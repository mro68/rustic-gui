<!-- LocalTab.svelte: Local File/Directory Browser -->
<script lang="ts">
  import Button from '../../shared/Button.svelte';
  import Input from '../../shared/Input.svelte';

  interface LocalTabProps {
    /** Modus: 'init' für neues Repo, 'open' für bestehendes */
    mode: 'init' | 'open';
    /** Ausgewählter Pfad */
    selectedPath?: string;
    /** Neuer Ordner-Name (nur bei mode='init') */
    newFolderName?: string;
  }

  let {
    mode = 'init',
    selectedPath = $bindable(''),
    newFolderName = $bindable(''),
  }: LocalTabProps = $props();

  async function browseLocalDirectory() {
    try {
      // TODO: Implement with Tauri v2 API or custom file picker
      console.warn('File browser not yet implemented for Tauri v2');
    } catch (error) {
      console.error('File browser error:', error);
    }
  }

  async function browseLocalFile() {
    try {
      // TODO: Implement with Tauri v2 API or custom file picker
      console.warn('File browser not yet implemented for Tauri v2');
    } catch (error) {
      console.error('File browser error:', error);
    }
  }
</script>

<div class="tab-panel">
  <div class="info-box">💾 Wählen Sie ein lokales Verzeichnis für Ihr Repository</div>

  <div class="browser-toolbar">
    <Button variant="secondary" size="sm" onclick={browseLocalDirectory}>
      📁 Verzeichnis wählen
    </Button>
    {#if mode === 'open'}
      <Button variant="secondary" size="sm" onclick={browseLocalFile}>📄 Datei wählen</Button>
    {/if}
  </div>

  {#if selectedPath}
    <div class="form-group">
      <span class="form-label">Ausgewählter Pfad</span>
      <Input bind:value={selectedPath} placeholder="/pfad/zum/repository" />
    </div>
  {/if}

  {#if mode === 'init'}
    <div class="form-group">
      <span class="form-label">Neuer Ordner-Name (optional)</span>
      <Input bind:value={newFolderName} placeholder="z.B. mein-neues-repo" />
      <div class="form-help">
        Leer lassen um ausgewähltes Verzeichnis zu verwenden, oder Namen eingeben um neuen Ordner zu
        erstellen
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

  .browser-toolbar {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
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
</style>

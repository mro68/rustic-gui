<!-- PathsTab.svelte: Source Paths & Exclusion Patterns -->
<script lang="ts">
  interface PathsTabProps {
    sourcePaths?: string[];
    excludePatterns?: string[];
    oneFileSystem?: boolean;
    errors?: Record<string, string>;
  }

  let {
    sourcePaths = $bindable<string[]>(['']),
    excludePatterns = $bindable<string[]>(['']),
    oneFileSystem = $bindable(true),
    errors = {},
  }: PathsTabProps = $props();

  async function browseDirectory(index: number) {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Quell-Verzeichnis auswählen',
      });

      if (selected && typeof selected === 'string') {
        sourcePaths[index] = selected;
        sourcePaths = [...sourcePaths]; // Trigger reactivity
      }
    } catch (error) {
      console.error('Directory browser error:', error);
    }
  }

  function addPath() {
    sourcePaths = [...sourcePaths, ''];
  }

  function removePath(index: number) {
    if (sourcePaths.length > 1) {
      sourcePaths = sourcePaths.filter((_, i) => i !== index);
    }
  }

  function addExcludePattern() {
    excludePatterns = [...excludePatterns, ''];
  }

  function removeExcludePattern(index: number) {
    if (excludePatterns.length > 1) {
      excludePatterns = excludePatterns.filter((_, i) => i !== index);
    }
  }
</script>

<div class="form-group">
  <div class="form-label">Quell-Pfade</div>
  {#each sourcePaths as _path, index}
    <div class="input-with-button">
      <input
        class="form-input"
        type="text"
        bind:value={sourcePaths[index]}
        placeholder="/home/user/documents"
      />
      <button class="btn-browse" onclick={() => browseDirectory(index)}>📁</button>
      {#if sourcePaths.length > 1}
        <button class="btn-browse" onclick={() => removePath(index)}>✕</button>
      {/if}
    </div>
  {/each}
  <div style="margin-top: 8px;">
    <button class="btn-browse" onclick={addPath}>➕ Weiterer Pfad</button>
  </div>
  <div class="form-help">Einen Pfad pro Zeile. Absolute Pfade verwenden.</div>
  {#if errors.sourcePaths}<div class="error">{errors.sourcePaths}</div>{/if}
</div>

<div class="form-group">
  <div class="form-label">Ausschluss-Muster</div>
  {#each excludePatterns as _pattern, index}
    <div class="input-with-button">
      <input
        class="form-input"
        type="text"
        bind:value={excludePatterns[index]}
        placeholder="*.tmp"
      />
      {#if excludePatterns.length > 1}
        <button class="btn-browse" onclick={() => removeExcludePattern(index)}>✕</button>
      {/if}
    </div>
  {/each}
  <div style="margin-top: 8px;">
    <button class="btn-browse" onclick={addExcludePattern}>➕ Muster hinzufügen</button>
  </div>
  <div class="form-help">Glob-Muster zum Ausschließen von Dateien und Verzeichnissen</div>
</div>

<div class="checkbox-group">
  <input type="checkbox" id="one-file-system" bind:checked={oneFileSystem} />
  <label for="one-file-system"
    >Nur innerhalb desselben Dateisystems bleiben (Mount-Punkte nicht überschreiten)</label
  >
</div>

<style>
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

  .form-input {
    width: 100%;
    padding: 10px 12px;
    background: #1a1d2e;
    border: 1px solid #2d3348;
    border-radius: 6px;
    color: #e4e4e7;
    font-size: 14px;
    font-family: inherit;
  }

  .form-input:focus {
    outline: none;
    border-color: #3b82f6;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
    margin-bottom: 8px;
  }

  .input-with-button .form-input {
    flex: 1;
  }

  .btn-browse {
    padding: 10px 16px;
    background: #2d3348;
    border: 1px solid #3e4457;
    color: #e4e4e7;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    white-space: nowrap;
  }

  .btn-browse:hover {
    background: #3e4457;
  }

  .checkbox-group {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 12px;
  }

  .checkbox-group input[type='checkbox'] {
    width: 18px;
    height: 18px;
    cursor: pointer;
  }

  .checkbox-group label {
    font-size: 14px;
    color: #e4e4e7;
    cursor: pointer;
  }

  .form-help {
    font-size: 12px;
    color: #71717a;
    margin-top: 4px;
  }

  .error {
    color: #ef4444;
    font-size: 12px;
    margin-top: 4px;
  }
</style>

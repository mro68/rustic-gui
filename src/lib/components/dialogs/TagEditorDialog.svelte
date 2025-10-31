<!-- TagEditorDialog.svelte: Manage tags for a snapshot -->
<script lang="ts">
  /**
   * Tag-Editor Dialog für Snapshot-Tags.
   *
   * Ermöglicht das Hinzufügen und Entfernen von Tags für einen Snapshot.
   *
   * @component
   *
   * @example
   * ```svelte
   * <TagEditorDialog
   *   bind:open={showTagEditor}
   *   {snapshotId}
   *   {currentTags}
   *   on:save={handleTagsSaved}
   * />
   * ```
   */
  import { createEventDispatcher } from 'svelte';
  import { addSnapshotTags, removeSnapshotTags } from '$lib/api/snapshots';
  import { toastStore } from '$lib/stores/toast';
  import Button from '$lib/components/shared/Button.svelte';
  import Input from '$lib/components/shared/Input.svelte';

  interface TagEditorDialogProps {
    /** Steuert Sichtbarkeit */
    open?: boolean;
    /** Snapshot-ID */
    snapshotId?: string;
    /** Aktuelle Tags des Snapshots */
    currentTags?: string[];
  }

  let {
    open = $bindable(false),
    snapshotId = '',
    currentTags = [],
  }: TagEditorDialogProps = $props();

  const dispatch = createEventDispatcher<{
    save: { snapshotId: string; tags: string[] };
    cancel: void;
  }>();

  let tags = $state<string[]>([...currentTags]);
  let newTagInput = $state('');
  let isSaving = $state(false);

  // Update tags when currentTags prop changes
  $effect(() => {
    tags = [...currentTags];
  });

  function addTag() {
    const tag = newTagInput.trim();
    if (!tag) {
      toastStore.warning('Bitte einen Tag-Namen eingeben');
      return;
    }
    if (tags.includes(tag)) {
      toastStore.warning('Tag existiert bereits');
      return;
    }
    tags = [...tags, tag];
    newTagInput = '';
  }

  function removeTag(tag: string) {
    tags = tags.filter((t) => t !== tag);
  }

  async function handleSave() {
    if (!snapshotId) {
      toastStore.error('Keine Snapshot-ID angegeben');
      return;
    }

    isSaving = true;
    try {
      // Determine what changed
      const tagsToAdd = tags.filter((t) => !currentTags.includes(t));
      const tagsToRemove = currentTags.filter((t) => !tags.includes(t));

      // Execute API calls
      if (tagsToAdd.length > 0) {
        await addSnapshotTags(snapshotId, tagsToAdd);
      }
      if (tagsToRemove.length > 0) {
        await removeSnapshotTags(snapshotId, tagsToRemove);
      }

      toastStore.success('Tags erfolgreich aktualisiert');
      dispatch('save', { snapshotId, tags });
      handleClose();
    } catch (error: any) {
      console.error('Failed to update tags:', error);
      toastStore.error(`Fehler beim Aktualisieren der Tags: ${error.message || 'Unbekannter Fehler'}`);
    } finally {
      isSaving = false;
    }
  }

  function handleClose() {
    open = false;
    dispatch('cancel');
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      addTag();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={handleClose} role="presentation">
    <div class="modal-dialog" onclick={(e) => e.stopPropagation()}>
      <header class="modal-header">
        <h2>Tags bearbeiten</h2>
        <button class="modal-close" aria-label="Schließen" onclick={handleClose}> &times; </button>
      </header>

      <div class="modal-content">
        <div class="tag-editor">
          <div class="tag-input-group">
            <Input
              bind:value={newTagInput}
              placeholder="Neuen Tag hinzufügen..."
              onkeypress={handleKeyPress}
              disabled={isSaving}
            />
            <Button variant="primary" onclick={addTag} disabled={isSaving || !newTagInput.trim()}>
              + Hinzufügen
            </Button>
          </div>

          <div class="tags-list">
            {#if tags.length === 0}
              <p class="empty-message">Keine Tags vorhanden. Fügen Sie oben einen hinzu.</p>
            {:else}
              <div class="tags-container">
                {#each tags as tag}
                  <div class="tag-chip">
                    <span class="tag-name">{tag}</span>
                    <button
                      class="tag-remove"
                      onclick={() => removeTag(tag)}
                      disabled={isSaving}
                      aria-label={`Tag ${tag} entfernen`}
                    >
                      ×
                    </button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </div>

      <footer class="modal-footer">
        <Button variant="secondary" onclick={handleClose} disabled={isSaving}> Abbrechen </Button>
        <Button variant="primary" onclick={handleSave} disabled={isSaving} loading={isSaving}>
          Speichern
        </Button>
      </footer>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(2px);
  }

  .modal-dialog {
    background: #1a1d2e;
    border-radius: 12px;
    border: 1px solid #2d3348;
    max-width: 600px;
    width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    border-bottom: 1px solid #2d3348;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    color: #e4e4e7;
  }

  .modal-close {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #71717a;
    padding: 0.25rem;
    line-height: 1;
  }

  .modal-close:hover {
    color: #e4e4e7;
  }

  .modal-content {
    padding: 1.5rem;
    overflow-y: auto;
    flex: 1;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-top: 1px solid #2d3348;
  }

  .tag-editor {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .tag-input-group {
    display: flex;
    gap: 0.75rem;
  }

  .tags-list {
    min-height: 100px;
  }

  .empty-message {
    color: #71717a;
    text-align: center;
    padding: 2rem;
    font-size: 0.875rem;
  }

  .tags-container {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag-chip {
    background: rgba(59, 130, 246, 0.15);
    color: #60a5fa;
    padding: 0.5rem 0.75rem;
    border-radius: 12px;
    font-size: 0.875rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border: 1px solid rgba(59, 130, 246, 0.3);
  }

  .tag-name {
    font-weight: 500;
  }

  .tag-remove {
    background: none;
    border: none;
    color: #60a5fa;
    cursor: pointer;
    font-size: 1.25rem;
    line-height: 1;
    padding: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: all 0.2s;
  }

  .tag-remove:hover {
    background: rgba(59, 130, 246, 0.2);
    color: #fff;
  }

  .tag-remove:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>

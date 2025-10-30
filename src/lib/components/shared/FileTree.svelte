<!-- src/lib/components/shared/FileTree.svelte -->
<script lang="ts" module>
  /**
   * FileTree-Komponente für hierarchische Datei-Anzeige.
   *
   * Verwendet für Restore-Browser und Pfad-Auswahl.
   */

  export interface TreeNode {
    path: string;
    name: string;
    isDir: boolean;
    size?: number;
    children?: TreeNode[];
  }
</script>

<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Self from './FileTree.svelte';

  interface Props {
    nodes: TreeNode[];
    selectedPaths?: Set<string>;
    onSelect?: (path: string) => void; // eslint-disable-line no-unused-vars
    onExpand?: (path: string) => Promise<TreeNode[]>; // eslint-disable-line no-unused-vars
  }

  let { nodes, selectedPaths = new Set(), onSelect, onExpand }: Props = $props();

  let expandedPaths = $state(new Set<string>());
  let loadingPaths = $state(new Set<string>());

  const dispatch = createEventDispatcher<{
    select: string;
  }>();

  async function toggleExpand(node: TreeNode) {
    if (!node.isDir) return;

    const isExpanded = expandedPaths.has(node.path);

    if (isExpanded) {
      expandedPaths.delete(node.path);
      expandedPaths = expandedPaths; // eslint-disable-line no-self-assign
    } else {
      // Lazy-load children if needed
      if (!node.children && onExpand) {
        loadingPaths.add(node.path);
        loadingPaths = loadingPaths; // eslint-disable-line no-self-assign

        try {
          node.children = await onExpand(node.path);
        } finally {
          loadingPaths.delete(node.path);
          loadingPaths = loadingPaths; // eslint-disable-line no-self-assign
        }
      }

      expandedPaths.add(node.path);
      expandedPaths = expandedPaths; // eslint-disable-line no-self-assign
    }
  }

  function handleSelect(node: TreeNode) {
    onSelect?.(node.path);
    dispatch('select', node.path);
  }

  // Helper function for formatting bytes
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }
</script>

<ul class="file-tree">
  {#each nodes as node}
    <li>
      <div
        class="tree-node"
        class:selected={selectedPaths.has(node.path)}
        class:directory={node.isDir}
      >
        {#if node.isDir}
          <button class="expand-btn" onclick={() => toggleExpand(node)}>
            {#if loadingPaths.has(node.path)}
              ⟳
            {:else if expandedPaths.has(node.path)}
              ▼
            {:else}
              ▶
            {/if}
          </button>
        {:else}
          <span class="expand-placeholder"></span>
        {/if}

        <button class="node-label" onclick={() => handleSelect(node)}>
          <span class="icon">
            {node.isDir ? '📁' : '📄'}
          </span>
          <span class="name">{node.name}</span>
          {#if node.size !== undefined}
            <span class="size">{formatBytes(node.size)}</span>
          {/if}
        </button>
      </div>

      {#if node.isDir && expandedPaths.has(node.path) && node.children}
        <Self nodes={node.children} {selectedPaths} {onSelect} {onExpand} />
      {/if}
    </li>
  {/each}
</ul>

<style>
  .file-tree {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .tree-node {
    display: flex;
    align-items: center;
    padding: 0.25rem;
    border-radius: 0.25rem;
  }

  .tree-node:hover {
    background: var(--bg-hover);
  }

  .tree-node.selected {
    background: var(--bg-selected);
  }

  .expand-btn {
    background: none;
    border: none;
    cursor: pointer;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .expand-placeholder {
    width: 1.5rem;
  }

  .node-label {
    background: none;
    border: none;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex: 1;
    text-align: left;
    padding: 0.25rem;
  }

  .size {
    margin-left: auto;
    font-size: 0.875rem;
    opacity: 0.7;
  }
</style>

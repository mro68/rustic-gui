<!-- src/lib/components/pages/Snapshots/SnapshotContextMenu.svelte -->
<script lang="ts">
  import ContextMenu from '$lib/components/shared/ContextMenu.svelte';
  import type { SnapshotDto } from '$lib/types';

  type ContextMenuAction =
    | { label: string; icon: string; danger?: boolean; action: () => void }
    | 'divider';

  interface Props {
    visible?: boolean;
    x?: number;
    y?: number;
    snapshot: SnapshotDto | null;
    onClose?: () => void;
    onShowDetails?: (snapshotId: string) => void;
    onCompare?: (snapshot: SnapshotDto) => void;
    onEditTags?: (snapshot: SnapshotDto) => void;
    onRestore?: (snapshot: SnapshotDto) => void;
    onDelete?: (snapshotId: string) => void;
  }

  let {
    visible = $bindable(false),
    x = 0,
    y = 0,
    snapshot,
    onClose,
    onShowDetails,
    onCompare,
    onEditTags,
    onRestore,
    onDelete,
  }: Props = $props();

  function close() {
    visible = false;
    onClose?.();
  }

  const actions = $derived(() => {
    if (!snapshot) return [];

    const menuActions: ContextMenuAction[] = [
      {
        label: 'Details anzeigen',
        icon: 'ℹ️',
        action: () => {
          onShowDetails?.(snapshot.id);
          close();
        },
      },
      {
        label: 'Vergleichen',
        icon: '🔍',
        action: () => {
          onCompare?.(snapshot);
          close();
        },
      },
      {
        label: 'Tags bearbeiten',
        icon: '🏷️',
        action: () => {
          onEditTags?.(snapshot);
          close();
        },
      },
      'divider' as const,
      {
        label: 'Wiederherstellen',
        icon: '📂',
        action: () => {
          onRestore?.(snapshot);
          close();
        },
      },
      'divider' as const,
      {
        label: 'Löschen',
        icon: '🗑️',
        danger: true,
        action: () => {
          onDelete?.(snapshot.id);
          close();
        },
      },
    ];

    return menuActions;
  });
</script>

{#if visible && snapshot}
  <ContextMenu {visible} {x} {y} actions={actions()} />
{/if}

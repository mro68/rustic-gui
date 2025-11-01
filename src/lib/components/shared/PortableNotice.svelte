<script lang="ts">
  import type { PortableStoreStatus } from '$lib/types';

  interface NoticeAction {
    label: string;
    onClick?: () => void;
    href?: string;
  }

  // Props als normale Variablen (test-kompatibel)
  export let status: PortableStoreStatus;
  export let layout: 'banner' | 'modal' = 'banner';
  export let dismissible: boolean = true;
  export let actions: NoticeAction[] = [];
  export let ondismiss: (() => void) | undefined = undefined;

  // Reactive derived values
  $: title = resolveTitle(status);
  $: description = resolveDescription(status);
  $: variant = resolveVariant(status);
  $: metaItems = buildMetaItems(status);

  function handleDismiss() {
    ondismiss?.();
  }

  function handleAction(action: NoticeAction) {
    if (action.onClick) {
      action.onClick();
    }

    if (action.href) {
      window.open(action.href, '_blank', 'noopener');
    }
  }

  function resolveTitle(current: PortableStoreStatus): string {
    if (current.fallback_used) {
      return 'Portabler Speicher im Fallback-Modus';
    }

    if (current.read_only) {
      return 'Portabler Speicher schreibgeschützt';
    }

    return 'Portable Konfiguration aktiv';
  }

  function resolveDescription(current: PortableStoreStatus): string {
    if (current.fallback_used) {
      return `Die Konfiguration wird aktuell unter ${current.effective_dir} gespeichert, weil ${current.portable_dir} nicht beschreibbar war.`;
    }

    if (current.read_only) {
      return `Das portable Verzeichnis ${current.portable_dir} ist schreibgeschützt. Bitte prüfe die Schreibrechte, damit Konfigurationsupdates übernommen werden.`;
    }

    return `Die portable Konfiguration wird unter ${current.effective_dir} bereitgestellt.`;
  }

  function resolveVariant(current: PortableStoreStatus): 'warning' | 'info' | 'success' {
    if (current.fallback_used || current.read_only) {
      return 'warning';
    }

    return current.encrypted ? 'info' : 'success';
  }

  function buildMetaItems(current: PortableStoreStatus): string[] {
    const items = new Set<string>();
    items.add(`Original: ${current.portable_dir}`);
    items.add(`Aktuell: ${current.effective_dir}`);

    if (current.read_only) {
      items.add('Status: Schreibschutz erkannt');
    }

    if (current.fallback_used) {
      items.add('Fallback aktiv');
    }

    if (current.encrypted) {
      items.add('Verschlüsselung aktiv');
    }

    return Array.from(items);
  }
</script>

{#if layout === 'modal'}
  <div class="portable-notice__backdrop">
    <div class="portable-notice__modal" role="dialog" aria-modal="true">
      <header class="portable-notice__modal-header">
        <h2>{title}</h2>
        {#if dismissible}
          <button class="portable-notice__close" onclick={handleDismiss} aria-label="Hinweis schließen">×</button>
        {/if}
      </header>
      <section class="portable-notice__modal-body">
        <div class={`portable-notice portable-notice--${variant}`}>
          <div class="portable-notice__icon">{variant === 'warning' ? '!' : 'ℹ'}</div>
          <div class="portable-notice__content">
            <p class="portable-notice__description">{description}</p>
            {#if metaItems.length > 0}
              <ul class="portable-notice__meta">
                {#each metaItems as meta}
                  <li>{meta}</li>
                {/each}
              </ul>
            {/if}
            {#if actions.length > 0}
              <div class="portable-notice__actions">
                {#each actions as action}
                  <button type="button" class="portable-notice__action" onclick={() => handleAction(action)}>
                    {action.label}
                  </button>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      </section>
    </div>
  </div>
{:else}
  <div class={`portable-notice portable-notice--${variant}`}>
    <div class="portable-notice__icon">{variant === 'warning' ? '!' : variant === 'success' ? '✓' : 'ℹ'}</div>
    <div class="portable-notice__content">
      <h3 class="portable-notice__title">{title}</h3>
      <p class="portable-notice__description">{description}</p>
      {#if metaItems.length > 0}
        <ul class="portable-notice__meta">
          {#each metaItems as meta}
            <li>{meta}</li>
          {/each}
        </ul>
      {/if}
      {#if actions.length > 0}
        <div class="portable-notice__actions">
              {#each actions as action}
                <button type="button" class="portable-notice__action" onclick={() => handleAction(action)}>
              {action.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>
    {#if dismissible}
      <button class="portable-notice__close" onclick={handleDismiss} aria-label="Hinweis schließen">×</button>
    {/if}
  </div>
{/if}

<style>
  :global(:root) {
    font-family: inherit;
  }

  .portable-notice {
    position: relative;
    display: flex;
    gap: 16px;
    align-items: flex-start;
    padding: 20px 24px;
    border-radius: 12px;
    border: 1px solid rgba(59, 130, 246, 0.35);
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.16) 0%, rgba(37, 99, 235, 0.08) 100%);
    color: var(--text-primary, #f4f4f5);
  }

  .portable-notice--warning {
    border-color: rgba(251, 191, 36, 0.55);
    background: linear-gradient(135deg, rgba(251, 191, 36, 0.18) 0%, rgba(251, 191, 36, 0.08) 100%);
  }

  .portable-notice--success {
    border-color: rgba(34, 197, 94, 0.45);
    background: linear-gradient(135deg, rgba(34, 197, 94, 0.18) 0%, rgba(34, 197, 94, 0.08) 100%);
  }

  .portable-notice__icon {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    font-weight: 600;
    background: rgba(59, 130, 246, 0.15);
    color: var(--color-primary, #3b82f6);
    flex-shrink: 0;
  }

  .portable-notice--warning .portable-notice__icon {
    background: rgba(251, 191, 36, 0.15);
    color: #fbbf24;
  }

  .portable-notice--success .portable-notice__icon {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }

  .portable-notice__content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .portable-notice__title {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: inherit;
  }

  .portable-notice__description {
    margin: 0;
    font-size: 14px;
    color: var(--text-secondary, #a1a1aa);
    line-height: 1.45;
  }

  .portable-notice__meta {
    display: flex;
    flex-wrap: wrap;
    gap: 8px 16px;
    margin: 4px 0 0;
    padding: 0;
    list-style: none;
    font-size: 12px;
    color: var(--text-secondary, #a1a1aa);
  }

  .portable-notice__meta li {
    padding: 6px 10px;
    border-radius: 8px;
    border: 1px solid rgba(82, 91, 122, 0.6);
    background: rgba(45, 51, 72, 0.6);
    white-space: nowrap;
  }

  .portable-notice__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    margin-top: 4px;
  }

  .portable-notice__action {
    padding: 8px 14px;
    border-radius: 8px;
    border: 1px solid rgba(59, 130, 246, 0.5);
    background: rgba(59, 130, 246, 0.15);
    color: var(--color-primary, #3b82f6);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s ease, border 0.2s ease;
  }

  .portable-notice__action:hover {
    background: rgba(59, 130, 246, 0.25);
    border-color: rgba(59, 130, 246, 0.7);
  }

  .portable-notice__close {
    border: none;
    background: transparent;
    color: var(--text-secondary, #a1a1aa);
    font-size: 18px;
    cursor: pointer;
    padding: 2px;
  }

  .portable-notice__close:hover {
    color: var(--text-primary, #f4f4f5);
  }

  .portable-notice__backdrop {
    position: fixed;
    inset: 0;
    background: rgba(17, 24, 39, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    backdrop-filter: blur(12px);
    z-index: 1000;
  }

  .portable-notice__modal {
    background: var(--bg-secondary, #22273a);
    border-radius: 14px;
    border: 1px solid var(--border-color, #343a52);
    width: min(94vw, 480px);
    box-shadow: 0 28px 64px rgba(7, 12, 24, 0.45);
    display: flex;
    flex-direction: column;
  }

  .portable-notice__modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 24px 28px 0;
  }

  .portable-notice__modal-header h2 {
    font-size: 18px;
    font-weight: 600;
    margin: 0;
  }

  .portable-notice__modal-body {
    padding: 20px 28px 28px;
  }

  .portable-notice__modal .portable-notice {
    margin-bottom: 16px;
  }
</style>

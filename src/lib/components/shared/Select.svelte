<!-- Select.svelte: Einfache Select-Komponente -->
<script lang="ts">
  /**
   * Select-Dropdown-Komponente mit Label.
   *
   * Unterstützt bidirektionales Binding.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Select bind:value={selectedRepo} label="Repository" required>
   *   <option value="">-- Auswählen --</option>
   *   {#each repositories as repo}
   *     <option value={repo.id}>{repo.name}</option>
   *   {/each}
   * </Select>
   * ```
   */
  let {
    label = '',
    value = $bindable(''),
    required = false,
    disabled = false,
    ariaLabel = '',
    class: className = '',
    children,
    ...rest
  }: {
    label?: string;
    value?: string;
    required?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    class?: string;
    children?: any;
    [key: string]: any;
  } = $props();
</script>

<div class="select-wrapper {className}">
  {#if label}
    <label class="select-label" for={rest.id}>
      {label}
      {#if required}<span class="required">*</span>{/if}
    </label>
  {/if}
  <select
    {required}
    {disabled}
    bind:value
    class="select-field"
    aria-label={ariaLabel || undefined}
    {...rest}
  >
    {@render children?.()}
  </select>
</div>

<style>
  .select-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .select-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .required {
    color: var(--color-error);
  }

  .select-field {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-size: 0.875rem;
    cursor: pointer;
  }

  /* Removed unused .select-field option - browser controls native option styling */

  .select-field:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .select-field:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>

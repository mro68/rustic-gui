<!-- Input.svelte: Einfache Input-Komponente -->
<script lang="ts">
  /**
   * Universelle Input-Komponente mit Label und Validation.
   *
   * Unterstützt alle HTML-Input-Typen mit bidirektionalem Binding.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Input
   *   bind:value={name}
   *   label="Repository-Name"
   *   type="text"
   *   required
   *   placeholder="Mein Backup"
   * />
   * ```
   */
  let {
    label = '',
    type = 'text',
    value = $bindable(''),
    placeholder = '',
    required = false,
    disabled = false,
    ariaLabel = '',
    class: className = '',
    ...rest
  }: {
    label?: string;
    type?: string;
    value?: string | number;
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    class?: string;
    [key: string]: any;
  } = $props();
</script>

<div class="input-wrapper {className}">
  {#if label}
    <label class="input-label" for={rest.id}>
      {label}
      {#if required}<span class="required">*</span>{/if}
    </label>
  {/if}
  <input
    {type}
    {placeholder}
    {required}
    {disabled}
    bind:value
    class="input-field"
    aria-label={ariaLabel || undefined}
    {...rest}
  />
</div>

<style>
  .input-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .input-label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .required {
    color: var(--color-error);
  }

  .input-field {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.375rem;
    color: var(--text-primary);
    font-size: 0.875rem;
  }

  .input-field:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .input-field:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>

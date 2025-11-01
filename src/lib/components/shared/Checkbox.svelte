<!-- Checkbox.svelte: Einfache Checkbox-Komponente -->
<script lang="ts">
  /**
   * Checkbox-Komponente mit Label und bidirektionalem Binding.
   *
   * @component
   *
   * @example
   * ```svelte
   * <Checkbox
   *   bind:checked={rememberPassword}
   *   label="Passwort speichern"
   * />
   * ```
   */
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let {
    label = '',
    checked = $bindable(false),
    disabled = false,
    ariaLabel = '',
    class: className = '',
    ...rest
  }: {
    label?: string;
    checked?: boolean;
    disabled?: boolean;
    ariaLabel?: string;
    class?: string;
    [key: string]: any;
  } = $props();

  function handleChange(e: Event) {
    checked = (e.target as HTMLInputElement).checked;
    dispatch('change', checked);
  }
</script>

<label class="checkbox-wrapper {className}">
  <input
    type="checkbox"
    {disabled}
    {checked}
    onchange={handleChange}
    class="checkbox-input"
    aria-label={ariaLabel || undefined}
    {...rest}
  />
  <span class="checkbox-checkmark"></span>
  {#if label}
    <span class="checkbox-label">{label}</span>
  {/if}
</label>

<style>
  .checkbox-wrapper {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    user-select: none;
  }

  .checkbox-wrapper:has(.checkbox-input:disabled) {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .checkbox-input {
    position: absolute;
    opacity: 0;
    cursor: pointer;
  }

  .checkbox-checkmark {
    width: 1.125rem;
    height: 1.125rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 0.25rem;
    position: relative;
    transition: all 0.2s ease;
  }

  .checkbox-wrapper:hover .checkbox-checkmark {
    border-color: var(--color-primary);
  }

  .checkbox-input:checked + .checkbox-checkmark {
    background: var(--color-primary);
    border-color: var(--color-primary);
  }

  .checkbox-input:checked + .checkbox-checkmark::after {
    content: '';
    position: absolute;
    left: 0.3rem;
    top: 0.1rem;
    width: 0.3rem;
    height: 0.6rem;
    border: solid white;
    border-width: 0 2px 2px 0;
    transform: rotate(45deg);
  }

  .checkbox-input:focus + .checkbox-checkmark {
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .checkbox-label {
    font-size: 0.875rem;
    color: var(--text-primary);
    cursor: pointer;
  }
</style>

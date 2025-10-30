<!-- Checkbox.svelte: Einfache Checkbox-Komponente -->
<script lang="ts">
  let {
    label = '',
    checked = $bindable(false),
    disabled = false,
    class: className = '',
    ...rest
  }: {
    label?: string;
    checked?: boolean;
    disabled?: boolean;
    class?: string;
    [key: string]: any;
  } = $props();
</script>

<label class="checkbox-wrapper {className}">
  <input type="checkbox" {disabled} bind:checked class="checkbox-input" {...rest} />
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

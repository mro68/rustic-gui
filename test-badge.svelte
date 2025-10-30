<script>
  import Badge from '../src/lib/components/shared/Badge.svelte';
</script>

<div style="padding: 20px; background: #1a1d2e; min-height: 100vh;">
  <h1 style="color: #e4e4e7; margin-bottom: 20px;">Badge Component Test</h1>

  <div style="display: flex; flex-direction: column; gap: 16px;">
    <Badge variant="healthy" text="Healthy" />
    <Badge variant="warning" text="Attention" />
    <Badge variant="error" text="Error" />
    <Badge variant="info" text="Running..." />

    <Badge variant="healthy" text="Last: 2h ago" icon="⏰" />
    <Badge variant="warning" text="Needs Check" icon="⚠" />
    <Badge variant="error" text="Failed" icon="❌" />
  </div>
</div>

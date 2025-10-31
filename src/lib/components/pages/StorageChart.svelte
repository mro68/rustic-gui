<!--
  StorageChart.svelte
  ------------------
  Donut-Chart für Storage Usage (gemäß Mockup)
-->

<script lang="ts">
  /**
   * Donut-Chart für Speicher-Nutzung.
   *
   * Visualisiert Used/Total Space als SVG-Donut.
   *
   * @component
   *
   * @example
   * ```svelte
   * <StorageChart
   *   totalSpace={500}
   *   usedSpace={350}
   *   label="Lokaler Speicher"
   *   sublabel="70% belegt"
   * />
   * ```
   */

  interface StorageChartProps {
    /** Gesamtspeicher in GB */
    totalSpace?: number;
    /** Genutzter Speicher in GB */
    usedSpace?: number;
    /** Haupt-Label */
    label?: string;
    /** Sub-Label */
    sublabel?: string;
  }

  let { totalSpace = 0, usedSpace = 0, label = '', sublabel = '' }: StorageChartProps = $props();

  // Berechne Prozentsatz für SVG
  $: percentage = totalSpace > 0 ? (usedSpace / totalSpace) * 100 : 0;
  $: strokeDasharray = `${(percentage / 100) * 283} 283`; // 283 = 2 * π * 45 (Radius 45)
</script>

<div class="donut-chart">
  <svg width="140" height="140" viewBox="0 0 140 140">
    <!-- Hintergrund-Kreis -->
    <circle cx="70" cy="70" r="60" fill="none" stroke="#2d3348" stroke-width="12"></circle>
    <!-- Fortschritts-Kreis -->
    <circle
      cx="70"
      cy="70"
      r="60"
      fill="none"
      stroke="#3b82f6"
      stroke-width="12"
      stroke-dasharray={strokeDasharray}
      stroke-dashoffset="0"
      transform="rotate(-90 70 70)"
      stroke-linecap="round"
    ></circle>
    <!-- Zentraler Text -->
    <text x="70" y="65" text-anchor="middle" font-size="28" font-weight="700" fill="#f4f4f5">
      {totalSpace}
    </text>
    <text x="70" y="85" text-anchor="middle" font-size="14" fill="#71717a"> GB </text>
  </svg>
  <div class="donut-label">{label}</div>
  <div class="donut-sublabel">{sublabel}</div>
</div>

<style>
  .donut-chart {
    position: relative;
    width: 140px;
    height: 140px;
    text-align: center;
  }

  .donut-label {
    text-align: center;
    margin-top: 12px;
    font-size: 13px;
    color: #a1a1aa;
  }

  .donut-sublabel {
    font-size: 12px;
    color: #71717a;
    margin-top: 2px;
  }
</style>

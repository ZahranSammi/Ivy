<script setup lang="ts">
withDefaults(
  defineProps<{
    target?: string;
    percent?: number;
    newAssets?: number;
    findingsThisRun?: number;
  }>(),
  {
    target: "staging.acme.com",
    percent: 68,
    newAssets: 14,
    findingsThisRun: 3,
  },
);
</script>

<template>
  <div class="w-[236px] rounded-[10px] border border-ivy-border bg-ivy-panel px-3.5 py-3">
    <div class="mb-2 flex items-center justify-between">
      <span class="font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-muted"
        >Active scan</span
      >
      <span class="font-mono text-[11px] text-ivy-network">{{ percent }}%</span>
    </div>
    <div class="mb-2.5 font-mono text-xs text-ivy-text">{{ target }}</div>
    <div class="h-[5px] overflow-hidden rounded-full bg-ivy-border">
      <div
        class="h-full rounded-full bg-ivy-network shadow-[0_0_10px_rgba(31,215,181,0.5)] transition-[width] duration-300 ease-linear"
        :style="{ width: `${percent}%` }"
      />
    </div>
    <div class="mt-[9px] font-sans text-[11px] text-ivy-faint">
      {{ newAssets }} new assets · {{ findingsThisRun }} findings this run
    </div>
  </div>
</template>

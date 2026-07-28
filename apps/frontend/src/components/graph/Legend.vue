<script setup lang="ts">
const items = [
  { label: "Domain", color: "#3D8BFF" },
  { label: "Network", color: "#1FD7B5" },
  { label: "Service", color: "#9B7BFF" },
  { label: "Vulnerability", color: "#FF4D6A" },
  { label: "Other asset", color: "#8A93A8" },
];
</script>

<template>
  <div
    class="absolute bottom-[18px] left-[18px] rounded-[10px] border border-ivy-border bg-ivy-bg/72 px-3.5 py-3 backdrop-blur-[6px]"
  >
    <div class="mb-[9px] font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-faint">
      Legend
    </div>
    <div class="grid grid-cols-2 gap-x-[18px] gap-y-[7px]">
      <div v-for="item in items" :key="item.label" class="flex items-center gap-2">
        <span
          class="h-[9px] w-[9px] flex-none rounded-full"
          :style="{ background: item.color, boxShadow: `0 0 6px ${item.color}88` }"
        />
        <span class="font-sans text-xs text-ivy-muted">{{ item.label }}</span>
      </div>
    </div>
  </div>
</template>

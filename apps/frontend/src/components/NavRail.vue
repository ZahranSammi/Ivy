<script setup lang="ts">
const items = [
  { label: "Graph", icon: "◈", active: true },
  { label: "Findings", icon: "△", active: false },
  { label: "Assets", icon: "▦", active: false },
  { label: "Term", icon: "▸_", active: false },
  { label: "Audit", icon: "≡", active: false },
  { label: "Config", icon: "◉", active: false },
];
</script>

<template>
  <aside
    class="flex w-16 flex-none flex-col items-center gap-1 border-r border-ivy-border bg-ivy-bg py-3"
  >
    <div
      v-for="item in items"
      :key="item.label"
      :title="item.label"
      class="relative flex w-full flex-col items-center gap-[3px] rounded-[10px] py-[9px]"
      :class="item.active ? 'bg-ivy-panel text-ivy-text' : 'text-ivy-faint'"
    >
      <div
        class="absolute left-0 top-1/2 w-[3px] -translate-y-1/2 rounded-r-[3px] bg-ivy-domain transition-[height]"
        :style="{ height: item.active ? '18px' : '0px' }"
      />
      <div class="flex h-5 items-center justify-center">{{ item.icon }}</div>
      <span class="font-sans text-[9px] font-medium">{{ item.label }}</span>
    </div>
  </aside>
</template>

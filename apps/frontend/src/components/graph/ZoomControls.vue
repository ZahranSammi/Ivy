<script setup lang="ts">
import { inject } from "vue";

const transform = inject<{ zoomIn: () => void; zoomOut: () => void; fit: () => void }>("graphTransform");
</script>

<template>
  <div class="flex gap-1.5">
    <button
      type="button"
      class="h-[34px] w-[34px] rounded-lg border border-ivy-border bg-ivy-panel text-base text-ivy-muted transition-colors hover:border-ivy-border-hover hover:text-ivy-text"
      @click="transform?.zoomIn()"
    >
      +
    </button>
    <button
      type="button"
      class="h-[34px] w-[34px] rounded-lg border border-ivy-border bg-ivy-panel text-base text-ivy-muted transition-colors hover:border-ivy-border-hover hover:text-ivy-text"
      @click="transform?.zoomOut()"
    >
      -
    </button>
    <button
      type="button"
      class="h-[34px] rounded-lg border border-ivy-border bg-ivy-panel px-3 font-sans text-xs text-ivy-muted transition-colors hover:border-ivy-border-hover hover:text-ivy-text"
      @click="transform?.fit()"
    >
      Fit
    </button>
  </div>
</template>


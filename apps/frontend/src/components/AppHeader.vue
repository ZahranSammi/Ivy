<script setup lang="ts"></script>

<template>
  <header
    class="flex h-14 flex-none items-center gap-5 border-b border-ivy-border bg-ivy-bg px-4"
  >
    <div class="flex items-center gap-2.5">
      <div
        class="flex h-[26px] w-[26px] items-center justify-center rounded-[7px] bg-gradient-to-br from-ivy-domain to-ivy-network font-display text-[15px] font-semibold text-[#05060A]"
      >
        I
      </div>
      <span class="font-display text-base font-semibold tracking-wide text-ivy-text">IVY</span>
    </div>

    <div class="h-[22px] w-px bg-ivy-border" />

    <button
      class="flex items-center gap-2 rounded-lg border border-ivy-border bg-ivy-panel px-2.5 py-1.5 transition-colors hover:border-ivy-border-hover"
      type="button"
    >
      <span class="font-sans text-[13px] font-medium text-ivy-text">acme.com</span>
      <span class="font-sans text-[11px] text-ivy-faint">External ASM</span>
      <span class="text-[10px] text-ivy-faint">▾</span>
    </button>

    <div class="flex flex-1 justify-center">
      <div
        class="flex w-[340px] items-center gap-2 rounded-lg border border-ivy-border bg-ivy-panel px-3 py-[7px]"
      >
        <span class="text-[13px] text-ivy-faint">⌕</span>
        <input
          placeholder="Search assets, IPs, findings…"
          class="flex-1 bg-transparent font-sans text-[13px] text-ivy-text outline-none placeholder:text-ivy-faint"
        />
        <span
          class="rounded border border-ivy-border-hover px-[5px] py-px font-mono text-[10px] text-ivy-faint"
          >⌘K</span
        >
      </div>
    </div>

    <div
      class="flex items-center gap-2.5 rounded-full border border-ivy-border bg-ivy-panel px-3 py-[5px]"
    >
      <span class="h-[7px] w-[7px] animate-ivy-blink rounded-full bg-ivy-medium" />
      <span class="font-sans text-xs font-medium text-ivy-muted">Scan running</span>
    </div>

    <button
      class="flex items-center gap-2 rounded-lg bg-ivy-critical px-3.5 py-2 transition-[box-shadow,background] hover:bg-[#FF6079] hover:shadow-[0_0_12px_rgba(255,77,106,0.55)]"
      type="button"
    >
      <span class="inline-block h-3 w-3 rounded-[3px] bg-white" />
      <span class="font-sans text-[13px] font-semibold text-white">Kill Switch</span>
    </button>
  </header>
</template>

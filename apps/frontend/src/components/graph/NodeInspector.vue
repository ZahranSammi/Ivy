<script setup lang="ts">
import { ref } from "vue";
import type { InspectorTab, NodeSelection } from "@/types/graph";

const props = defineProps<{
  selection: NodeSelection;
  activeTab: InspectorTab;
}>();

const emit = defineEmits<{
  "update:activeTab": [tab: InspectorTab];
  "select-connection": [id: string];
}>();

const tabs: { key: InspectorTab; label: string }[] = [
  { key: "overview", label: "Overview" },
  { key: "findings", label: "Findings" },
  { key: "connections", label: "Connections" },
];

const isHidden = ref(false);
const panelWidth = ref(376);
let startX = 0;
let startW = 0;

function copyValue() {
  navigator.clipboard?.writeText(props.selection.node.value);
}

function startResize(e: MouseEvent) {
  startX = e.clientX;
  startW = panelWidth.value;
  document.body.style.cursor = 'col-resize';
  const onMove = (evt: MouseEvent) => {
    // calculate diff (moving left increases width)
    const newW = startW + (startX - evt.clientX);
    panelWidth.value = Math.max(250, Math.min(newW, 800));
  };
  const onUp = () => {
    document.body.style.cursor = '';
    window.removeEventListener('mousemove', onMove);
    window.removeEventListener('mouseup', onUp);
  };
  window.addEventListener('mousemove', onMove);
  window.addEventListener('mouseup', onUp);
}
</script>

<template>
  <button 
    class="absolute right-[18px] top-[18px] z-20 rounded-md border border-ivy-border-light bg-ivy-panel-light px-3 py-1.5 text-xs text-ivy-text-light hover:bg-ivy-surface-light"
    @click="isHidden = !isHidden"
  >
    {{ isHidden ? 'Show Details' : 'Hide' }}
  </button>
  
  <aside
    v-show="!isHidden"
    class="ivy-scroll absolute bottom-[18px] right-[18px] top-[48px] overflow-y-auto rounded-2xl border border-ivy-border-light bg-ivy-panel-light text-ivy-text-light shadow-[0_8px_24px_rgba(0,0,0,0.35)]"
    :style="{ width: `${panelWidth}px` }"
  >
    <div 
      class="fixed bottom-[18px] top-[48px] w-2 cursor-col-resize hover:bg-ivy-domain/30 z-50" 
      :style="{ right: `${panelWidth + 18 - 4}px` }"
      @mousedown.prevent="startResize"
    ></div>

    <div class="px-5 pt-5">
      <div class="mb-3.5 flex items-center justify-between">
        <span
          class="inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 font-sans text-[11px] font-semibold uppercase tracking-[0.03em]"
          :style="{
            background: `${selection.accentColor}1A`,
            color: selection.accentText,
            border: `1px solid ${selection.accentColor}40`,
          }"
        >
          <span
            class="h-[7px] w-[7px] rounded-full"
            :style="{ background: selection.accentColor }"
          />
          {{ selection.typeLabel }}
        </span>
        <span class="inline-flex items-center gap-1.5 font-sans text-xs text-ivy-faint">
          <span
            class="h-[7px] w-[7px] rounded-full"
            :style="{ background: selection.statusDotColor }"
          />
          {{ selection.node.status }}
        </span>
      </div>

      <h2 class="m-0 break-words font-display text-2xl font-semibold leading-[30px] text-ivy-text-light">
        {{ selection.node.label }}
      </h2>

      <div
        class="mt-2.5 flex items-center gap-2 rounded-lg border border-ivy-border-light bg-ivy-surface-light px-[11px] py-2"
      >
        <span class="flex-1 break-all font-mono text-[13px] text-ivy-text-light">{{
          selection.node.value
        }}</span>
        <button
          type="button"
          class="flex-none rounded-md border border-ivy-line-light bg-white px-2 py-[3px] font-sans text-[11px] font-medium text-ivy-faint transition-colors hover:border-ivy-domain hover:text-ivy-domain"
          @click="copyValue"
        >
          Copy
        </button>
      </div>

      <div class="mt-4 grid grid-cols-2 gap-3.5">
        <div>
          <div class="mb-[3px] font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-muted">
            Source tool
          </div>
          <div class="font-sans text-[13px] text-ivy-text-light">{{ selection.node.tool }}</div>
        </div>
        <div>
          <div class="mb-[3px] font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-muted">
            First seen
          </div>
          <div class="font-sans text-[13px] text-ivy-text-light">{{ selection.node.seen }}</div>
        </div>
        <div>
          <div class="mb-[3px] font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-muted">
            Connections
          </div>
          <div class="font-mono text-[13px] text-ivy-text-light">{{ selection.connections.length }}</div>
        </div>
        <div>
          <div class="mb-[3px] font-sans text-[11px] font-medium uppercase tracking-[0.04em] text-ivy-muted">
            Findings
          </div>
          <div class="font-mono text-[13px] text-ivy-text-light">{{ selection.findings.length }}</div>
        </div>
      </div>
    </div>

    <div class="mx-5 mt-[18px] flex gap-0.5 border-b border-ivy-border-light">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        type="button"
        class="-mb-px px-3 py-[9px] font-sans text-[13px] font-medium"
        :class="
          activeTab === tab.key
            ? 'border-b-2 border-ivy-domain text-ivy-text-light'
            : 'border-b-2 border-transparent text-ivy-muted'
        "
        @click="emit('update:activeTab', tab.key)"
      >
        {{ tab.label }}
      </button>
    </div>

    <div class="px-5 pb-[22px] pt-4">
      <template v-if="activeTab === 'overview'">
        <p class="m-0 mb-4 text-pretty font-sans text-sm leading-[22px] text-ivy-text-light">
          {{ selection.node.desc }}
        </p>
        <div class="flex flex-col gap-px overflow-hidden rounded-[10px] border border-ivy-border-light">
          <div
            v-for="fact in selection.facts"
            :key="fact.k"
            class="flex justify-between gap-3.5 bg-white px-3 py-2.5"
          >
            <span class="font-sans text-[13px] text-ivy-faint">{{ fact.k }}</span>
            <span class="text-right font-mono text-[13px] text-ivy-text-light">{{ fact.v }}</span>
          </div>
        </div>
      </template>

      <template v-else-if="activeTab === 'findings'">
        <div v-if="selection.findings.length > 0" class="flex flex-col gap-2.5">
          <div
            v-for="finding in selection.findings"
            :key="finding.name"
            class="rounded-[10px] border border-ivy-border-light px-[13px] py-3"
          >
            <div class="mb-1.5 flex items-center gap-2">
              <span
                class="rounded-full px-2 py-0.5 font-sans text-[10px] font-semibold uppercase tracking-[0.04em]"
                :style="{
                  background: `${finding.color}22`,
                  color: finding.colorText,
                  border: `1px solid ${finding.color}55`,
                }"
                >{{ finding.sevLabel }}</span
              >
              <span class="ml-auto font-mono text-[11px] text-ivy-faint">CVSS {{ finding.cvss }}</span>
            </div>
            <div class="mb-[3px] font-sans text-sm font-medium text-ivy-text-light">{{ finding.name }}</div>
            <div class="font-mono text-xs text-ivy-muted">{{ finding.cve }}</div>
          </div>
        </div>
        <div v-else class="px-2.5 py-7 text-center font-sans text-[13px] text-ivy-muted">
          No findings on this asset.
        </div>
      </template>

      <template v-else>
        <div class="flex flex-col gap-px overflow-hidden rounded-[10px] border border-ivy-border-light">
          <button
            v-for="conn in selection.connections"
            :key="conn.id"
            type="button"
            class="flex items-center gap-2.5 bg-white px-3 py-2.5 text-left transition-colors hover:bg-ivy-surface-light"
            @click="emit('select-connection', conn.id)"
          >
            <span
              class="h-[9px] w-[9px] flex-none rounded-full"
              :style="{ background: conn.color, boxShadow: `0 0 6px ${conn.color}66` }"
            />
            <span class="flex-1 font-mono text-[13px] text-ivy-text-light">{{ conn.label }}</span>
            <span class="font-sans text-[11px] text-ivy-muted">{{ conn.typeLabel }}</span>
          </button>
        </div>
      </template>
    </div>

    <div class="sticky bottom-0 flex gap-2 border-t border-ivy-border-light bg-white px-5 py-3.5">
      <button
        type="button"
        class="flex-1 rounded-lg bg-ivy-domain px-2.5 py-2.5 font-sans text-[13px] font-semibold text-white transition-[background,box-shadow] hover:bg-[#5C9DFF] hover:shadow-[0_0_12px_rgba(61,139,255,0.45)]"
      >
        Expand node
      </button>
      <button
        type="button"
        class="flex-none rounded-lg border border-ivy-line-light bg-white px-3.5 py-2.5 font-sans text-[13px] font-medium text-ivy-text-light transition-colors hover:border-ivy-muted"
      >
        Add to report
      </button>
    </div>
  </aside>
</template>

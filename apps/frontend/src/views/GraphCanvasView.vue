<script setup lang="ts">
import { computed, ref, provide } from "vue";
import AppHeader from "@/components/AppHeader.vue";
import NavRail from "@/components/NavRail.vue";
import GraphSvg from "@/components/graph/GraphSvg.vue";
import Legend from "@/components/graph/Legend.vue";
import ScanProgressCard from "@/components/graph/ScanProgressCard.vue";
import ZoomControls from "@/components/graph/ZoomControls.vue";
import NodeInspector from "@/components/graph/NodeInspector.vue";
import { EDGES, NODES, SEVERITY, accentFor } from "@/data/graph-data";
import type { GraphNode, InspectorTab, NodeSelection } from "@/types/graph";
import { useElementBounding } from '@vueuse/core';

const selectedId = ref("staging");
const hoveredId = ref<string | null>(null);
const activeTab = ref<InspectorTab>("overview");

// Canvas transformations
const scale = ref(1);
const translateX = ref(0);
const translateY = ref(0);

const graphContainer = ref<HTMLElement | null>(null);
const { width, height } = useElementBounding(graphContainer);

// Provide transform state to children
provide("graphTransform", {
  scale, translateX, translateY,
  zoomIn: () => { scale.value = Math.min(scale.value * 1.2, 5); },
  zoomOut: () => { scale.value = Math.max(scale.value / 1.2, 0.1); },
  fit: () => {
    scale.value = 1;
    translateX.value = 0;
    translateY.value = 0;
  }
});

let isDragging = false;
let startX = 0;
let startY = 0;
let currentTx = 0;
let currentTy = 0;

function startDrag(e: MouseEvent) {
  if (e.target instanceof Element && e.target.tagName !== 'svg' && e.target.tagName !== 'div') return;
  isDragging = true;
  startX = e.clientX;
  startY = e.clientY;
  currentTx = translateX.value;
  currentTy = translateY.value;
  document.body.style.cursor = 'grabbing';
}

function onDrag(e: MouseEvent) {
  if (!isDragging) return;
  translateX.value = currentTx + (e.clientX - startX);
  translateY.value = currentTy + (e.clientY - startY);
}

function endDrag() {
  if (!isDragging) return;
  isDragging = false;
  document.body.style.cursor = '';
}

function select(id: string) {
  selectedId.value = id;
  activeTab.value = "overview";
}

const selectedNode = computed<GraphNode>(
  () => NODES.find((n) => n.id === selectedId.value) ?? NODES[0],
);

const connections = computed(() => {
  const id = selectedNode.value.id;
  return EDGES.filter(([from, to]) => from === id || to === id)
    .map(([from, to]) => (to === id ? from : to))
    .map((neighborId) => NODES.find((n) => n.id === neighborId))
    .filter((n): n is GraphNode => !!n)
    .map((n) => ({ id: n.id, label: n.label, typeLabel: accentFor(n).l, color: accentFor(n).c }));
});

const findings = computed(() =>
  (selectedNode.value.findings ?? []).map((f) => {
    const sev = SEVERITY[f.sev];
    return {
      name: f.name,
      cve: f.cve ?? "No CVE assigned",
      sevLabel: sev.l,
      cvss: f.cvss || "-",
      color: sev.c,
      colorText: sev.t,
    };
  }),
);

const facts = computed(() => {
  const n = selectedNode.value;
  const accent = accentFor(n);
  return [
    { k: "Asset type", v: n.type === "vuln" ? "Vulnerability" : accent.l },
    { k: "Status", v: n.status },
    { k: "Source tool", v: n.tool },
    { k: "Last confirmed", v: n.seen },
  ];
});

const selection = computed<NodeSelection>(() => {
  const n = selectedNode.value;
  const accent = accentFor(n);
  return {
    node: n,
    typeLabel: n.type === "vuln" ? "Vulnerability" : accent.l,
    accentColor: accent.c,
    accentText: accent.t,
    statusDotColor: n.active ? "#FFC93D" : "#1FD7B5",
    connections: connections.value,
    findings: findings.value,
    facts: facts.value,
  };
});
</script>

<template>
  <div class="flex h-screen flex-col overflow-hidden bg-ivy-bg text-ivy-text">
    <AppHeader />

    <div class="flex min-h-0 flex-1">
      <NavRail />

      <main
        class="relative min-w-0 flex-1 overflow-hidden"
        style="background: radial-gradient(circle at 48% 42%, #0b0f18 0%, #05060a 68%)"
      >
        <div 
          ref="graphContainer"
          class="absolute inset-0 cursor-grab active:cursor-grabbing"
          @mousedown="startDrag"
          @mousemove="onDrag"
          @mouseup="endDrag"
          @mouseleave="endDrag"
        >
          <div :style="{ transform: `translate(${translateX}px, ${translateY}px) scale(${scale})`, transformOrigin: 'center center', width: '100%', height: '100%', transition: isDragging ? 'none' : 'transform 0.2s ease-out' }">
            <GraphSvg
              :nodes="NODES"
              :edges="EDGES"
              :selected-id="selectedId"
              :hovered-id="hoveredId"
              @select="select"
              @hover="(id) => (hoveredId = id)"
            />
          </div>
        </div>

        <div class="absolute left-[18px] top-[18px] flex flex-col gap-2.5 pointer-events-none">
          <ScanProgressCard class="pointer-events-auto" />
          <ZoomControls class="pointer-events-auto" />
        </div>

        <Legend />

        <NodeInspector
          :selection="selection"
          :active-tab="activeTab"
          @update:active-tab="(tab) => (activeTab = tab)"
          @select-connection="select"
        />
      </main>
    </div>
  </div>
</template>

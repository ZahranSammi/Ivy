<script setup lang="ts">
import { computed } from "vue";
import { accentFor } from "@/data/graph-data";
import type { GraphEdge, GraphNode } from "@/types/graph";

const props = withDefaults(
  defineProps<{
    nodes: GraphNode[];
    edges: GraphEdge[];
    selectedId: string | null;
    hoveredId: string | null;
    glow?: number;
    showLabels?: boolean;
    pulseActive?: boolean;
  }>(),
  {
    glow: 0.6,
    showLabels: true,
    pulseActive: true,
  },
);

const emit = defineEmits<{
  select: [id: string];
  hover: [id: string | null];
}>();

const nodesById = computed(() => {
  const map = new Map<string, GraphNode>();
  for (const n of props.nodes) map.set(n.id, n);
  return map;
});

const neighbors = computed(() => {
  const map = new Map<string, Set<string>>();
  for (const n of props.nodes) map.set(n.id, new Set());
  for (const [a, b] of props.edges) {
    map.get(a)?.add(b);
    map.get(b)?.add(a);
  }
  return map;
});

const lines = computed(() =>
  props.edges
    .map(([from, to, active], i) => {
      const a = nodesById.value.get(from);
      const b = nodesById.value.get(to);
      if (!a || !b) return null;

      const hovered = props.hoveredId;
      const related = !!hovered && (from === hovered || to === hovered);

      let stroke = "#2A3145";
      let width = 1.2;
      let opacity = hovered ? (related ? 0.95 : 0.1) : 0.5;
      let filter = "none";

      if (active) {
        stroke = "#1FD7B5";
        width = 1.7;
        opacity = hovered ? (related ? 1 : 0.14) : 0.85;
        filter = "drop-shadow(0 0 4px rgba(31,215,181,0.5))";
      }
      if (related) {
        stroke = active ? "#1FD7B5" : "#3D8BFF";
        width = active ? 1.9 : 1.6;
        opacity = 1;
        filter = active
          ? "drop-shadow(0 0 5px rgba(31,215,181,0.6))"
          : "drop-shadow(0 0 4px rgba(61,139,255,0.5))";
      }

      return {
        key: `e${i}`,
        x1: a.x,
        y1: a.y,
        x2: b.x,
        y2: b.y,
        stroke,
        width,
        opacity,
        filter,
      };
    })
    .filter((l): l is NonNullable<typeof l> => l !== null),
);

const renderNodes = computed(() =>
  props.nodes.map((n) => {
    const color = accentFor(n).c;
    const isSelected = n.id === props.selectedId;
    const isHovered = n.id === props.hoveredId;
    const dimmed = !!props.hoveredId && !isHovered && !neighbors.value.get(props.hoveredId)?.has(n.id);
    const glowPx = Math.round((isSelected ? 15 : 9) * props.glow);
    return {
      node: n,
      color,
      isSelected,
      isHovered,
      dimmed,
      glowFilter: `drop-shadow(0 0 ${glowPx}px ${color})`,
      labelFill: isSelected || isHovered ? "#F1F3F9" : "#8A93A8",
    };
  }),
);

function select(id: string) {
  emit("select", id);
}
function onEnter(id: string) {
  emit("hover", id);
}
function onLeave() {
  emit("hover", null);
}
function onKeydown(event: KeyboardEvent, id: string) {
  if (event.key === "Enter" || event.key === " ") {
    event.preventDefault();
    select(id);
  }
}
</script>

<template>
  <svg viewBox="0 0 1080 760" preserveAspectRatio="xMidYMid meet" class="block h-full w-full">
    <line
      v-for="line in lines"
      :key="line.key"
      :x1="line.x1"
      :y1="line.y1"
      :x2="line.x2"
      :y2="line.y2"
      :stroke="line.stroke"
      :stroke-width="line.width"
      stroke-linecap="round"
      :style="{ opacity: line.opacity, filter: line.filter, transition: 'opacity .18s, filter .18s' }"
    />

    <g
      v-for="entry in renderNodes"
      :key="entry.node.id"
      role="button"
      tabindex="0"
      :aria-label="entry.node.label"
      class="cursor-pointer"
      :style="{ opacity: entry.dimmed ? 0.2 : 1, transition: 'opacity .18s' }"
      @click="select(entry.node.id)"
      @mouseenter="onEnter(entry.node.id)"
      @mouseleave="onLeave"
      @keydown="onKeydown($event, entry.node.id)"
    >
      <circle
        v-if="entry.node.active && pulseActive"
        :cx="entry.node.x"
        :cy="entry.node.y"
        :r="entry.node.r + 7"
        :fill="entry.color"
        class="animate-ivy-pulse"
        style="opacity: 0.18"
      />
      <circle
        v-if="entry.isSelected"
        :cx="entry.node.x"
        :cy="entry.node.y"
        :r="entry.node.r + 6"
        fill="none"
        :stroke="entry.color"
        stroke-width="1.5"
        style="opacity: 0.9"
      />
      <circle
        :cx="entry.node.x"
        :cy="entry.node.y"
        :r="entry.node.r"
        :fill="entry.color"
        stroke="#05060A"
        stroke-width="2.5"
        :style="{ filter: entry.glowFilter }"
      />
      <text
        v-if="showLabels"
        :x="entry.node.x"
        :y="entry.node.y + entry.node.r + 15"
        text-anchor="middle"
        class="pointer-events-none font-mono"
        :style="{
          fontSize: '10.5px',
          fontWeight: entry.isSelected ? 500 : 400,
          fill: entry.labelFill,
          transition: 'fill .15s',
        }"
      >
        {{ entry.node.label }}
      </text>
    </g>
  </svg>
</template>

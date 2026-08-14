<script setup lang="ts">
/**
 * The legend.
 *
 * A distribution board keeps its schedule card inside the door, because a
 * symbol nobody can decode is worse than no symbol. Kit's four link states are
 * unavoidably jargon, so the key stays on the surface that uses them rather
 * than being filed away in Help.
 */
import type { LinkState } from "@/types";
import { linkStateLabels } from "@/utils/statusLabels";
import LinkStateMark from "./LinkStateMark.vue";

const states = Object.keys(linkStateLabels) as LinkState[];
</script>

<template>
  <div class="legend">
    <span class="plate-bare">Key</span>
    <LinkStateMark v-for="state in states" :key="state" :state="state" />
  </div>
</template>

<style scoped>
.legend {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-2);
}
</style>

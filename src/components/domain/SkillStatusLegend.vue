<script setup lang="ts">
import type { LinkState } from "@/types";
import {
  linkStateBadgeVariant,
  linkStateLabels,
} from "@/utils/statusLabels";
import { SBadge } from "@stuntrocket/ui";

const states = Object.keys(linkStateLabels) as LinkState[];
</script>

<template>
  <div class="status-legend">
    <span class="legend-heading">Status</span>
    <SBadge
      v-for="state in states"
      :key="state"
      :variant="linkStateBadgeVariant(state)"
    >
      {{ linkStateLabels[state] }}
    </SBadge>
  </div>
</template>

<style scoped>
.status-legend {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: var(--space-1);
}

.legend-heading {
  margin-right: var(--space-1);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
</style>

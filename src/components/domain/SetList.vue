<script setup lang="ts">
import type { SetAssignment } from "@/types";

defineProps<{
  sets: SetAssignment[];
}>();
</script>

<template>
  <div class="set-list">
    <div class="section-header">
      <span class="section-title">Sets</span>
      <span class="section-count">{{ sets.length }}</span>
    </div>
    <div class="section-group">
      <div v-for="set in sets" :key="set.setId" class="set-row">
        <span class="set-name">{{ set.name }}</span>
        <span class="set-skill-count">
          {{ set.skillCount }} {{ set.skillCount === 1 ? 'skill' : 'skills' }}
        </span>
      </div>
      <div v-if="sets.length === 0" class="list-empty">
        <span class="list-empty-text">No sets installed</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.set-list {
  display: flex;
  flex-direction: column;
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
}

.section-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.section-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.section-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.set-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: var(--list-row-height);
  padding: 0 var(--space-3);
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.set-row + .set-row {
  border-top: 1px solid var(--border-subtle);
}

.set-row:hover {
  background: var(--surface-hover);
}

.set-name {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.set-skill-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

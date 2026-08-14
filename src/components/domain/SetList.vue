<script setup lang="ts">
/**
 * The sets installed at a location.
 *
 * A set is a named group of circuits wired together, so the row says what it
 * is and how many it carries — nothing else fits on one line usefully.
 */
import type { SetAssignment } from "@/types";

defineProps<{
  sets: SetAssignment[];
}>();
</script>

<template>
  <section class="section">
    <div class="section-head">
      <h3 class="section-title">Sets</h3>
      <span class="section-count">{{ sets.length }}</span>
    </div>
    <ul class="rows">
      <li v-for="set in sets" :key="set.setId">
        <span class="set-name">{{ set.name }}</span>
        <span class="set-count rating tabular">{{ set.skillCount }}</span>
        <span class="rating-unit">{{ set.skillCount === 1 ? "skill" : "skills" }}</span>
      </li>
      <li v-if="sets.length === 0" class="row-empty">No sets installed here.</li>
    </ul>
  </section>
</template>

<style scoped>
.set-name {
  flex: 1;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.set-count {
  font-size: var(--text-md);
  color: var(--text-secondary);
  flex-shrink: 0;
}

.rating-unit {
  margin-left: 0;
  flex-shrink: 0;
}
</style>

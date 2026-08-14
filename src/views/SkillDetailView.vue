<script setup lang="ts">
import { watch, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useLibraryStore } from "@/stores/libraryStore";
import { useBulkAssignStore } from "@/stores/bulkAssignStore";
import { SBadge } from "@stuntrocket/ui";
import LinkedLocationsList from "@/components/domain/LinkedLocationsList.vue";
import UsageSummaryPanel from "@/components/domain/UsageSummaryPanel.vue";

const route = useRoute();
const libraryStore = useLibraryStore();

const skillId = computed(() => route.params.skillId as string);

const bulkAssignStore = useBulkAssignStore();

const detail = computed(() => libraryStore.selectedDetail);

function loadDetail() {
  const id = skillId.value;
  if (id) {
    libraryStore.selectSkill(id);
  }
}

onMounted(loadDetail);

watch(skillId, loadDetail);
</script>

<template>
  <div v-if="detail" class="skill-detail">
    <div class="detail-header">
      <div class="header-title-row">
        <h2 class="header-name">{{ detail.name }}</h2>
        <SBadge v-if="detail.archived" variant="default">Archived</SBadge>
        <button
          class="bulk-assign-btn"
          title="Assign this skill to multiple locations"
          @click="bulkAssignStore.open([detail.id])"
        >
          Assign to locations
        </button>
      </div>
      <span class="header-path">{{ detail.path }}</span>
    </div>
    <div class="detail-content">
      <!-- What the skill says it does. This is the text that costs context in
           every session it loads into, so it leads. -->
      <p v-if="detail.summary" class="summary-text">{{ detail.summary }}</p>

      <section class="section">
        <div class="section-head">
          <h3 class="section-title">Wired into</h3>
          <span class="section-count">{{ detail.linkedLocations.length }}</span>
        </div>
        <LinkedLocationsList :locations="detail.linkedLocations" />
      </section>

      <section v-if="detail.includedInSets.length > 0" class="section">
        <div class="section-head">
          <h3 class="section-title">Included in sets</h3>
          <span class="section-count">{{ detail.includedInSets.length }}</span>
        </div>
        <ul class="rows">
          <li v-for="set in detail.includedInSets" :key="set.id" class="row">
            <span class="set-name">{{ set.name }}</span>
          </li>
        </ul>
      </section>

      <section class="section">
        <div class="section-head">
          <h3 class="section-title">Usage</h3>
        </div>
        <UsageSummaryPanel :usage="detail.usage" />
      </section>
    </div>
  </div>
  <div v-else class="loading-state">
    <span class="loading-text">Loading...</span>
  </div>
</template>

<style scoped>
.skill-detail {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.detail-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: var(--space-4) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.header-title-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.header-name {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
  flex: 1;
}

.bulk-assign-btn {
  font-family: inherit;
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-default);
  background: var(--surface-panel);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
  white-space: nowrap;
}

.bulk-assign-btn:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.header-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.detail-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.section-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
}

.summary-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
}

.sets-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.set-row {
  display: flex;
  align-items: center;
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.set-row + .set-row {
  border-top: 1px solid var(--border-subtle);
}

.set-name {
  flex: 1;
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.loading-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

<script setup lang="ts">
import { watch, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useLibraryStore } from "@/stores/libraryStore";
import Badge from "@/components/base/Badge.vue";
import LinkedLocationsList from "@/components/domain/LinkedLocationsList.vue";
import UsageSummaryPanel from "@/components/domain/UsageSummaryPanel.vue";

const route = useRoute();
const libraryStore = useLibraryStore();

const skillId = computed(() => route.params.skillId as string);

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
        <Badge v-if="detail.archived" variant="default">Archived</Badge>
      </div>
      <span class="header-path">{{ detail.path }}</span>
    </div>
    <div class="detail-content">
      <div v-if="detail.summary" class="detail-section">
        <span class="section-label">Summary</span>
        <p class="summary-text">{{ detail.summary }}</p>
      </div>

      <div class="detail-section">
        <span class="section-label">Linked Locations</span>
        <LinkedLocationsList :locations="detail.linkedLocations" />
      </div>

      <div v-if="detail.includedInSets.length > 0" class="detail-section">
        <span class="section-label">Included in Sets</span>
        <div class="sets-group">
          <div
            v-for="set in detail.includedInSets"
            :key="set.id"
            class="set-row"
          >
            <span class="set-name">{{ set.name }}</span>
          </div>
        </div>
      </div>

      <div class="detail-section">
        <span class="section-label">Usage</span>
        <UsageSummaryPanel :usage="detail.usage" />
      </div>
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
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
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

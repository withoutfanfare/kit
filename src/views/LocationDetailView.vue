<script setup lang="ts">
import { watch, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import { useLocationsStore } from "@/stores/locationsStore";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import LocationHeader from "@/components/domain/LocationHeader.vue";
import LocationOverviewCard from "@/components/domain/LocationOverviewCard.vue";
import SetList from "@/components/domain/SetList.vue";
import SkillList from "@/components/domain/SkillList.vue";
import IssueList from "@/components/domain/IssueList.vue";

const route = useRoute();
const locationsStore = useLocationsStore();
const skillPeekStore = useSkillPeekStore();

const locationId = computed(() => route.params.locationId as string);

const detail = computed(() => locationsStore.selectedDetail);

const linkedSkills = computed(
  () => detail.value?.skills.filter((s) => s.linkState === "linked") ?? []
);

const localOnlySkills = computed(
  () => detail.value?.skills.filter((s) => s.linkState === "local_only") ?? []
);

function peekSkill(skillId: string) {
  skillPeekStore.peek(skillId);
}

function loadDetail() {
  const id = locationId.value;
  if (id) {
    locationsStore.selectLocation(id);
  }
}

onMounted(loadDetail);

watch(locationId, loadDetail);
</script>

<template>
  <div v-if="detail" class="location-detail">
    <LocationHeader :detail="detail" />
    <div class="detail-content">
      <LocationOverviewCard
        :linked-count="detail.stats.linkedCount"
        :local-only-count="detail.stats.localOnlyCount"
        :broken-count="detail.stats.brokenCount"
      />

      <SetList
        v-if="detail.sets.length > 0"
        :sets="detail.sets"
      />

      <SkillList
        :skills="linkedSkills"
        title="Linked Skills"
        show-link-state
        @select-skill="peekSkill"
      />

      <SkillList
        v-if="localOnlySkills.length > 0"
        :skills="localOnlySkills"
        title="Local-Only Skills"
        show-link-state
        @select-skill="peekSkill"
      />

      <IssueList
        v-if="detail.issues.length > 0"
        :issues="detail.issues"
        :location-id="detail.id"
      />

      <div
        v-if="linkedSkills.length === 0 && localOnlySkills.length === 0 && detail.issues.length === 0"
        class="no-skills-hint"
      >
        <p class="hint-title">No skills installed</p>
        <p class="hint-desc">Use 'Add Skills' to link skills from your library to this project.</p>
      </div>
    </div>
  </div>
  <div v-else-if="locationsStore.isLoadingDetail" class="loading-state">
    <span class="loading-text">Loading...</span>
  </div>
</template>

<style scoped>
.location-detail {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
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

.no-skills-hint {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-6) var(--space-4);
  text-align: center;
}

.hint-title {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  margin: 0;
}

.hint-desc {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  line-height: 1.5;
  margin: 0;
  max-width: 280px;
}
</style>

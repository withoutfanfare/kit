<script setup lang="ts">
import { ref, watch, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useLocationsStore } from "@/stores/locationsStore";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import { useAppStore } from "@/stores/appStore";
import { useAssignmentStore } from "@/stores/assignmentStore";
import { useDismissedRecommendations } from "@/composables/useDismissedRecommendations";
import LocationHeader from "@/components/domain/LocationHeader.vue";
import SkillDiffModal from "@/components/domain/SkillDiffModal.vue";
import LocationOverviewCard from "@/components/domain/LocationOverviewCard.vue";
import SetList from "@/components/domain/SetList.vue";
import SkillList from "@/components/domain/SkillList.vue";
import IssueList from "@/components/domain/IssueList.vue";
import PanelIcon from "@/components/base/PanelIcon.vue";
import { SBadge, SButton } from "@stuntrocket/ui";
import type { SkillId, SkillRecommendation } from "@/types";

const route = useRoute();
const router = useRouter();
const locationsStore = useLocationsStore();
const skillPeekStore = useSkillPeekStore();
const appStore = useAppStore();
const assignmentStore = useAssignmentStore();
const { isDismissed, dismiss, restore, restoreAll } = useDismissedRecommendations();

const locationId = computed(() => route.params.locationId as string);

const diffSkillId = ref<string | null>(null);
const diffSkillName = ref("");
const isDiffOpen = ref(false);

function openDiff(skillId: string) {
  const skill = detail.value?.skills.find((s) => s.skillId === skillId);
  diffSkillId.value = skillId;
  diffSkillName.value = skill?.name ?? skillId;
  isDiffOpen.value = true;
}

function closeDiff() {
  isDiffOpen.value = false;
}

const detail = computed(() => locationsStore.selectedDetail);
const selectedRecommendationIds = ref<Set<SkillId>>(new Set());

const visibleRecommendations = computed(
  () => detail.value?.skillRecommendations.filter(
    (recommendation) => !isDismissed(detail.value!.id, recommendation.skillId)
  ) ?? []
);

const dismissedRecommendations = computed(
  () => detail.value?.skillRecommendations.filter(
    (recommendation) => isDismissed(detail.value!.id, recommendation.skillId)
  ) ?? []
);

const groupedRecommendations = computed(() => {
  const groups = new Map<string, SkillRecommendation[]>();
  for (const recommendation of visibleRecommendations.value) {
    const group = groups.get(recommendation.projectType) ?? [];
    group.push(recommendation);
    groups.set(recommendation.projectType, group);
  }
  return [...groups].map(([projectType, recommendations]) => ({
    projectType,
    recommendations,
  }));
});

function toggleRecommendation(skillId: SkillId) {
  const next = new Set(selectedRecommendationIds.value);
  next.has(skillId) ? next.delete(skillId) : next.add(skillId);
  selectedRecommendationIds.value = next;
}

function selectAllRecommendations() {
  selectedRecommendationIds.value = new Set(
    visibleRecommendations.value.map((recommendation) => recommendation.skillId)
  );
}

function addSelectedRecommendations() {
  if (!detail.value || selectedRecommendationIds.value.size === 0) return;
  assignmentStore.open(detail.value.id, [...selectedRecommendationIds.value]);
}

function dismissRecommendation(skillId: SkillId) {
  if (!detail.value) return;
  dismiss(detail.value.id, skillId);
}

const linkedSkills = computed(
  () => detail.value?.skills.filter((s) => s.linkState === "linked") ?? []
);

const localOnlySkills = computed(
  () => detail.value?.skills.filter((s) => s.linkState === "local_only") ?? []
);

const healthActionLabel = computed(() => {
  const count = detail.value?.issues.length ?? 0;
  return `Resolve ${count} issue${count === 1 ? "" : "s"}`;
});

function formatScanTime(iso: string | null): string {
  if (!iso) return "Never scanned";
  const date = new Date(iso);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / (1000 * 60));
  if (diffMins < 1) return "Just now";
  if (diffMins < 60) return `${diffMins}m ago`;
  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) return `${diffHours}h ago`;
  return date.toLocaleDateString("en-GB", { day: "numeric", month: "short" });
}

function navigateToHealth() {
  if (detail.value) {
    router.push({ path: "/health", query: { locationId: detail.value.id } });
  }
}

function peekSkill(skillId: string) {
  const skill = detail.value?.skills.find((s) => s.skillId === skillId);
  skillPeekStore.peek(skillId, skill?.path);
}

async function handleToggleActivation(skillId: string) {
  if (!detail.value) return;
  try {
    // Read the current state before the toggle refreshes the cached detail
    const wasDisabled = detail.value.skills.find((s) => s.skillId === skillId)?.disabled;
    await locationsStore.toggleSkillActivation(detail.value.id, skillId);
    appStore.toast(`Skill ${wasDisabled ? "enabled" : "disabled"}`, "success");
  } catch {
    appStore.toast("Failed to toggle skill", "error");
  }
}

function loadDetail() {
  const id = locationId.value;
  if (id) {
    locationsStore.selectLocation(id);
  }
}

onMounted(loadDetail);

watch(locationId, loadDetail);

watch(
  () => visibleRecommendations.value.map((recommendation) => recommendation.skillId),
  (visibleIds) => {
    const visible = new Set(visibleIds);
    selectedRecommendationIds.value = new Set(
      [...selectedRecommendationIds.value].filter((id) => visible.has(id))
    );
  },
  { immediate: true }
);
</script>

<template>
  <div v-if="detail" class="location-detail">
    <LocationHeader :detail="detail" />

    <!-- The data plate. A board carries its facts on one stamped line, not as
         a row of tiles: label above, figure below, ruled between. -->
    <dl class="data-plate">
      <div class="fact">
        <dt class="label">Skills</dt>
        <dd class="num">{{ detail.skills.length }}</dd>
      </div>

      <div class="fact">
        <dt class="label">Issues</dt>
        <dd class="num" :class="{ bad: detail.issues.length > 0 }">
          {{ detail.issues.length }}
        </dd>
      </div>

      <div class="fact">
        <dt class="label">Last scan</dt>
        <dd class="fact-text">{{ formatScanTime(detail.lastScannedAt) }}</dd>
      </div>

      <div v-if="detail.detectedProjectTypes.length > 0" class="fact">
        <dt class="label">Detected</dt>
        <dd class="fact-text">
          {{ detail.detectedProjectTypes.map((t) => t.name).join(" · ") }}
        </dd>
      </div>

      <button
        v-if="detail.issues.length > 0"
        type="button"
        class="fix"
        @click="navigateToHealth"
      >
        <PanelIcon name="caution" :size="13" />
        {{ healthActionLabel }}
      </button>
    </dl>

    <div class="detail-content">
      <!-- Skill recommendations -->
      <div
        v-if="visibleRecommendations.length > 0 || dismissedRecommendations.length > 0"
        class="recommendations-section"
      >
        <div class="section-header-row">
          <span class="section-title">Recommended skills</span>
          <SBadge variant="count">{{ visibleRecommendations.length }}</SBadge>
          <div v-if="visibleRecommendations.length > 0" class="recommendation-actions">
            <SButton variant="secondary" size="sm" @click="selectAllRecommendations">
              Select all
            </SButton>
            <SButton
              size="sm"
              :disabled="selectedRecommendationIds.size === 0"
              @click="addSelectedRecommendations"
            >
              Add selected
            </SButton>
          </div>
        </div>
        <div
          v-for="group in groupedRecommendations"
          :key="group.projectType"
          class="recommendation-group"
        >
          <span class="recommendation-group-title">{{ group.projectType }}</span>
          <div class="section-group">
            <div
              v-for="rec in group.recommendations"
              :key="rec.skillId"
              class="recommendation-row"
            >
              <input
                type="checkbox"
                :checked="selectedRecommendationIds.has(rec.skillId)"
                :aria-label="`Select ${rec.skillName}`"
                @change="toggleRecommendation(rec.skillId)"
              />
              <button type="button" class="rec-content" @click="peekSkill(rec.skillId)">
                <span class="rec-name">{{ rec.skillName }}</span>
                <span v-if="rec.reason" class="rec-reason">{{ rec.reason }}</span>
              </button>
              <SButton variant="secondary" size="sm" @click="dismissRecommendation(rec.skillId)">
                Dismiss
              </SButton>
            </div>
          </div>
        </div>
        <details v-if="dismissedRecommendations.length > 0" class="dismissed-recommendations">
          <summary>{{ dismissedRecommendations.length }} dismissed</summary>
          <div class="dismissed-actions">
            <div v-for="rec in dismissedRecommendations" :key="rec.skillId" class="dismissed-row">
              <span>{{ rec.skillName }}</span>
              <SButton variant="secondary" size="sm" @click="restore(detail.id, rec.skillId)">
                Restore
              </SButton>
            </div>
            <SButton variant="secondary" size="sm" @click="restoreAll(detail.id)">
              Restore all
            </SButton>
          </div>
        </details>
      </div>

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
        title="Assigned skills"
        show-link-state
        show-legend
        @select-skill="peekSkill"
        @toggle-activation="handleToggleActivation"
        @view-diff="openDiff"
      />

      <SkillList
        v-if="localOnlySkills.length > 0"
        :skills="localOnlySkills"
        title="Not in your library"
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
    <SkillDiffModal
      v-if="diffSkillId"
      :location-id="detail.id"
      :skill-id="diffSkillId"
      :skill-name="diffSkillName"
      :open="isDiffOpen"
      @close="closeDiff"
    />
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
  container-type: inline-size;
}

/* The data plate: one stamped line of facts, ruled between, rather than a row
   of stat tiles. Label above in plate lettering, figure below, tabular. */
.data-plate {
  display: flex;
  align-items: stretch;
  gap: 0;
  margin: 0;
  padding: var(--space-5) var(--space-7);
  border-bottom: 1px solid var(--border-subtle);
  background: var(--surface-panel);
  flex-shrink: 0;
  flex-wrap: wrap;
}

.fact {
  display: flex;
  flex-direction: column;
  gap: 1px;
  padding: 0 var(--space-5) 0 0;
  margin-right: var(--space-5);
  border-right: 1px solid var(--border-subtle);
}

.fact:last-of-type {
  border-right: 0;
  margin-right: 0;
}

.fact dd {
  margin: 0;
  font-size: var(--text-lg);
  color: var(--text-primary);
}

.fact dd.bad {
  color: var(--warning);
}

.fact-text {
  font-size: var(--text-md) !important;
  font-weight: var(--weight-medium);
  color: var(--text-secondary) !important;
  padding-top: 2px;
}

.fix {
  margin-left: auto;
  align-self: center;
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--warning);
  background: var(--warning-subtle);
  border: 1px solid color-mix(in srgb, var(--warning) 38%, transparent);
  border-radius: var(--radius-sm);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
}

.fix:hover {
  background: color-mix(in srgb, var(--warning) 18%, transparent);
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-6) var(--space-7) var(--space-9);
  display: flex;
  flex-direction: column;
  gap: var(--space-7);
}

.recommendations-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.section-header-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) 0;
}

.recommendation-actions {
  display: flex;
  gap: var(--space-2);
  margin-left: auto;
}

.recommendation-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.recommendation-group-title {
  padding: 0 var(--space-4);
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
}

.section-title {
  font-size: var(--text-sm);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
}

.section-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.recommendation-row {
  display: flex;
  align-items: center;
  padding: var(--space-4) var(--space-5);
  transition: background var(--duration-fast) var(--ease-default);
}

.recommendation-row:hover {
  background: var(--surface-hover);
}

.recommendation-row + .recommendation-row {
  border-top: 1px solid var(--border-subtle);
}

.rec-content {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 1px;
  padding: 0;
  border: 0;
  background: none;
  text-align: left;
  cursor: pointer;
}

.rec-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.rec-reason {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.dismissed-recommendations {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.dismissed-recommendations summary {
  cursor: pointer;
}

.dismissed-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: var(--space-2);
}

.dismissed-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-left: var(--space-3);
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

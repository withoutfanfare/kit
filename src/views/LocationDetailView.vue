<script setup lang="ts">
import { watch, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useLocationsStore } from "@/stores/locationsStore";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import LocationHeader from "@/components/domain/LocationHeader.vue";
import LocationOverviewCard from "@/components/domain/LocationOverviewCard.vue";
import SetList from "@/components/domain/SetList.vue";
import SkillList from "@/components/domain/SkillList.vue";
import IssueList from "@/components/domain/IssueList.vue";
import { SBadge } from "@stuntrocket/ui";

const route = useRoute();
const router = useRouter();
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

const healthStatus = computed(() => {
  if (!detail.value) return "unknown";
  if (detail.value.issues.some((i) => i.kind === "broken_link")) return "error";
  if (detail.value.issues.length > 0) return "warning";
  return "healthy";
});

const healthLabel = computed(() => {
  switch (healthStatus.value) {
    case "error": return "Issues found";
    case "warning": return "Warnings";
    default: return "Healthy";
  }
});

function healthBadgeVariant(status: string): "success" | "warning" | "error" | "default" {
  switch (status) {
    case "error": return "error";
    case "warning": return "warning";
    case "healthy": return "success";
    default: return "default";
  }
}

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

    <!-- Dashboard summary header -->
    <div class="dashboard-header">
      <div class="dashboard-stat">
        <span class="stat-value">{{ detail.skills.length }}</span>
        <span class="stat-label">Skills</span>
      </div>
      <div class="dashboard-stat">
        <span class="stat-value">{{ detail.issues.length }}</span>
        <span class="stat-label">Issues</span>
      </div>
      <div class="dashboard-stat clickable" @click="navigateToHealth">
        <SBadge :variant="healthBadgeVariant(healthStatus)" compact>
          {{ healthLabel }}
        </SBadge>
        <span class="stat-label">Health</span>
      </div>
      <div class="dashboard-stat">
        <span class="stat-value-sm">{{ formatScanTime(detail.lastScannedAt) }}</span>
        <span class="stat-label">Last scan</span>
      </div>
      <div v-if="detail.detectedProjectTypes.length > 0" class="dashboard-types">
        <SBadge
          v-for="pt in detail.detectedProjectTypes"
          :key="pt.name"
          variant="accent"
          compact
        >
          {{ pt.name }}
        </SBadge>
      </div>
    </div>

    <div class="detail-content">
      <!-- Skill recommendations -->
      <div v-if="detail.skillRecommendations.length > 0" class="recommendations-section">
        <div class="section-header-row">
          <span class="section-title">Recommended Skills</span>
          <SBadge variant="count">{{ detail.skillRecommendations.length }}</SBadge>
        </div>
        <div class="section-group">
          <div
            v-for="rec in detail.skillRecommendations"
            :key="rec.skillId"
            class="recommendation-row"
            @click="peekSkill(rec.skillId)"
          >
            <div class="rec-content">
              <span class="rec-name">{{ rec.skillName }}</span>
              <span class="rec-reason">{{ rec.reason }}</span>
            </div>
          </div>
        </div>
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

.dashboard-header {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-3) var(--space-5);
  border-bottom: 1px solid var(--border-subtle);
  background: var(--surface-panel);
  flex-shrink: 0;
}

.dashboard-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.dashboard-stat.clickable {
  cursor: pointer;
}

.dashboard-stat.clickable:hover {
  opacity: 0.8;
}

.stat-value {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.stat-value-sm {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.stat-label {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.dashboard-types {
  display: flex;
  gap: var(--space-1);
  margin-left: auto;
  flex-wrap: wrap;
}

.detail-content {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-4) var(--space-5);
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.recommendations-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.section-header-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-1) var(--space-3);
}

.section-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
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
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
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
  gap: 1px;
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

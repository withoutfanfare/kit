<script setup lang="ts">
import type { LocationIssue } from "@/types";
import { computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useLocationsStore } from "@/stores/locationsStore";
import { useAppStore } from "@/stores/appStore";
import Badge from "@/components/base/Badge.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";

const props = defineProps<{
  issues: LocationIssue[];
  locationId: string;
}>();

const locationsStore = useLocationsStore();
const appStore = useAppStore();

type IssueGroup = {
  kind: LocationIssue["kind"];
  label: string;
  variant: "warning" | "danger";
  issues: LocationIssue[];
};

const groupedIssues = computed<IssueGroup[]>(() => {
  const kindMeta: Record<
    LocationIssue["kind"],
    { label: string; variant: "warning" | "danger" }
  > = {
    broken_link: { label: "Broken Links", variant: "danger" },
    declared_missing: { label: "Declared but Missing", variant: "warning" },
    linked_undeclared: { label: "Linked but Undeclared", variant: "warning" },
    stale: { label: "Stale", variant: "warning" },
    missing_set: { label: "Missing Sets", variant: "warning" },
  };

  const groups: IssueGroup[] = [];
  const seen = new Set<string>();

  for (const issue of props.issues) {
    if (!seen.has(issue.kind)) {
      seen.add(issue.kind);
      const meta = kindMeta[issue.kind];
      groups.push({
        kind: issue.kind,
        label: meta.label,
        variant: meta.variant,
        issues: props.issues.filter((i) => i.kind === issue.kind),
      });
    }
  }

  return groups;
});

async function addToManifest(skillId: string) {
  try {
    await invoke("update_manifest_entry", {
      locationId: props.locationId,
      skillId,
      action: "add",
    });
    await locationsStore.fetchDetail(props.locationId);
    appStore.toast(`Added '${skillId}' to manifest`, "success");
  } catch {
    appStore.toast("Failed to update manifest", "error");
  }
}

async function removeFromManifest(skillId: string) {
  try {
    await invoke("update_manifest_entry", {
      locationId: props.locationId,
      skillId,
      action: "remove",
    });
    await locationsStore.fetchDetail(props.locationId);
    appStore.toast(`Removed '${skillId}' from manifest`, "success");
  } catch {
    appStore.toast("Failed to update manifest", "error");
  }
}

async function unlinkSkill(skillId: string) {
  try {
    await invoke("apply_assignment", {
      locationId: props.locationId,
      skillIdsToAdd: [],
      skillIdsToRemove: [skillId],
      setIdsToAdd: [],
      setIdsToRemove: [],
      updateManifest: false,
    });
    await locationsStore.fetchDetail(props.locationId);
    appStore.toast(`Unlinked '${skillId}'`, "success");
  } catch {
    appStore.toast("Failed to unlink skill", "error");
  }
}

async function syncLocation() {
  try {
    await locationsStore.syncLocation(props.locationId);
    appStore.toast("Location synced", "success");
  } catch {
    appStore.toast("Sync failed", "error");
  }
}
</script>

<template>
  <div class="issue-list">
    <div class="section-header">
      <span class="section-title">Issues</span>
      <Badge variant="warning" compact>{{ issues.length }}</Badge>
    </div>
    <div v-for="group in groupedIssues" :key="group.kind" class="issue-group">
      <div class="group-banner" :class="group.variant">
        <span class="group-label">{{ group.label }}</span>
        <span class="group-count">{{ group.issues.length }}</span>
      </div>
      <div class="group-items">
        <div v-for="(issue, idx) in group.issues" :key="idx" class="issue-row">
          <div class="issue-content">
            <span class="issue-skill">{{ issue.skillName }}</span>
            <span class="issue-message">{{ issue.message }}</span>
          </div>
          <div class="issue-actions">
            <!-- Broken link: Remove the broken symlink -->
            <template v-if="issue.kind === 'broken_link'">
              <SecondaryButton
                label="Remove"
                @click="unlinkSkill(issue.skillId!)"
              />
            </template>

            <!-- Declared but missing: Remove from manifest -->
            <template v-else-if="issue.kind === 'declared_missing'">
              <SecondaryButton
                label="Remove declaration"
                @click="removeFromManifest(issue.skillId!)"
              />
            </template>

            <!-- Linked but undeclared: two separate buttons -->
            <template v-else-if="issue.kind === 'linked_undeclared'">
              <SecondaryButton
                label="Add to manifest"
                @click="addToManifest(issue.skillId!)"
              />
              <SecondaryButton
                label="Unlink"
                @click="unlinkSkill(issue.skillId!)"
              />
            </template>

            <!-- Stale -->
            <template v-else-if="issue.kind === 'stale'">
              <SecondaryButton
                label="Sync"
                @click="syncLocation"
              />
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.issue-list {
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

.issue-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.issue-group + .issue-group {
  margin-top: var(--space-2);
}

.group-banner {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
}

.group-banner.warning {
  background: var(--warning-subtle);
  color: var(--warning);
}

.group-banner.danger {
  background: var(--danger-subtle);
  color: var(--danger);
}

.group-label {
  flex: 1;
}

.group-count {
  font-variant-numeric: tabular-nums;
}

.issue-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-top: 1px solid var(--border-subtle);
}

.issue-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.issue-skill {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.issue-message {
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.issue-actions {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-shrink: 0;
}
</style>

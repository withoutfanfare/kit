<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { useLoadoutStore } from "@/stores/loadoutStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { SBadge, SEmptyState } from "@stuntrocket/ui";
import type { SkillOrigin } from "@/types";

const route = useRoute();
const loadoutStore = useLoadoutStore();
const locationsStore = useLocationsStore();

/** Falls back to Global, which is the honest default: it always loads. */
const activeLocationId = computed(() => {
  const fromRoute = route.params.locationId as string | undefined;
  if (fromRoute) return fromRoute;
  const list = locationsStore.locationList;
  return list.find((l) => l.kind === "global")?.id ?? list[0]?.id ?? null;
});

const originLabel: Record<SkillOrigin, string> = {
  global: "Every session",
  project: "This project",
  plugin: "Plugin",
  account: "Account",
};

function refresh() {
  const id = activeLocationId.value;
  if (id) loadoutStore.load(id);
}

// Reachable straight from the sidebar, so the locations may not have been
// fetched yet by the Locations view. Without this the page sits on its empty
// state with nothing to select.
onMounted(async () => {
  if (locationsStore.locationList.length === 0) {
    await locationsStore.fetchList();
  }
  refresh();
});
watch(activeLocationId, refresh);
</script>

<template>
  <div class="loadout-view">
    <div v-if="loadoutStore.isLoading" class="loadout-status">Working it out…</div>

    <SEmptyState
      v-else-if="!loadoutStore.loadout"
      title="Nothing to show"
      description="Select a location to see what loads into a session there."
    />

    <template v-else>
      <header class="loadout-header">
        <div>
          <h2 class="loadout-title">
            What loads in {{ loadoutStore.loadout.locationLabel }}
          </h2>
          <p class="loadout-subtitle">
            {{ loadoutStore.loadout.modelFacingCount }} skills reach the model,
            costing roughly
            {{ loadoutStore.loadout.tokenEstimate.toLocaleString() }} tokens every
            session.
            <template v-if="loadoutStore.loadout.commandOnlyCount > 0">
              A further {{ loadoutStore.loadout.commandOnlyCount }} are
              slash-command only and cost nothing.
            </template>
          </p>
        </div>
      </header>

      <!-- Linked against actually used. The gap is the point. -->
      <section v-if="loadoutStore.usage" class="loadout-usage">
        <template v-if="!loadoutStore.usage.available">
          No usage logs found yet, so Kit can't say what's earning its place
          here. That's missing data, not zero use.
        </template>
        <template v-else-if="loadoutStore.usage.linkedCount > 0">
          <strong>
            {{ loadoutStore.usage.usedHereCount }} of
            {{ loadoutStore.usage.linkedCount }}
          </strong>
          skills linked here have actually been used here, across
          {{ loadoutStore.usage.eventCount.toLocaleString() }} recorded
          invocations.
        </template>
      </section>

      <!-- The failure this view exists to catch. -->
      <section v-if="loadoutStore.conflictCount > 0" class="loadout-alert">
        <strong>
          {{ loadoutStore.conflictCount }}
          {{ loadoutStore.conflictCount === 1 ? "skill is" : "skills are" }}
          linked here but switched off globally
        </strong>
        <p>
          A <code>skillOverrides</code> entry beats every symlink, so these look
          active and are not. Unlinking them instead keeps per-project tailoring
          working.
        </p>
        <ul class="loadout-conflicts">
          <li v-for="skill in loadoutStore.loadout.vetoed" :key="skill.path">
            {{ skill.folderName }}
          </li>
        </ul>
      </section>

      <section v-if="loadoutStore.hasOverrideProblems" class="loadout-alert muted">
        <p v-if="loadoutStore.loadout.deadOverrides.length">
          <strong>Overrides pointing at nothing:</strong>
          {{ loadoutStore.loadout.deadOverrides.join(", ") }} — no such skill
          exists, so these entries do nothing.
        </p>
        <p v-if="loadoutStore.loadout.unreachableOverrides.length">
          <strong>Overrides that can't bite:</strong>
          {{ loadoutStore.loadout.unreachableOverrides.join(", ") }} — these exist
          only under a <code>plugin:skill</code> name, which a bare-name override
          never matches. They are still loading.
        </p>
      </section>

      <section
        v-for="group in loadoutStore.countedGroups"
        :key="`${group.origin}-${group.label}`"
        class="loadout-group"
      >
        <div class="group-head">
          <span class="group-label">{{ group.label }}</span>
          <SBadge>{{ originLabel[group.origin] }}</SBadge>
          <span class="group-cost">
            {{ group.modelFacingCount }} skills · ~{{
              group.tokenEstimate.toLocaleString()
            }}
            tokens
          </span>
        </div>
        <ul class="group-skills">
          <li
            v-for="skill in group.skills.filter((s) => !s.vetoedBy)"
            :key="skill.path"
            :class="{ 'command-only': !skill.modelFacing }"
          >
            {{ skill.id }}
            <span v-if="!skill.modelFacing" class="hint">command only</span>
          </li>
        </ul>
      </section>

      <section v-if="loadoutStore.uncountedGroups.length" class="loadout-group">
        <div class="group-head">
          <span class="group-label">Account packs</span>
          <SBadge variant="warning">Not counted</SBadge>
        </div>
        <p class="group-caveat">
          {{ loadoutStore.uncountedGroups[0].caveat }}
        </p>
        <ul class="group-skills">
          <li v-for="group in loadoutStore.uncountedGroups" :key="group.label">
            {{ group.label }}
            <span class="hint">{{ group.modelFacingCount }} skills cached</span>
          </li>
        </ul>
      </section>
    </template>
  </div>
</template>

<style scoped>
.loadout-view {
  padding: var(--space-4);
  overflow-y: auto;
  height: 100%;
}

.loadout-status {
  padding: var(--space-6);
  color: var(--text-tertiary);
  font-size: var(--text-sm);
}

.loadout-header {
  margin-bottom: var(--space-4);
}

.loadout-title {
  font-size: var(--text-base);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.loadout-subtitle {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: var(--space-1) 0 0;
}

.loadout-usage {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  padding: var(--space-2) var(--space-3);
  margin-bottom: var(--space-3);
  border-radius: var(--radius-md);
  background: var(--surface-hover);
}

.loadout-alert {
  border: 1px solid var(--border-warning, var(--border-default));
  border-radius: var(--radius-md);
  padding: var(--space-3);
  margin-bottom: var(--space-4);
}

.loadout-alert.muted {
  border-color: var(--border-default);
}

.loadout-alert p {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: var(--space-1) 0 0;
}

.loadout-conflicts {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin: var(--space-2) 0 0;
  padding: 0;
  list-style: none;
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.loadout-group {
  margin-bottom: var(--space-4);
}

.group-head {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}

.group-label {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.group-cost {
  margin-left: auto;
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.group-caveat {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  margin: 0 0 var(--space-2);
}

.group-skills {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: var(--space-1) var(--space-3);
  margin: 0;
  padding: 0;
  list-style: none;
  font-size: var(--text-xs);
  color: var(--text-secondary);
}

.group-skills .command-only {
  color: var(--text-tertiary);
}

.hint {
  color: var(--text-tertiary);
  font-size: var(--text-xs);
}
</style>

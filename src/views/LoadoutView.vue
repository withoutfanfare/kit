<script setup lang="ts">
import { computed, onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { useLoadoutStore } from "@/stores/loadoutStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { SBadge, SButton, SEmptyState } from "@stuntrocket/ui";
import type { LoadoutGroup, SkillOrigin } from "@/types";

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

/** Heaviest source first — the thing worth acting on leads. */
const ranked = computed(() =>
  [...loadoutStore.countedGroups].sort((a, b) => b.tokenEstimate - a.tokenEstimate)
);

const total = computed(() => loadoutStore.loadout?.tokenEstimate ?? 0);

function share(group: LoadoutGroup): number {
  if (!total.value) return 0;
  return Math.round((group.tokenEstimate / total.value) * 100);
}

/**
 * A segment is only useful if you can find its name in the key, so each group
 * gets its own step. Global and the project keep the accent; the plugins walk
 * down one neutral ramp in rank order, which stays restrained while remaining
 * separable.
 */
function tint(group: LoadoutGroup, index: number): string {
  if (group.origin === "global") return "var(--accent)";
  if (group.origin === "project") return "var(--accent-hover)";
  const step = Math.min(index, 5);
  return `color-mix(in srgb, var(--text-tertiary) ${88 - step * 13}%, transparent)`;
}

/** Used / linked, as a proportion — the "is it earning its place" number. */
const usedShare = computed(() => {
  const u = loadoutStore.usage;
  if (!u?.available || !u.linkedCount) return null;
  return Math.round((u.usedHereCount / u.linkedCount) * 100);
});

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
  <div class="loadout">
    <!-- Skeleton rather than a spinner: the shape of the answer arrives first. -->
    <div v-if="loadoutStore.isLoading" class="skeleton" aria-busy="true">
      <div class="sk sk-title"></div>
      <div class="sk sk-bar"></div>
      <div class="sk sk-row"></div>
      <div class="sk sk-row"></div>
    </div>

    <SEmptyState
      v-else-if="!loadoutStore.loadout"
      title="Nothing loading yet"
      description="Pick a location and Kit will work out which skills reach a session there, and what they cost."
    />

    <template v-else>
      <header class="head">
        <h1 class="page-title">{{ loadoutStore.loadout.locationLabel }}</h1>
        <p class="head-line">
          <strong class="figure">{{ loadoutStore.loadout.modelFacingCount }}</strong>
          skills reach the model, costing about
          <strong class="figure">{{ total.toLocaleString() }}</strong>
          tokens of context every session.
        </p>
        <p v-if="loadoutStore.loadout.commandOnlyCount" class="head-aside">
          {{ loadoutStore.loadout.commandOnlyCount }} more are slash-command only,
          and cost nothing until you call them.
        </p>
      </header>

      <!-- The one decisive device: cost as proportion, not as a number. -->
      <section class="budget" aria-label="Context cost by source">
        <div class="budget-bar">
          <span
            v-for="(group, i) in ranked"
            :key="`${group.origin}-${group.label}`"
            class="seg"
            :style="{
              width: `${share(group)}%`,
              background: tint(group, i),
              animationDelay: `${i * 40}ms`,
            }"
            :title="`${group.label} — ${group.tokenEstimate.toLocaleString()} tokens`"
          />
        </div>
        <ol class="budget-key">
          <li v-for="(group, i) in ranked" :key="`k-${group.origin}-${group.label}`">
            <span class="dot" :style="{ background: tint(group, i) }" />
            <span class="key-name">{{ group.label }}</span>
            <span class="key-share">{{ share(group) }}%</span>
          </li>
        </ol>
      </section>

      <!-- Is any of it earning its place. -->
      <section v-if="loadoutStore.usage" class="earning">
        <template v-if="!loadoutStore.usage.available">
          <p class="earning-none">
            No usage logs found yet, so Kit can't say what's earning its place.
            That's missing data, not zero use.
          </p>
        </template>
        <template v-else-if="usedShare !== null">
          <div class="earning-row">
            <div class="meter" :style="{ '--pct': usedShare / 100 }">
              <span class="meter-fill" />
            </div>
            <p class="earning-text">
              <strong class="figure">{{ loadoutStore.usage.usedHereCount }}</strong>
              of {{ loadoutStore.usage.linkedCount }} skills linked here have
              actually been used here, across
              {{ loadoutStore.usage.eventCount.toLocaleString() }} recorded runs.
            </p>
          </div>
        </template>
      </section>

      <!-- Conflicts: named, counted, and collapsed so they don't become a wall. -->
      <details v-if="loadoutStore.conflictCount > 0" class="flag flag-warn">
        <summary>
          <span class="flag-count">{{ loadoutStore.conflictCount }}</span>
          <span class="flag-headline">
            linked here, but switched off globally
          </span>
          <span class="flag-more">Show</span>
        </summary>
        <p class="flag-body">
          A <code>skillOverrides</code> entry beats every symlink, so these look
          active and are not. Unlink them instead and per-project tailoring keeps
          working.
        </p>
        <ul class="chips">
          <li v-for="skill in loadoutStore.loadout.vetoed" :key="skill.path">
            {{ skill.folderName }}
          </li>
        </ul>
      </details>

      <details v-if="loadoutStore.hasOverrideProblems" class="flag">
        <summary>
          <span class="flag-count">
            {{
              loadoutStore.loadout.deadOverrides.length +
              loadoutStore.loadout.unreachableOverrides.length
            }}
          </span>
          <span class="flag-headline">overrides that aren't doing what you think</span>
          <span class="flag-more">Show</span>
        </summary>
        <p v-if="loadoutStore.loadout.deadOverrides.length" class="flag-body">
          <strong>Pointing at nothing.</strong> No skill by these names exists, so
          the entries do nothing at all.
        </p>
        <ul v-if="loadoutStore.loadout.deadOverrides.length" class="chips">
          <li v-for="name in loadoutStore.loadout.deadOverrides" :key="name">{{ name }}</li>
        </ul>
        <p v-if="loadoutStore.loadout.unreachableOverrides.length" class="flag-body">
          <strong>Can't bite.</strong> These exist only under a
          <code>plugin:skill</code> name, which a bare-name override never matches.
          They are still loading.
        </p>
        <ul v-if="loadoutStore.loadout.unreachableOverrides.length" class="chips">
          <li v-for="name in loadoutStore.loadout.unreachableOverrides" :key="name">
            {{ name }}
          </li>
        </ul>
      </details>

      <section
        v-for="group in ranked"
        :key="`g-${group.origin}-${group.label}`"
        class="group"
      >
        <div class="group-head">
          <span class="dot" :style="{ background: tint(group, ranked.indexOf(group)) }" />
          <h3 class="group-name">{{ group.label }}</h3>
          <SBadge>{{ originLabel[group.origin] }}</SBadge>
          <span class="group-cost">
            {{ group.modelFacingCount }} skills
            <span class="sep">·</span>
            {{ group.tokenEstimate.toLocaleString() }} tokens
          </span>
        </div>
        <ul class="skills">
          <li
            v-for="skill in group.skills.filter((s) => !s.vetoedBy)"
            :key="skill.path"
            :class="{ quiet: !skill.modelFacing }"
          >
            <span class="skill-name">{{ skill.id }}</span>
            <span v-if="!skill.modelFacing" class="skill-tag">command</span>
          </li>
        </ul>
      </section>

      <section v-if="loadoutStore.uncountedGroups.length" class="group group-muted">
        <div class="group-head">
          <span class="dot dot-unknown" />
          <h3 class="group-name">Account packs</h3>
          <SBadge variant="warning">Not counted</SBadge>
        </div>
        <p class="group-caveat">{{ loadoutStore.uncountedGroups[0].caveat }}</p>
        <ul class="packs">
          <li v-for="group in loadoutStore.uncountedGroups" :key="group.label">
            <span class="pack-name">{{ group.label }}</span>
            <span class="pack-count">{{ group.modelFacingCount }}</span>
          </li>
        </ul>
      </section>

      <footer class="foot">
        <SButton size="sm" @click="refresh">Rescan</SButton>
      </footer>
    </template>
  </div>
</template>

<style scoped>
.loadout {
  height: 100%;
  overflow-y: auto;
  padding: var(--space-6) var(--space-6) var(--space-10);
  max-width: 920px;
}

/* ── Header ─────────────────────────────────────────────── */

.head {
  margin-bottom: var(--space-6);
}

.page-title {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--space-2);
}

.head-line {
  font-size: var(--text-lg);
  font-weight: var(--weight-normal);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
  max-width: 68ch;
  text-wrap: pretty;
}

.figure {
  color: var(--text-primary);
  font-weight: var(--weight-semibold);
  font-variant-numeric: tabular-nums;
}

.head-aside {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  margin: var(--space-2) 0 0;
  max-width: 60ch;
}

/* ── Budget bar ─────────────────────────────────────────── */

.budget {
  margin-bottom: var(--space-6);
}

.budget-bar {
  display: flex;
  gap: 2px;
  height: 12px;
  margin-bottom: var(--space-3);
}

.seg {
  border-radius: var(--radius-full);
  min-width: 3px;
  transform-origin: left center;
  animation: grow var(--duration-normal) var(--ease-out) both;
}

@keyframes grow {
  from {
    transform: scaleX(0);
  }
  to {
    transform: scaleX(1);
  }
}

@media (prefers-reduced-motion: reduce) {
  .seg {
    animation: none;
  }
}

.budget-key {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1) var(--space-4);
  list-style: none;
  margin: 0;
  padding: 0;
}

.budget-key li {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.dot {
  width: 7px;
  height: 7px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.dot-unknown {
  background: var(--border-strong);
}

.key-share {
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

/* ── Earning its place ──────────────────────────────────── */

.earning {
  margin-bottom: var(--space-6);
}

.earning-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.meter {
  position: relative;
  width: 64px;
  height: 6px;
  border-radius: var(--radius-full);
  background: var(--surface-hover);
  flex-shrink: 0;
  overflow: hidden;
}

.meter-fill {
  position: absolute;
  inset: 0;
  background: var(--color-success);
  border-radius: inherit;
  /* scaleX rather than width: no layout work on a value that animates. */
  transform: scaleX(var(--pct));
  transform-origin: left center;
  transition: transform var(--duration-normal) var(--ease-out);
}

.earning-text,
.earning-none {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
  max-width: 62ch;
}

/* ── Flags ──────────────────────────────────────────────── */

.flag {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  margin-bottom: var(--space-3);
  background: var(--surface-panel);
}

.flag-warn {
  border-color: color-mix(in srgb, var(--color-warning) 45%, transparent);
}

.flag summary {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  cursor: default;
  list-style: none;
}

.flag summary::-webkit-details-marker {
  display: none;
}

.flag-count {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
  min-width: 1.5em;
}

.flag-warn .flag-count {
  color: var(--color-warning);
}

.flag-headline {
  flex: 1;
  font-size: var(--text-md);
  color: var(--text-primary);
}

.flag-more {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.flag[open] .flag-more {
  color: var(--accent);
}

.flag-body {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: var(--space-3) 0 var(--space-2);
  max-width: 68ch;
}

.flag code {
  font-family: var(--font-mono);
  font-size: 0.92em;
  color: var(--text-primary);
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1) var(--space-2);
  list-style: none;
  margin: 0 0 var(--space-2);
  padding: 0;
}

.chips li {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  background: var(--surface-hover);
  border-radius: var(--radius-xs);
  padding: 2px var(--space-2);
}

/* ── Groups ─────────────────────────────────────────────── */

.group {
  margin-bottom: var(--space-6);
}

.group-head {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--border-subtle);
  margin-bottom: var(--space-3);
}

.group-name {
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.group-cost {
  margin-left: auto;
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.sep {
  opacity: 0.5;
  margin: 0 2px;
}

.skills {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--space-1) var(--space-4);
  list-style: none;
  margin: 0;
  padding: 0;
}

.skills li {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  min-width: 0;
}

.skill-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.skills li.quiet .skill-name {
  color: var(--text-tertiary);
}

.skill-tag {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xs);
  padding: 0 4px;
  flex-shrink: 0;
}

.group-muted .group-name {
  color: var(--text-secondary);
}

.group-caveat {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  margin: 0 0 var(--space-3);
  max-width: 68ch;
}

.packs {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(170px, 1fr));
  gap: var(--space-1) var(--space-4);
  list-style: none;
  margin: 0;
  padding: 0;
}

.packs li {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-2);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.pack-count {
  font-variant-numeric: tabular-nums;
}

/* ── Foot & skeleton ────────────────────────────────────── */

.foot {
  padding-top: var(--space-2);
}

.skeleton {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  padding: var(--space-2) 0;
}

.sk {
  border-radius: var(--radius-sm);
  background: var(--surface-hover);
  animation: pulse 1.4s var(--ease-default) infinite;
}

.sk-title {
  height: 24px;
  width: 40%;
}
.sk-bar {
  height: 10px;
  width: 100%;
}
.sk-row {
  height: 56px;
  width: 100%;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.55;
  }
}

@media (prefers-reduced-motion: reduce) {
  .sk {
    animation: none;
  }
}
</style>

<script setup lang="ts">
/**
 * The Panel — Kit's front door.
 *
 * The one screen that answers "what is the state of things" before you decide
 * anything. It is a panel schedule: a rated capacity strip showing what the
 * session draws and where from, the circuits themselves, and — kept firmly
 * below the total — the sources Kit cannot verify.
 *
 * The load figure is deliberately the only large type in the app. Everything
 * else on this screen exists to qualify it.
 */
import { computed, onMounted, ref, watch } from "vue";
import { useLoadoutStore } from "@/stores/loadoutStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useHealthStore } from "@/stores/healthStore";
import PanelIcon from "@/components/base/PanelIcon.vue";
import type { LoadoutGroup } from "@/types";

const loadout = useLoadoutStore();
const locations = useLocationsStore();
const health = useHealthStore();

/** Which sub-panel the schedule is showing. Global is the honest default. */
const activeId = ref<string | null>(null);

const activeLocation = computed(
  () => locations.locationList.find((l) => l.id === activeId.value) ?? null
);

onMounted(async () => {
  if (locations.locationList.length === 0) await locations.fetchList();
  activeId.value =
    locations.locationList.find((l) => l.kind === "global")?.id ??
    locations.locationList[0]?.id ??
    null;
  health.runCheck();
});

watch(activeId, (id) => {
  if (id) loadout.load(id);
});

/** Counted groups, heaviest first: the thing worth acting on leads. */
const counted = computed(() =>
  [...loadout.countedGroups].sort((a, b) => b.tokenEstimate - a.tokenEstimate)
);

const unverified = computed(() => loadout.uncountedGroups);

const total = computed(() => loadout.loadout?.tokenEstimate ?? 0);

const unverifiedTotal = computed(() =>
  unverified.value.reduce((n, g) => n + g.tokenEstimate, 0)
);

function share(group: LoadoutGroup): number {
  if (!total.value) return 0;
  return (group.tokenEstimate / total.value) * 100;
}

/**
 * Global is always ramp 1 and the project you are in is always copper, so those
 * two never move whatever the sort order. Everything else walks down the ramp
 * in rank order, which keeps adjacent segments distinguishable.
 */
const tints = computed(() => {
  const map = new Map<string, string>();
  let step = 3;
  for (const g of counted.value) {
    const key = `${g.origin}-${g.label}`;
    if (g.origin === "global") map.set(key, "var(--load-1)");
    else if (g.origin === "project") map.set(key, "var(--load-2)");
    else map.set(key, `var(--load-${Math.min(step++, 6)})`);
  }
  return map;
});

function tint(group: LoadoutGroup): string {
  return tints.value.get(`${group.origin}-${group.label}`) ?? "var(--load-6)";
}

const conflicts = computed(() => loadout.loadout?.vetoed ?? []);

const issueCount = computed(
  () => (health.result?.errorCount ?? 0) + (health.result?.warningCount ?? 0)
);
</script>

<template>
  <div class="panel-view">
    <!-- ── Rating plate: which board, and what it draws ───────────── -->
    <header class="board">
      <div class="board-id">
        <label class="picker">
          <span class="sr-only">Sub-panel</span>
          <select v-model="activeId" class="picker-select">
            <option v-for="l in locations.locationList" :key="l.id" :value="l.id">
              {{ l.label }}
            </option>
          </select>
        </label>
        <p class="board-path" :title="activeLocation?.path">
          {{ activeLocation?.path ?? "—" }}
        </p>
      </div>

      <div class="draw">
        <span class="rating draw-figure tabular">{{ total.toLocaleString() }}</span>
        <span class="draw-unit">
          <span class="plate-bare">tokens / session</span>
          <span class="draw-note">
            {{ loadout.loadout?.modelFacingCount ?? 0 }} skills reach the model
            <template v-if="loadout.loadout?.commandOnlyCount">
              · {{ loadout.loadout.commandOnlyCount }} command-only
            </template>
          </span>
        </span>
      </div>
    </header>

    <div v-if="loadout.isLoading" class="loading" aria-busy="true">
      <span class="plate-bare">Reading the board…</span>
    </div>

    <template v-else-if="loadout.loadout">
      <!-- ── The capacity strip ──────────────────────────────────── -->
      <section class="strip-block" aria-label="Load by source">
        <div class="strip">
          <span
            v-for="g in counted"
            :key="`${g.origin}-${g.label}`"
            class="seg"
            :style="{ width: `${share(g)}%`, background: tint(g) }"
            :title="`${g.label} — ${g.tokenEstimate.toLocaleString()} tokens`"
          />
        </div>

        <ol class="legend">
          <li v-for="g in counted" :key="`k-${g.origin}-${g.label}`">
            <span class="swatch" :style="{ background: tint(g) }" aria-hidden="true" />
            <span class="legend-name">{{ g.label }}</span>
            <span class="legend-load rating tabular">{{ g.tokenEstimate.toLocaleString() }}</span>
          </li>
        </ol>
      </section>

      <!-- ── What needs attention. Only shown when it exists. ─────── -->
      <section v-if="conflicts.length || issueCount" class="attention">
        <RouterLink v-if="conflicts.length" to="/loadout" class="flag flag-caution">
          <PanelIcon name="caution" />
          <span class="flag-body">
            <strong class="rating tabular">{{ conflicts.length }}</strong>
            linked here but switched off globally — they look active and are not.
          </span>
          <PanelIcon name="chevron" :size="13" class="flag-go" />
        </RouterLink>

        <RouterLink v-if="issueCount" to="/health" class="flag">
          <PanelIcon name="health" />
          <span class="flag-body">
            <strong class="rating tabular">{{ issueCount }}</strong>
            {{ issueCount === 1 ? "issue" : "issues" }} across your locations —
            broken links and declarations that don't match.
          </span>
          <PanelIcon name="chevron" :size="13" class="flag-go" />
        </RouterLink>
      </section>

      <!-- ── The schedule ─────────────────────────────────────────── -->
      <section class="schedule">
        <div class="schedule-head">
          <h2 class="schedule-title">Circuits</h2>
          <RouterLink :to="`/loadout/${activeId}`" class="schedule-link">
            Full schedule
            <PanelIcon name="chevron" :size="12" />
          </RouterLink>
        </div>

        <ol class="circuits">
          <li
            v-for="(g, i) in counted"
            :key="`c-${g.origin}-${g.label}`"
            class="circuit"
          >
            <span class="position tabular">{{ String(i + 1).padStart(2, "0") }}</span>
            <span class="swatch" :style="{ background: tint(g) }" aria-hidden="true" />
            <span class="circuit-name">{{ g.label }}</span>
            <span class="plate circuit-origin">{{ g.origin }}</span>
            <span class="circuit-count tabular">
              {{ g.modelFacingCount }}
              <span class="circuit-count-unit">{{ g.modelFacingCount === 1 ? "skill" : "skills" }}</span>
            </span>
            <span class="circuit-load rating tabular">
              {{ g.tokenEstimate.toLocaleString() }}
            </span>
          </li>
        </ol>
      </section>

      <!-- ── Below the total, and hatched, because it is not in it ── -->
      <section v-if="unverified.length" class="unverified unsurveyed">
        <div class="unverified-head">
          <span class="plate">Not counted</span>
          <p class="unverified-why">
            Kit can read these but cannot tell whether they are switched on, so
            they are outside every figure above.
          </p>
          <span class="unverified-load rating tabular">
            ~{{ unverifiedTotal.toLocaleString() }}
          </span>
        </div>
        <ul class="packs">
          <li v-for="g in unverified" :key="`u-${g.label}`">
            <span class="pack-name">{{ g.label }}</span>
            <span class="pack-count tabular">{{ g.modelFacingCount }}</span>
          </li>
        </ul>
      </section>
    </template>
  </div>
</template>

<style scoped>
.panel-view {
  height: 100%;
  overflow-y: auto;
  padding: var(--space-6) var(--space-7) var(--space-9);
  max-width: 1080px;
}

/* ── Rating plate ─────────────────────────────────────────── */

.board {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: var(--space-6);
  padding-bottom: var(--space-5);
  border-bottom: 1px solid var(--border-default);
}

.board-id {
  min-width: 0;
}

.picker-select {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  background: transparent;
  border: 0;
  padding: 0;
  margin: 0 0 2px -2px;
  cursor: pointer;
  max-width: 42ch;
}

.board-path {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  margin: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 52ch;
}

.draw {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  flex-shrink: 0;
}

.draw-figure {
  font-size: var(--text-rating);
  line-height: 1;
  letter-spacing: -0.02em;
}

.draw-unit {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 1px;
  padding-bottom: 2px;
}

.draw-note {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.loading {
  padding: var(--space-7) 0;
}

/* ── Capacity strip ───────────────────────────────────────── */

.strip-block {
  margin-top: var(--space-6);
}

.strip {
  display: flex;
  gap: 1.5px;
  height: 14px;
  margin-bottom: var(--space-4);
}

.seg {
  min-width: 2px;
  transform-origin: left center;
  animation: throw var(--duration-slow) var(--ease-out) both;
}

/* The one authored moment: the strip energises left to right, the way a
   panel comes up. Damped, not bouncy — a switch throws, it does not spring. */
@keyframes throw {
  from {
    transform: scaleX(0);
  }
  to {
    transform: scaleX(1);
  }
}

.legend {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2) var(--space-6);
  list-style: none;
  margin: 0;
  padding: 0;
}

.legend li {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.swatch {
  width: 7px;
  height: 7px;
  flex-shrink: 0;
  align-self: center;
}

.legend-load {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

/* ── Attention ────────────────────────────────────────────── */

.attention {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: var(--space-7);
}

.flag {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: var(--text-sm);
  transition: border-color var(--duration-fast) var(--ease-default),
    background var(--duration-fast) var(--ease-default);
}

.flag:hover {
  background: var(--surface-hover);
  border-color: var(--border-default);
}

.flag-caution {
  border-color: color-mix(in srgb, var(--warning) 38%, transparent);
  color: var(--text-primary);
}

.flag-caution :deep(.icon) {
  color: var(--warning);
}

.flag-body {
  flex: 1;
  text-wrap: pretty;
}

.flag-body strong {
  color: var(--text-primary);
  margin-right: 2px;
}

.flag-go {
  color: var(--text-tertiary);
  flex-shrink: 0;
}

/* ── Schedule ─────────────────────────────────────────────── */

.schedule {
  margin-top: var(--space-8);
}

.schedule-head {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-4);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--border-default);
}

.schedule-title {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.schedule-link {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  font-size: var(--text-sm);
  color: var(--accent);
  text-decoration: none;
}

.schedule-link:hover {
  text-decoration: underline;
}

.circuits {
  list-style: none;
  margin: 0;
  padding: 0;
}

.circuit {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-2);
  border-bottom: 1px solid var(--border-subtle);
  font-size: var(--text-md);
}

.circuit-name {
  color: var(--text-primary);
  font-weight: var(--weight-medium);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.circuit-origin {
  font-size: 9.5px;
  opacity: 0.85;
}

.circuit-count {
  margin-left: auto;
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.circuit-count-unit {
  margin-left: 2px;
}

.circuit-load {
  font-size: var(--text-md);
  min-width: 6ch;
  text-align: right;
  flex-shrink: 0;
}

/* ── Not counted ──────────────────────────────────────────── */

.unverified {
  margin-top: var(--space-8);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: var(--space-4) var(--space-5) var(--space-5);
}

.unverified-head {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
  margin-bottom: var(--space-4);
}

.unverified-why {
  flex: 1;
  min-width: 24ch;
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
  text-wrap: pretty;
}

.unverified-load {
  font-size: var(--text-lg);
  color: var(--unsurveyed);
}

.packs {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(168px, 1fr));
  gap: var(--space-2) var(--space-5);
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
  border-bottom: 1px solid var(--border-subtle);
  padding-bottom: 2px;
}

@media (max-width: 900px) {
  .board {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-4);
  }
}
</style>

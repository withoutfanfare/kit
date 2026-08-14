<script setup lang="ts">
/**
 * The full schedule.
 *
 * Where the Panel gives the board at a glance, this is the schedule card
 * itself: every circuit, its position, what it feeds, what it draws, and its
 * state. It is a table because a schedule is a table — the job here is to make
 * a long dense list scannable, not to make it look like something else.
 *
 * Three things this screen exists to say, which the old one buried:
 *   · a skill can be linked and still not load (an upstream veto);
 *   · a skill can be present and cost nothing (command-only);
 *   · some sources cannot be verified at all, and are not in the total.
 */
import { computed, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useLoadoutStore } from "@/stores/loadoutStore";
import { useLocationsStore } from "@/stores/locationsStore";
import PanelIcon from "@/components/base/PanelIcon.vue";
import type { LoadoutGroup, ResolvedSkill } from "@/types";

const route = useRoute();
const router = useRouter();
const loadout = useLoadoutStore();
const locations = useLocationsStore();

const activeLocationId = computed(() => {
  const fromRoute = route.params.locationId as string | undefined;
  if (fromRoute) return fromRoute;
  const list = locations.locationList;
  return list.find((l) => l.kind === "global")?.id ?? list[0]?.id ?? null;
});

function chooseLocation(id: string) {
  if (id !== activeLocationId.value) router.push(`/loadout/${id}`);
}

function refresh() {
  const id = activeLocationId.value;
  if (id) loadout.load(id);
}

onMounted(() => {
  if (locations.locationList.length === 0) locations.fetchList();
});
watch(activeLocationId, refresh, { immediate: true });

const counted = computed(() =>
  [...loadout.countedGroups].sort((a, b) => b.tokenEstimate - a.tokenEstimate)
);
const unverified = computed(() => loadout.uncountedGroups);
const total = computed(() => loadout.loadout?.tokenEstimate ?? 0);

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

function tint(g: LoadoutGroup): string {
  return tints.value.get(`${g.origin}-${g.label}`) ?? "var(--load-6)";
}

/** Live circuits first, then command-only, then the vetoed ones. */
function ordered(group: LoadoutGroup): ResolvedSkill[] {
  const rank = (s: ResolvedSkill) => (s.vetoedBy ? 2 : s.modelFacing ? 0 : 1);
  return [...group.skills].sort(
    (a, b) => rank(a) - rank(b) || a.id.localeCompare(b.id)
  );
}

const usedShare = computed(() => {
  const u = loadout.usage;
  if (!u?.available || !u.linkedCount) return null;
  return Math.round((u.usedHereCount / u.linkedCount) * 100);
});

const overrideProblems = computed(() => {
  const l = loadout.loadout;
  if (!l) return 0;
  return l.deadOverrides.length + l.unreachableOverrides.length;
});
</script>

<template>
  <div class="schedule-view">
    <!-- ── Header ──────────────────────────────────────────────── -->
    <header class="head">
      <div class="head-id">
        <h1 class="head-title">Schedule</h1>
        <label class="picker">
          <span class="sr-only">Sub-panel</span>
          <select
            class="picker-select"
            :value="activeLocationId ?? ''"
            @change="chooseLocation(($event.target as HTMLSelectElement).value)"
          >
            <option v-for="l in locations.locationList" :key="l.id" :value="l.id">
              {{ l.label }}
            </option>
          </select>
        </label>
      </div>

      <button class="rescan" type="button" @click="refresh">
        <PanelIcon name="rescan" :size="13" />
        Rescan
      </button>
    </header>

    <div v-if="loadout.isLoading" class="loading" aria-busy="true">
      <span class="plate-bare">Reading the board…</span>
    </div>

    <template v-else-if="loadout.loadout">
      <!-- ── Standing conditions ────────────────────────────────── -->
      <p v-if="loadout.loadout.settingsUnreadable" class="notice notice-caution">
        <PanelIcon name="caution" :size="14" />
        <span>
          Kit couldn't read <code>~/.claude/settings.json</code>, so it can't see
          which skills you've switched off. Everything below is what's on disk,
          not necessarily what loads.
        </span>
      </p>

      <p class="summary">
        <strong class="rating tabular">{{ loadout.loadout.modelFacingCount }}</strong>
        skills reach the model here, drawing about
        <strong class="rating tabular">{{ total.toLocaleString() }}</strong>
        tokens every session.
        <template v-if="loadout.loadout.commandOnlyCount">
          Another {{ loadout.loadout.commandOnlyCount }} are command-only and cost
          nothing until called.
        </template>
        <template v-if="usedShare !== null">
          Of the {{ loadout.usage!.linkedCount }} linked here,
          {{ loadout.usage!.usedHereCount }} have actually run.
        </template>
        <template v-else-if="loadout.usage && !loadout.usage.available">
          Kit found no usage logs, so it can't say what's earning its place —
          that's missing data, not zero use.
        </template>
      </p>

      <!-- ── Conflicts ──────────────────────────────────────────── -->
      <details v-if="loadout.conflictCount" class="fold fold-caution">
        <summary>
          <PanelIcon name="caution" :size="14" />
          <span class="fold-count rating tabular">{{ loadout.conflictCount }}</span>
          <span class="fold-title">linked here, but switched off globally</span>
          <span class="fold-more">Show</span>
        </summary>
        <p class="fold-body">
          A <code>skillOverrides</code> entry is an upstream breaker: it cuts the
          circuit whatever this panel says. These look active and are not.
          Unlinking them instead keeps per-project tailoring working.
        </p>
        <ul class="chips">
          <li v-for="s in loadout.loadout.vetoed" :key="s.path">{{ s.folderName }}</li>
        </ul>
      </details>

      <details v-if="overrideProblems" class="fold">
        <summary>
          <PanelIcon name="declared" :size="14" />
          <span class="fold-count rating tabular">{{ overrideProblems }}</span>
          <span class="fold-title">overrides that aren't doing what you think</span>
          <span class="fold-more">Show</span>
        </summary>
        <template v-if="loadout.loadout.deadOverrides.length">
          <p class="fold-body">
            <strong>Pointing at nothing here.</strong> No skill by these names is
            in Global, your installed plugins, or this location — so here they do
            nothing. Kit hasn't looked in your other projects, and a skill in one
            of those would still be switched off.
          </p>
          <ul class="chips">
            <li v-for="n in loadout.loadout.deadOverrides" :key="n">{{ n }}</li>
          </ul>
        </template>
        <template v-if="loadout.loadout.unreachableOverrides.length">
          <p class="fold-body">
            <strong>Can't bite.</strong> These exist only under a
            <code>plugin:skill</code> name, which a bare-name override never
            matches. They are still loading.
          </p>
          <ul class="chips">
            <li v-for="n in loadout.loadout.unreachableOverrides" :key="n">{{ n }}</li>
          </ul>
        </template>
      </details>

      <!-- ── The schedule proper ────────────────────────────────── -->
      <section
        v-for="group in counted"
        :key="`${group.origin}-${group.label}`"
        class="circuit-group"
      >
        <div class="group-head">
          <span class="swatch" :style="{ background: tint(group) }" aria-hidden="true" />
          <h2 class="group-name">{{ group.label }}</h2>
          <span class="plate">{{ group.origin }}</span>
          <span class="group-draw">
            <span class="rating tabular">{{ group.tokenEstimate.toLocaleString() }}</span>
            <span class="rating-unit">tokens</span>
          </span>
        </div>

        <ol class="rows">
          <li
            v-for="(skill, i) in ordered(group)"
            :key="skill.path"
            class="row"
            :class="{ 'row-off': skill.vetoedBy, 'row-quiet': !skill.modelFacing }"
          >
            <span class="position tabular">{{ String(i + 1).padStart(2, "0") }}</span>
            <span class="row-name">{{ skill.id }}</span>

            <span v-if="skill.vetoedBy" class="tag-lockout row-state">
              <PanelIcon name="lockout" :size="11" />
              Cut off
            </span>
            <span v-else-if="!skill.modelFacing" class="plate row-state">Command only</span>

            <span class="row-draw rating tabular">
              <template v-if="skill.vetoedBy">—</template>
              <template v-else-if="!skill.modelFacing">0</template>
              <template v-else>{{ skill.tokenEstimate.toLocaleString() }}</template>
            </span>
          </li>
        </ol>
      </section>

      <!-- ── Not counted ────────────────────────────────────────── -->
      <section v-if="unverified.length" class="unverified unsurveyed">
        <div class="unverified-head">
          <span class="plate">Not counted</span>
          <p class="unverified-why">{{ unverified[0].caveat }}</p>
        </div>
        <ul class="packs">
          <li v-for="g in unverified" :key="g.label">
            <span class="pack-name">{{ g.label }}</span>
            <span class="pack-count tabular">{{ g.modelFacingCount }}</span>
          </li>
        </ul>
      </section>
    </template>
  </div>
</template>

<style scoped>
.schedule-view {
  height: 100%;
  overflow-y: auto;
  padding: var(--space-6) var(--space-7) var(--space-9);
  max-width: 1000px;
}

/* ── Header ───────────────────────────────────────────────── */

.head {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: var(--space-5);
  padding-bottom: var(--space-4);
  border-bottom: 1px solid var(--border-default);
}

.head-id {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  min-width: 0;
}

.head-title {
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.picker-select {
  font-family: var(--font-sans);
  font-size: var(--text-lg);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  background: transparent;
  border: 0;
  padding: 0;
  cursor: pointer;
  max-width: 34ch;
}

.rescan {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  background: var(--surface-panel);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  padding: var(--space-2) var(--space-3);
  cursor: pointer;
  flex-shrink: 0;
}

.rescan:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.loading {
  padding: var(--space-7) 0;
}

/* ── Summary ──────────────────────────────────────────────── */

.summary {
  font-size: var(--text-lg);
  line-height: 1.55;
  color: var(--text-secondary);
  margin: var(--space-5) 0 0;
  max-width: 68ch;
  text-wrap: pretty;
}

.summary strong {
  color: var(--text-primary);
}

.notice {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  padding: var(--space-3) var(--space-4);
  margin: var(--space-5) 0 0;
  max-width: 68ch;
  text-wrap: pretty;
}

.notice-caution {
  border-color: color-mix(in srgb, var(--warning) 42%, transparent);
  background: var(--warning-subtle);
}

.notice-caution :deep(.icon) {
  color: var(--warning);
  margin-top: 1px;
}

code {
  font-family: var(--font-mono);
  font-size: 0.92em;
  color: var(--text-primary);
}

/* ── Folds ────────────────────────────────────────────────── */

.fold {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  padding: var(--space-3) var(--space-4);
  margin-top: var(--space-4);
}

.fold-caution {
  border-color: color-mix(in srgb, var(--warning) 42%, transparent);
}

.fold summary {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  cursor: default;
  list-style: none;
}

.fold summary::-webkit-details-marker {
  display: none;
}

.fold-caution summary :deep(.icon) {
  color: var(--warning);
}

.fold-count {
  font-size: var(--text-lg);
  color: var(--text-primary);
  min-width: 1.6em;
}

.fold-title {
  flex: 1;
  font-size: var(--text-md);
  color: var(--text-primary);
}

.fold-more {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.fold[open] .fold-more {
  color: var(--accent);
}

.fold-body {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: var(--space-4) 0 var(--space-3);
  max-width: 68ch;
  text-wrap: pretty;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  list-style: none;
  margin: 0 0 var(--space-2);
  padding: 0;
}

.chips li {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xs);
  padding: 1px var(--space-2);
}

/* ── Circuit groups ───────────────────────────────────────── */

.circuit-group {
  margin-top: var(--space-8);
}

.group-head {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--border-default);
}

.swatch {
  width: 8px;
  height: 8px;
  flex-shrink: 0;
}

.group-name {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.group-draw {
  margin-left: auto;
  display: inline-flex;
  align-items: baseline;
  font-size: var(--text-md);
}

.rows {
  list-style: none;
  margin: 0;
  padding: 0;
}

.row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-2);
  border-bottom: 1px solid var(--border-subtle);
  font-size: var(--text-md);
}

.row:hover {
  background: var(--surface-hover);
}

.row-name {
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-state {
  flex-shrink: 0;
}

/* Cut off: the row reads as struck through, because that is what it is. */
.row-off .row-name {
  color: var(--text-tertiary);
  text-decoration: line-through;
  text-decoration-thickness: 1px;
  text-decoration-color: var(--border-strong);
}

.row-quiet .row-name {
  color: var(--text-secondary);
}

.row-draw {
  margin-left: auto;
  min-width: 5ch;
  text-align: right;
  flex-shrink: 0;
  font-size: var(--text-md);
}

.row-off .row-draw,
.row-quiet .row-draw {
  color: var(--text-tertiary);
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
  align-items: flex-start;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.unverified-why {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
  max-width: 68ch;
  text-wrap: pretty;
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
</style>

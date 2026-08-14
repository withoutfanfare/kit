<script setup lang="ts">
/**
 * Loadout — the full detail behind Panel.
 *
 * Panel answers "what is the state of things". This answers "which skills,
 * exactly, and what does each cost". Three facts it has to make plain, because
 * all three are invisible on the filesystem:
 *   · a skill can be linked and still not load (switched off globally);
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
const uncertain = computed(() => loadout.loadout?.settingsUnreadable === true);

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

const tint = (g: LoadoutGroup) =>
  tints.value.get(`${g.origin}-${g.label}`) ?? "var(--load-6)";

/** Loading first, then command-only, then the ones that aren't loading. */
function ordered(group: LoadoutGroup): ResolvedSkill[] {
  const rank = (s: ResolvedSkill) => (s.vetoedBy ? 2 : s.modelFacing ? 0 : 1);
  return [...group.skills].sort(
    (a, b) => rank(a) - rank(b) || a.id.localeCompare(b.id)
  );
}

const usedShare = computed(() => {
  const u = loadout.usage;
  if (!u?.available || !u.linkedCount) return null;
  return u;
});

const overrideProblems = computed(() => {
  const l = loadout.loadout;
  if (!l) return 0;
  return l.deadOverrides.length + l.unreachableOverrides.length;
});

const originLabel: Record<string, string> = {
  global: "Every session",
  project: "This project",
  plugin: "Plugin",
  account: "Account pack",
};
</script>

<template>
  <div class="view">
    <div class="wrap">
      <header class="head">
        <div class="head-main">
          <h1 class="page-title">Loadout</h1>
          <div class="picker-wrap">
            <select
              class="picker"
              :value="activeLocationId ?? ''"
              aria-label="Location"
              @change="chooseLocation(($event.target as HTMLSelectElement).value)"
            >
              <option v-for="l in locations.locationList" :key="l.id" :value="l.id">
                {{ l.label }}
              </option>
            </select>
            <PanelIcon name="chevron" :size="12" class="picker-chevron" />
          </div>
        </div>
        <button class="btn btn-secondary btn-sm" type="button" @click="refresh">
          <PanelIcon name="rescan" :size="13" />
          Rescan
        </button>
      </header>

      <div v-if="loadout.isLoading" class="loading" aria-busy="true">
        <div class="skeleton sk-line"></div>
        <div class="skeleton sk-row"></div>
        <div class="skeleton sk-row"></div>
        <div class="skeleton sk-row"></div>
      </div>

      <template v-else-if="loadout.loadout">
        <p v-if="uncertain" class="notice">
          <PanelIcon name="caution" :size="14" />
          <span>
            Kit couldn't read <code>~/.claude/settings.json</code>, so it can't
            tell which skills you've switched off. Everything below is what's on
            disk — not necessarily what loads.
          </span>
        </p>

        <p v-else class="summary">
          <strong>{{ loadout.loadout.modelFacingCount }}</strong> skills reach the
          model here, drawing about <strong>{{ total.toLocaleString() }}</strong>
          tokens every session.<template v-if="loadout.loadout.commandOnlyCount">
            Another {{ loadout.loadout.commandOnlyCount }} are command-only and
            cost nothing until called.</template>
          <template v-if="usedShare">
            Of the {{ usedShare.linkedCount }} linked here,
            {{ usedShare.usedHereCount }} have actually run.</template>
          <template v-else-if="loadout.usage && !loadout.usage.available">
            Kit found no usage logs, so it can't say what's earning its place —
            that's missing data, not zero use.</template>
        </p>

        <section v-if="loadout.conflictCount || overrideProblems" class="alerts">
          <details v-if="loadout.conflictCount" class="alert alert-warn">
            <summary>
              <PanelIcon name="caution" :size="15" />
              <span class="alert-text">
                <strong>
                  {{ loadout.conflictCount }} linked here but switched off globally
                </strong>
                They look active and aren't loading.
              </span>
              <span class="alert-more">Show</span>
            </summary>
            <div class="alert-body">
              <p>
                A <code>skillOverrides</code> entry in your Claude Code settings
                beats every shortcut, so these stay off even though this location
                asks for them. Unlinking them here keeps per-project tailoring
                working.
              </p>
              <ul class="chips">
                <li v-for="s in loadout.loadout.vetoed" :key="s.path">
                  {{ s.folderName }}
                </li>
              </ul>
            </div>
          </details>

          <details v-if="overrideProblems" class="alert">
            <summary>
              <PanelIcon name="declared" :size="15" />
              <span class="alert-text">
                <strong>{{ overrideProblems }} overrides aren't doing what you think</strong>
                Entries that point at nothing, or can't take effect.
              </span>
              <span class="alert-more">Show</span>
            </summary>
            <div class="alert-body">
              <template v-if="loadout.loadout.deadOverrides.length">
                <p>
                  <strong>Pointing at nothing here.</strong> No skill by these
                  names is in Global, your installed plugins, or this location.
                  Kit hasn't looked in your other projects, and a skill in one of
                  those would still be switched off.
                </p>
                <ul class="chips">
                  <li v-for="n in loadout.loadout.deadOverrides" :key="n">{{ n }}</li>
                </ul>
              </template>
              <template v-if="loadout.loadout.unreachableOverrides.length">
                <p>
                  <strong>Can't take effect.</strong> These exist only under a
                  <code>plugin:skill</code> name, which a bare-name override never
                  matches. They are still loading.
                </p>
                <ul class="chips">
                  <li v-for="n in loadout.loadout.unreachableOverrides" :key="n">
                    {{ n }}
                  </li>
                </ul>
              </template>
            </div>
          </details>
        </section>

        <section
          v-for="group in counted"
          :key="`${group.origin}-${group.label}`"
          class="section"
        >
          <div class="section-head">
            <span class="swatch" :style="{ background: tint(group) }" aria-hidden="true"></span>
            <h2 class="section-title">{{ group.label }}</h2>
            <span class="badge">{{ originLabel[group.origin] }}</span>
            <span class="section-aside group-total num">
              {{ group.tokenEstimate.toLocaleString() }}
              <span class="unit">tokens</span>
            </span>
          </div>

          <ul class="rows">
            <li
              v-for="skill in ordered(group)"
              :key="skill.path"
              class="row"
              :class="{ off: skill.vetoedBy, quiet: !skill.modelFacing }"
            >
              <span class="skill-name">{{ skill.id }}</span>

              <span v-if="skill.vetoedBy" class="badge badge-warn">
                <PanelIcon name="lockout" :size="11" />
                Not loading
              </span>
              <span v-else-if="!skill.modelFacing" class="badge">Command only</span>

              <span class="skill-cost num">
                <template v-if="skill.vetoedBy">—</template>
                <template v-else-if="!skill.modelFacing">0</template>
                <template v-else>{{ skill.tokenEstimate.toLocaleString() }}</template>
              </span>
            </li>
          </ul>
        </section>

        <section v-if="unverified.length" class="section">
          <div class="section-head">
            <h2 class="section-title">Not counted</h2>
            <span class="section-count">{{ unverified.length }}</span>
          </div>
          <p class="note">{{ unverified[0].caveat }}</p>
          <ul class="packs">
            <li v-for="g in unverified" :key="g.label">
              <span>{{ g.label }}</span>
              <span class="num">{{ g.modelFacingCount }}</span>
            </li>
          </ul>
        </section>
      </template>
    </div>
  </div>
</template>

<style scoped>
.view {
  height: 100%;
  overflow-y: auto;
}

.wrap {
  max-width: 880px;
  padding: var(--space-9) var(--space-9) var(--space-12);
}

.head {
  display: flex;
  align-items: center;
  gap: var(--space-6);
  margin-bottom: var(--space-7);
}

.head-main {
  display: flex;
  align-items: baseline;
  gap: var(--space-4);
  min-width: 0;
}

.picker-wrap {
  position: relative;
  display: inline-flex;
  align-items: center;
}

.picker {
  appearance: none;
  font-family: var(--font-sans);
  font-size: var(--text-lg);
  font-weight: var(--weight-medium);
  letter-spacing: var(--track-snug);
  color: var(--k-text-3);
  background: transparent;
  border: 0;
  border-radius: var(--radius-md);
  padding: 2px 22px 2px 6px;
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-inout),
    color var(--duration-fast) var(--ease-inout);
}

.picker:hover {
  background: var(--k-layer-2);
  color: var(--k-text);
}

.picker-chevron {
  position: absolute;
  right: 5px;
  color: var(--k-text-4);
  pointer-events: none;
  transform: rotate(90deg);
}

.head .btn {
  margin-left: auto;
  flex-shrink: 0;
}

.loading {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
}

.sk-line {
  height: 20px;
  width: 70%;
}

.sk-row {
  height: 32px;
  width: 100%;
}

.summary {
  font-size: var(--text-base);
  line-height: var(--lh-normal);
  color: var(--k-text-3);
  margin: 0 0 var(--space-8);
  max-width: 68ch;
  text-wrap: pretty;
}

.summary strong {
  font-weight: var(--weight-semibold);
  color: var(--k-text);
}

.notice {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  margin: 0 0 var(--space-8);
  padding: var(--space-5);
  border-radius: var(--radius-lg);
  background: var(--k-warn-quiet);
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--k-warn) 30%, transparent);
  font-size: var(--text-md);
  line-height: var(--lh-snug);
  color: var(--k-text-2);
  max-width: 68ch;
}

.notice :deep(.icon) {
  color: var(--k-warn);
  flex-shrink: 0;
  margin-top: 1px;
}

code {
  font-family: var(--font-mono);
  font-size: 0.92em;
  color: var(--k-text);
}

/* Alerts — the same component as Panel, with a disclosure. */
.alerts {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-bottom: var(--space-8);
}

.alert {
  border-radius: var(--radius-lg);
  background: var(--k-layer-1);
  box-shadow: inset 0 0 0 1px var(--k-line);
  transition: box-shadow var(--duration-fast) var(--ease-inout);
}

.alert-warn {
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--k-warn) 30%, transparent);
}

.alert summary {
  display: flex;
  align-items: center;
  gap: var(--space-5);
  padding: var(--space-5);
  cursor: pointer;
  list-style: none;
  border-radius: var(--radius-lg);
}

.alert summary::-webkit-details-marker {
  display: none;
}

.alert summary:hover {
  background: var(--k-layer-2);
}

.alert :deep(.icon) {
  color: var(--k-text-4);
  flex-shrink: 0;
}

.alert-warn :deep(.icon) {
  color: var(--k-warn);
}

.alert-text {
  display: flex;
  flex-direction: column;
  gap: 1px;
  flex: 1;
  min-width: 0;
  font-size: var(--text-md);
  color: var(--k-text-4);
}

.alert-text strong {
  font-weight: var(--weight-medium);
  color: var(--k-text);
}

.alert-more {
  font-size: var(--text-md);
  color: var(--k-text-4);
  flex-shrink: 0;
}

.alert[open] .alert-more {
  color: var(--k-accent);
}

.alert-body {
  padding: 0 var(--space-5) var(--space-5);
}

.alert-body p {
  font-size: var(--text-md);
  line-height: var(--lh-snug);
  color: var(--k-text-3);
  margin: 0 0 var(--space-4);
  max-width: 68ch;
}

.alert-body p strong {
  color: var(--k-text-2);
  font-weight: var(--weight-medium);
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-3);
  list-style: none;
  margin: 0 0 var(--space-5);
  padding: 0;
}

.chips li {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--k-text-2);
  background: var(--k-layer-2);
  box-shadow: inset 0 0 0 1px var(--k-line);
  border-radius: var(--radius-sm);
  padding: 2px var(--space-4);
}

/* Groups */
.swatch {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  flex-shrink: 0;
}

.group-total {
  font-size: var(--text-md);
  color: var(--k-text-2);
}

.unit {
  color: var(--k-text-4);
  margin-left: 2px;
}

.skill-name {
  flex: 1;
  min-width: 0;
  color: var(--k-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row.off .skill-name {
  color: var(--k-text-4);
  text-decoration: line-through;
  text-decoration-color: var(--k-line-heavy);
}

.row.quiet .skill-name {
  color: var(--k-text-3);
}

.skill-cost {
  color: var(--k-text-2);
  min-width: 5ch;
  text-align: right;
  flex-shrink: 0;
}

.row.off .skill-cost,
.row.quiet .skill-cost {
  color: var(--k-text-4);
}

/* Not counted */
.note {
  font-size: var(--text-md);
  line-height: var(--lh-snug);
  color: var(--k-text-4);
  margin: 0 0 var(--space-5);
  max-width: 68ch;
}

.packs {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: var(--space-2) var(--space-6);
  list-style: none;
  margin: 0;
  padding: 0;
}

.packs li {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-3) 0;
  font-size: var(--text-md);
  color: var(--k-text-3);
  border-bottom: 1px solid var(--k-line);
}

.packs .num {
  color: var(--k-text-4);
}
</style>

<script setup lang="ts">
/**
 * Panel — the overview Kit opens on.
 *
 * The concept survives from the previous pass: what loads here, what it costs,
 * what needs attention, and what Kit could not verify. The drawing is new.
 *
 * Two things it must never do: present a figure as definitive when the
 * settings that determine it could not be read, and let a stale loadout sit
 * under a newly-chosen location's name.
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

const counted = computed(() =>
  [...loadout.countedGroups].sort((a, b) => b.tokenEstimate - a.tokenEstimate)
);
const unverified = computed(() => loadout.uncountedGroups);
const total = computed(() => loadout.loadout?.tokenEstimate ?? 0);
const unverifiedTotal = computed(() =>
  unverified.value.reduce((n, g) => n + g.tokenEstimate, 0)
);

/** True when the totals below rest on settings Kit could not read. */
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

const share = (g: LoadoutGroup) =>
  total.value ? (g.tokenEstimate / total.value) * 100 : 0;

const conflicts = computed(() => loadout.loadout?.vetoed ?? []);
const issueCount = computed(
  () => (health.result?.errorCount ?? 0) + (health.result?.warningCount ?? 0)
);

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
        <div class="picker-wrap">
          <select v-model="activeId" class="picker" aria-label="Location">
            <option v-for="l in locations.locationList" :key="l.id" :value="l.id">
              {{ l.label }}
            </option>
          </select>
          <PanelIcon name="chevron" :size="13" class="picker-chevron" />
        </div>
        <p class="path" :title="activeLocation?.path">
          {{ activeLocation?.path ?? "—" }}
        </p>
      </header>

      <div v-if="loadout.isLoading" class="loading" aria-busy="true">
        <div class="skeleton sk-figure"></div>
        <div class="skeleton sk-meter"></div>
        <div class="skeleton sk-row"></div>
        <div class="skeleton sk-row"></div>
        <div class="skeleton sk-row"></div>
      </div>

      <template v-else-if="loadout.loadout">
        <section class="figure-block" :class="{ uncertain }">
          <div class="figure-row">
            <span class="figure num">
              <template v-if="uncertain">—</template>
              <template v-else>{{ total.toLocaleString() }}</template>
            </span>
            <span class="figure-side">
              <span class="figure-unit">tokens per session</span>
              <span class="figure-sub">
                <template v-if="uncertain">Can't be calculated right now</template>
                <template v-else>
                  {{ loadout.loadout.modelFacingCount }} skills reach the model<template
                    v-if="loadout.loadout.commandOnlyCount"
                  >
                    · {{ loadout.loadout.commandOnlyCount }} command-only</template>
                </template>
              </span>
            </span>
          </div>

          <p v-if="uncertain" class="uncertain-note">
            <PanelIcon name="caution" :size="14" />
            <span>
              Kit couldn't read <code>~/.claude/settings.json</code>, so it can't
              tell which skills you've switched off. The sources below are what's
              on disk — not necessarily what loads.
            </span>
          </p>

          <div
            v-else
            class="meter"
            role="img"
            :aria-label="`Load by source, ${total} tokens in total`"
          >
            <span
              v-for="g in counted"
              :key="`${g.origin}-${g.label}`"
              class="meter-seg"
              :style="{ width: `${share(g)}%`, background: tint(g) }"
              :title="`${g.label} — ${g.tokenEstimate.toLocaleString()} tokens`"
            ></span>
          </div>
        </section>

        <section v-if="conflicts.length || issueCount" class="alerts">
          <RouterLink v-if="conflicts.length" to="/loadout" class="alert alert-warn">
            <PanelIcon name="caution" :size="15" />
            <span class="alert-text">
              <strong>{{ conflicts.length }} linked here but switched off globally</strong>
              They look active and aren't loading.
            </span>
            <PanelIcon name="chevron" :size="14" class="alert-go" />
          </RouterLink>

          <RouterLink v-if="issueCount" to="/health" class="alert">
            <PanelIcon name="health" :size="15" />
            <span class="alert-text">
              <strong>
                {{ issueCount }} {{ issueCount === 1 ? "issue" : "issues" }} to resolve
              </strong>
              Broken links and declarations that don't match.
            </span>
            <PanelIcon name="chevron" :size="14" class="alert-go" />
          </RouterLink>
        </section>

        <section class="section">
          <div class="section-head">
            <h2 class="section-title">Sources</h2>
            <span class="section-count">{{ counted.length }}</span>
            <RouterLink :to="`/loadout/${activeId}`" class="section-aside link">
              Full loadout
              <PanelIcon name="chevron" :size="12" />
            </RouterLink>
          </div>

          <ul class="rows">
            <li v-for="g in counted" :key="`c-${g.origin}-${g.label}`" class="row src">
              <span class="swatch" :style="{ background: tint(g) }" aria-hidden="true"></span>
              <span class="src-name">{{ g.label }}</span>
              <span class="badge">{{ originLabel[g.origin] }}</span>
              <span class="src-skills num">{{ g.modelFacingCount }}</span>
              <span class="src-tokens num">{{ g.tokenEstimate.toLocaleString() }}</span>
            </li>
          </ul>
        </section>

        <section v-if="unverified.length" class="section">
          <div class="section-head">
            <h2 class="section-title">Not counted</h2>
            <span class="section-count">~{{ unverifiedTotal.toLocaleString() }} tokens</span>
          </div>
          <p class="note">
            Kit can see these but can't tell whether they're switched on, so
            they're left out of the figure above.
          </p>
          <ul class="packs">
            <li v-for="g in unverified" :key="`u-${g.label}`">
              <span class="pack-name">{{ g.label }}</span>
              <span class="pack-count num">{{ g.modelFacingCount }}</span>
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
  margin-bottom: var(--space-9);
}

.picker-wrap {
  position: relative;
  display: inline-flex;
  align-items: center;
  margin-left: -6px;
}

.picker {
  appearance: none;
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  letter-spacing: var(--track-tight);
  color: var(--k-text);
  background: transparent;
  border: 0;
  border-radius: var(--radius-md);
  padding: 2px 26px 2px 6px;
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-inout);
}

.picker:hover {
  background: var(--k-layer-2);
}

.picker-chevron {
  position: absolute;
  right: 7px;
  color: var(--k-text-4);
  pointer-events: none;
  transform: rotate(90deg);
}

.path {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--k-text-4);
  margin: var(--space-2) 0 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loading {
  display: flex;
  flex-direction: column;
  gap: var(--space-6);
}

.sk-figure {
  height: 40px;
  width: 220px;
}

.sk-meter {
  height: 8px;
  width: 100%;
}

.sk-row {
  height: 32px;
  width: 100%;
}

.figure-block {
  margin-bottom: var(--space-9);
}

.figure-row {
  display: flex;
  align-items: baseline;
  gap: var(--space-5);
  margin-bottom: var(--space-6);
}

.figure {
  font-size: var(--text-3xl);
  font-weight: var(--weight-semibold);
  letter-spacing: var(--track-tight);
  line-height: 1;
  color: var(--k-text);
}

.uncertain .figure {
  color: var(--k-text-4);
}

.figure-side {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.figure-unit {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--k-text-2);
}

.figure-sub {
  font-size: var(--text-md);
  color: var(--k-text-4);
}

.uncertain-note {
  display: flex;
  align-items: flex-start;
  gap: var(--space-4);
  margin: 0;
  padding: var(--space-5);
  border-radius: var(--radius-lg);
  background: var(--k-warn-quiet);
  box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--k-warn) 30%, transparent);
  font-size: var(--text-md);
  line-height: var(--lh-snug);
  color: var(--k-text-2);
  max-width: 68ch;
}

.uncertain-note :deep(.icon) {
  color: var(--k-warn);
  flex-shrink: 0;
  margin-top: 1px;
}

code {
  font-family: var(--font-mono);
  font-size: 0.92em;
  color: var(--k-text);
}

.alerts {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-bottom: var(--space-9);
}

.alert {
  display: flex;
  align-items: center;
  gap: var(--space-5);
  padding: var(--space-5);
  border-radius: var(--radius-lg);
  background: var(--k-layer-1);
  box-shadow: inset 0 0 0 1px var(--k-line);
  text-decoration: none;
  transition: background var(--duration-fast) var(--ease-inout),
    box-shadow var(--duration-fast) var(--ease-inout);
}

.alert:hover {
  background: var(--k-layer-2);
  box-shadow: inset 0 0 0 1px var(--k-line-strong);
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

.link {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--k-text-3);
  text-decoration: none;
}

.link:hover {
  color: var(--k-accent);
}

.src {
  cursor: default;
}

.swatch {
  width: 8px;
  height: 8px;
  border-radius: 2px;
  flex-shrink: 0;
}

.src-name {
  color: var(--k-text);
  font-weight: var(--weight-medium);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.src-skills {
  margin-left: auto;
  color: var(--k-text-4);
  flex-shrink: 0;
}

.src-tokens {
  color: var(--k-text-2);
  min-width: 6ch;
  text-align: right;
  flex-shrink: 0;
}

.note {
  font-size: var(--text-md);
  color: var(--k-text-4);
  margin: 0 0 var(--space-5);
  max-width: 64ch;
  line-height: var(--lh-snug);
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

.pack-count {
  color: var(--k-text-4);
}
</style>

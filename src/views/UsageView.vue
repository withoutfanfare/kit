<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useUsageStore } from "@/stores/usageStore";
import { SButton, SEmptyState } from "@stuntrocket/ui";
import type { UsageSkillRow } from "@/types";

const usageStore = useUsageStore();
const showAll = ref(false);

/** The busiest skill sets the scale every other bar is read against. */
const busiest = computed(() => usageStore.report?.rows[0]?.runs ?? 1);

const rows = computed<UsageSkillRow[]>(() => {
  const all = usageStore.report?.rows ?? [];
  return showAll.value ? all : all.slice(0, 20);
});

const hiddenCount = computed(() =>
  Math.max(0, (usageStore.report?.rows.length ?? 0) - rows.value.length)
);

const since = computed(() => {
  const raw = usageStore.report?.recordedSince;
  if (!raw) return null;
  return new Date(raw).toLocaleDateString("en-GB", {
    day: "numeric",
    month: "long",
    year: "numeric",
  });
});

/** Plain relative time — "3 days ago" beats a timestamp for a keep-or-drop call. */
function lastUsed(iso: string | null): string {
  if (!iso) return "never";
  const days = Math.floor((Date.now() - new Date(iso).getTime()) / 86_400_000);
  if (days <= 0) return "today";
  if (days === 1) return "yesterday";
  if (days < 31) return `${days} days ago`;
  const months = Math.round(days / 30);
  return months === 1 ? "a month ago" : `${months} months ago`;
}

/** Older than a month, but it did run once — the "probably drop it" band. */
function isStale(row: UsageSkillRow): boolean {
  if (!row.lastUsedAt) return true;
  return Date.now() - new Date(row.lastUsedAt).getTime() > 30 * 86_400_000;
}

onMounted(() => usageStore.fetchReport());
</script>

<template>
  <div class="usage">
    <div v-if="usageStore.isLoading && !usageStore.report" class="skeleton" aria-busy="true">
      <div class="sk sk-title"></div>
      <div class="sk sk-row"></div>
      <div class="sk sk-row"></div>
      <div class="sk sk-row"></div>
    </div>

    <SEmptyState
      v-else-if="usageStore.report && !usageStore.report.available"
      title="No usage logs yet"
      description="Kit reads the log the Skill hook writes into your library. Nothing has been recorded there yet, so there's nothing to judge — that's missing data, not zero use."
    />

    <template v-else-if="usageStore.report">
      <header class="head">
        <h1 class="page-title">What you actually use</h1>
        <p class="head-line">
          <strong class="figure">{{ usageStore.report.distinctSkills }}</strong>
          skills have run,
          <strong class="figure">{{ usageStore.report.eventCount.toLocaleString() }}</strong>
          times in total<template v-if="since"> since {{ since }}</template>.
          <template v-if="usageStore.report.neverUsed.length">
            <strong class="figure">{{ usageStore.report.neverUsed.length }}</strong>
            more have never run at all.
          </template>
        </p>
      </header>

      <section class="ranked" aria-label="Skills by number of runs">
        <!-- Column heads: three unlabelled columns of numbers and dates left
             the reader guessing which was which. -->
        <div class="col-heads" aria-hidden="true">
          <span class="plate-bare">Skill</span>
          <span />
          <span class="plate-bare col-runs">Runs</span>
          <span class="plate-bare">Last run</span>
          <span class="plate-bare">Most in</span>
        </div>

        <ol class="rows">
          <li v-for="row in rows" :key="row.skill" class="row" :class="{ stale: isStale(row) }">
            <span class="row-name" :title="row.skill">
              {{ row.skill }}
              <span v-if="row.skill.includes(':')" class="plate row-plate">Plugin</span>
            </span>
            <span class="row-bar">
              <span
                class="row-fill"
                :style="{ transform: `scaleX(${row.runs / busiest})` }"
              />
            </span>
            <span class="row-runs">{{ row.runs }}</span>
            <span class="row-when">{{ lastUsed(row.lastUsedAt) }}</span>
            <span class="row-where">{{ row.projects[0]?.name ?? "—" }}</span>
          </li>
        </ol>

        <SButton v-if="hiddenCount > 0" size="sm" @click="showAll = true">
          Show {{ hiddenCount }} more
        </SButton>
      </section>

      <!-- The decision list: in the library, costing context, never once used. -->
      <section v-if="usageStore.report.neverUsed.length" class="never">
        <div class="never-head">
          <h3 class="never-title">Never used</h3>
          <span class="never-count">{{ usageStore.report.neverUsed.length }}</span>
        </div>
        <p class="never-line">
          In your library and never called once in
          {{ usageStore.report.eventCount.toLocaleString() }} recorded runs. Leaving
          these unlinked costs you nothing and keeps them a click away.
        </p>
        <ul class="chips">
          <li v-for="name in usageStore.report.neverUsed" :key="name">{{ name }}</li>
        </ul>
      </section>

      <footer class="foot">
        <SButton size="sm" @click="usageStore.fetchReport()">Reload log</SButton>
      </footer>
    </template>
  </div>
</template>

<style scoped>
.usage {
  height: 100%;
  overflow-y: auto;
  padding: var(--space-6) var(--space-6) var(--space-10);
  max-width: 920px;
}

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

/* ── Ranked list ────────────────────────────────────────── */

.rows {
  list-style: none;
  margin: 0 0 var(--space-3);
  padding: 0;
}

/* Heads share the row grid exactly, so a column label always sits over its
   column whatever the window width. */
.col-heads {
  display: grid;
  grid-template-columns: minmax(0, 19rem) minmax(3rem, 1fr) 3rem 7rem 8rem;
  align-items: baseline;
  gap: var(--space-3);
  padding: 0 var(--space-2) var(--space-2);
  border-bottom: 1px solid var(--border-default);
  margin-bottom: var(--space-2);
}

.col-runs {
  text-align: right;
}

.row-plate {
  font-size: 9px;
  padding: 1px var(--space-2) 0;
  margin-left: var(--space-2);
}

.row {
  display: grid;
  grid-template-columns: minmax(0, 19rem) minmax(3rem, 1fr) 3rem 7rem 8rem;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-2);
  border-radius: var(--radius-sm);
  font-size: var(--text-sm);
}

.row:hover {
  background: var(--surface-hover);
}

.row-name {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-bar {
  position: relative;
  height: 6px;
  border-radius: var(--radius-full);
  background: var(--surface-hover);
  overflow: hidden;
}

.row:hover .row-bar {
  background: var(--border-subtle);
}

.row-fill {
  position: absolute;
  inset: 0;
  background: var(--accent);
  border-radius: inherit;
  transform-origin: left center;
}

.stale .row-fill {
  background: var(--text-tertiary);
  opacity: 0.6;
}

.row-runs {
  text-align: right;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}

.row-when,
.row-where {
  color: var(--text-tertiary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.stale .row-name {
  color: var(--text-secondary);
}

/* ── Never used ─────────────────────────────────────────── */

.never {
  margin-top: var(--space-8);
  padding-top: var(--space-4);
  border-top: 1px solid var(--border-subtle);
}

.never-head {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  margin-bottom: var(--space-2);
}

.never-title {
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.never-count {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.never-line {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0 0 var(--space-3);
  max-width: 68ch;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-1) var(--space-2);
  list-style: none;
  margin: 0;
  padding: 0;
}

.chips li {
  font-size: var(--text-xs);
  color: var(--text-secondary);
  background: var(--surface-hover);
  border-radius: var(--radius-xs);
  padding: 2px var(--space-2);
}

.foot {
  padding-top: var(--space-6);
}

/* ── Skeleton ───────────────────────────────────────────── */

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
.sk-row {
  height: 40px;
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

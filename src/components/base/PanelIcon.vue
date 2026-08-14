<script setup lang="ts">
/**
 * Kit's icon set, drawn as engineering line-work.
 *
 * One weight, one cap style, one grid. They are not decorative: each is the
 * schematic symbol for the thing it names, which is why the loadout icon is a
 * bus with branches and the lock-out icon is a padlock on a hasp rather than a
 * generic shield. Everything is stroked on a 16-unit grid at 1.25 so the
 * weight sits beside 11–13px text without shouting.
 */

withDefaults(
  defineProps<{
    name: IconName;
    size?: number;
  }>(),
  { size: 16 }
);
</script>

<script lang="ts">
export type IconName =
  | "panel"
  | "location"
  | "library"
  | "loadout"
  | "usage"
  | "health"
  | "changelog"
  | "help"
  | "settings"
  | "search"
  | "add"
  | "check"
  | "close"
  | "caution"
  | "lockout"
  | "linked"
  | "broken"
  | "local"
  | "declared"
  | "chevron"
  | "external"
  | "rescan"
  | "compare";
</script>

<template>
  <svg
    class="icon"
    :width="size"
    :height="size"
    viewBox="0 0 16 16"
    fill="none"
    stroke="currentColor"
    stroke-width="1.25"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
    focusable="false"
  >
    <!-- The board itself: an enclosure with two rows of circuits. -->
    <template v-if="name === 'panel'">
      <rect x="2.5" y="2.5" width="11" height="11" rx="1" />
      <path d="M5.5 5.5h2M5.5 8h2M5.5 10.5h2M9 5.5h1.5M9 8h1.5M9 10.5h1.5" />
    </template>

    <!-- A sub-panel: an enclosure fed from outside. -->
    <template v-else-if="name === 'location'">
      <rect x="3" y="5" width="10" height="8.5" rx="1" />
      <path d="M8 5V2.5M5.5 8.5h5" />
    </template>

    <!-- Stacked plates. -->
    <template v-else-if="name === 'library'">
      <rect x="2.5" y="3" width="11" height="3.5" rx="0.5" />
      <rect x="2.5" y="9.5" width="11" height="3.5" rx="0.5" />
    </template>

    <!-- The bus with branches: what is actually drawing current. -->
    <template v-else-if="name === 'loadout'">
      <path d="M4 2.5v11" />
      <path d="M4 5h5.5M4 8h8M4 11h3.5" />
    </template>

    <!-- A run chart: measurement over time. -->
    <template v-else-if="name === 'usage'">
      <path d="M2.5 13.5h11" />
      <path d="M3.5 10.5l3-3 2.5 2 3.5-4.5" />
    </template>

    <!-- Continuity test: a probe closing a circuit. -->
    <template v-else-if="name === 'health'">
      <circle cx="8" cy="8" r="5.5" />
      <path d="M5.5 8h1.5l1-2 1 4 1-2h1.5" />
    </template>

    <template v-else-if="name === 'changelog'">
      <path d="M4 2.5h6l2.5 2.5v8.5H4z" />
      <path d="M6.5 7.5h4M6.5 10h2.5" />
    </template>

    <template v-else-if="name === 'help'">
      <circle cx="8" cy="8" r="5.5" />
      <path d="M6.4 6.3a1.7 1.7 0 013.2.7c0 1.1-1.6 1.4-1.6 2.4" />
      <path d="M8 11.6v.01" />
    </template>

    <template v-else-if="name === 'settings'">
      <circle cx="8" cy="8" r="2" />
      <path d="M8 2v1.6M8 12.4V14M14 8h-1.6M3.6 8H2M12.2 3.8l-1.1 1.1M4.9 11.1l-1.1 1.1M12.2 12.2l-1.1-1.1M4.9 4.9L3.8 3.8" />
    </template>

    <template v-else-if="name === 'search'">
      <circle cx="7" cy="7" r="4.5" />
      <path d="M10.4 10.4L13.5 13.5" />
    </template>

    <template v-else-if="name === 'add'">
      <path d="M8 3.5v9M3.5 8h9" />
    </template>

    <template v-else-if="name === 'check'">
      <path d="M3.5 8.5l3 3 6-7" />
    </template>

    <template v-else-if="name === 'close'">
      <path d="M4 4l8 8M12 4l-8 8" />
    </template>

    <!-- Caution: the triangle, because that is what caution looks like. -->
    <template v-else-if="name === 'caution'">
      <path d="M8 2.5l5.5 10h-11z" />
      <path d="M8 6.5v3M8 11.2v.01" />
    </template>

    <!-- Lock-out: a padlock through a hasp. Do not throw this breaker. -->
    <template v-else-if="name === 'lockout'">
      <rect x="3.5" y="7" width="9" height="6.5" rx="1" />
      <path d="M5.75 7V5.25a2.25 2.25 0 014.5 0V7" />
    </template>

    <!-- Linked: a closed circuit. -->
    <template v-else-if="name === 'linked'">
      <path d="M6.5 9.5l3-3" />
      <path d="M9 4.5l.75-.75a2.5 2.5 0 013.5 3.5L12.5 8" />
      <path d="M7 11.5l-.75.75a2.5 2.5 0 01-3.5-3.5L3.5 8" />
    </template>

    <!-- Broken: the same circuit, open. -->
    <template v-else-if="name === 'broken'">
      <path d="M9 4.5l.75-.75a2.5 2.5 0 013.5 3.5L12.5 8" />
      <path d="M7 11.5l-.75.75a2.5 2.5 0 01-3.5-3.5L3.5 8" />
      <path d="M6 6l1.25 1.25M10 10L8.75 8.75" />
    </template>

    <!-- Local only: present, but not fed from the library. -->
    <template v-else-if="name === 'local'">
      <rect x="3" y="3" width="10" height="10" rx="1" stroke-dasharray="2.2 1.8" />
    </template>

    <!-- Declared only: a space on the schedule with nothing wired to it. -->
    <template v-else-if="name === 'declared'">
      <rect x="3" y="3" width="10" height="10" rx="1" />
      <path d="M6 6l4 4M10 6l-4 4" opacity="0.45" />
    </template>

    <template v-else-if="name === 'chevron'">
      <path d="M6 3.5L10.5 8L6 12.5" />
    </template>

    <template v-else-if="name === 'external'">
      <path d="M9 3.5h3.5V7" />
      <path d="M12.5 3.5L7.5 8.5" />
      <path d="M11 9.5v3h-8v-8h3" />
    </template>

    <template v-else-if="name === 'rescan'">
      <path d="M13 8a5 5 0 11-1.6-3.7" />
      <path d="M13.2 2.8v2.6h-2.6" />
    </template>

    <template v-else-if="name === 'compare'">
      <path d="M8 2.5v11" />
      <rect x="2.5" y="4.5" width="3.5" height="7" rx="0.5" />
      <rect x="10" y="4.5" width="3.5" height="7" rx="0.5" />
    </template>
  </svg>
</template>

<style scoped>
.icon {
  display: block;
  flex-shrink: 0;
}
</style>

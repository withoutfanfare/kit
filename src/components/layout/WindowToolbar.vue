<script setup lang="ts">
/**
 * The window's top bar.
 *
 * Carries the app identity, which library Kit is reading — the first question
 * anyone asks when the numbers look wrong — and the search field. Previously
 * this was an engraved "rating plate" with a pilot lamp; both were costume.
 */
import { computed } from "vue";
import GlobalSearchResults from "@/components/domain/GlobalSearchResults.vue";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useWatcherStore } from "@/stores/watcherStore";

const preferences = usePreferencesStore();
const watcher = useWatcherStore();

const libraryName = computed(() => {
  const root = preferences.libraryRoot?.replace(/\/+$/, "") ?? "";
  if (!root) return "No library set";
  return root.split("/").pop() || root;
});

const watching = computed(() => watcher.status === "active");
</script>

<template>
  <header class="topbar titlebar-drag-region">
    <div class="identity">
      <span class="mark">Kit</span>
      <span class="sep" aria-hidden="true">/</span>
      <span class="library" :title="preferences.libraryRoot || undefined">
        {{ libraryName }}
      </span>
      <span
        class="dot"
        :class="{ live: watching }"
        :title="watching ? 'Watching the library for changes' : 'Not watching the library'"
      >
        <span class="sr-only">
          {{ watching ? "Watching the library for changes" : "Not watching the library" }}
        </span>
      </span>
    </div>

    <div class="search">
      <GlobalSearchResults />
    </div>

    <div class="actions">
      <slot name="actions" />
    </div>
  </header>
</template>

<style scoped>
.topbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 40;
  height: var(--toolbar-height);
  display: flex;
  align-items: center;
  gap: var(--space-7);
  /* Traffic lights own the first 76px. */
  padding: 0 var(--space-6) 0 76px;
  background: var(--k-bg);
  border-bottom: 1px solid var(--k-line);
}

.identity {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex-shrink: 0;
  min-width: 0;
}

.mark {
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  letter-spacing: var(--track-snug);
  color: var(--k-text);
}

.sep {
  color: var(--k-text-4);
}

.library {
  font-size: var(--text-md);
  color: var(--k-text-3);
  max-width: 20ch;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Watcher state. Colour plus a title and screen-reader text — never colour
   on its own. */
.dot {
  width: 6px;
  height: 6px;
  border-radius: var(--radius-full);
  background: var(--k-layer-4);
  flex-shrink: 0;
}

.dot.live {
  background: var(--k-ok);
}

.search {
  flex: 1;
  min-width: 0;
  max-width: 420px;
}

.actions {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: var(--space-4);
  flex-shrink: 0;
}

@media (max-width: 1120px) {
  .sep,
  .library {
    display: none;
  }
}
</style>

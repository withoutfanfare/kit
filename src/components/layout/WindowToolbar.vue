<script setup lang="ts">
/**
 * The board's header.
 *
 * Every distribution board carries a rating plate saying which board this is
 * and what feeds it. Kit's equivalent is the library root: the one fact that
 * determines what every other screen can possibly show. It was previously
 * buried in Settings, which meant the app never told you which library it was
 * reading — the first question anyone asks when the numbers look wrong.
 */
import { computed } from "vue";
import GlobalSearchResults from "@/components/domain/GlobalSearchResults.vue";
import PanelIcon from "@/components/base/PanelIcon.vue";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useWatcherStore } from "@/stores/watcherStore";

const preferences = usePreferencesStore();
const watcher = useWatcherStore();

/** The library's own folder name; the full path is the title attribute. */
const libraryName = computed(() => {
  const root = preferences.libraryRoot?.replace(/\/+$/, "") ?? "";
  if (!root) return "No library set";
  return root.split("/").pop() || root;
});

const watching = computed(() => watcher.status === "active");
</script>

<template>
  <header class="board-head titlebar-drag-region">
    <div class="head-left">
      <span class="plate board-plate" :title="preferences.libraryRoot || undefined">
        <span class="board-mark">Kit</span>
        <span class="board-sep" aria-hidden="true">/</span>
        <span class="board-lib">{{ libraryName }}</span>
      </span>
      <span
        class="watch"
        :class="{ live: watching }"
        :title="watching ? 'Watching the library for changes' : 'Not watching the library'"
      >
        <span class="watch-lamp" aria-hidden="true" />
        <span class="sr-only">
          {{ watching ? "Watching the library for changes" : "Not watching the library" }}
        </span>
      </span>
    </div>

    <div class="head-search">
      <GlobalSearchResults />
    </div>

    <div class="head-right">
      <slot name="actions">
        <span class="plate-bare head-hint">
          <PanelIcon name="search" :size="12" />
          <kbd>⌘K</kbd>
        </span>
      </slot>
    </div>
  </header>
</template>

<style scoped>
.board-head {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 40;
  height: var(--toolbar-height);
  display: flex;
  align-items: center;
  gap: var(--space-5);
  /* Traffic lights own the first 76px. */
  padding: 0 var(--space-5) 0 76px;
  background: var(--surface-sidebar);
  border-bottom: 1px solid var(--border-subtle);
}

.head-left {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-shrink: 0;
}

/* The rating plate. Engraved, not a logo. */
.board-plate {
  gap: var(--space-2);
  max-width: 260px;
  overflow: hidden;
}

.board-mark {
  color: var(--plate-ink);
  letter-spacing: 0.18em;
}

.board-sep {
  color: var(--text-tertiary);
  opacity: 0.6;
}

.board-lib {
  color: var(--text-secondary);
  font-weight: var(--weight-medium);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* A pilot lamp: on when the watcher is running. Never colour alone — the
   title and the screen-reader text both say which state this is. */
.watch {
  display: inline-flex;
  align-items: center;
}

.watch-lamp {
  width: 6px;
  height: 6px;
  border-radius: var(--radius-full);
  background: transparent;
  border: 1px solid var(--border-strong);
}

.watch.live .watch-lamp {
  background: var(--bus);
  border-color: var(--bus);
  box-shadow: 0 0 0 3px var(--bus-glow);
}

.head-search {
  flex: 1;
  min-width: 0;
  max-width: 460px;
}

.head-right {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-shrink: 0;
}

.head-hint {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--text-tertiary);
}

.head-hint kbd {
  font-family: var(--font-plate);
  font-size: var(--text-xs);
  letter-spacing: 0.08em;
}

@media (max-width: 1120px) {
  .board-plate .board-sep,
  .board-plate .board-lib {
    display: none;
  }

  .head-hint {
    display: none;
  }
}
</style>

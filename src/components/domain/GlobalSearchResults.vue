<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { useRouter } from "vue-router";
import { useAppStore } from "@/stores/appStore";
import { useLibraryStore } from "@/stores/libraryStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useSetsStore } from "@/stores/setsStore";
import { setKeyFromSummary } from "@/utils/setKey";

type SearchResult = {
  key: string;
  type: "location" | "skill" | "set";
  id: string;
  name: string;
  detail: string;
  setKey?: string;
};

type ResultGroup = {
  label: string;
  results: SearchResult[];
};

const router = useRouter();
const appStore = useAppStore();
const libraryStore = useLibraryStore();
const locationsStore = useLocationsStore();
const preferencesStore = usePreferencesStore();
const setsStore = useSetsStore();
const root = ref<HTMLElement | null>(null);
const selectedIndex = ref(0);
let hasLoadedOnFocus = false;

const query = computed(() => appStore.globalSearchQuery.trim().toLowerCase());

function matches(fields: Array<string | null | undefined>): boolean {
  return !query.value || fields.some((field) => field?.toLowerCase().includes(query.value));
}

function skillPath(id: string): string {
  const root = preferencesStore.libraryRoot.replace(/\/+$/, "");
  return preferencesStore.libraryRoot ? `${root}/${id}` : id;
}

const groups = computed<ResultGroup[]>(() => [
  {
    label: "Locations",
    results: locationsStore.locationList
      .filter((location) => matches([location.label, location.path]))
      .slice(0, 6)
      .map((location) => ({
        key: `location:${location.id}`,
        type: "location",
        id: location.id,
        name: location.label,
        detail: location.path,
      })),
  },
  {
    label: "Skills",
    results: libraryStore.items
      .filter((item) => item.kind === "skill")
      .filter((item) => matches([item.name, skillPath(item.id), item.summary, ...item.tags]))
      .slice(0, 6)
      .map((item) => ({
        key: `skill:${item.id}`,
        type: "skill",
        id: item.id,
        name: item.name,
        detail: item.summary ?? skillPath(item.id),
      })),
  },
  {
    label: "Sets",
    results: setsStore.items
      .filter((set) => matches([set.name, set.path, set.description]))
      .slice(0, 6)
      .map((set) => {
        const setKey = setKeyFromSummary(set);
        return {
          key: `set:${setKey}`,
          type: "set",
          id: set.id,
          name: set.name,
          detail: set.description ?? set.path,
          setKey,
        };
      }),
  },
]);

const visibleGroups = computed(() => groups.value.filter((group) => group.results.length > 0));
const results = computed(() => visibleGroups.value.flatMap((group) => group.results));
const isLoading = computed(() => libraryStore.isLoading || setsStore.isLoading);
const activeOptionId = computed(() =>
  appStore.isGlobalSearchOpen && results.value.length > 0
    ? `global-search-option-${selectedIndex.value}`
    : undefined
);

watch([query, () => results.value.length], () => {
  selectedIndex.value = 0;
});

function handleFocus() {
  appStore.openGlobalSearch();
  if (hasLoadedOnFocus) return;
  hasLoadedOnFocus = true;
  if (libraryStore.items.length === 0) libraryStore.fetchItems();
  if (setsStore.items.length === 0) setsStore.fetchSets();
}

function updateQuery(value: string) {
  appStore.globalSearchQuery = value;
  appStore.openGlobalSearch();
}

function handleInput(event: Event) {
  updateQuery((event.target as HTMLInputElement).value);
}

function resultIndex(result: SearchResult): number {
  return results.value.findIndex((item) => item.key === result.key);
}

async function selectResult(result: SearchResult) {
  if (result.type === "location") {
    locationsStore.selectLocation(result.id);
    appStore.closeGlobalSearch();
    await router.push(`/locations/${result.id}`);
  } else if (result.type === "skill") {
    libraryStore.selectSkill(result.id);
    appStore.closeGlobalSearch();
    await router.push(`/skills/${result.id}`);
  } else if (result.setKey) {
    setsStore.selectSet(result.setKey);
    appStore.closeGlobalSearch();
    await router.push(`/sets/${encodeURIComponent(result.setKey)}`);
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    event.preventDefault();
    event.stopPropagation();
    appStore.closeGlobalSearch();
    return;
  }

  if (event.key === "ArrowDown" || event.key === "ArrowUp") {
    event.preventDefault();
    event.stopPropagation();
    appStore.openGlobalSearch();
    if (results.value.length === 0) return;
    const direction = event.key === "ArrowDown" ? 1 : -1;
    selectedIndex.value =
      (selectedIndex.value + direction + results.value.length) % results.value.length;
    return;
  }

  if (event.key === "Enter" && appStore.isGlobalSearchOpen) {
    event.preventDefault();
    event.stopPropagation();
    const result = results.value[selectedIndex.value];
    if (result) selectResult(result);
  }
}

function handlePointerDown(event: PointerEvent) {
  if (!root.value?.contains(event.target as Node)) appStore.closeGlobalSearch();
}

onMounted(() => document.addEventListener("pointerdown", handlePointerDown));
onUnmounted(() => document.removeEventListener("pointerdown", handlePointerDown));
</script>

<template>
  <div
    ref="root"
    class="global-search"
    data-global-search
  >
    <svg
      class="search-icon"
      viewBox="0 0 20 20"
      fill="currentColor"
      aria-hidden="true"
    >
      <path
        fill-rule="evenodd"
        d="M9 3.5a5.5 5.5 0 1 0 0 11 5.5 5.5 0 0 0 0-11ZM2 9a7 7 0 1 1 12.452 4.391l3.328 3.329a.75.75 0 1 1-1.06 1.06l-3.329-3.328A7 7 0 0 1 2 9Z"
        clip-rule="evenodd"
      />
    </svg>
    <input
      :value="appStore.globalSearchQuery"
      type="search"
      class="global-search-input"
      placeholder="Search skills, sets and locations"
      role="combobox"
      aria-label="Search skills, sets and locations"
      aria-haspopup="listbox"
      aria-autocomplete="list"
      :aria-expanded="appStore.isGlobalSearchOpen"
      aria-controls="global-search-results"
      :aria-activedescendant="activeOptionId"
      autocomplete="off"
      spellcheck="false"
      @focus="handleFocus"
      @input="handleInput"
      @keydown="handleKeydown"
    />
    <kbd class="shortcut-hint">⌘K</kbd>

    <div
      v-if="appStore.isGlobalSearchOpen"
      id="global-search-results"
      class="search-results"
      role="listbox"
      aria-label="Global search results"
    >
      <template v-if="visibleGroups.length > 0">
        <section
          v-for="group in visibleGroups"
          :key="group.label"
          class="result-group"
          role="group"
          :aria-label="group.label"
        >
          <div class="group-label">{{ group.label }}</div>
          <button
            v-for="result in group.results"
            :id="`global-search-option-${resultIndex(result)}`"
            :key="result.key"
            type="button"
            class="result-row"
            :class="{ selected: resultIndex(result) === selectedIndex }"
            role="option"
            :aria-selected="resultIndex(result) === selectedIndex"
            @mouseenter="selectedIndex = resultIndex(result)"
            @click="selectResult(result)"
          >
            <span class="result-name">{{ result.name }}</span>
            <span class="result-detail">{{ result.detail }}</span>
          </button>
        </section>
      </template>
      <div v-else class="empty-result">
        {{ isLoading ? "Loading…" : "No results found" }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.global-search {
  position: relative;
  width: 280px;
}

.search-icon {
  position: absolute;
  top: 6px;
  left: var(--space-2);
  z-index: 1;
  width: 16px;
  height: 16px;
  color: var(--text-tertiary);
  pointer-events: none;
}

.global-search-input {
  width: 100%;
  height: 28px;
  padding: 0 42px 0 28px;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  outline: none;
  background: var(--surface-panel);
  color: var(--text-primary);
  font: inherit;
  font-size: var(--text-sm);
  transition: border-color var(--duration-fast) var(--ease-default),
    box-shadow var(--duration-fast) var(--ease-default);
}

.global-search-input::placeholder {
  color: var(--text-tertiary);
}

.global-search-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

.global-search-input::-webkit-search-cancel-button {
  display: none;
}

.shortcut-hint {
  position: absolute;
  top: 6px;
  right: var(--space-2);
  color: var(--text-tertiary);
  font-family: inherit;
  font-size: var(--text-xs);
  pointer-events: none;
}

.search-results {
  position: absolute;
  top: calc(100% + var(--space-2));
  left: 0;
  z-index: 60;
  width: 360px;
  max-height: min(480px, calc(100vh - 72px));
  overflow-y: auto;
  padding: var(--space-1);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  box-shadow: var(--shadow-lg);
}

.result-group + .result-group {
  border-top: 1px solid var(--border-subtle);
}

.group-label {
  padding: var(--space-2) var(--space-2) var(--space-1);
  color: var(--text-tertiary);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.result-row {
  display: flex;
  width: 100%;
  flex-direction: column;
  gap: 1px;
  padding: var(--space-2);
  border: 0;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-primary);
  font: inherit;
  text-align: left;
  cursor: pointer;
}

.result-row:hover,
.result-row.selected {
  background: var(--surface-selected);
}

.result-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
}

.result-detail {
  overflow: hidden;
  max-width: 100%;
  color: var(--text-secondary);
  font-size: var(--text-xs);
  text-overflow: ellipsis;
  white-space: nowrap;
}

.empty-result {
  padding: var(--space-4) var(--space-3);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  text-align: center;
}
</style>

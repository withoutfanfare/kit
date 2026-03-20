<script setup lang="ts">
import { onMounted } from "vue";
import { useLibraryStore } from "@/stores/libraryStore";
import { useRouter } from "vue-router";
import SplitPaneLayout from "@/components/layout/SplitPaneLayout.vue";
import EmptyState from "@/components/layout/EmptyState.vue";
import SkillInspector from "@/components/domain/SkillInspector.vue";
import SearchField from "@/components/base/SearchField.vue";
import SegmentedControl from "@/components/base/SegmentedControl.vue";
import Badge from "@/components/base/Badge.vue";

const libraryStore = useLibraryStore();
const router = useRouter();

const filterOptions = [
  { label: "All", value: "all" },
  { label: "Skills", value: "skill" },
  { label: "Sets", value: "set" },
];

function selectItem(id: string, kind: string) {
  if (kind === "skill") {
    libraryStore.selectSkill(id);
    router.push(`/skills/${id}`);
  }
}

onMounted(() => {
  libraryStore.fetchItems();
});

</script>

<template>
  <SplitPaneLayout :show-inspector="false">
    <template #sidebar>
      <div class="library-sidebar">
        <div class="sidebar-controls">
          <SearchField
            v-model="libraryStore.searchQuery"
            placeholder="Search library..."
            compact
          />
          <SegmentedControl
            v-model="libraryStore.filterKind"
            :options="filterOptions"
          />
        </div>
        <div class="sidebar-items">
          <div
            v-for="item in libraryStore.filteredItems"
            :key="item.id"
            class="library-row"
            :class="{
              selected: item.id === libraryStore.selectedSkillId,
              archived: item.archived,
            }"
            @click="selectItem(item.id, item.kind)"
          >
            <div class="row-content">
              <span class="row-name">{{ item.name }}</span>
              <span v-if="item.summary" class="row-summary">{{ item.summary }}</span>
            </div>
            <div class="row-meta">
              <Badge v-if="item.archived" variant="default" compact>archived</Badge>
              <Badge :variant="item.kind === 'skill' ? 'accent' : 'default'" compact>
                {{ item.kind }}
              </Badge>
            </div>
          </div>
          <div v-if="libraryStore.filteredItems.length === 0 && !libraryStore.isLoading" class="list-empty">
            <span class="list-empty-text">No items found</span>
          </div>
        </div>
      </div>
    </template>
    <template #main>
      <router-view v-if="libraryStore.selectedSkillId" />
      <EmptyState
        v-else-if="libraryStore.items.length === 0 && !libraryStore.isLoading"
        title="No skills in library"
        description="Set your skill library root in Settings to browse and manage skills."
      />
      <EmptyState
        v-else-if="!libraryStore.isLoading"
        title="Select a skill"
        description="Choose a skill from the sidebar to see where it's used and manage it."
      />
    </template>
    <template #inspector>
      <SkillInspector
        v-if="libraryStore.selectedDetail"
        :detail="libraryStore.selectedDetail"
      />
    </template>
  </SplitPaneLayout>
</template>

<style scoped>
.library-sidebar {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.sidebar-controls {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.sidebar-items {
  flex: 1;
  overflow-y: auto;
  padding: var(--space-1);
}

.library-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.library-row:hover {
  background: var(--surface-hover);
}

.library-row.selected {
  background: var(--surface-selected);
}

.library-row.selected:hover {
  background: var(--surface-selected-strong);
}

.library-row.archived {
  opacity: 0.6;
}

.row-content {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.row-name {
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-summary {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.row-meta {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-shrink: 0;
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-6);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

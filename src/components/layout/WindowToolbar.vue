<script setup lang="ts">
import SearchField from "@/components/base/SearchField.vue";
import { useAppStore } from "@/stores/appStore";

const appStore = useAppStore();
</script>

<template>
  <div class="window-toolbar">
    <div class="toolbar-content">
      <SearchField
        v-model="appStore.globalSearchQuery"
        placeholder="Search skills, locations..."
        compact
      />
    </div>
  </div>
</template>

<style scoped>
.window-toolbar {
  height: var(--toolbar-height);
  flex-shrink: 0;
  display: flex;
  align-items: center;
  padding: 0 var(--space-4) 0 76px; /* right padding + left offset for macOS traffic lights */
  background: var(--surface-sidebar);
  border-bottom: 1px solid var(--border-subtle);
  -webkit-app-region: drag;
}

.toolbar-content {
  display: flex;
  align-items: center;
  -webkit-app-region: no-drag;
  flex-shrink: 0;
}
</style>

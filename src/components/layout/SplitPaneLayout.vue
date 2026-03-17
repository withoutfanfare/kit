<script setup lang="ts">
defineProps<{
  showInspector?: boolean;
}>();
</script>

<template>
  <div class="split-pane" :class="{ 'has-inspector': showInspector }">
    <div class="split-sidebar">
      <slot name="sidebar" />
    </div>
    <div class="split-main">
      <slot name="main" />
    </div>
    <div v-if="showInspector" class="split-inspector">
      <slot name="inspector" />
    </div>
  </div>
</template>

<style scoped>
.split-pane {
  display: flex;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.split-sidebar {
  width: 240px;
  flex-shrink: 0;
  border-right: 1px solid var(--border-subtle);
  overflow-y: auto;
  background: var(--surface-app);
}

.split-main {
  flex: 1;
  min-width: 0;
  overflow-y: auto;
  background: var(--surface-app);
}

.split-inspector {
  width: var(--inspector-width);
  flex-shrink: 0;
  border-left: 1px solid var(--border-subtle);
  overflow-y: auto;
  background: var(--surface-panel);
  padding: var(--space-4);
}
</style>

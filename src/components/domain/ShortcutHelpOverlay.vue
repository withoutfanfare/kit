<script setup lang="ts">
import { showShortcutHelp } from "@/composables/useKeyboardShortcuts";
import { SModal } from "@stuntrocket/ui";

const shortcuts = [
  { group: "Navigation", items: [
    { keys: ["Cmd", "1"], description: "Go to Locations" },
    { keys: ["Cmd", "2"], description: "Go to Library" },
    { keys: ["Cmd", "3"], description: "Go to Sets" },
    { keys: ["Cmd", "4"], description: "Go to Changelog" },
    { keys: ["Cmd", "5"], description: "Go to Health" },
  ]},
  { group: "List", items: [
    { keys: ["j"], description: "Move down in list" },
    { keys: ["k"], description: "Move up in list" },
    { keys: ["Enter"], description: "Open selected item" },
    { keys: ["/"], description: "Focus search input" },
  ]},
  { group: "General", items: [
    { keys: ["Cmd", "/"], description: "Toggle this help" },
    { keys: ["Esc"], description: "Close overlay" },
  ]},
];

function close() {
  showShortcutHelp.value = false;
}
</script>

<template>
  <SModal
    :open="showShortcutHelp"
    title="Keyboard shortcuts"
    @close="close"
  >
    <div class="shortcut-groups">
      <div v-for="group in shortcuts" :key="group.group" class="shortcut-group">
        <h3 class="group-title">{{ group.group }}</h3>
        <div class="shortcut-list">
          <div v-for="s in group.items" :key="s.description" class="shortcut-row">
            <span class="shortcut-keys">
              <kbd v-for="(key, i) in s.keys" :key="i" class="shortcut-key">{{ key }}</kbd>
            </span>
            <span class="shortcut-desc">{{ s.description }}</span>
          </div>
        </div>
      </div>
    </div>
  </SModal>
</template>

<style scoped>
.shortcut-groups {
  display: flex;
  flex-direction: column;
  gap: var(--space-5);
  padding: var(--space-2) 0;
}

.group-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 var(--space-2) 0;
}

.shortcut-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.shortcut-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-1) 0;
}

.shortcut-keys {
  display: flex;
  align-items: center;
  gap: 3px;
}

.shortcut-key {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 22px;
  height: 22px;
  padding: 0 var(--space-1);
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: 4px;
  font-family: inherit;
  font-size: 11px;
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  line-height: 1;
}

.shortcut-desc {
  font-size: var(--text-sm);
  color: var(--text-primary);
}
</style>

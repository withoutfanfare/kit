<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { LocationId } from "@/types";
import { SInlineTextField } from "@stuntrocket/ui";

const props = defineProps<{
  locationId: LocationId;
  entryKey: string;
  entryValue: string;
}>();

const emit = defineEmits<{
  updated: [value: string];
}>();

const error = ref<string | undefined>();
const saving = ref(false);

async function onSave(newValue: string) {
  if (newValue === props.entryValue) return;
  saving.value = true;
  error.value = undefined;
  try {
    await invoke("update_manifest_entry", {
      locationId: props.locationId,
      skillId: props.entryKey,
      action: newValue,
    });
    emit("updated", newValue);
  } catch (err) {
    error.value = err instanceof Error ? err.message : "Failed to save";
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <div class="manifest-entry-editor">
    <span class="entry-key">{{ entryKey }}</span>
    <div class="entry-value">
      <SInlineTextField
        :model-value="entryValue"
        :error="error"
        placeholder="Value"
        @update:model-value="onSave"
      />
      <span v-if="saving" class="saving-indicator">Saving...</span>
    </div>
  </div>
</template>

<style scoped>
.manifest-entry-editor {
  display: flex;
  align-items: baseline;
  gap: var(--space-3);
  padding: var(--space-1) 0;
}

.entry-key {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-secondary);
  flex-shrink: 0;
  min-width: 80px;
}

.entry-value {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
}

.saving-indicator {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  flex-shrink: 0;
}
</style>

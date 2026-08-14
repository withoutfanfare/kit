<script setup lang="ts">
import type { SetDetail } from "@/types";
import { useSetsStore } from "@/stores/setsStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { useRouter } from "vue-router";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";
import { SButton, SConfirmDialog } from "@stuntrocket/ui";

const props = defineProps<{
  detail: SetDetail;
}>();

const setsStore = useSetsStore();
const preferencesStore = usePreferencesStore();
const router = useRouter();
const showDeleteConfirm = ref(false);

async function openInEditor() {
  await invoke("open_path_in_editor", {
    path: props.detail.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function revealInFinder() {
  await invoke("reveal_in_finder", { path: props.detail.path });
}

async function confirmDelete() {
  await setsStore.deleteSet(
    props.detail.id,
    props.detail.scope,
    props.detail.ownerLocationId ?? undefined
  );
  showDeleteConfirm.value = false;
  router.push("/sets");
}
</script>

<template>
  <InspectorPanel title="Set">
    <!-- Only what the main pane cannot show: where the file physically lives.
         The skills, assigned locations and description are all in full there,
         and repeating them made neither copy authoritative. -->
    <div class="field">
      <span class="label">On disk</span>
      <span class="path">{{ detail.path }}</span>
    </div>

    <div class="field">
      <span class="label">Scope</span>
      <span class="badge">
        {{ detail.scope === "global" ? "Everywhere" : "This project" }}
      </span>
    </div>

    <div class="actions">
      <SButton variant="secondary" size="sm" @click="openInEditor">Open in Editor</SButton>
      <SButton variant="secondary" size="sm" @click="revealInFinder">Reveal in Finder</SButton>
      <SButton variant="secondary" size="sm" @click="showDeleteConfirm = true">Delete Set</SButton>
    </div>
  </InspectorPanel>

  <SConfirmDialog
    :open="showDeleteConfirm"
    title="Delete set?"
    :message="`This will permanently remove '${detail.name}' and unlink it from all locations.`"
    confirm-label="Delete"
    danger
    @confirm="confirmDelete"
    @cancel="showDeleteConfirm = false"
    @close="showDeleteConfirm = false"
  />
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 3px;
}

.path {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  word-break: break-all;
  line-height: 1.45;
}

.actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: var(--space-2);
}

.actions :deep(button) {
  width: 100%;
  justify-content: center;
}
</style>

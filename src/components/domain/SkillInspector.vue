<script setup lang="ts">
/**
 * The skill's rating plate and its controls.
 *
 * This panel used to repeat the summary, linked locations, sets and usage that
 * the main pane already shows in full, so the screen said everything twice and
 * neither copy was the authoritative one. It now carries only what the main
 * pane cannot: where the skill physically lives, whether it is archived, and
 * the actions that act on the folder itself.
 */
import { computed } from "vue";
import type { SkillDetail } from "@/types";
import { useLibraryStore } from "@/stores/libraryStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import InspectorPanel from "@/components/layout/InspectorPanel.vue";
import PanelIcon from "@/components/base/PanelIcon.vue";
import { SButton } from "@stuntrocket/ui";

const props = defineProps<{
  detail: SkillDetail;
}>();

const libraryStore = useLibraryStore();
const preferencesStore = usePreferencesStore();

/** Folder name last, path above it: the tail is what identifies the skill. */
const parentPath = computed(() => {
  const parts = props.detail.path.replace(/\/+$/, "").split("/");
  parts.pop();
  return parts.join("/");
});

const folderName = computed(
  () => props.detail.path.replace(/\/+$/, "").split("/").pop() ?? props.detail.path
);

async function toggleArchive() {
  if (props.detail.archived) {
    await libraryStore.unarchiveSkill(props.detail.id);
  } else {
    await libraryStore.archiveSkill(props.detail.id);
  }
}

async function openInEditor() {
  await invoke("open_path_in_editor", {
    path: props.detail.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function revealInFinder() {
  await invoke("reveal_in_finder", { path: props.detail.path });
}
</script>

<template>
  <InspectorPanel title="Skill">
    <div class="field">
      <span class="plate-bare">On disk</span>
      <span class="path-parent">{{ parentPath }}/</span>
      <span class="path-name">{{ folderName }}</span>
    </div>

    <div class="field">
      <span class="plate-bare">State</span>
      <span v-if="detail.archived" class="plate">Archived</span>
      <span v-else class="state-on">
        <PanelIcon name="check" :size="12" />
        In the library
      </span>
    </div>

    <div class="actions">
      <SButton variant="secondary" size="sm" @click="toggleArchive">
        {{ detail.archived ? "Unarchive" : "Archive" }}
      </SButton>
      <SButton variant="secondary" size="sm" @click="openInEditor">
        Open in editor
      </SButton>
      <SButton variant="secondary" size="sm" @click="revealInFinder">
        Reveal in Finder
      </SButton>
    </div>
  </InspectorPanel>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 3px;
}

.path-parent {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  word-break: break-all;
  line-height: 1.4;
}

.path-name {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--text-primary);
  word-break: break-all;
}

.state-on {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-sm);
  color: var(--success);
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

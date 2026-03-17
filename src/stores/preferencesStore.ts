import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { Preferences, UpdatePreferencesInput } from "@/types";

export const usePreferencesStore = defineStore("preferences", () => {
  const libraryRoot = ref<string | null>(null);
  const editorCommand = ref<string | null>(null);
  const defaultView = ref<"locations" | "skills">("locations");
  const showArchived = ref(false);

  async function update(input: UpdatePreferencesInput) {
    const result = await invoke<Preferences>("update_preferences", { prefs: input });
    libraryRoot.value = result.libraryRoot;
    editorCommand.value = result.editorCommand;
    defaultView.value = result.defaultView;
    showArchived.value = result.showArchived;
  }

  return {
    libraryRoot,
    editorCommand,
    defaultView,
    showArchived,
    update,
  };
});

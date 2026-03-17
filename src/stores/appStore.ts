import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { AppBootstrap } from "@/types";
import { useLocationsStore } from "./locationsStore";
import { usePreferencesStore } from "./preferencesStore";
import { useLibraryStore } from "./libraryStore";

export type ToastVariant = "success" | "error" | "info";

export type Toast = {
  id: number;
  message: string;
  variant: ToastVariant;
};

let toastId = 0;

export const useAppStore = defineStore("app", () => {
  const isBootstrapped = ref(false);
  const isLoading = ref(false);
  const globalSearchQuery = ref("");
  const globalError = ref<string | null>(null);
  const needsSetup = ref(false);
  const toasts = ref<Toast[]>([]);

  function toast(message: string, variant: ToastVariant = "info") {
    const id = ++toastId;
    toasts.value.push({ id, message, variant });
    setTimeout(() => {
      toasts.value = toasts.value.filter((t) => t.id !== id);
    }, 3000);
  }

  function dismissToast(id: number) {
    toasts.value = toasts.value.filter((t) => t.id !== id);
  }

  async function bootstrap() {
    isLoading.value = true;
    try {
      const data = await invoke<AppBootstrap>("get_app_bootstrap");

      const prefs = usePreferencesStore();
      prefs.libraryRoot = data.libraryRoot;
      prefs.editorCommand = data.editorCommand;
      prefs.defaultView = data.defaultView;
      prefs.showArchived = data.showArchived;

      const locations = useLocationsStore();
      locations.locationList = data.locations;

      const library = useLibraryStore();
      library.totalSkills = data.counts.skills;
      library.totalSets = data.counts.sets;

      needsSetup.value = !data.libraryRoot;
      isBootstrapped.value = true;
    } catch (err) {
      globalError.value =
        err instanceof Error ? err.message : "Failed to load app data";
    } finally {
      isLoading.value = false;
    }
  }

  function clearError() {
    globalError.value = null;
  }

  return {
    isBootstrapped,
    isLoading,
    globalSearchQuery,
    globalError,
    needsSetup,
    toasts,
    bootstrap,
    clearError,
    toast,
    dismissToast,
  };
});

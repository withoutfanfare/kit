import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { WatcherStatus, WatcherStatusResponse } from "@/types";
import { useLibraryStore } from "./libraryStore";
import { useAppStore } from "./appStore";

export const useWatcherStore = defineStore("watcher", () => {
  const status = ref<WatcherStatus>("stopped");
  const watchedPath = ref<string | null>(null);
  const isStarting = ref(false);

  let unlisten: (() => void) | null = null;

  async function start() {
    isStarting.value = true;
    try {
      const resp = await invoke<WatcherStatusResponse>("start_library_watcher");
      status.value = resp.status;
      watchedPath.value = resp.watchedPath;

      // Listen for library-changed events
      if (!unlisten) {
        unlisten = await listen("library-changed", () => {
          const library = useLibraryStore();
          library.fetchItems();
          useAppStore().toast("Library updated from filesystem", "info");
        });
      }
    } catch {
      status.value = "error";
    } finally {
      isStarting.value = false;
    }
  }

  async function stop() {
    try {
      const resp = await invoke<WatcherStatusResponse>("stop_library_watcher");
      status.value = resp.status;
      watchedPath.value = resp.watchedPath;
      if (unlisten) {
        unlisten();
        unlisten = null;
      }
    } catch {
      status.value = "error";
    }
  }

  async function fetchStatus() {
    try {
      const resp = await invoke<WatcherStatusResponse>("get_watcher_status");
      status.value = resp.status;
      watchedPath.value = resp.watchedPath;
    } catch {
      // Ignore
    }
  }

  return {
    status,
    watchedPath,
    isStarting,
    start,
    stop,
    fetchStatus,
  };
});

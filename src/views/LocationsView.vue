<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useLocationsStore } from "@/stores/locationsStore";
import { useRouter } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import SplitPaneLayout from "@/components/layout/SplitPaneLayout.vue";
import EmptyState from "@/components/layout/EmptyState.vue";
import LocationList from "@/components/domain/LocationList.vue";
import AssignmentSheet from "@/components/domain/AssignmentSheet.vue";

const locationsStore = useLocationsStore();
const router = useRouter();
const isDragging = ref(false);

async function addLocation() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose a project location",
  });
  if (selected && typeof selected === "string") {
    await locationsStore.addLocation(selected);
    if (locationsStore.selectedLocationId) {
      router.push(`/locations/${locationsStore.selectedLocationId}`);
    }
  }
}

async function addLocationByPath(path: string) {
  try {
    await locationsStore.addLocation(path);
    if (locationsStore.selectedLocationId) {
      router.push(`/locations/${locationsStore.selectedLocationId}`);
    }
  } catch {
    // silently ignore invalid paths (e.g. files, not directories)
  }
}

function onDragOver(e: DragEvent) {
  e.preventDefault();
  if (e.dataTransfer) {
    e.dataTransfer.dropEffect = "copy";
  }
  isDragging.value = true;
}

function onDragLeave(e: DragEvent) {
  // Only hide if we're leaving the view entirely, not entering a child
  const related = e.relatedTarget as Node | null;
  const target = e.currentTarget as Node;
  if (!related || !target.contains(related)) {
    isDragging.value = false;
  }
}

function onDrop(e: DragEvent) {
  e.preventDefault();
  isDragging.value = false;

  if (!e.dataTransfer) return;

  // Handle file:// URIs from Finder
  const uriList = e.dataTransfer.getData("text/uri-list");
  if (uriList) {
    for (const line of uriList.split("\n")) {
      const trimmed = line.trim();
      if (trimmed.startsWith("file://")) {
        const path = decodeURIComponent(trimmed.replace("file://", ""));
        addLocationByPath(path);
        return;
      }
    }
  }

  // Handle dropped files/folders via the Files API
  const items = e.dataTransfer.files;
  if (items.length > 0) {
    // Tauri exposes the path via webkitRelativePath or we get it from the name
    // In Tauri, dropped folders come through as file entries
    const item = items[0];
    // @ts-expect-error - Tauri extends File with path property
    const path = item.path as string | undefined;
    if (path) {
      addLocationByPath(path);
    }
  }
}

let unlistenDragDrop: UnlistenFn | null = null;

onMounted(async () => {
  locationsStore.fetchList();

  // Tauri native drag-drop gives us proper file paths on macOS
  const webview = getCurrentWebviewWindow();
  unlistenDragDrop = await webview.onDragDropEvent((event) => {
    if (event.payload.type === "enter") {
      isDragging.value = true;
    } else if (event.payload.type === "drop") {
      isDragging.value = false;
      const paths = event.payload.paths;
      if (paths.length > 0) {
        addLocationByPath(paths[0]);
      }
    } else if (event.payload.type === "leave") {
      isDragging.value = false;
    }
  });
});

onUnmounted(() => {
  unlistenDragDrop?.();
});
</script>

<template>
  <div
    class="locations-view"
    @dragover="onDragOver"
    @dragleave="onDragLeave"
    @drop="onDrop"
  >
    <div v-if="isDragging" class="drop-overlay">
      <div class="drop-zone">
        <svg width="32" height="32" viewBox="0 0 16 16" fill="none">
          <path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.29a1 1 0 01.7.29L8 4.3a1 1 0 00.71.29H12.5c.83 0 1.5.67 1.5 1.5v5.4c0 .83-.67 1.5-1.5 1.5h-9A1.5 1.5 0 012 11.5v-7z" fill="currentColor" opacity="0.5"/>
        </svg>
        <span class="drop-label">Drop a project folder to add it</span>
      </div>
    </div>
    <SplitPaneLayout>
      <template #sidebar>
        <LocationList />
      </template>
      <template #main>
        <router-view v-if="locationsStore.selectedLocationId" />
        <EmptyState
          v-else-if="locationsStore.locationList.length === 0"
          title="Add your first project"
          description="Drag a project folder here, or click the button below."
          action-label="Add Location"
          @action="addLocation"
        />
        <EmptyState
          v-else
          title="Select a location"
          description="Choose a project from the sidebar, or drag a folder here to add it."
        />
      </template>
    </SplitPaneLayout>
    <AssignmentSheet />
  </div>
</template>

<style scoped>
.locations-view {
  display: flex;
  flex: 1;
  min-height: 0;
  position: relative;
}

.drop-overlay {
  position: absolute;
  inset: 0;
  z-index: 50;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  backdrop-filter: blur(2px);
}

.drop-zone {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-8) var(--space-10);
  border: 2px dashed var(--accent);
  border-radius: var(--radius-lg);
  background: var(--surface-panel);
  color: var(--accent);
  box-shadow: var(--shadow-lg);
}

.drop-label {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}
</style>

import { onMounted, onUnmounted, ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useLibraryStore } from "@/stores/libraryStore";
import { useLocationsStore } from "@/stores/locationsStore";
import { useSetsStore } from "@/stores/setsStore";
import { setKeyFromSummary } from "@/utils/setKey";

export const showShortcutHelp = ref(false);

/** Active element tags where keyboard shortcuts should be suppressed. */
const INPUT_TAGS = new Set(["INPUT", "TEXTAREA", "SELECT"]);

function isInputFocused(): boolean {
  const el = document.activeElement;
  if (!el) return false;
  if (INPUT_TAGS.has(el.tagName)) return true;
  if ((el as HTMLElement).isContentEditable) return true;
  return false;
}

export function useKeyboardShortcuts() {
  const router = useRouter();
  const route = useRoute();
  const libraryStore = useLibraryStore();
  const locationsStore = useLocationsStore();
  const setsStore = useSetsStore();

  function handleKeydown(e: KeyboardEvent) {
    const meta = e.metaKey || e.ctrlKey;

    // Cmd+/ — toggle shortcut help overlay (works even in inputs)
    if (meta && e.key === "/") {
      e.preventDefault();
      showShortcutHelp.value = !showShortcutHelp.value;
      return;
    }

    // Escape — close shortcut help overlay
    if (e.key === "Escape" && showShortcutHelp.value) {
      e.preventDefault();
      showShortcutHelp.value = false;
      return;
    }

    // Don't capture shortcuts when typing in an input
    if (isInputFocused()) return;

    // Cmd+1 — Locations view
    if (meta && e.key === "1") {
      e.preventDefault();
      router.push("/locations");
      return;
    }

    // Cmd+2 — Skills / Library view
    if (meta && e.key === "2") {
      e.preventDefault();
      router.push("/skills");
      return;
    }

    // Cmd+3 — Sets view
    if (meta && e.key === "3") {
      e.preventDefault();
      router.push("/sets");
      return;
    }

    // Cmd+4 — Changelog view
    if (meta && e.key === "4") {
      e.preventDefault();
      router.push("/changelog");
      return;
    }

    // Cmd+5 — Health view
    if (meta && e.key === "5") {
      e.preventDefault();
      router.push("/health");
      return;
    }

    // / — focus search input
    if (e.key === "/") {
      e.preventDefault();
      const searchInput = document.querySelector<HTMLInputElement>(
        'input[type="search"], input[placeholder*="Search"]'
      );
      searchInput?.focus();
      return;
    }

    // j / k — navigate list items
    if (e.key === "j" || e.key === "k") {
      e.preventDefault();
      navigateList(e.key === "j" ? 1 : -1);
      return;
    }

    // Enter — open detail for selected item
    if (e.key === "Enter") {
      openSelectedDetail();
      return;
    }
  }

  function navigateList(direction: 1 | -1) {
    const currentPath = route.path;

    if (currentPath.startsWith("/locations")) {
      const list = locationsStore.locationList;
      if (list.length === 0) return;
      const currentIdx = list.findIndex(
        (l) => l.id === locationsStore.selectedLocationId
      );
      const nextIdx = Math.max(
        0,
        Math.min(list.length - 1, currentIdx + direction)
      );
      locationsStore.selectLocation(list[nextIdx].id);
      router.push(`/locations/${list[nextIdx].id}`);
    } else if (currentPath.startsWith("/skills")) {
      const list = libraryStore.filteredItems;
      if (list.length === 0) return;
      const currentIdx = list.findIndex(
        (i) => i.id === libraryStore.selectedSkillId
      );
      const nextIdx = Math.max(
        0,
        Math.min(list.length - 1, currentIdx + direction)
      );
      const item = list[nextIdx];
      if (item.kind === "skill") {
        libraryStore.selectSkill(item.id);
        router.push(`/skills/${item.id}`);
      }
    } else if (currentPath.startsWith("/sets")) {
      const list = setsStore.filteredItems;
      if (list.length === 0) return;
      const currentKey = setsStore.selectedSetKey;
      const currentIdx = currentKey
        ? list.findIndex((s) => setKeyFromSummary(s) === currentKey)
        : -1;
      const nextIdx = Math.max(
        0,
        Math.min(list.length - 1, currentIdx + direction)
      );
      const key = setKeyFromSummary(list[nextIdx]);
      setsStore.selectSet(key);
      router.push(`/sets/${encodeURIComponent(key)}`);
    }
  }

  function openSelectedDetail() {
    const currentPath = route.path;

    if (currentPath === "/locations" && locationsStore.selectedLocationId) {
      router.push(`/locations/${locationsStore.selectedLocationId}`);
    } else if (currentPath === "/skills" && libraryStore.selectedSkillId) {
      router.push(`/skills/${libraryStore.selectedSkillId}`);
    } else if (currentPath === "/sets" && setsStore.selectedSetKey) {
      router.push(`/sets/${encodeURIComponent(setsStore.selectedSetKey)}`);
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
}

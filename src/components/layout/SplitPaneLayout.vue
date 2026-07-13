<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { SButton } from "@stuntrocket/ui";

type CompactPane = "list" | "main" | "detail";

const props = withDefaults(defineProps<{
  showInspector?: boolean;
  compactPane?: CompactPane;
  backLabel?: string;
}>(), {
  compactPane: "list",
  backLabel: "list",
});

defineEmits<{
  back: [];
}>();

const inspectorOpen = ref(false);
const inspectorRef = ref<HTMLElement | null>(null);
const previousFocus = ref<HTMLElement | null>(null);
let compactMedia: MediaQueryList | null = null;

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape" && inspectorOpen.value) {
    inspectorOpen.value = false;
  }
}

function handleViewportChange(event: MediaQueryListEvent | MediaQueryList) {
  if (!event.matches) inspectorOpen.value = false;
}

watch(
  () => props.showInspector,
  (showInspector) => {
    if (!showInspector) inspectorOpen.value = false;
  }
);

watch(inspectorOpen, async (open) => {
  if (open) {
    previousFocus.value = document.activeElement as HTMLElement | null;
    document.addEventListener("keydown", handleKeydown);
    await nextTick();
    inspectorRef.value?.querySelector<HTMLElement>("button")?.focus();
  } else {
    document.removeEventListener("keydown", handleKeydown);
    await nextTick();
    const previous = previousFocus.value;
    if (previous?.isConnected && previous.getClientRects().length > 0) {
      previous.focus();
    } else {
      inspectorRef.value?.focus();
    }
    previousFocus.value = null;
  }
});

onMounted(() => {
  if (typeof window === "undefined" || typeof window.matchMedia !== "function") return;
  compactMedia = window.matchMedia("(max-width: 960px)");
  compactMedia.addEventListener("change", handleViewportChange);
  handleViewportChange(compactMedia);
});

onBeforeUnmount(() => {
  document.removeEventListener("keydown", handleKeydown);
  compactMedia?.removeEventListener("change", handleViewportChange);
});
</script>

<template>
  <div
    class="split-pane"
    :class="[`compact-${compactPane}`, { 'has-inspector': showInspector, 'inspector-open': inspectorOpen }]"
  >
    <div class="split-sidebar" :inert="inspectorOpen">
      <slot name="sidebar" />
    </div>
    <div class="split-main" :inert="inspectorOpen">
      <div v-if="compactPane === 'detail'" class="compact-toolbar">
        <SButton type="button" variant="secondary" size="sm" @click="$emit('back')">
          Back to {{ backLabel }}
        </SButton>
        <SButton
          v-if="showInspector"
          type="button"
          variant="secondary"
          size="sm"
          aria-controls="compact-inspector"
          :aria-expanded="inspectorOpen"
          @click="inspectorOpen = !inspectorOpen"
        >
          Details
        </SButton>
      </div>
      <slot name="main" />
    </div>
    <div
      v-if="showInspector"
      ref="inspectorRef"
      id="compact-inspector"
      class="split-inspector"
      :role="inspectorOpen ? 'dialog' : 'region'"
      :aria-modal="inspectorOpen ? 'true' : undefined"
      aria-label="Details"
      tabindex="-1"
    >
      <div class="inspector-close-row">
        <SButton type="button" variant="secondary" size="sm" @click="inspectorOpen = false">
          Close
        </SButton>
      </div>
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
  position: relative;
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

.compact-toolbar,
.inspector-close-row {
  display: none;
}

@media (max-width: 960px) {
  .split-sidebar {
    width: 100%;
    border-right: 0;
  }

  .compact-list .split-main,
  .compact-main .split-sidebar,
  .compact-detail .split-sidebar {
    display: none;
  }

  .compact-detail .compact-toolbar {
    position: sticky;
    top: 0;
    z-index: 1;
    display: flex;
    justify-content: space-between;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--border-subtle);
    background: var(--surface-app);
  }

  .split-inspector {
    position: absolute;
    inset: 0 0 0 auto;
    z-index: var(--z-overlay);
    display: none;
    width: min(var(--inspector-width), 100%);
    border-left: 1px solid var(--border-subtle);
    box-shadow: var(--shadow-lg);
  }

  .inspector-open .split-inspector {
    display: block;
  }

  .inspector-close-row {
    display: flex;
    justify-content: flex-end;
    margin-bottom: var(--space-3);
  }
}
</style>

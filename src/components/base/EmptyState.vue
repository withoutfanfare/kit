<script setup lang="ts">
/**
 * An empty state that teaches the interface rather than saying "nothing here".
 *
 * Kit's own, because the shared library's version draws a filled 20px glyph
 * while every other icon in the app is a 16px stroke — one inconsistent icon
 * is enough to make a screen feel assembled rather than designed.
 */
import PanelIcon, { type IconName } from "@/components/base/PanelIcon.vue";

withDefaults(
  defineProps<{
    icon?: IconName;
    title: string;
    description?: string;
    actionLabel?: string;
  }>(),
  { icon: "panel" }
);

defineEmits<{ action: [] }>();
</script>

<template>
  <div class="empty">
    <span class="empty-icon">
      <PanelIcon :name="icon" :size="18" />
    </span>
    <p class="empty-title">{{ title }}</p>
    <p v-if="description" class="empty-desc">{{ description }}</p>
    <button
      v-if="actionLabel"
      type="button"
      class="btn btn-secondary btn-sm empty-action"
      @click="$emit('action')"
    >
      {{ actionLabel }}
    </button>
  </div>
</template>

<style scoped>
.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--space-3);
  height: 100%;
  min-height: 220px;
  padding: var(--space-9);
  text-align: center;
}

.empty-icon {
  display: grid;
  place-items: center;
  width: 36px;
  height: 36px;
  margin-bottom: var(--space-2);
  border-radius: var(--radius-lg);
  background: var(--k-layer-2);
  box-shadow: inset 0 0 0 1px var(--k-line);
  color: var(--k-text-4);
}

.empty-title {
  font-size: var(--text-base);
  font-weight: var(--weight-medium);
  color: var(--k-text);
  margin: 0;
}

.empty-desc {
  font-size: var(--text-md);
  line-height: var(--lh-snug);
  color: var(--k-text-4);
  margin: 0;
  max-width: 40ch;
  text-wrap: pretty;
}

.empty-action {
  margin-top: var(--space-4);
}
</style>

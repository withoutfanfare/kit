<script setup lang="ts">
/**
 * A term plus the one line that explains it.
 *
 * Kit's vocabulary is unavoidably specific — library, global, context cost —
 * and hiding every definition behind a hover means nobody reads them. So the
 * meaning shows inline where there is room for it, and falls back to a native
 * tooltip where there isn't. Either way the words come from one glossary.
 */
withDefaults(
  defineProps<{
    label: string;
    meaning: string;
    /** Show the explanation as visible text rather than only on hover. */
    inline?: boolean;
  }>(),
  { inline: false }
);
</script>

<template>
  <span v-if="inline" class="term term-inline">
    <span class="term-label">{{ label }}</span>
    <span class="term-meaning">{{ meaning }}</span>
  </span>
  <abbr v-else class="term term-hint" :title="meaning">{{ label }}</abbr>
</template>

<style scoped>
.term-inline {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.term-label {
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.term-meaning {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  line-height: 1.45;
  text-wrap: pretty;
}

/* A dotted underline is the long-standing convention for "there's a
   definition here", and it survives without colour. */
.term-hint {
  text-decoration: underline dotted var(--border-strong);
  text-underline-offset: 3px;
  cursor: help;
}
</style>

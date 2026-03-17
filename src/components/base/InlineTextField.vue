<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'

const props = defineProps<{
  modelValue: string
  placeholder?: string
  error?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const editing = ref(false)
const draft = ref(props.modelValue)
const inputRef = ref<HTMLInputElement | null>(null)

watch(
  () => props.modelValue,
  (v) => {
    if (!editing.value) draft.value = v
  },
)

function startEditing() {
  draft.value = props.modelValue
  editing.value = true
  nextTick(() => inputRef.value?.select())
}

function save() {
  editing.value = false
  if (draft.value !== props.modelValue) {
    emit('update:modelValue', draft.value)
  }
}

function revert() {
  draft.value = props.modelValue
  editing.value = false
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    ;(e.target as HTMLInputElement).blur()
  } else if (e.key === 'Escape') {
    revert()
  }
}
</script>

<template>
  <div class="inline-text-field" :class="{ editing, 'has-error': error }">
    <input
      v-if="editing"
      ref="inputRef"
      v-model="draft"
      :placeholder="placeholder"
      class="field-input"
      @blur="save"
      @keydown="onKeydown"
    />
    <span
      v-else
      class="field-display"
      :class="{ empty: !modelValue }"
      tabindex="0"
      @click="startEditing"
      @keydown.enter="startEditing"
    >
      {{ modelValue || placeholder || 'Click to edit' }}
    </span>
    <span v-if="error && !editing" class="field-error">{{ error }}</span>
  </div>
</template>

<style scoped>
.inline-text-field {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.field-display {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  color: var(--text-primary);
  cursor: text;
  padding: 1px 2px;
  border-radius: var(--radius-sm);
  transition: background var(--duration-fast) var(--ease-default);
  line-height: 1.4;
}

.field-display:hover {
  background: var(--surface-hover);
}

.field-display:focus-visible {
  outline: none;
  box-shadow: 0 0 0 2px var(--accent);
}

.field-display.empty {
  color: var(--text-tertiary);
}

.field-input {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  color: var(--text-primary);
  background: transparent;
  border: none;
  border-bottom: 1px solid var(--accent);
  outline: none;
  padding: 1px 2px;
  line-height: 1.4;
  width: 100%;
}

.has-error .field-input {
  border-bottom-color: var(--danger);
}

.field-error {
  font-size: var(--text-xs);
  color: var(--danger);
}
</style>

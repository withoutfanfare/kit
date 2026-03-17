<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useLibraryStore } from "@/stores/libraryStore";
import SecondaryButton from "@/components/base/SecondaryButton.vue";

const preferencesStore = usePreferencesStore();
const libraryStore = useLibraryStore();

const editorDraft = ref(preferencesStore.editorCommand ?? "");
const isTestingEditor = ref(false);
const editorTestResult = ref<{ success: boolean; message: string } | null>(null);
const appDataPath = ref<string | null>(null);

const libraryPath = computed(() => preferencesStore.libraryRoot ?? "Not set");

const skillCount = computed(() => libraryStore.totalSkills);
const setCount = computed(() => libraryStore.totalSets);

// General
async function onDefaultViewChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value as "locations" | "skills";
  await preferencesStore.update({ defaultView: value });
}

async function onShowArchivedChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  await preferencesStore.update({ showArchived: checked });
}

// Library
async function changeLibraryRoot() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Select Library Root",
  });
  if (selected && typeof selected === "string") {
    await preferencesStore.update({ libraryRoot: selected });
  }
}

// Editor
async function saveEditorCommand() {
  const trimmed = editorDraft.value.trim();
  if (trimmed === (preferencesStore.editorCommand ?? "")) return;
  await preferencesStore.update({
    editorCommand: trimmed || null,
  });
}

async function testEditor() {
  const command = editorDraft.value.trim();
  isTestingEditor.value = true;
  editorTestResult.value = null;
  if (!command) {
    editorTestResult.value = { success: false, message: "Editor command is empty" };
    isTestingEditor.value = false;
    return;
  }
  try {
    const resolvedPath = appDataPath.value ?? (await invoke<string>("get_app_data_path"));
    await invoke("open_path_in_editor", { path: resolvedPath, editorCommand: command });
    editorTestResult.value = { success: true, message: `"${command}" launched successfully` };
  } catch (err) {
    editorTestResult.value = {
      success: false,
      message: err instanceof Error ? err.message : "Test failed",
    };
  } finally {
    isTestingEditor.value = false;
  }
}

// Advanced
async function revealAppData() {
  const resolvedPath = appDataPath.value ?? (await invoke<string>("get_app_data_path"));
  await invoke("reveal_in_finder", { path: resolvedPath });
}

onMounted(async () => {
  editorDraft.value = preferencesStore.editorCommand ?? "";
  try {
    appDataPath.value = await invoke<string>("get_app_data_path");
  } catch {
    // Fall back to displaying ~/.kit if the command fails
  }
});
</script>

<template>
  <div class="settings-view">
    <h1 class="page-title">Settings</h1>

    <!-- General -->
    <section class="settings-section">
      <h2 class="section-title">General</h2>
      <div class="settings-group">
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Default view</span>
            <span class="label-description">The view shown when the app opens</span>
          </div>
          <select
            class="setting-select"
            :value="preferencesStore.defaultView"
            @change="onDefaultViewChange"
          >
            <option value="locations">Locations</option>
            <option value="skills">Skills</option>
          </select>
        </div>
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Show archived by default</span>
            <span class="label-description">Include archived skills in lists</span>
          </div>
          <label class="toggle">
            <input
              type="checkbox"
              :checked="preferencesStore.showArchived"
              @change="onShowArchivedChange"
            />
            <span class="toggle-track" />
          </label>
        </div>
      </div>
    </section>

    <!-- Library -->
    <section class="settings-section">
      <h2 class="section-title">Library</h2>
      <div class="settings-group">
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Library root</span>
            <span class="label-path">{{ libraryPath }}</span>
          </div>
          <SecondaryButton label="Change" @click="changeLibraryRoot" />
        </div>
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Contents</span>
          </div>
          <div class="setting-counts">
            <span class="count-item">{{ skillCount }} skills</span>
            <span class="count-divider">&middot;</span>
            <span class="count-item">{{ setCount }} sets</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Editor -->
    <section class="settings-section">
      <h2 class="section-title">Editor</h2>
      <div class="settings-group">
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Editor command</span>
            <span class="label-description">e.g. "code", "cursor", "zed"</span>
          </div>
          <div class="editor-input-group">
            <input
              v-model="editorDraft"
              class="setting-input"
              type="text"
              placeholder="code"
              @blur="saveEditorCommand"
              @keydown.enter="($event.target as HTMLInputElement).blur()"
            />
            <SecondaryButton
              label="Test"
              :disabled="!editorDraft.trim()"
              :loading="isTestingEditor"
              @click="testEditor"
            />
          </div>
        </div>
        <div v-if="editorTestResult" class="setting-row test-result">
          <span
            class="test-message"
            :class="editorTestResult.success ? 'success' : 'failure'"
          >
            {{ editorTestResult.message }}
          </span>
        </div>
      </div>
    </section>

    <!-- Advanced -->
    <section class="settings-section">
      <h2 class="section-title">Advanced</h2>
      <div class="settings-group">
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">App data</span>
            <span class="label-path">{{ appDataPath ?? "~/.kit" }}</span>
          </div>
          <SecondaryButton label="Reveal" @click="revealAppData" />
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.settings-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
  padding: var(--space-5) var(--space-6);
}

.page-title {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--space-6);
}

/* Sections */
.settings-section {
  margin-bottom: var(--space-6);
}

.section-title {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  margin: 0 0 var(--space-2);
}

/* Grouped settings */
.settings-group {
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-3) var(--space-4);
  gap: var(--space-4);
  min-height: 44px;
}

.setting-row + .setting-row {
  border-top: 1px solid var(--border-subtle);
}

.setting-label {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
  flex: 1;
}

.label-text {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.label-description {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
}

.label-path {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Select */
.setting-select {
  height: 28px;
  padding: 0 var(--space-6) 0 var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: var(--surface-hover);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  outline: none;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' stroke='%236b6b6b' stroke-width='1.5' fill='none' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
}

.setting-select:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

/* Toggle */
.toggle {
  position: relative;
  display: inline-flex;
  cursor: pointer;
  flex-shrink: 0;
}

.toggle input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-track {
  width: 36px;
  height: 20px;
  background: var(--border-strong);
  border-radius: 10px;
  transition: background var(--duration-fast) var(--ease-default);
  position: relative;
}

.toggle-track::after {
  content: "";
  position: absolute;
  top: 2px;
  left: 2px;
  width: 16px;
  height: 16px;
  background: white;
  border-radius: 50%;
  box-shadow: var(--shadow-sm);
  transition: transform var(--duration-fast) var(--ease-default);
}

.toggle input:checked + .toggle-track {
  background: var(--accent);
}

.toggle input:checked + .toggle-track::after {
  transform: translateX(16px);
}

.toggle input:focus-visible + .toggle-track {
  box-shadow: 0 0 0 2px var(--surface-panel), 0 0 0 4px var(--accent);
}

/* Editor input */
.editor-input-group {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.setting-input {
  height: 28px;
  width: 120px;
  padding: 0 var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  background: var(--surface-hover);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  outline: none;
  transition: border-color var(--duration-fast) var(--ease-default),
    box-shadow var(--duration-fast) var(--ease-default);
}

.setting-input::placeholder {
  color: var(--text-tertiary);
}

.setting-input:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-subtle);
}

/* Test result */
.test-result {
  min-height: auto;
  padding: var(--space-2) var(--space-4);
}

.test-message {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  line-height: 1.4;
}

.test-message.success {
  color: var(--success);
}

.test-message.failure {
  color: var(--danger);
}

/* Counts */
.setting-counts {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  flex-shrink: 0;
}

.count-divider {
  color: var(--text-tertiary);
}
</style>

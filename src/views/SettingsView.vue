<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useLibraryStore } from "@/stores/libraryStore";
import { useAppStore } from "@/stores/appStore";
import type { SkillsRepoStatus, RepoState } from "@/types";
import { SButton, SBadge } from "@stuntrocket/ui";

const preferencesStore = usePreferencesStore();
const libraryStore = useLibraryStore();
const appStore = useAppStore();

const editorDraft = ref(preferencesStore.editorCommand ?? "");
const isTestingEditor = ref(false);
const editorTestResult = ref<{ success: boolean; message: string } | null>(null);
const appDataPath = ref<string | null>(null);

const libraryPath = computed(() => preferencesStore.libraryRoot ?? "Not set");

const skillCount = computed(() => libraryStore.totalSkills);
const setCount = computed(() => libraryStore.totalSets);

// Skills Repository
const repoStatus = ref<SkillsRepoStatus | null>(null);
const isCheckingRepo = ref(false);
const repoStatusError = ref<string | null>(null);

function repoStateColour(state: RepoState): "success" | "warning" | "danger" | "default" {
  switch (state) {
    case "up_to_date":
      return "success";
    case "behind":
    case "ahead":
    case "diverged":
    case "dirty":
      return "warning";
    case "unavailable":
      return "danger";
    default:
      return "default";
  }
}

function repoStateLabel(state: RepoState): string {
  switch (state) {
    case "up_to_date":
      return "Up to date";
    case "behind":
      return "Behind remote";
    case "ahead":
      return "Ahead of remote";
    case "diverged":
      return "Diverged";
    case "dirty":
      return "Uncommitted changes";
    case "unavailable":
      return "Unavailable";
    default:
      return state;
  }
}

async function fetchRepoStatus() {
  isCheckingRepo.value = true;
  repoStatusError.value = null;
  try {
    repoStatus.value = await invoke<SkillsRepoStatus>("get_skills_repo_status");
  } catch (err) {
    repoStatusError.value =
      err instanceof Error ? err.message : "Failed to check repository status";
  } finally {
    isCheckingRepo.value = false;
  }
}

async function recheckRepoStatus() {
  isCheckingRepo.value = true;
  repoStatusError.value = null;
  try {
    repoStatus.value = await invoke<SkillsRepoStatus>("recheck_skills_repo_status");
  } catch (err) {
    repoStatusError.value =
      err instanceof Error ? err.message : "Failed to check repository status";
  } finally {
    isCheckingRepo.value = false;
  }
}

async function changeRepoPath() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose Skills Repository",
  });
  if (selected && typeof selected === "string") {
    await preferencesStore.update({ libraryRoot: selected });
    await fetchRepoStatus();
  }
}

async function revealRepo() {
  if (!preferencesStore.libraryRoot) return;
  await invoke("reveal_in_finder", { path: preferencesStore.libraryRoot });
}

async function openRepoInEditor() {
  if (!preferencesStore.libraryRoot) return;
  const command = preferencesStore.editorCommand;
  if (!command) {
    appStore.toast("Set an editor command first", "info");
    return;
  }
  try {
    await invoke("open_path_in_editor", {
      path: preferencesStore.libraryRoot,
      editorCommand: command,
    });
  } catch (err) {
    appStore.toast(
      err instanceof Error ? err.message : "Failed to open in editor",
      "error"
    );
  }
}

async function copyPullCommand() {
  try {
    const cmd = await invoke<string>("copy_repo_pull_command");
    await navigator.clipboard.writeText(cmd);
    appStore.toast("Pull command copied to clipboard", "success");
  } catch {
    // Fallback: construct the command manually
    const branch = repoStatus.value?.branch ?? "main";
    const cmd = `cd "${preferencesStore.libraryRoot}" && git pull origin ${branch}`;
    try {
      await navigator.clipboard.writeText(cmd);
      appStore.toast("Pull command copied to clipboard", "success");
    } catch (err) {
      appStore.toast(
        err instanceof Error ? err.message : "Failed to copy",
        "error"
      );
    }
  }
}

// General
async function onDefaultViewChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value as "locations" | "skills";
  await preferencesStore.update({ defaultView: value });
}

async function onShowArchivedChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked;
  await preferencesStore.update({ showArchived: checked });
}

// Editor
async function saveEditorCommand() {
  const trimmed = editorDraft.value.trim();
  if (trimmed === (preferencesStore.editorCommand ?? "")) return;
  await preferencesStore.update({
    editorCommand: trimmed,
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
  // Fetch skills repo status
  if (preferencesStore.libraryRoot) {
    await fetchRepoStatus();
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

    <!-- Skills Repository -->
    <section class="settings-section">
      <h2 class="section-title">Skills Repository</h2>
      <div class="settings-group">
        <div class="setting-row">
          <div class="setting-label">
            <span class="label-text">Path</span>
            <span class="label-path mono">{{ libraryPath }}</span>
          </div>
          <SButton variant="secondary" size="sm" @click="changeRepoPath">Change</SButton>
        </div>

        <!-- Branch -->
        <div v-if="repoStatus?.branch" class="setting-row">
          <div class="setting-label">
            <span class="label-text">Branch</span>
          </div>
          <SBadge variant="accent">{{ repoStatus.branch }}</SBadge>
        </div>

        <!-- Status -->
        <div v-if="repoStatus" class="setting-row">
          <div class="setting-label">
            <span class="label-text">Status</span>
          </div>
          <div class="repo-status-display">
            <span class="repo-status-dot" :class="repoStateColour(repoStatus.state)" />
            <span class="repo-status-text" :class="repoStateColour(repoStatus.state)">
              {{ repoStateLabel(repoStatus.state) }}
            </span>
          </div>
        </div>

        <!-- Ahead / Behind counts -->
        <div v-if="repoStatus && (repoStatus.aheadBy > 0 || repoStatus.behindBy > 0)" class="setting-row">
          <div class="setting-label">
            <span class="label-text">Sync</span>
          </div>
          <div class="setting-counts">
            <span v-if="repoStatus.aheadBy > 0" class="count-item">{{ repoStatus.aheadBy }} ahead</span>
            <span v-if="repoStatus.aheadBy > 0 && repoStatus.behindBy > 0" class="count-divider">&middot;</span>
            <span v-if="repoStatus.behindBy > 0" class="count-item">{{ repoStatus.behindBy }} behind</span>
          </div>
        </div>

        <!-- Uncommitted changes warning -->
        <div v-if="repoStatus?.hasUncommittedChanges" class="setting-row">
          <div class="setting-label">
            <span class="label-text warning-text">Has uncommitted changes</span>
          </div>
        </div>

        <!-- Message -->
        <div v-if="repoStatus?.message" class="setting-row">
          <div class="setting-label">
            <span class="label-description">{{ repoStatus.message }}</span>
          </div>
        </div>

        <!-- Last checked -->
        <div v-if="repoStatus?.lastCheckedAt" class="setting-row">
          <div class="setting-label">
            <span class="label-text">Last checked</span>
          </div>
          <span class="label-description">{{ repoStatus.lastCheckedAt }}</span>
        </div>

        <!-- Error -->
        <div v-if="repoStatusError" class="setting-row">
          <span class="repo-status-text danger">{{ repoStatusError }}</span>
        </div>

        <!-- Actions -->
        <div class="setting-row repo-actions-row">
          <div class="repo-actions">
            <SButton variant="secondary" size="sm" @click="revealRepo">Reveal</SButton>
            <SButton variant="secondary" size="sm" @click="openRepoInEditor">Open in Editor</SButton>
            <SButton
              variant="secondary"
              size="sm"
              :loading="isCheckingRepo"
              @click="recheckRepoStatus"
            >Check for Updates</SButton>
            <SButton
              v-if="repoStatus && repoStatus.behindBy > 0"
              variant="secondary"
              size="sm"
              @click="copyPullCommand"
            >Copy Pull Command</SButton>
          </div>
        </div>
      </div>
    </section>

    <!-- Library -->
    <section class="settings-section">
      <h2 class="section-title">Library</h2>
      <div class="settings-group">
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
            <SButton
              variant="secondary"
              size="sm"
              :disabled="!editorDraft.trim()"
              :loading="isTestingEditor"
              @click="testEditor"
            >Test</SButton>
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
          <SButton variant="secondary" size="sm" @click="revealAppData">Reveal</SButton>
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

/* Mono path */
.mono {
  font-family: ui-monospace, "SF Mono", Monaco, "Cascadia Code", monospace;
}

/* Repo status */
.repo-status-display {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-shrink: 0;
}

.repo-status-dot {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.repo-status-dot.success {
  background: var(--success);
}

.repo-status-dot.warning {
  background: var(--warning);
}

.repo-status-dot.danger {
  background: var(--danger);
}

.repo-status-dot.default {
  background: var(--text-tertiary);
}

.repo-status-text {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
}

.repo-status-text.success {
  color: var(--success);
}

.repo-status-text.warning {
  color: var(--warning);
}

.repo-status-text.danger {
  color: var(--danger);
}

.repo-status-text.default {
  color: var(--text-secondary);
}

.warning-text {
  color: var(--warning);
}

/* Repo action buttons */
.repo-actions-row {
  flex-direction: column;
  align-items: flex-start;
}

.repo-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}
</style>

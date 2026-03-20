<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useRouter } from "vue-router";
import { useAppStore } from "@/stores/appStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import { useLocationsStore } from "@/stores/locationsStore";
import type { SkillsRepoValidation } from "@/types";
import { SButton } from "@stuntrocket/ui";

const router = useRouter();
const appStore = useAppStore();
const preferencesStore = usePreferencesStore();
const locationsStore = useLocationsStore();

type OnboardingStep = "welcome" | "validated";

const step = ref<OnboardingStep>("welcome");
const selectedPath = ref<string | null>(null);
const validation = ref<SkillsRepoValidation | null>(null);
const isValidating = ref(false);
const validationError = ref<string | null>(null);
const editorDraft = ref(preferencesStore.editorCommand ?? "");
const addedProjectPath = ref<string | null>(null);
const isAddingProject = ref(false);
const isCompleting = ref(false);

const isValid = computed(() => validation.value?.valid === true);
const hasIssues = computed(
  () => validation.value && validation.value.issues.length > 0
);

async function chooseRepository() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose Skills Repository",
  });
  if (selected && typeof selected === "string") {
    selectedPath.value = selected;
    isValidating.value = true;
    validationError.value = null;
    try {
      validation.value = await invoke<SkillsRepoValidation>(
        "validate_skills_repository",
        { path: selected }
      );
      step.value = "validated";
    } catch (err) {
      validationError.value =
        err instanceof Error ? err.message : "Failed to validate repository";
    } finally {
      isValidating.value = false;
    }
  }
}

async function addFirstProject() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Add a Project Location",
  });
  if (selected && typeof selected === "string") {
    isAddingProject.value = true;
    try {
      await locationsStore.addLocation(selected);
      addedProjectPath.value = selected;
    } catch (err) {
      appStore.toast(
        err instanceof Error ? err.message : "Failed to add project",
        "error"
      );
    } finally {
      isAddingProject.value = false;
    }
  }
}

async function completeSetup() {
  if (!selectedPath.value) return;
  isCompleting.value = true;
  try {
    await preferencesStore.update({ libraryRoot: selectedPath.value });
    const trimmedEditor = editorDraft.value.trim();
    if (trimmedEditor) {
      await preferencesStore.update({ editorCommand: trimmedEditor });
    }
    appStore.needsSetup = false;
    await appStore.bootstrap();
    router.push("/locations");
  } catch (err) {
    appStore.toast(
      err instanceof Error ? err.message : "Failed to complete setup",
      "error"
    );
  } finally {
    isCompleting.value = false;
  }
}
</script>

<template>
  <div class="onboarding-backdrop">
    <div class="onboarding-card">
      <!-- Step: Welcome -->
      <template v-if="step === 'welcome'">
        <h1 class="onboarding-title">Welcome to Kit</h1>
        <p class="onboarding-description">
          Kit helps you manage Claude Code skills across your projects.
        </p>

        <div v-if="validationError" class="validation-error">
          {{ validationError }}
        </div>

        <SButton
          variant="primary"
          :loading="isValidating"
          @click="chooseRepository"
        >Choose Skills Repository</SButton>
      </template>

      <!-- Step: Validated -->
      <template v-if="step === 'validated' && validation">
        <h1 class="onboarding-title">Repository Selected</h1>

        <div class="validation-results">
          <div class="validation-row">
            <span class="validation-label">Path</span>
            <span class="validation-value mono">{{ validation.path }}</span>
          </div>
          <div class="validation-row">
            <span class="validation-label">Git repository</span>
            <span class="validation-value">
              <span v-if="validation.isGitRepo" class="icon-yes">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                  <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
                  <path d="M4.5 7l2 2 3-3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
                </svg>
                Yes
              </span>
              <span v-else class="icon-no">
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                  <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
                  <path d="M5 5l4 4M9 5l-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
                </svg>
                No
              </span>
            </span>
          </div>
          <div v-if="validation.detectedBranch" class="validation-row">
            <span class="validation-label">Branch</span>
            <span class="validation-value">
              <span class="branch-badge">{{ validation.detectedBranch }}</span>
            </span>
          </div>
          <div class="validation-row">
            <span class="validation-label">Skills found</span>
            <span class="validation-value">{{ validation.skillCount }}</span>
          </div>
        </div>

        <div v-if="isValid && !hasIssues" class="validation-message success">
          This looks like a valid skills repository.
        </div>

        <div v-if="hasIssues" class="validation-issues">
          <div
            v-for="(issue, i) in validation.issues"
            :key="i"
            class="validation-issue"
          >
            {{ issue }}
          </div>
        </div>

        <!-- Optional: Editor command -->
        <div class="optional-section">
          <label class="optional-label" for="editor-command">
            Editor command
            <span class="optional-hint">(optional)</span>
          </label>
          <input
            id="editor-command"
            v-model="editorDraft"
            class="setting-input"
            type="text"
            placeholder="e.g. code, cursor, zed"
          />
        </div>

        <!-- Optional: Add first project -->
        <div class="optional-section">
          <div v-if="addedProjectPath" class="added-project">
            <svg width="14" height="14" viewBox="0 0 14 14" fill="none" class="added-project-icon">
              <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
              <path d="M4.5 7l2 2 3-3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
            </svg>
            <span class="added-project-path">{{ addedProjectPath }}</span>
          </div>
          <SButton
            v-else
            variant="secondary"
            :loading="isAddingProject"
            @click="addFirstProject"
          >Add First Project</SButton>
        </div>

        <div class="onboarding-actions">
          <SButton variant="secondary" @click="step = 'welcome'">Back</SButton>
          <SButton
            variant="primary"
            :loading="isCompleting"
            @click="completeSetup"
          >Open Kit</SButton>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.onboarding-backdrop {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: var(--space-6);
  background: var(--surface-app);
}

.onboarding-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  width: 100%;
  max-width: 480px;
  padding: var(--space-6);
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
}

.onboarding-title {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.onboarding-description {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

/* Validation results */
.validation-results {
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.validation-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2) var(--space-3);
  gap: var(--space-3);
}

.validation-row + .validation-row {
  border-top: 1px solid var(--border-subtle);
}

.validation-label {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  flex-shrink: 0;
}

.validation-value {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-primary);
  text-align: right;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.validation-value.mono {
  font-family: ui-monospace, "SF Mono", Monaco, "Cascadia Code", monospace;
  font-size: var(--text-xs);
  color: var(--text-secondary);
  direction: rtl;
  text-align: right;
}

.icon-yes {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  color: var(--success);
}

.icon-no {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  color: var(--text-tertiary);
}

.branch-badge {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 var(--space-2);
  font-family: ui-monospace, "SF Mono", Monaco, "Cascadia Code", monospace;
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--accent);
  background: var(--accent-subtle);
  border-radius: var(--radius-sm);
}

/* Validation messages */
.validation-message.success {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--success);
  line-height: 1.4;
}

.validation-issues {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.validation-issue {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--warning);
  line-height: 1.4;
}

.validation-error {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--danger);
  line-height: 1.4;
}

/* Optional sections */
.optional-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.optional-label {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
}

.optional-hint {
  font-weight: var(--weight-normal);
  color: var(--text-tertiary);
}

.setting-input {
  height: 28px;
  width: 100%;
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

/* Added project feedback */
.added-project {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--success);
}

.added-project-path {
  font-family: ui-monospace, "SF Mono", Monaco, "Cascadia Code", monospace;
  font-size: var(--text-xs);
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Actions */
.onboarding-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-2);
  padding-top: var(--space-2);
}
</style>

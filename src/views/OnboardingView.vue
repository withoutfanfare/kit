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

type OnboardingStep = "library" | "project" | "scan" | "review";

const step = ref<OnboardingStep>("library");
const selectedPath = ref<string | null>(null);
const persistedLibraryPath = ref<string | null>(null);
const validation = ref<SkillsRepoValidation | null>(null);
const isValidating = ref(false);
const validationError = ref<string | null>(null);
const editorDraft = ref(preferencesStore.editorCommand ?? "");
const projectPath = ref<string | null>(null);
const addedLocationId = ref<string | null>(null);
const scanError = ref<string | null>(null);
const isCompleting = ref(false);

const isValid = computed(() => validation.value?.valid === true);
const reviewDetail = computed(() =>
  addedLocationId.value
    ? locationsStore.detailCache[addedLocationId.value] ?? null
    : null
);
const reviewSkillCount = computed(() => reviewDetail.value?.skills.length ?? 0);
const reviewIssueCount = computed(() => reviewDetail.value?.issues.length ?? 0);

function editorCommand() {
  return editorDraft.value.trim();
}

async function persistLibrary() {
  if (!selectedPath.value) return;
  await preferencesStore.update({
    libraryRoot: selectedPath.value,
    editorCommand: editorCommand(),
  });
  persistedLibraryPath.value = selectedPath.value;
}

async function chooseRepository() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Choose Skills Repository",
  });
  if (selected && typeof selected === "string") {
    selectedPath.value = selected;
    persistedLibraryPath.value = null;
    validation.value = null;
    isValidating.value = true;
    validationError.value = null;
    try {
      const result = await invoke<SkillsRepoValidation>(
        "validate_skills_repository",
        { path: selected }
      );
      validation.value = result;
      if (result.valid) {
        await persistLibrary();
        step.value = "project";
      }
    } catch (err) {
      validationError.value =
        err instanceof Error ? err.message : "Failed to validate repository";
    } finally {
      isValidating.value = false;
    }
  }
}

async function continueFromLibrary() {
  if (!isValid.value || !selectedPath.value) return;
  try {
    if (persistedLibraryPath.value === selectedPath.value) {
      await preferencesStore.update({ editorCommand: editorCommand() });
    } else {
      await persistLibrary();
    }
    step.value = "project";
  } catch (err) {
    validationError.value =
      err instanceof Error ? err.message : "Failed to save repository settings";
  }
}

async function addFirstProject() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Add a Project Location",
  });
  if (selected && typeof selected === "string") {
    if (selected !== projectPath.value) {
      addedLocationId.value = null;
    }
    projectPath.value = selected;
    await scanProject();
  }
}

async function scanProject() {
  if (!projectPath.value) return;
  const previousLocationId = locationsStore.selectedLocationId;
  step.value = "scan";
  scanError.value = null;
  try {
    if (addedLocationId.value) {
      await locationsStore.fetchDetail(addedLocationId.value);
    } else {
      await locationsStore.addLocation(projectPath.value);
      addedLocationId.value = locationsStore.selectedLocationId;
    }
    step.value = "review";
  } catch (err) {
    if (
      !addedLocationId.value &&
      locationsStore.selectedLocationId !== previousLocationId
    ) {
      addedLocationId.value = locationsStore.selectedLocationId;
    }
    scanError.value =
      err instanceof Error ? err.message : "Failed to scan project";
  }
}

async function completeSetup() {
  if (!addedLocationId.value) return;
  isCompleting.value = true;
  try {
    const locationId = addedLocationId.value;
    const bootstrapped = await appStore.bootstrap();
    if (!bootstrapped) return;
    appStore.needsSetup = false;
    await router.push(
      reviewIssueCount.value > 0
        ? { path: "/health", query: { locationId } }
        : `/locations/${locationId}`
    );
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
      <template v-if="step === 'library'">
        <h1 class="onboarding-title">Welcome to Kit</h1>
        <p class="onboarding-description">
          Choose the skills repository Kit should use across your projects.
        </p>

        <div v-if="validationError" class="validation-error">
          {{ validationError }}
        </div>

        <div v-if="validation" class="validation-results">
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

        <div v-if="isValid" class="validation-message success">
          This looks like a valid skills repository.
        </div>

        <div v-if="validation?.issues.length" class="validation-issues">
          <div
            v-for="(issue, i) in validation.issues"
            :key="i"
            class="validation-issue"
          >
            {{ issue }}
          </div>
        </div>

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

        <div class="onboarding-actions">
          <SButton
            v-if="isValid"
            variant="secondary"
            :disabled="isValidating"
            @click="chooseRepository"
          >Choose Another Repository</SButton>
          <SButton
            variant="primary"
            :loading="isValidating"
            @click="isValid ? continueFromLibrary() : chooseRepository()"
          >{{ isValid ? "Continue" : "Choose Skills Repository" }}</SButton>
        </div>
      </template>

      <template v-else-if="step === 'project'">
        <h1 class="onboarding-title">Add your first project</h1>
        <p class="onboarding-description">
          Kit will scan this location using the saved skills repository.
        </p>
        <div class="onboarding-actions">
          <SButton variant="secondary" @click="step = 'library'">Back</SButton>
          <SButton
            variant="primary"
            @click="addFirstProject"
          >Add First Project</SButton>
        </div>
      </template>

      <template v-else-if="step === 'scan'">
        <h1 class="onboarding-title">
          {{ scanError ? "Scan paused" : "Scanning project" }}
        </h1>
        <p class="onboarding-description mono-path">{{ projectPath }}</p>
        <div v-if="scanError" class="validation-error">{{ scanError }}</div>
        <div class="onboarding-actions">
          <SButton
            v-if="scanError"
            variant="secondary"
            @click="step = 'project'"
          >Back</SButton>
          <SButton
            variant="primary"
            :loading="!scanError"
            :disabled="!scanError"
            @click="scanProject"
          >{{ scanError ? "Retry Scan" : "Scanning" }}</SButton>
        </div>
      </template>

      <template v-else-if="step === 'review'">
        <h1 class="onboarding-title">Project ready</h1>
        <p class="onboarding-description">
          Found {{ reviewSkillCount }}
          {{ reviewSkillCount === 1 ? "skill" : "skills" }} and
          {{ reviewIssueCount }}
          {{ reviewIssueCount === 1 ? "issue" : "issues" }}.
        </p>
        <div class="onboarding-actions">
          <SButton
            variant="primary"
            :loading="isCompleting"
            @click="completeSetup"
          >{{
            reviewIssueCount > 0
              ? `Resolve ${reviewIssueCount} ${reviewIssueCount === 1 ? "issue" : "issues"}`
              : "Open location"
          }}</SButton>
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

.mono-path {
  font-family: ui-monospace, "SF Mono", Monaco, "Cascadia Code", monospace;
  font-size: var(--text-xs);
  direction: rtl;
  text-align: left;
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

<script setup lang="ts">
import type { AssignmentPreview } from "@/types";
import { SBadge } from "@stuntrocket/ui";

defineProps<{
  preview: AssignmentPreview | null;
  isLoading: boolean;
}>();
</script>

<template>
  <div class="selection-preview">
    <h3 class="preview-heading">Changes</h3>

    <div v-if="isLoading" class="preview-loading">
      <span class="spinner" />
      <span class="loading-label">Calculating changes...</span>
    </div>

    <template v-else-if="preview">
      <!-- Adds -->
      <div v-if="preview.adds.length > 0" class="preview-section">
        <div class="section-label">
          <SBadge variant="success" compact>+ {{ preview.adds.length }}</SBadge>
          <span>Add</span>
        </div>
        <ul class="change-list">
          <li v-for="change in preview.adds" :key="change.skillName" class="change-item add">
            <span class="change-icon">+</span>
            <span class="change-name">{{ change.skillName }}</span>
            <span v-if="change.detail" class="change-detail">{{ change.detail }}</span>
          </li>
        </ul>
      </div>

      <!-- Removes -->
      <div v-if="preview.removes.length > 0" class="preview-section">
        <div class="section-label">
          <SBadge variant="error" compact>- {{ preview.removes.length }}</SBadge>
          <span>Remove</span>
        </div>
        <ul class="change-list">
          <li v-for="change in preview.removes" :key="change.skillName" class="change-item remove">
            <span class="change-icon">-</span>
            <span class="change-name">{{ change.skillName }}</span>
            <span v-if="change.detail" class="change-detail">{{ change.detail }}</span>
          </li>
        </ul>
      </div>

      <!-- Manifest updates -->
      <div v-if="preview.manifestUpdates.length > 0" class="preview-section">
        <div class="section-label">
          <SBadge compact>{{ preview.manifestUpdates.length }}</SBadge>
          <span>Manifest updates</span>
        </div>
        <ul class="change-list">
          <li
            v-for="change in preview.manifestUpdates"
            :key="change.skillName + change.kind"
            class="change-item manifest"
          >
            <span class="change-icon manifest-icon">~</span>
            <span class="change-name">{{ change.skillName }}</span>
            <span v-if="change.detail" class="change-detail">{{ change.detail }}</span>
          </li>
        </ul>
      </div>

      <!-- Warnings -->
      <div v-if="preview.warnings.length > 0" class="preview-section">
        <div class="section-label">
          <SBadge variant="warning" compact>{{ preview.warnings.length }}</SBadge>
          <span>Warnings</span>
        </div>
        <ul class="warning-list">
          <li v-for="(warning, idx) in preview.warnings" :key="idx" class="warning-item">
            {{ warning }}
          </li>
        </ul>
      </div>
    </template>

    <div v-else class="preview-empty">
      <p class="empty-text">Select skills or sets to see a preview</p>
    </div>
  </div>
</template>

<style scoped>
.selection-preview {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  height: 100%;
}

.preview-heading {
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
}

.preview-loading {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-4) 0;
}

.spinner {
  width: 14px;
  height: 14px;
  border: 1.5px solid var(--border-default);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.loading-label {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.preview-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}

.section-label {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.change-list,
.warning-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
}

.change-item {
  display: flex;
  align-items: baseline;
  gap: var(--space-2);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  line-height: 1.4;
}

.change-icon {
  font-weight: var(--weight-semibold);
  font-size: var(--text-sm);
  flex-shrink: 0;
  width: 14px;
  text-align: center;
}

.add .change-icon {
  color: var(--success);
}

.remove .change-icon {
  color: var(--danger);
}

.manifest-icon {
  color: var(--text-tertiary);
}

.change-name {
  color: var(--text-primary);
  font-weight: var(--weight-medium);
}

.change-detail {
  color: var(--text-tertiary);
  font-size: var(--text-xs);
}

.warning-item {
  padding: var(--space-2);
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--warning);
  background: var(--warning-subtle);
  border-radius: var(--radius-sm);
  line-height: 1.4;
}

.warning-item + .warning-item {
  margin-top: var(--space-1);
}

.preview-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
  padding: var(--space-8);
}

.empty-text {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  text-align: center;
  line-height: 1.5;
  margin: 0;
}
</style>

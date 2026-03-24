<script setup lang="ts">
import type { SkillAssignment } from "@/types";
import { SBadge } from "@stuntrocket/ui";

defineProps<{
  skill: SkillAssignment;
}>();

defineEmits<{
  select: [];
  toggleActivation: [];
  viewDiff: [];
}>();

function dotVariant(linkState: string): string {
  switch (linkState) {
    case "linked":
      return "linked";
    case "local_only":
      return "local-only";
    case "declared_only":
      return "declared-only";
    case "broken_link":
      return "broken";
    default:
      return "linked";
  }
}

function sourceBadgeVariant(source: string): "default" | "accent" {
  return source === "library" ? "accent" : "default";
}
</script>

<template>
  <div class="skill-row" :class="{ archived: skill.archived, disabled: skill.disabled }" @click="$emit('select')">
    <span class="status-dot" :class="dotVariant(skill.linkState)" />
    <span class="skill-name">{{ skill.name }}</span>
    <SBadge v-if="skill.disabled" variant="count">disabled</SBadge>
    <SBadge v-else-if="skill.archived" variant="count">archived</SBadge>
    <SBadge :variant="sourceBadgeVariant(skill.source)">
      {{ skill.source }}
    </SBadge>
    <button
      v-if="skill.linkState === 'linked'"
      class="action-btn"
      title="View content changes"
      @click.stop="$emit('viewDiff')"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20h9"/><path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>
    </button>
    <button
      v-if="skill.linkState === 'linked'"
      class="action-btn"
      :title="skill.disabled ? 'Enable skill' : 'Disable skill'"
      @click.stop="$emit('toggleActivation')"
    >
      {{ skill.disabled ? '&#9654;' : '&#10074;&#10074;' }}
    </button>
  </div>
</template>

<style scoped>
.skill-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: var(--list-row-height);
  padding: 0 var(--space-3);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.skill-row:hover {
  background: var(--surface-hover);
}

.skill-row.archived,
.skill-row.disabled {
  opacity: 0.6;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-dot.linked {
  background: var(--success);
}

.status-dot.local-only {
  background: var(--accent);
}

.status-dot.declared-only {
  background: var(--warning);
}

.status-dot.broken {
  background: var(--danger);
}

.skill-name {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  font-size: 8px;
  cursor: pointer;
  flex-shrink: 0;
  transition: background var(--duration-fast) var(--ease-default),
    color var(--duration-fast) var(--ease-default);
}

.action-btn:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}
</style>

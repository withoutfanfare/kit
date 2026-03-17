<script setup lang="ts">
import type { SkillAssignment } from "@/types";
import Badge from "@/components/base/Badge.vue";

defineProps<{
  skill: SkillAssignment;
}>();

defineEmits<{
  select: [];
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
  <div class="skill-row" :class="{ archived: skill.archived }" @click="$emit('select')">
    <span class="status-dot" :class="dotVariant(skill.linkState)" />
    <span class="skill-name">{{ skill.name }}</span>
    <Badge v-if="skill.archived" variant="default" compact>archived</Badge>
    <Badge :variant="sourceBadgeVariant(skill.source)" compact>
      {{ skill.source }}
    </Badge>
  </div>
</template>

<style scoped>
.skill-row {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  height: var(--list-row-height);
  padding: 0 var(--space-3);
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default);
  user-select: none;
}

.skill-row:hover {
  background: var(--surface-hover);
}

.skill-row.archived {
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
</style>

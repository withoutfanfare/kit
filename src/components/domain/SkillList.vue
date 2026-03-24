<script setup lang="ts">
import type { SkillAssignment } from "@/types";
import SkillRow from "@/components/domain/SkillRow.vue";

defineProps<{
  skills: SkillAssignment[];
  title?: string;
  showLinkState?: boolean;
}>();

defineEmits<{
  selectSkill: [skillId: string];
  toggleActivation: [skillId: string];
  viewDiff: [skillId: string];
}>();
</script>

<template>
  <div class="skill-list">
    <div v-if="title" class="section-header">
      <span class="section-title">{{ title }}</span>
      <span class="section-count">{{ skills.length }}</span>
    </div>
    <div class="section-group">
      <SkillRow
        v-for="skill in skills"
        :key="skill.skillId"
        :skill="skill"
        @select="$emit('selectSkill', skill.skillId)"
        @toggle-activation="$emit('toggleActivation', skill.skillId)"
        @view-diff="$emit('viewDiff', skill.skillId)"
      />
      <div v-if="skills.length === 0" class="list-empty">
        <span class="list-empty-text">No skills</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.skill-list {
  display: flex;
  flex-direction: column;
}

.section-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-3);
}

.section-title {
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.section-count {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-variant-numeric: tabular-nums;
}

.section-group {
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  background: var(--surface-panel);
  overflow: hidden;
}

.section-group > :deep(*:not(:first-child)) {
  border-top: 1px solid var(--border-subtle);
}

.section-group :deep(.list-empty) {
  border-top: none;
}

.list-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
}

.list-empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}
</style>

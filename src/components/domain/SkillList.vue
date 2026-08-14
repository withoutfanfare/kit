<script setup lang="ts">
import type { SkillAssignment } from "@/types";
import SkillRow from "@/components/domain/SkillRow.vue";
import SkillStatusLegend from "@/components/domain/SkillStatusLegend.vue";

defineProps<{
  skills: SkillAssignment[];
  title?: string;
  showLinkState?: boolean;
  /**
   * The key is per-screen, not per-group. A location detail renders several
   * lists, and repeating the legend above each one turned a helpful key into
   * four lines of noise.
   */
  showLegend?: boolean;
}>();

defineEmits<{
  selectSkill: [skillId: string];
  toggleActivation: [skillId: string];
  viewDiff: [skillId: string];
}>();
</script>

<template>
  <section class="section skill-list">
    <div v-if="title" class="section-head">
      <h3 class="section-title">{{ title }}</h3>
      <span class="section-count">{{ skills.length }}</span>
    </div>

    <!-- The key gets its own line: four state names never fit beside a heading
         without wrapping into a ragged block. -->
    <SkillStatusLegend
      v-if="showLinkState && showLegend"
      class="list-legend"
    />
    <div class="section-group">
      <SkillRow
        v-for="skill in skills"
        :key="skill.skillId"
        :skill="skill"
        @select="$emit('selectSkill', skill.skillId)"
        @toggle-activation="$emit('toggleActivation', skill.skillId)"
        @view-diff="$emit('viewDiff', skill.skillId)"
      />
      <div v-if="skills.length === 0" class="row-empty">No skills here yet.</div>
    </div>
  </section>
</template>

<style scoped>
/* The rows carry their own separators; the group is just the container. Local
   copies of .section-head / .section-title were removed — those live in
   panel.css now so every list in Kit rules its heading the same way. */
.section-group {
  border-bottom: 1px solid var(--border-subtle);
}

.list-legend {
  padding: var(--space-3) var(--space-1) var(--space-2);
}
</style>

<script setup lang="ts">
/**
 * One circuit on a location's schedule.
 *
 * The state symbol leads, because that is the column you scan; the name
 * follows; the switch sits at the end where your hand goes. The old row put a
 * coloured pill first and drew its controls with unicode play/pause
 * characters, which are neither icons nor the same weight as anything else.
 */
import type { SkillAssignment } from "@/types";
import LinkStateMark from "./LinkStateMark.vue";
import PanelIcon from "@/components/base/PanelIcon.vue";

defineProps<{
  skill: SkillAssignment;
}>();

defineEmits<{
  select: [];
  toggleActivation: [];
  viewDiff: [];
}>();
</script>

<template>
  <div
    class="skill-row"
    :class="{ archived: skill.archived, off: skill.disabled }"
    @click="$emit('select')"
  >
    <LinkStateMark :state="skill.linkState" compact />

    <span class="skill-name">{{ skill.name }}</span>

    <span v-if="skill.source === 'local'" class="plate row-plate">Local</span>
    <span v-if="skill.archived" class="plate row-plate">Archived</span>

    <span class="row-actions">
      <button
        v-if="skill.linkState === 'linked'"
        class="action"
        :title="`View content changes for ${skill.name}`"
        :aria-label="`View content changes for ${skill.name}`"
        @click.stop="$emit('viewDiff')"
      >
        <PanelIcon name="changelog" :size="13" />
      </button>

      <!-- The breaker. Thrown left it is off; the handle moves, the track
           doesn't, and the label says which state it is in. -->
      <button
        v-if="skill.linkState === 'linked'"
        class="breaker"
        :class="{ thrown: skill.disabled }"
        role="switch"
        :aria-checked="!skill.disabled"
        :title="skill.disabled ? `Switch ${skill.name} on here` : `Switch ${skill.name} off here`"
        @click.stop="$emit('toggleActivation')"
      >
        <span class="breaker-handle" aria-hidden="true" />
        <span class="sr-only">
          {{ skill.disabled ? "Switched off in this location" : "On in this location" }}
        </span>
      </button>
    </span>
  </div>
</template>

<style scoped>
.skill-row {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  min-height: 30px;
  padding: var(--space-2) var(--space-3);
  border-bottom: 1px solid var(--border-subtle);
  cursor: pointer;
  user-select: none;
  transition: background var(--duration-fast) var(--ease-default);
}

.skill-row:hover {
  background: var(--surface-hover);
}

.skill-name {
  flex: 1;
  font-size: var(--text-md);
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  min-width: 0;
}

/* Switched off here: struck through, not merely faded. Opacity alone reads as
   "loading" as often as it reads as "off". */
.skill-row.off .skill-name {
  color: var(--text-tertiary);
  text-decoration: line-through;
  text-decoration-thickness: 1px;
  text-decoration-color: var(--border-strong);
}

.skill-row.archived .skill-name {
  color: var(--text-tertiary);
}

.row-plate {
  font-size: 9px;
  padding: 1px var(--space-2) 0;
  flex-shrink: 0;
}

.row-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-shrink: 0;
}

.action {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  opacity: 0;
  transition: opacity var(--duration-fast) var(--ease-default),
    background var(--duration-fast) var(--ease-default);
}

.skill-row:hover .action,
.skill-row:focus-within .action {
  opacity: 1;
}

.action:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

/* ── The breaker ──────────────────────────────────────────── */

/* A breaker is a housing with a handle in it. The housing stays quiet — a
   column of saturated blocks down a list is the loudest thing on the screen
   and says nothing that the handle's position doesn't already say. */
.breaker {
  position: relative;
  width: 24px;
  height: 13px;
  flex-shrink: 0;
  padding: 0;
  cursor: pointer;
  background: var(--surface-input);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-xs);
  transition: border-color var(--duration-normal) var(--ease-default);
}

.breaker:hover {
  border-color: var(--border-strong);
}

.breaker-handle {
  position: absolute;
  top: 1px;
  bottom: 1px;
  right: 1px;
  width: 9px;
  background: var(--accent);
  border-radius: 1px;
  /* Damped, like a switch throwing — not a spring. */
  transition: transform var(--duration-normal) var(--ease-default),
    background var(--duration-normal) var(--ease-default);
}

/* Thrown: the handle moves to the off position and goes dead. */
.breaker.thrown .breaker-handle {
  transform: translateX(-11px);
  background: var(--border-strong);
}
</style>

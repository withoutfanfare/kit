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

    <span v-if="skill.source === 'local'" class="badge row-plate">Local</span>
    <span v-if="skill.archived" class="badge row-plate">Archived</span>

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

      <!-- A standard switch. The previous version drew a bespoke "breaker",
           which is an invented affordance for a task every user already knows. -->
      <button
        v-if="skill.linkState === 'linked'"
        class="switch"
        :class="{ on: !skill.disabled }"
        role="switch"
        :aria-checked="!skill.disabled"
        :title="skill.disabled ? `Switch ${skill.name} on here` : `Switch ${skill.name} off here`"
        @click.stop="$emit('toggleActivation')"
      >
        <span class="switch-knob" aria-hidden="true"></span>
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
  min-height: var(--control-lg);
  padding: var(--space-3) var(--space-4);
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

/* A switch, with its full state set. 26x16 with a 12px knob is the size every
   OS uses, so it reads instantly. */
.switch {
  position: relative;
  width: 26px;
  height: 16px;
  flex-shrink: 0;
  padding: 0;
  border: 0;
  cursor: pointer;
  border-radius: var(--radius-full);
  background: var(--k-layer-3);
  box-shadow: inset 0 0 0 1px var(--k-line);
  transition: background var(--duration-normal) var(--ease-inout),
    box-shadow var(--duration-normal) var(--ease-inout);
}

.switch:hover {
  background: var(--k-layer-4);
}

.switch.on {
  background: var(--k-accent);
  box-shadow: inset 0 0 0 1px transparent;
}

.switch.on:hover {
  background: var(--k-accent-hover);
}

.switch:active .switch-knob {
  width: 14px;
}

.switch-knob {
  position: absolute;
  top: 2px;
  left: 2px;
  width: 12px;
  height: 12px;
  border-radius: var(--radius-full);
  background: #fff;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
  transition: transform var(--duration-normal) var(--ease-inout),
    width var(--duration-fast) var(--ease-inout);
}

.switch.on .switch-knob {
  transform: translateX(10px);
}

</style>

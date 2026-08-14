<script setup lang="ts">
/**
 * The bus.
 *
 * A distribution board's circuits all branch off one spine, and where current
 * is flowing you can see it. That is the whole device here: a copper bus runs
 * down the inside edge, and the active destination is the branch it feeds.
 *
 * Subtitles show only on the active row. A permanent second line on every row
 * is furniture; on the active row it answers "where am I" for free.
 */
import { useRoute } from "vue-router";
import PanelIcon, { type IconName } from "@/components/base/PanelIcon.vue";

const route = useRoute();

const navItems: Array<{
  label: string;
  subtitle: string;
  to: string;
  icon: IconName;
}> = [
  { label: "Panel", subtitle: "The whole board at a glance", to: "/panel", icon: "panel" },
  { label: "Locations", subtitle: "Global and your projects", to: "/locations", icon: "location" },
  { label: "Library", subtitle: "Every skill and set you have", to: "/skills", icon: "library" },
  { label: "Loadout", subtitle: "What loads here, and its cost", to: "/loadout", icon: "loadout" },
  { label: "Usage", subtitle: "What has actually run", to: "/usage", icon: "usage" },
  { label: "Health", subtitle: "Broken links and bad declarations", to: "/health", icon: "health" },
  { label: "Modified", subtitle: "Recent SKILL.md edits", to: "/changelog", icon: "changelog" },
];

const footerItems: Array<{ label: string; to: string; icon: IconName }> = [
  { label: "Help", to: "/help", icon: "help" },
  { label: "Settings", to: "/settings", icon: "settings" },
];

function isActive(to: string): boolean {
  if (to === "/skills") {
    return route.path.startsWith("/skills") || route.path.startsWith("/sets");
  }
  return route.path.startsWith(to);
}
</script>

<template>
  <nav class="bus-nav" aria-label="Sections">
    <!-- The spine. Decorative in itself, so it is hidden from the reading order. -->
    <span class="bus-spine" aria-hidden="true" />

    <ul class="nav-list">
      <li v-for="item in navItems" :key="item.to">
        <RouterLink
          :to="item.to"
          class="branch"
          :class="{ live: isActive(item.to) }"
          :aria-current="isActive(item.to) ? 'page' : undefined"
        >
          <span class="tick" aria-hidden="true" />
          <PanelIcon :name="item.icon" class="branch-icon" />
          <span class="branch-text">
            <span class="branch-label">{{ item.label }}</span>
            <span v-if="isActive(item.to)" class="branch-sub">{{ item.subtitle }}</span>
          </span>
        </RouterLink>
      </li>
    </ul>

    <ul class="nav-list nav-foot">
      <li v-for="item in footerItems" :key="item.to">
        <RouterLink
          :to="item.to"
          class="branch branch-quiet"
          :class="{ live: isActive(item.to) }"
          :aria-current="isActive(item.to) ? 'page' : undefined"
        >
          <span class="tick" aria-hidden="true" />
          <PanelIcon :name="item.icon" class="branch-icon" />
          <span class="branch-text">
            <span class="branch-label">{{ item.label }}</span>
          </span>
        </RouterLink>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.bus-nav {
  position: relative;
  display: flex;
  flex-direction: column;
  width: var(--sidebar-width, 188px);
  flex-shrink: 0;
  background: var(--surface-sidebar);
  border-right: 1px solid var(--border-subtle);
  padding: var(--space-4) 0 var(--space-3);
  overflow: hidden;
}

/* Copper, and only ever here: this is the one place current genuinely runs. */
.bus-spine {
  position: absolute;
  top: var(--space-6);
  bottom: var(--space-6);
  left: 15px;
  width: 2px;
  background: linear-gradient(
    to bottom,
    transparent,
    var(--bus) 5%,
    var(--bus) 95%,
    transparent
  );
  opacity: 0.8;
}

.nav-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.nav-foot {
  margin-top: auto;
  padding-top: var(--space-4);
}

.branch {
  position: relative;
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4) var(--space-3) 26px;
  color: var(--text-secondary);
  text-decoration: none;
  border-radius: var(--radius-sm);
  margin: 0 var(--space-3) 0 0;
  transition: background var(--duration-fast) var(--ease-default),
    color var(--duration-fast) var(--ease-default);
}

/* The branch line: a short run from the bus to this circuit. */
.tick {
  position: absolute;
  left: 15px;
  top: 50%;
  width: 11px;
  height: 2px;
  background: var(--bus);
  opacity: 0;
  transform: scaleX(0);
  transform-origin: left center;
  transition: opacity var(--duration-normal) var(--ease-out),
    transform var(--duration-normal) var(--ease-out);
}

.branch:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.branch.live {
  background: var(--surface-selected);
  color: var(--text-primary);
}

/* Current reaches the live branch — the one authored moment in the chrome. */
.branch.live .tick {
  opacity: 0.95;
  transform: scaleX(1);
}

.branch-icon {
  margin-top: 1px;
  opacity: 0.75;
}

.branch.live .branch-icon {
  opacity: 1;
  color: var(--accent);
}

.branch-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
  gap: 1px;
}

.branch-label {
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  line-height: 1.35;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.branch.live .branch-label {
  font-weight: var(--weight-semibold);
}

.branch-sub {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  line-height: 1.35;
  text-wrap: pretty;
}

.branch-quiet .branch-label {
  font-weight: var(--weight-normal);
  color: var(--text-tertiary);
}

.branch-quiet.live .branch-label,
.branch-quiet:hover .branch-label {
  color: var(--text-primary);
}

/* Narrow window: the labels go, the bus and its branches stay — the spine is
   what makes an icon rail still read as a panel rather than a toolbar. */
@media (max-width: 1120px) {
  .bus-nav {
    width: 52px;
    align-items: stretch;
  }

  .branch {
    padding-left: 22px;
    padding-right: var(--space-2);
    margin-right: var(--space-2);
  }

  .branch-text {
    display: none;
  }

  .bus-spine {
    left: 11px;
  }

  .tick {
    left: 11px;
    width: 8px;
  }
}
</style>

<script setup lang="ts">
/**
 * The source list.
 *
 * A standard app sidebar, detailed properly: 28px rows, 6px radius, an icon
 * that takes the accent when active, and a full set of interaction states.
 * The previous version ran a decorative copper "bus" down the edge and hid
 * every label below 1120px while leaving the icons aria-hidden — which left
 * the navigation with no accessible names at all.
 */
import { useRoute } from "vue-router";
import PanelIcon, { type IconName } from "@/components/base/PanelIcon.vue";

const route = useRoute();

const navItems: Array<{ label: string; to: string; icon: IconName }> = [
  { label: "Panel", to: "/panel", icon: "panel" },
  { label: "Locations", to: "/locations", icon: "location" },
  { label: "Library", to: "/skills", icon: "library" },
  { label: "Loadout", to: "/loadout", icon: "loadout" },
  { label: "Usage", to: "/usage", icon: "usage" },
  { label: "Health", to: "/health", icon: "health" },
  { label: "Modified", to: "/changelog", icon: "changelog" },
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
  <nav class="sidebar" aria-label="Sections">
    <ul class="nav">
      <li v-for="item in navItems" :key="item.to">
        <RouterLink
          :to="item.to"
          class="item"
          :class="{ active: isActive(item.to) }"
          :aria-current="isActive(item.to) ? 'page' : undefined"
          :aria-label="item.label"
        >
          <PanelIcon :name="item.icon" :size="15" class="item-icon" />
          <span class="item-label">{{ item.label }}</span>
        </RouterLink>
      </li>
    </ul>

    <ul class="nav nav-foot">
      <li v-for="item in footerItems" :key="item.to">
        <RouterLink
          :to="item.to"
          class="item"
          :class="{ active: isActive(item.to) }"
          :aria-current="isActive(item.to) ? 'page' : undefined"
          :aria-label="item.label"
        >
          <PanelIcon :name="item.icon" :size="15" class="item-icon" />
          <span class="item-label">{{ item.label }}</span>
        </RouterLink>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  width: var(--sidebar-width);
  flex-shrink: 0;
  background: var(--k-bg);
  border-right: 1px solid var(--k-line);
  padding: var(--space-5) var(--space-4) var(--space-5);
}

.nav {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.nav-foot {
  margin-top: auto;
}

.item {
  display: flex;
  align-items: center;
  gap: var(--space-5);
  height: var(--control-md);
  padding: 0 var(--space-4);
  border-radius: var(--radius-md);
  color: var(--k-text-3);
  text-decoration: none;
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  letter-spacing: var(--track-normal);
  transition: background var(--duration-fast) var(--ease-inout),
    color var(--duration-fast) var(--ease-inout);
}

.item-icon {
  color: var(--k-text-4);
  transition: color var(--duration-fast) var(--ease-inout);
}

.item:hover {
  background: var(--k-layer-2);
  color: var(--k-text);
}

.item:hover .item-icon {
  color: var(--k-text-2);
}

.item:active {
  background: var(--k-layer-3);
}

.item.active {
  background: var(--k-layer-3);
  color: var(--k-text);
  font-weight: var(--weight-semibold);
}

.item.active .item-icon {
  color: var(--k-accent);
}

.item-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Narrow: icons only. The aria-label on the link keeps every destination
   named, which is the bug the previous version shipped with. */
@media (max-width: 1120px) {
  .sidebar {
    width: 52px;
    padding-left: var(--space-3);
    padding-right: var(--space-3);
  }

  .item {
    justify-content: center;
    padding: 0;
    gap: 0;
  }

  .item-label {
    display: none;
  }
}
</style>

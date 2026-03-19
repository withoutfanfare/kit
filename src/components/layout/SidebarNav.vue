<script setup lang="ts">
import { useRoute } from "vue-router";

const route = useRoute();

const navItems = [
  { label: "Locations", subtitle: "Your projects", to: "/locations", icon: "folder" },
  { label: "Skills", subtitle: "Skill library", to: "/skills", icon: "puzzle" },
  { label: "Sets", subtitle: "Skill groups", to: "/sets", icon: "set" },
];

function isActive(to: string): boolean {
  return route.path.startsWith(to);
}
</script>

<template>
  <nav class="sidebar-nav">
    <div class="nav-section">
      <router-link
        v-for="item in navItems"
        :key="item.to"
        :to="item.to"
        class="nav-item"
        :class="{ active: isActive(item.to) }"
      >
        <span class="nav-icon">
          <svg v-if="item.icon === 'folder'" width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M2 4.5C2 3.67 2.67 3 3.5 3H6.29a1 1 0 01.7.29L8 4.3a1 1 0 00.71.29H12.5c.83 0 1.5.67 1.5 1.5v5.4c0 .83-.67 1.5-1.5 1.5h-9A1.5 1.5 0 012 11.5v-7z" fill="currentColor" opacity="0.7"/>
          </svg>
          <svg v-else-if="item.icon === 'puzzle'" width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M8.5 2.5a1.5 1.5 0 00-3 0V4H4a1 1 0 00-1 1v1.5h1.5a1.5 1.5 0 010 3H3V11a1 1 0 001 1h1.5v-1.5a1.5 1.5 0 013 0V12H10a1 1 0 001-1V9.5h-1.5a1.5 1.5 0 010-3H11V5a1 1 0 00-1-1H8.5V2.5z" fill="currentColor" opacity="0.7"/>
          </svg>
          <svg v-else-if="item.icon === 'set'" width="16" height="16" viewBox="0 0 16 16" fill="none">
            <rect x="1.5" y="3" width="10" height="7" rx="1.5" fill="currentColor" opacity="0.4"/>
            <rect x="4.5" y="6" width="10" height="7" rx="1.5" fill="currentColor" opacity="0.7"/>
          </svg>
          <svg v-else-if="item.icon === 'chart'" width="16" height="16" viewBox="0 0 16 16" fill="none">
            <rect x="2" y="8" width="3" height="5" rx="0.5" fill="currentColor" opacity="0.7"/>
            <rect x="6.5" y="5" width="3" height="8" rx="0.5" fill="currentColor" opacity="0.7"/>
            <rect x="11" y="3" width="3" height="10" rx="0.5" fill="currentColor" opacity="0.7"/>
          </svg>
        </span>
        <span class="nav-text">
          <span class="nav-label">{{ item.label }}</span>
          <span class="nav-subtitle">{{ item.subtitle }}</span>
        </span>
      </router-link>
    </div>
    <div class="nav-footer">
      <router-link
        to="/help"
        class="footer-button"
        :class="{ active: isActive('/help') }"
        title="Help"
      >
        <svg width="18" height="18" viewBox="0 0 16 16" fill="none">
          <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.7"/>
          <path d="M6.5 6.5a1.5 1.5 0 012.83.7c0 1-1.33 1.3-1.33 1.3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" opacity="0.7" fill="none"/>
          <circle cx="8" cy="11" r="0.75" fill="currentColor" opacity="0.7"/>
        </svg>
        <span class="footer-label">Help</span>
      </router-link>
      <router-link
        to="/settings"
        class="footer-button"
        :class="{ active: isActive('/settings') }"
        title="Settings"
      >
        <svg width="18" height="18" viewBox="0 0 16 16" fill="none">
          <path d="M8 10a2 2 0 100-4 2 2 0 000 4z" fill="currentColor" opacity="0.7"/>
          <path d="M7.03 2a.75.75 0 00-.74.63l-.18 1.09a4.98 4.98 0 00-.94.55l-1.03-.41a.75.75 0 00-.9.33l-.97 1.68a.75.75 0 00.16.96l.85.68a5 5 0 000 1.1l-.85.68a.75.75 0 00-.16.96l.97 1.68a.75.75 0 00.9.33l1.03-.41c.29.22.6.4.94.55l.18 1.09a.75.75 0 00.74.63h1.94a.75.75 0 00.74-.63l.18-1.09c.33-.15.65-.33.94-.55l1.03.41a.75.75 0 00.9-.33l.97-1.68a.75.75 0 00-.16-.96l-.85-.68a5 5 0 000-1.1l.85-.68a.75.75 0 00.16-.96l-.97-1.68a.75.75 0 00-.9-.33l-1.03.41a4.98 4.98 0 00-.94-.55l-.18-1.09A.75.75 0 008.97 2H7.03z" fill="currentColor" opacity="0.3"/>
        </svg>
        <span class="footer-label">Settings</span>
      </router-link>
    </div>
  </nav>
</template>

<style scoped>
.sidebar-nav {
  width: var(--sidebar-width);
  flex-shrink: 0;
  background: var(--surface-sidebar);
  border-right: 1px solid var(--border-subtle);
  padding: var(--space-3) var(--space-2);
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.nav-section {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.nav-item {
  display: flex;
  align-items: flex-start;
  gap: var(--space-3);
  min-height: 38px;
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  text-decoration: none;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  transition: background var(--duration-fast) var(--ease-default),
    color var(--duration-fast) var(--ease-default);
  cursor: default;
}

.nav-item:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

.nav-item.active {
  background: var(--surface-selected);
  color: var(--text-primary);
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  margin-top: 1px;
}

.nav-icon svg {
  width: 18px;
  height: 18px;
}

.nav-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.nav-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-subtitle {
  font-size: var(--text-xs);
  font-weight: var(--weight-normal);
  color: var(--text-tertiary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.3;
}

/* Footer with Help + Settings */
.nav-footer {
  margin-top: auto;
  padding-top: var(--space-3);
  border-top: 1px solid var(--border-subtle);
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.footer-button {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  height: 34px;
  padding: 0 var(--space-3);
  border-radius: var(--radius-sm);
  color: var(--text-tertiary);
  text-decoration: none;
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  cursor: default;
  transition: background var(--duration-fast) var(--ease-default),
    color var(--duration-fast) var(--ease-default);
}

.footer-button:hover {
  background: var(--surface-hover);
  color: var(--text-secondary);
}

.footer-button.active {
  background: var(--surface-selected);
  color: var(--text-primary);
}

.footer-button svg {
  flex-shrink: 0;
}

.footer-label {
  white-space: nowrap;
}
</style>

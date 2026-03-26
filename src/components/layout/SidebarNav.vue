<script setup lang="ts">
import { useRoute } from "vue-router";
import { SSidebar, SSidebarLink } from "@stuntrocket/ui";

const route = useRoute();

const navItems = [
  { label: "Locations", subtitle: "Your projects", to: "/locations", icon: "folder" },
  { label: "Skills", subtitle: "Skill library", to: "/skills", icon: "puzzle" },
  { label: "Sets", subtitle: "Skill groups", to: "/sets", icon: "set" },
  { label: "Changelog", subtitle: "Recent changes", to: "/changelog", icon: "chart" },
  { label: "Health", subtitle: "Issue scanner", to: "/health", icon: "health" },
];

function isActive(to: string): boolean {
  return route.path.startsWith(to);
}
</script>

<template>
  <SSidebar width="var(--sidebar-width)" class="sidebar-nav">
    <template #default>
      <div class="nav-section">
        <SSidebarLink
          v-for="item in navItems"
          :key="item.to"
          :to="item.to"
          :active="isActive(item.to)"
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
            <svg v-else-if="item.icon === 'health'" width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path d="M8 14s-5.5-3.5-5.5-7.5C2.5 4 4 2.5 5.75 2.5 6.87 2.5 7.63 3.1 8 3.8c.37-.7 1.13-1.3 2.25-1.3C12 2.5 13.5 4 13.5 6.5 13.5 10.5 8 14 8 14z" fill="currentColor" opacity="0.5"/>
              <path d="M5.5 7.5h2v-2h1v2h2v1h-2v2h-1v-2h-2v-1z" fill="currentColor" opacity="0.9"/>
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
        </SSidebarLink>
      </div>
    </template>
    <template #footer>
      <SSidebarLink to="/help" :active="isActive('/help')">
        <span class="footer-icon">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <circle cx="8" cy="8" r="6" stroke="currentColor" stroke-width="1.5" opacity="0.7"/>
            <path d="M6.5 6.5a1.5 1.5 0 012.83.7c0 1-1.33 1.3-1.33 1.3" stroke="currentColor" stroke-width="1.3" stroke-linecap="round" stroke-linejoin="round" opacity="0.7" fill="none"/>
            <circle cx="8" cy="11" r="0.75" fill="currentColor" opacity="0.7"/>
          </svg>
        </span>
        <span class="footer-label">Help</span>
      </SSidebarLink>
      <SSidebarLink to="/settings" :active="isActive('/settings')">
        <span class="footer-icon">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M8 10a2 2 0 100-4 2 2 0 000 4z" fill="currentColor" opacity="0.7"/>
            <path d="M7.03 2a.75.75 0 00-.74.63l-.18 1.09a4.98 4.98 0 00-.94.55l-1.03-.41a.75.75 0 00-.9.33l-.97 1.68a.75.75 0 00.16.96l.85.68a5 5 0 000 1.1l-.85.68a.75.75 0 00-.16.96l.97 1.68a.75.75 0 00.9.33l1.03-.41c.29.22.6.4.94.55l.18 1.09a.75.75 0 00.74.63h1.94a.75.75 0 00.74-.63l.18-1.09c.33-.15.65-.33.94-.55l1.03.41a.75.75 0 00.9-.33l.97-1.68a.75.75 0 00-.16-.96l-.85-.68a5 5 0 000-1.1l.85-.68a.75.75 0 00.16-.96l-.97-1.68a.75.75 0 00-.9-.33l-1.03.41a4.98 4.98 0 00-.94-.55l-.18-1.09A.75.75 0 008.97 2H7.03z" fill="currentColor" opacity="0.3"/>
          </svg>
        </span>
        <span class="footer-label">Settings</span>
      </SSidebarLink>
    </template>
  </SSidebar>
</template>

<style scoped>
.nav-section {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.nav-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  flex-shrink: 0;
  align-self: flex-start;
  margin-top: 2px;
}

.nav-icon svg {
  width: 18px;
  height: 18px;
}

.nav-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
  margin-left: 8px;
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

.footer-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.footer-label {
  margin-left: 8px;
}
</style>

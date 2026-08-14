import { createRouter, createWebHistory } from "vue-router";
import { useAppStore } from "@/stores/appStore";
import { usePreferencesStore } from "@/stores/preferencesStore";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      // Honours the Default view preference. It used to hard-redirect, which
      // made the setting in Settings a control that changed nothing.
      //
      // A guard rather than a `redirect`, because a record's redirect is
      // resolved before any guard runs and so cannot wait for anything. The
      // component is never rendered: the guard always sends the navigation on.
      path: "/",
      name: "root",
      component: () => import("@/views/PanelView.vue"),
      beforeEnter: async () => {
        // localStorage is only a mirror of the preference, written during
        // bootstrap. Warm, it answers instantly and the window paints at once.
        const stored = localStorage.getItem("kit.defaultView");
        if (stored === "panel" || stored === "locations" || stored === "skills") {
          return `/${stored}`;
        }
        // Cold — first run after upgrading, or storage cleared. Guessing here
        // would ignore a saved default and open the wrong view, so wait for the
        // real one the once. If bootstrap fails, Panel is the safe landing.
        const app = useAppStore();
        const ok = await app.ensureBootstrapped();
        return ok ? `/${usePreferencesStore().defaultView}` : "/panel";
      },
    },
    {
      // The board at a glance: what loads, what it costs, what needs attention.
      path: "/panel",
      name: "panel",
      component: () => import("@/views/PanelView.vue"),
    },
    {
      path: "/locations",
      name: "locations",
      component: () => import("@/views/LocationsView.vue"),
      children: [
        {
          path: ":locationId",
          name: "location-detail",
          component: () => import("@/views/LocationDetailView.vue"),
          props: true,
        },
      ],
    },
    {
      path: "/skills",
      name: "skills",
      component: () => import("@/views/SkillsView.vue"),
      children: [
        {
          path: ":skillId",
          name: "skill-detail",
          component: () => import("@/views/SkillDetailView.vue"),
          props: true,
        },
      ],
    },
    {
      path: "/sets",
      name: "sets",
      component: () => import("@/views/SetsView.vue"),
      children: [
        {
          path: ":setKey",
          name: "set-detail",
          component: () => import("@/views/SetDetailView.vue"),
          props: true,
        },
      ],
    },
    {
      path: "/compare",
      name: "compare",
      component: () => import("@/views/CompareLocationsView.vue"),
    },
    {
      path: "/changelog",
      name: "changelog",
      component: () => import("@/views/ChangelogView.vue"),
    },
    {
      path: "/loadout/:locationId?",
      name: "loadout",
      component: () => import("@/views/LoadoutView.vue"),
      props: true,
    },
    {
      path: "/usage",
      name: "usage",
      component: () => import("@/views/UsageView.vue"),
    },
    {
      path: "/health",
      name: "health",
      component: () => import("@/views/HealthView.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/SettingsView.vue"),
    },
    {
      path: "/help",
      name: "help",
      component: () => import("@/views/HelpView.vue"),
    },
  ],
});

export default router;

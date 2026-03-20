import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/locations",
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

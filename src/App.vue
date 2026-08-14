<script setup lang="ts">
import AppShell from "@/components/layout/AppShell.vue";
import { useAppStore } from "@/stores/appStore";
import { useTheme } from "@/composables/useTheme";
import { onMounted } from "vue";

const appStore = useAppStore();

// Initialise theme from localStorage (applies .dark/.light class to <html>)
useTheme();

onMounted(async () => {
  // The root route may already have started this; both share the one request.
  await appStore.ensureBootstrapped();
});
</script>

<template>
  <AppShell />
</template>

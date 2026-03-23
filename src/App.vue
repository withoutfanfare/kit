<script setup lang="ts">
import AppShell from "@/components/layout/AppShell.vue";
import { useAppStore } from "@/stores/appStore";
import { useTheme } from "@/composables/useTheme";
import { onMounted } from "vue";

const appStore = useAppStore();

// Initialise theme from localStorage (applies .dark/.light class to <html>)
useTheme();

onMounted(async () => {
  await appStore.bootstrap();
});
</script>

<template>
  <AppShell />
</template>

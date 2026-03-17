<script setup lang="ts">
import AppShell from "@/components/layout/AppShell.vue";
import { useAppStore } from "@/stores/appStore";
import { onMounted } from "vue";

const appStore = useAppStore();

onMounted(async () => {
  await appStore.bootstrap();
});
</script>

<template>
  <AppShell />
</template>

<script setup lang="ts">
import { STopbar, SSearchInput } from "@stuntrocket/ui";
import { useAppStore } from "@/stores/appStore";

const appStore = useAppStore();
</script>

<template>
  <STopbar traffic-light-padding="76px">
    <template #left>
      <SSearchInput
        v-model="appStore.globalSearchQuery"
        placeholder="Search skills, locations..."
        compact
      />
    </template>
  </STopbar>
</template>

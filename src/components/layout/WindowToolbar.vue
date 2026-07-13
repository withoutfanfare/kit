<script setup lang="ts">
import { STopbar } from "@stuntrocket/ui";
import GlobalSearchResults from "@/components/domain/GlobalSearchResults.vue";
</script>

<template>
  <STopbar traffic-light-padding="76px">
    <template #left>
      <GlobalSearchResults />
    </template>
  </STopbar>
</template>

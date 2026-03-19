<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useRouter } from "vue-router";
import type { UnlistenFn } from "@tauri-apps/api/event";
import WindowToolbar from "./WindowToolbar.vue";
import SidebarNav from "./SidebarNav.vue";
import NoticeBanner from "@/components/base/NoticeBanner.vue";
import OnboardingView from "@/views/OnboardingView.vue";
import SkillPeekPanel from "@/components/domain/SkillPeekPanel.vue";
import { useAppStore } from "@/stores/appStore";
import type { SkillsRepoStatus } from "@/types";

const router = useRouter();
let unlistenNavigate: UnlistenFn | null = null;

onMounted(async () => {
  try {
    unlistenNavigate = await listen<string>("navigate", (event) => {
      router.push(event.payload);
    });
  } catch {
    // Listen may fail when running frontend-only dev mode without Tauri
  }
});

onUnmounted(() => {
  unlistenNavigate?.();
});

const appStore = useAppStore();
const repoBannerDismissed = ref(false);
const repoStatus = ref<SkillsRepoStatus | null>(null);

watch(
  () => appStore.isBootstrapped,
  (bootstrapped) => {
    if (bootstrapped && !appStore.needsSetup) {
      invoke<SkillsRepoStatus>("get_skills_repo_status")
        .then((status) => {
          repoStatus.value = status;
        })
        .catch(() => {
          // Silently ignore — status is optional
        });
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="app-shell">
    <!-- Onboarding: full-screen replacement when setup is needed -->
    <template v-if="appStore.needsSetup">
      <OnboardingView />
    </template>

    <template v-else>
    <WindowToolbar />
    <NoticeBanner
      v-if="repoStatus && repoStatus.state === 'behind' && !repoBannerDismissed"
      variant="warning"
      dismissible
      class="welcome-banner"
      @dismiss="repoBannerDismissed = true"
    >
      Skills repository is {{ repoStatus.behindBy }} commit{{ repoStatus.behindBy === 1 ? '' : 's' }} behind {{ repoStatus.upstream ?? 'remote' }} — pull to get the latest skills.
    </NoticeBanner>
    <div class="app-body">
      <SidebarNav />
      <main class="app-content">
        <router-view v-slot="{ Component }">
          <transition name="fade" mode="out-in">
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>
    </template>

    <!-- Skill peek panel -->
    <SkillPeekPanel />

    <!-- Global error -->
    <div v-if="appStore.globalError" class="global-error">
      <span>{{ appStore.globalError }}</span>
      <button @click="appStore.clearError()">Dismiss</button>
    </div>

    <!-- Toast notifications -->
    <TransitionGroup name="toast" tag="div" class="toast-container">
      <div
        v-for="t in appStore.toasts"
        :key="t.id"
        class="toast"
        :class="t.variant"
        @click="appStore.dismissToast(t.id)"
      >
        <svg v-if="t.variant === 'success'" class="toast-icon" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
          <path d="M4.5 7l2 2 3-3.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>
        </svg>
        <svg v-else-if="t.variant === 'error'" class="toast-icon" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
          <path d="M5 5l4 4M9 5l-4 4" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <svg v-else class="toast-icon" width="14" height="14" viewBox="0 0 14 14" fill="none">
          <circle cx="7" cy="7" r="6" stroke="currentColor" stroke-width="1.5"/>
          <path d="M7 5v3M7 9.5v.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
        <span class="toast-message">{{ t.message }}</span>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.app-shell {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--surface-app);
  overflow: hidden;
}

.app-body {
  display: flex;
  flex: 1;
  min-height: 0;
}

.app-content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.global-error {
  position: fixed;
  bottom: var(--space-4);
  left: 50%;
  transform: translateX(-50%);
  background: var(--danger);
  color: var(--text-inverse);
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  display: flex;
  align-items: center;
  gap: var(--space-3);
  box-shadow: var(--shadow-lg);
  z-index: 100;
}

.global-error button {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: var(--text-inverse);
  padding: var(--space-1) var(--space-2);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
}

.welcome-banner {
  flex-shrink: 0;
  border-radius: 0;
}

/* Toast system */
.toast-container {
  position: fixed;
  bottom: var(--space-5);
  right: var(--space-5);
  display: flex;
  flex-direction: column-reverse;
  gap: var(--space-2);
  z-index: 150;
  pointer-events: none;
}

.toast {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-md);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  box-shadow: var(--shadow-md);
  cursor: pointer;
  pointer-events: auto;
  max-width: 320px;
  backdrop-filter: blur(8px);
}

.toast.success {
  background: var(--success);
  color: #fff;
}

.toast.error {
  background: var(--danger);
  color: #fff;
}

.toast.info {
  background: var(--surface-panel);
  color: var(--text-primary);
  border: 1px solid var(--border-default);
}

.toast-icon {
  flex-shrink: 0;
}

.toast-message {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Toast transitions */
.toast-enter-active {
  transition: all var(--duration-normal) var(--ease-out);
}

.toast-leave-active {
  transition: all var(--duration-fast) var(--ease-default);
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(8px) scale(0.95);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(16px);
}
</style>

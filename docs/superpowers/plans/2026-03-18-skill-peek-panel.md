# Skill Peek Panel Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a global slide-over panel that shows skill details when clicking any skill name outside the dedicated `/skills` view.

**Architecture:** A new Pinia store (`skillPeekStore`) manages peek state and caches fetched skill details. A new `SkillPeekPanel` component is mounted once in `AppShell.vue`, teleported to `body`, and slides in from the right edge. Each view/component that displays skills is wired to call `peek(skillId)` on click.

**Tech Stack:** Vue 3 Composition API, Pinia, Tauri IPC (`invoke`), CSS custom properties

**Spec:** `docs/superpowers/specs/2026-03-18-skill-peek-panel-design.md`

---

## File Map

| File | Action | Responsibility |
|---|---|---|
| `src/stores/skillPeekStore.ts` | Create | Peek state, detail fetching, cache |
| `src/components/domain/SkillPeekPanel.vue` | Create | Slide-over panel UI with loading/error/loaded states |
| `src/components/layout/AppShell.vue` | Modify | Mount SkillPeekPanel globally |
| `src/components/domain/SkillRow.vue` | Modify | Change cursor to pointer |
| `src/views/LocationDetailView.vue` | Modify | Handle @selectSkill to peek |
| `src/views/SetDetailView.vue` | Modify | Add peek to skill rows + picker |
| `src/views/UsageView.vue` | Modify | Replace router.push with peek |
| `src/components/domain/AssignmentSheet.vue` | Modify | Add info icon buttons |
| `src/stores/libraryStore.ts` | Modify | Clear peek cache on fetchItems |

---

### Task 1: Create skillPeekStore

**Files:**
- Create: `src/stores/skillPeekStore.ts`

- [ ] **Step 1: Create the store file**

```ts
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { SkillDetail, SkillId } from "@/types";

export const useSkillPeekStore = defineStore("skillPeek", () => {
  const peekSkillId = ref<SkillId | null>(null);
  const detail = ref<SkillDetail | null>(null);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const cache = ref<Record<SkillId, SkillDetail>>({});

  const isOpen = computed(() => peekSkillId.value !== null);

  async function peek(id: SkillId) {
    peekSkillId.value = id;
    error.value = null;

    if (cache.value[id]) {
      detail.value = cache.value[id];
      return;
    }

    isLoading.value = true;
    detail.value = null;
    try {
      const result = await invoke<SkillDetail>("get_skill_detail", {
        skillId: id,
      });
      cache.value[id] = result;
      // Only set detail if we're still peeking at the same skill
      if (peekSkillId.value === id) {
        detail.value = result;
      }
    } catch {
      if (peekSkillId.value === id) {
        error.value = "Could not load skill details";
      }
    } finally {
      isLoading.value = false;
    }
  }

  function close() {
    peekSkillId.value = null;
    detail.value = null;
    error.value = null;
  }

  function clearCache() {
    cache.value = {};
  }

  return {
    peekSkillId,
    detail,
    isLoading,
    error,
    isOpen,
    peek,
    close,
    clearCache,
  };
});
```

- [ ] **Step 2: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 3: Commit**

```bash
git add src/stores/skillPeekStore.ts
git commit -m "feat: add skillPeekStore for global skill peek panel"
```

---

### Task 2: Create SkillPeekPanel component

**Files:**
- Create: `src/components/domain/SkillPeekPanel.vue`

- [ ] **Step 1: Create the component file**

```vue
<script setup lang="ts">
import { watch, ref, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useSkillPeekStore } from "@/stores/skillPeekStore";
import { usePreferencesStore } from "@/stores/preferencesStore";
import Badge from "@/components/base/Badge.vue";
import SecondaryButton from "@/components/base/SecondaryButton.vue";

const peekStore = useSkillPeekStore();
const preferencesStore = usePreferencesStore();
const panelRef = ref<HTMLElement | null>(null);
const previousFocus = ref<HTMLElement | null>(null);

function handleBackdropClick(event: MouseEvent) {
  event.stopPropagation();
  peekStore.close();
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Escape") {
    peekStore.close();
  }
}

async function openInEditor() {
  if (!peekStore.detail) return;
  await invoke("open_path_in_editor", {
    path: peekStore.detail.path,
    editorCommand: preferencesStore.editorCommand ?? "code",
  });
}

async function revealInFinder() {
  if (!peekStore.detail) return;
  await invoke("reveal_in_finder", { path: peekStore.detail.path });
}

watch(
  () => peekStore.isOpen,
  async (open) => {
    if (open) {
      previousFocus.value = document.activeElement as HTMLElement | null;
      await nextTick();
      panelRef.value?.focus();
      document.addEventListener("keydown", handleKeydown);
    } else {
      document.removeEventListener("keydown", handleKeydown);
      previousFocus.value?.focus();
      previousFocus.value = null;
    }
  }
);
</script>

<template>
  <Teleport to="body">
    <Transition name="peek">
      <div
        v-if="peekStore.isOpen"
        class="peek-backdrop"
        @click.self="handleBackdropClick"
      >
        <div
          ref="panelRef"
          class="peek-panel"
          tabindex="-1"
          role="dialog"
          aria-modal="true"
          :aria-label="peekStore.detail ? peekStore.detail.name : 'Skill details'"
        >
          <!-- Close button -->
          <button class="close-button" @click="peekStore.close()">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18" />
              <line x1="6" y1="6" x2="18" y2="18" />
            </svg>
          </button>

          <!-- Loading state -->
          <div v-if="peekStore.isLoading" class="peek-loading">
            <span class="spinner" />
          </div>

          <!-- Error state -->
          <div v-else-if="peekStore.error" class="peek-error">
            <p class="error-message">{{ peekStore.error }}</p>
            <SecondaryButton label="Close" @click="peekStore.close()" />
          </div>

          <!-- Loaded state -->
          <template v-else-if="peekStore.detail">
            <div class="peek-header">
              <h3 class="peek-name">{{ peekStore.detail.name }}</h3>
              <Badge v-if="peekStore.detail.archived" variant="default" compact>Archived</Badge>
            </div>

            <span class="peek-path">{{ peekStore.detail.path }}</span>

            <div v-if="peekStore.detail.summary" class="peek-section">
              <span class="section-label">Summary</span>
              <p class="summary-text">{{ peekStore.detail.summary }}</p>
            </div>

            <div class="peek-section">
              <span class="section-label">Linked locations</span>
              <div v-if="peekStore.detail.linkedLocations.length > 0" class="compact-list">
                <span
                  v-for="loc in peekStore.detail.linkedLocations"
                  :key="loc.id"
                  class="compact-item"
                >
                  {{ loc.label }}
                </span>
              </div>
              <span v-else class="empty-text">None</span>
            </div>

            <div class="peek-section">
              <span class="section-label">Included in sets</span>
              <div v-if="peekStore.detail.includedInSets.length > 0" class="compact-list">
                <span
                  v-for="set in peekStore.detail.includedInSets"
                  :key="set.id"
                  class="compact-item"
                >
                  {{ set.name }}
                </span>
              </div>
              <span v-else class="empty-text">None</span>
            </div>

            <div class="peek-section">
              <span class="section-label">Usage (30 days)</span>
              <span class="usage-count">{{ peekStore.detail.usage.useCount30d }} uses</span>
            </div>

            <div class="peek-actions">
              <SecondaryButton label="Open in Editor" @click="openInEditor" />
              <SecondaryButton label="Reveal in Finder" @click="revealInFinder" />
            </div>
          </template>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.peek-backdrop {
  position: fixed;
  inset: 0;
  z-index: 250;
  background: rgba(0, 0, 0, 0.08);
}

.peek-panel {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  width: 320px;
  background: var(--surface-panel);
  box-shadow: var(--shadow-sheet);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  padding: var(--space-4);
  overflow-y: auto;
  outline: none;
}

.close-button {
  position: absolute;
  top: var(--space-3);
  right: var(--space-3);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-default);
}

.close-button:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

/* Loading */
.peek-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 1.5px solid var(--border-default);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 600ms linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Error */
.peek-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  gap: var(--space-3);
}

.error-message {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0;
}

/* Header */
.peek-header {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  padding-right: var(--space-6);
}

.peek-name {
  font-size: var(--text-lg);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.peek-path {
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  font-family: ui-monospace, "SF Mono", SFMono-Regular, monospace;
  word-break: break-all;
}

/* Sections */
.peek-section {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.section-label {
  font-size: var(--text-xs);
  font-weight: var(--weight-medium);
  color: var(--text-tertiary);
}

.summary-text {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
}

.compact-list {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.compact-item {
  font-size: var(--text-sm);
  color: var(--text-secondary);
  padding: 1px 0;
}

.empty-text {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
}

.usage-count {
  font-size: var(--text-sm);
  color: var(--text-primary);
}

/* Actions */
.peek-actions {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-top: auto;
  padding-top: var(--space-3);
}

.peek-actions :deep(.secondary-button) {
  width: 100%;
  justify-content: center;
}

/* Transition */
.peek-enter-active {
  transition: opacity var(--duration-normal) var(--ease-out);
}

.peek-leave-active {
  transition: opacity var(--duration-fast) var(--ease-default);
}

.peek-enter-from,
.peek-leave-to {
  opacity: 0;
}

.peek-enter-active .peek-panel {
  animation: slide-in var(--duration-normal) var(--ease-out);
}

.peek-leave-active .peek-panel {
  animation: slide-out var(--duration-fast) var(--ease-default);
}

@keyframes slide-in {
  from { transform: translateX(100%); }
  to { transform: translateX(0); }
}

@keyframes slide-out {
  from { transform: translateX(0); }
  to { transform: translateX(100%); }
}
</style>
```

- [ ] **Step 2: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 3: Commit**

```bash
git add src/components/domain/SkillPeekPanel.vue
git commit -m "feat: add SkillPeekPanel slide-over component"
```

---

### Task 3: Mount SkillPeekPanel in AppShell

**Files:**
- Modify: `src/components/layout/AppShell.vue`

- [ ] **Step 1: Add the import**

Add after the existing imports (after line 7):

```ts
import SkillPeekPanel from "@/components/domain/SkillPeekPanel.vue";
```

- [ ] **Step 2: Add the component to the template**

Add `<SkillPeekPanel />` just before the global error div. Insert before the line `<!-- Global error -->`:

```html
    <!-- Skill peek panel -->
    <SkillPeekPanel />

    <!-- Global error -->
```

- [ ] **Step 3: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 4: Manual test**

Run: `npm run tauri dev`
Verify: app loads normally, no visual changes yet (panel is hidden by default)

- [ ] **Step 5: Commit**

```bash
git add src/components/layout/AppShell.vue
git commit -m "feat: mount SkillPeekPanel globally in AppShell"
```

---

### Task 4: Fix SkillRow cursor

**Files:**
- Modify: `src/components/domain/SkillRow.vue`

- [ ] **Step 1: Change cursor style**

In the `.skill-row` CSS rule, change `cursor: default` to `cursor: pointer`.

- [ ] **Step 2: Commit**

```bash
git add src/components/domain/SkillRow.vue
git commit -m "fix: change SkillRow cursor to pointer for click affordance"
```

---

### Task 5: Wire LocationDetailView

**Files:**
- Modify: `src/views/LocationDetailView.vue`

- [ ] **Step 1: Import the peek store**

Add to the `<script setup>` imports:

```ts
import { useSkillPeekStore } from "@/stores/skillPeekStore";
```

Add after the existing store initialisation:

```ts
const skillPeekStore = useSkillPeekStore();
```

- [ ] **Step 2: Add peek handler**

Add a function:

```ts
function peekSkill(skillId: string) {
  skillPeekStore.peek(skillId);
}
```

- [ ] **Step 3: Wire both SkillList instances**

On the first `<SkillList>` (linked skills), add the event handler:

```html
<SkillList
  :skills="linkedSkills"
  title="Linked Skills"
  show-link-state
  @select-skill="peekSkill"
/>
```

On the second `<SkillList>` (local-only skills), add the same:

```html
<SkillList
  v-if="localOnlySkills.length > 0"
  :skills="localOnlySkills"
  title="Local-Only Skills"
  show-link-state
  @select-skill="peekSkill"
/>
```

- [ ] **Step 4: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 5: Manual test**

Run: `npm run tauri dev`
Navigate to a location with skills. Click a skill row. Verify the peek panel slides in from the right showing the skill details. Click outside or press Escape to close.

- [ ] **Step 6: Commit**

```bash
git add src/views/LocationDetailView.vue
git commit -m "feat: wire skill peek to LocationDetailView skill rows"
```

---

### Task 6: Wire SetDetailView

**Files:**
- Modify: `src/views/SetDetailView.vue`

- [ ] **Step 1: Import the peek store**

Add to the `<script setup>` imports:

```ts
import { useSkillPeekStore } from "@/stores/skillPeekStore";
```

Add after the existing store initialisations:

```ts
const skillPeekStore = useSkillPeekStore();
```

- [ ] **Step 2: Wire skill rows in the skills section**

In the template, the `.skill-row` div (around line 222) currently only has a hover effect. Add a click handler to the `.skill-row-content` span (not the whole row, to avoid conflicting with the remove button):

Change:

```html
<div class="skill-row-content">
  <span class="skill-name">{{ skill.name }}</span>
  <Badge v-if="skill.archived" variant="default" compact>Archived</Badge>
</div>
```

To:

```html
<div class="skill-row-content" @click="skillPeekStore.peek(skill.id)">
  <span class="skill-name">{{ skill.name }}</span>
  <Badge v-if="skill.archived" variant="default" compact>Archived</Badge>
</div>
```

Add to the `.skill-row-content` CSS:

```css
.skill-row-content {
  cursor: pointer;
}
```

- [ ] **Step 3: Add info icon to skill picker rows**

In the skill picker sheet section, each `.picker-row` currently calls `addSkill(skill.id)` on click. Add an info icon button before the add icon. Change the picker row template from:

```html
<div
  v-for="skill in availableSkills"
  :key="skill.id"
  class="picker-row"
  @click="addSkill(skill.id)"
>
  <div class="picker-row-content">
    <span class="picker-row-name">{{ skill.name }}</span>
    <span v-if="skill.summary" class="picker-row-summary">{{ skill.summary }}</span>
  </div>
  <svg
    class="add-icon"
```

To:

```html
<div
  v-for="skill in availableSkills"
  :key="skill.id"
  class="picker-row"
  @click="addSkill(skill.id)"
>
  <div class="picker-row-content">
    <span class="picker-row-name">{{ skill.name }}</span>
    <span v-if="skill.summary" class="picker-row-summary">{{ skill.summary }}</span>
  </div>
  <button
    class="info-button"
    title="View skill details"
    @click.stop="skillPeekStore.peek(skill.id)"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="10" />
      <path d="M12 16v-4M12 8h.01" />
    </svg>
  </button>
  <svg
    class="add-icon"
```

Add the `.info-button` CSS to the scoped styles:

```css
.info-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  flex-shrink: 0;
  transition: all var(--duration-fast) var(--ease-default);
}

.info-button:hover {
  background: var(--surface-hover);
  color: var(--accent);
}
```

Note: `@click.stop` on the info button prevents the row's `addSkill` click from also firing.

- [ ] **Step 4: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 5: Manual test**

Run: `npm run tauri dev`
Navigate to a set detail view. Click a skill name — peek panel should open. Open the skill picker, click the info (ⓘ) icon on a skill — peek panel should open on top of the picker sheet.

- [ ] **Step 6: Commit**

```bash
git add src/views/SetDetailView.vue
git commit -m "feat: wire skill peek to SetDetailView rows and picker"
```

---

### Task 7: Wire UsageView

**Files:**
- Modify: `src/views/UsageView.vue`

- [ ] **Step 1: Replace router import with peek store**

In the `<script setup>`, replace:

```ts
import { useRouter } from "vue-router";
```

With:

```ts
import { useSkillPeekStore } from "@/stores/skillPeekStore";
```

Replace:

```ts
const router = useRouter();
```

With:

```ts
const skillPeekStore = useSkillPeekStore();
```

- [ ] **Step 2: Replace the navigateToSkill function**

Replace:

```ts
function navigateToSkill(skillId: string) {
  router.push({ name: "skill-detail", params: { skillId } });
}
```

With:

```ts
function navigateToSkill(skillId: string) {
  skillPeekStore.peek(skillId);
}
```

(Keeping the function name avoids changing every `@click` in the template.)

- [ ] **Step 3: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 4: Manual test**

Run: `npm run tauri dev`
Navigate to Usage view. Click a skill in Most Used, Recently Used, or Unused. Peek panel should open. User should stay on the Usage page.

- [ ] **Step 5: Commit**

```bash
git add src/views/UsageView.vue
git commit -m "feat: wire skill peek to UsageView, replacing navigation"
```

---

### Task 8: Wire AssignmentSheet

**Files:**
- Modify: `src/components/domain/AssignmentSheet.vue`

- [ ] **Step 1: Import the peek store**

Add to the `<script setup>` imports:

```ts
import { useSkillPeekStore } from "@/stores/skillPeekStore";
```

Add after the existing store initialisations:

```ts
const skillPeekStore = useSkillPeekStore();
```

- [ ] **Step 2: Add info icon to library skill rows**

In the library skills section (the `v-for="item in skills"` block), add an info button after the `item-info` div and before the `Badge` elements. Inside the `<label>` for each skill, after:

```html
<div class="item-info">
  <span class="item-name">{{ item.name }}</span>
  <span v-if="item.summary" class="item-summary">{{ item.summary }}</span>
</div>
```

Add:

```html
<button
  class="info-button"
  title="View skill details"
  @click.prevent.stop="skillPeekStore.peek(item.id)"
>
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="12" cy="12" r="10" />
    <path d="M12 16v-4M12 8h.01" />
  </svg>
</button>
```

Note: `@click.prevent.stop` prevents both the label's checkbox toggle and event propagation.

- [ ] **Step 3: Add info icon to installed skills section**

In the installed skills section (the `v-for="skill in installedSkills"` block), add the same info button after the `item-info` div:

```html
<button
  class="info-button"
  title="View skill details"
  @click.prevent.stop="skillPeekStore.peek(skill.skillId)"
>
  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <circle cx="12" cy="12" r="10" />
    <path d="M12 16v-4M12 8h.01" />
  </svg>
</button>
```

- [ ] **Step 4: Add info-button CSS**

Add to the scoped styles:

```css
.info-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  flex-shrink: 0;
  transition: all var(--duration-fast) var(--ease-default);
}

.info-button:hover {
  background: var(--surface-hover);
  color: var(--accent);
}
```

- [ ] **Step 5: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 6: Manual test**

Run: `npm run tauri dev`
Open a location, click "Add Skills". In the sheet, click the ⓘ icon next to a library skill — peek panel should open on top of the sheet. Click outside peek to close it but stay in the sheet. Scroll to "Installed" section and test the ⓘ icon there too.

- [ ] **Step 7: Commit**

```bash
git add src/components/domain/AssignmentSheet.vue
git commit -m "feat: add skill peek info icons to AssignmentSheet"
```

---

### Task 9: Wire cache invalidation

**Files:**
- Modify: `src/stores/libraryStore.ts`

- [ ] **Step 1: Import the peek store**

Add to the imports:

```ts
import { useSkillPeekStore } from "./skillPeekStore";
```

- [ ] **Step 2: Clear peek cache in fetchItems**

Inside the `fetchItems` function, after `items.value = await invoke(...)`, add:

```ts
useSkillPeekStore().clearCache();
```

The full `fetchItems` function becomes:

```ts
async function fetchItems() {
  isLoading.value = true;
  try {
    items.value = await invoke<LibraryListItem[]>("list_library_items");
    useSkillPeekStore().clearCache();
  } finally {
    isLoading.value = false;
  }
}
```

Note: calling `useSkillPeekStore()` inside the function (not at module level) avoids circular dependency issues since the peek store also imports from `@/types`.

- [ ] **Step 3: Verify it compiles**

Run: `npx vue-tsc --noEmit`
Expected: no new errors

- [ ] **Step 4: Commit**

```bash
git add src/stores/libraryStore.ts
git commit -m "feat: clear skill peek cache when library items are refreshed"
```

---

### Task 10: Final verification

- [ ] **Step 1: Full type check**

Run: `npx vue-tsc --noEmit`
Expected: no errors

- [ ] **Step 2: Full build**

Run: `npm run build`
Expected: builds successfully

- [ ] **Step 3: End-to-end manual test**

Run: `npm run tauri dev`

Test each integration point:

1. **Locations** — open a location, click a linked skill row → peek opens
2. **Locations** — click a local-only skill row → peek opens
3. **Sets** — open a set, click a skill name → peek opens
4. **Sets** — open skill picker, click ⓘ icon → peek opens above picker sheet
5. **Usage** — click any skill name → peek opens, stay on Usage page
6. **Assignment** — open "Add Skills", click ⓘ on library skill → peek opens above sheet
7. **Assignment** — click ⓘ on installed skill → peek opens above sheet
8. **Skills view** — click a skill in sidebar → navigates to detail as before (no peek)
9. **Close** — verify Escape, click-outside, and X button all close the peek
10. **Error** — temporarily rename a skill folder, peek that skill → error state shown

- [ ] **Step 4: Commit any final adjustments if needed**

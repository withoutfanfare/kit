# Production Readiness Fixes Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all 11 issues from the production readiness audit (PRA-001 through PRA-011), making the Kit desktop app functionally correct and release-ready.

**Architecture:** Each fix is isolated to the minimum files needed. The approach favours smallest-possible changes that resolve each issue without restructuring. Set identity (PRA-002) is the largest change, introducing a composite key helper that threads through the frontend routing, store, and components.

**Tech Stack:** Tauri v2, Rust (backend), Vue 3 + TypeScript + Pinia (frontend)

---

## File Map

### New files
- `src/utils/setKey.ts` — Composite set key helper (encode/decode scope+ownerLocationId+setId)

### Modified files (by task)

| Task | Files |
|------|-------|
| 1 (PRA-001) | `src/views/SetsView.vue`, `src/stores/locationsStore.ts` |
| 2 (PRA-003) | `src-tauri/src/commands/sets.rs` |
| 3 (PRA-002) | `src/utils/setKey.ts` (new), `src/stores/setsStore.ts`, `src/router.ts`, `src/views/SetsView.vue`, `src/views/SetDetailView.vue`, `src/components/domain/SetInspector.vue`, `src/components/domain/SetRow.vue` |
| 4 (PRA-004) | `src-tauri/src/commands/sets.rs`, `src-tauri/src/scanner.rs`, `src/types/index.ts`, `src-tauri/src/domain.rs` |
| 5 (PRA-005) | `src/views/SettingsView.vue` |
| 6 (PRA-006) | `src/components/layout/AppShell.vue` |
| 7 (PRA-007) | `src/components/layout/SidebarNav.vue`, `src/router.ts` |
| 8 (PRA-008) | `src/views/SkillsView.vue` |
| 9 (PRA-009) | `src-tauri/src/commands/assignment.rs`, `src-tauri/src/commands/sets.rs`, `src-tauri/src/tray.rs` |
| 10 (PRA-010) | `src/stores/locationsStore.ts`, `src/stores/assignmentStore.ts` |
| 11 (PRA-011) | `src-tauri/src/commands/external.rs`, `src-tauri/Cargo.toml` |

---

## Task 1: Fix project-scoped set creation (PRA-001)

**Files:**
- Modify: `src/views/SetsView.vue`

The new-set dialog offers a "Project" scope but `handleCreateSet()` always passes `undefined` for `ownerLocationId`. The backend rejects project-scoped creation without it.

**Fix:** When scope is "project", show a location selector dropdown. Disable "Create" until a location is selected.

- [ ] **Step 1: Add location selection state and import to SetsView.vue**

In `src/views/SetsView.vue`, add the locations store import and a new ref for the selected owner location:

```typescript
// Add import at top (after existing imports)
import { useLocationsStore } from "@/stores/locationsStore";

// In script setup, after existing refs
const locationsStore = useLocationsStore();
const newSetOwnerLocationId = ref<string | undefined>(undefined);
```

- [ ] **Step 2: Reset owner location in openNewSetDialog**

Update `openNewSetDialog()` to also reset the new ref:

```typescript
function openNewSetDialog() {
  newSetName.value = "";
  newSetScope.value = "global";
  newSetDescription.value = "";
  newSetOwnerLocationId.value = undefined;
  showNewSetDialog.value = true;
}
```

- [ ] **Step 3: Pass ownerLocationId in handleCreateSet**

Update `handleCreateSet()` to pass the selected location:

```typescript
async function handleCreateSet() {
  if (!newSetName.value.trim()) return;
  if (newSetScope.value === "project" && !newSetOwnerLocationId.value) return;
  isCreating.value = true;
  try {
    await setsStore.createSet(
      newSetName.value.trim(),
      newSetScope.value,
      newSetScope.value === "project" ? newSetOwnerLocationId.value : undefined,
      newSetDescription.value.trim() || undefined
    );
    showNewSetDialog.value = false;
    if (setsStore.selectedSetId) {
      router.push(`/sets/${setsStore.selectedSetId}`);
    }
  } finally {
    isCreating.value = false;
  }
}
```

- [ ] **Step 4: Add location dropdown in the dialog template**

After the scope SegmentedControl form-field, add a conditional location selector:

```html
<div v-if="newSetScope === 'project'" class="form-field">
  <label class="form-label" for="set-owner">Location</label>
  <select
    id="set-owner"
    v-model="newSetOwnerLocationId"
    class="form-input"
  >
    <option :value="undefined" disabled>Select a location...</option>
    <option
      v-for="loc in locationsStore.locationList"
      :key="loc.id"
      :value="loc.id"
    >
      {{ loc.label }}
    </option>
  </select>
</div>
```

- [ ] **Step 5: Update Create button disabled condition**

Change the PrimaryButton `:disabled` to also check for location when project scope:

```html
<PrimaryButton
  label="Create"
  :disabled="!newSetName.trim() || (newSetScope === 'project' && !newSetOwnerLocationId)"
  :loading="isCreating"
  @click="handleCreateSet"
/>
```

- [ ] **Step 6: Fetch locations on mount**

In the `onMounted` callback, also fetch locations:

```typescript
onMounted(() => {
  setsStore.fetchSets();
  locationsStore.fetchList();
});
```

- [ ] **Step 7: Verify**

Run: `npm run build`
Expected: No compile errors.

- [ ] **Step 8: Commit**

```bash
fix: enable project-scoped set creation in UI (PRA-001)
```

---

## Task 2: Fix create_set return type mismatch (PRA-003)

**Files:**
- Modify: `src-tauri/src/commands/sets.rs`

`create_set` returns `SetDetail` but the frontend store invokes it as `SetSummary`. The camelCase serialisation means `skillCount` is missing from the response, so newly created rows show `undefined` skill count.

**Fix:** Change `create_set` to return `SetSummary`.

- [ ] **Step 1: Update create_set return type and body**

In `src-tauri/src/commands/sets.rs`, change the `create_set` function's return type from `Result<SetDetail, AppError>` to `Result<SetSummary, AppError>` and update the return value:

Replace the end of `create_set` (from `let library_skills = ...` to `Ok(SetDetail { ... })`) with:

```rust
    Ok(SetSummary {
        id: set_id,
        name,
        description,
        scope: parsed_scope,
        owner_location_id,
        skill_count: 0,
        assigned_location_count: 0,
        path: set_file.to_string_lossy().to_string(),
    })
```

Also remove the now-unused lines that scan library_skills and library_sets for building the SetDetail.

- [ ] **Step 2: Verify**

Run: `cd src-tauri && cargo check`
Expected: No errors.

- [ ] **Step 3: Commit**

```text
fix: return SetSummary from create_set command (PRA-003)
```

---

## Task 3: Fix set identity collisions (PRA-002)

**Files:**
- Create: `src/utils/setKey.ts`
- Modify: `src/stores/setsStore.ts`
- Modify: `src/router.ts`
- Modify: `src/views/SetsView.vue`
- Modify: `src/views/SetDetailView.vue`
- Modify: `src/components/domain/SetInspector.vue`
- Modify: `src/components/domain/SetRow.vue`

Sets are keyed only by `setId` throughout the frontend, but global and project sets can share the same ID. This causes collisions in cache, selection, routing, and deletion.

**Fix:** Introduce a composite key `setKey` = `"scope:ownerLocationId:setId"` (or `"scope::setId"` for global). Use this as the unique identity everywhere in the frontend.

### Step 3a: Create the composite key utility

- [ ] **Step 1: Create src/utils/setKey.ts**

```typescript
import type { SetScope, LocationId, SetId } from "@/types";

export type SetKey = string;

export function makeSetKey(
  scope: SetScope,
  ownerLocationId: LocationId | null | undefined,
  id: SetId
): SetKey {
  if (scope === "project" && ownerLocationId) {
    return `project:${ownerLocationId}:${id}`;
  }
  return `global::${id}`;
}

export function parseSetKey(key: SetKey): {
  scope: SetScope;
  ownerLocationId: LocationId | undefined;
  id: SetId;
} {
  if (key.startsWith("project:")) {
    const rest = key.slice("project:".length);
    const colonIdx = rest.indexOf(":");
    return {
      scope: "project",
      ownerLocationId: rest.slice(0, colonIdx),
      id: rest.slice(colonIdx + 1),
    };
  }
  // "global::setId"
  return {
    scope: "global",
    ownerLocationId: undefined,
    id: key.slice("global::".length),
  };
}

export function setKeyFromSummary(s: {
  scope: SetScope;
  ownerLocationId?: LocationId | null;
  id: SetId;
}): SetKey {
  return makeSetKey(s.scope, s.ownerLocationId, s.id);
}
```

### Step 3b: Update the store to use composite keys

- [ ] **Step 2: Update setsStore.ts**

Replace `selectedSetId` with `selectedSetKey`, and key `detailCache` by `SetKey`. All lookups and mutations switch to using `setKeyFromSummary`/`makeSetKey`.

The full updated store (replace the entire file):

```typescript
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  SetSummary,
  SetDetail,
  SetId,
  SetScope,
  LocationId,
} from "@/types";
import {
  type SetKey,
  makeSetKey,
  parseSetKey,
  setKeyFromSummary,
} from "@/utils/setKey";

export const useSetsStore = defineStore("sets", () => {
  const items = ref<SetSummary[]>([]);
  const selectedSetKey = ref<SetKey | null>(null);
  const detailCache = ref<Record<SetKey, SetDetail>>({});
  const isLoading = ref(false);
  const searchQuery = ref("");
  const scopeFilter = ref<"all" | "global" | "project">("all");

  const filteredItems = computed(() => {
    let result = items.value;
    if (scopeFilter.value !== "all") {
      result = result.filter((s) => s.scope === scopeFilter.value);
    }
    if (searchQuery.value.trim()) {
      const q = searchQuery.value.trim().toLowerCase();
      result = result.filter(
        (s) =>
          s.name.toLowerCase().includes(q) ||
          (s.description && s.description.toLowerCase().includes(q))
      );
    }
    return result;
  });

  const selectedDetail = computed(() =>
    selectedSetKey.value
      ? detailCache.value[selectedSetKey.value] ?? null
      : null
  );

  async function fetchSets() {
    isLoading.value = true;
    try {
      items.value = await invoke<SetSummary[]>("list_sets");
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchSetDetail(
    id: SetId,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("get_set_detail", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    detailCache.value[key] = detail;
  }

  async function createSet(
    name: string,
    scope: SetScope,
    ownerLocationId?: LocationId,
    description?: string
  ) {
    const summary = await invoke<SetSummary>("create_set", {
      name,
      scope,
      ownerLocationId: ownerLocationId ?? null,
      description: description ?? null,
    });
    items.value.push(summary);
    const key = setKeyFromSummary(summary);
    selectedSetKey.value = key;
    await fetchSetDetail(summary.id, summary.scope, summary.ownerLocationId ?? undefined);
  }

  async function updateSet(
    id: SetId,
    scope: SetScope,
    ownerLocationId: LocationId | undefined,
    updates: { name?: string; description?: string | null }
  ) {
    const detail = await invoke<SetDetail>("update_set", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
      name: updates.name,
      description: updates.description,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      if (updates.name !== undefined) items.value[idx].name = updates.name;
      if (updates.description !== undefined)
        items.value[idx].description = updates.description ?? null;
    }
  }

  async function deleteSet(
    id: SetId,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    await invoke("delete_set", {
      setId: id,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, id);
    items.value = items.value.filter(
      (s) => setKeyFromSummary(s) !== key
    );
    delete detailCache.value[key];
    if (selectedSetKey.value === key) {
      selectedSetKey.value = null;
    }
  }

  async function addSkillToSet(
    setId: SetId,
    skillId: string,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("add_skill_to_set", {
      setId,
      skillId,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, setId);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      items.value[idx].skillCount = detail.skills.length;
    }
  }

  async function removeSkillFromSet(
    setId: SetId,
    skillId: string,
    scope: SetScope,
    ownerLocationId?: LocationId
  ) {
    const detail = await invoke<SetDetail>("remove_skill_from_set", {
      setId,
      skillId,
      scope,
      ownerLocationId: ownerLocationId ?? null,
    });
    const key = makeSetKey(scope, ownerLocationId, setId);
    detailCache.value[key] = detail;
    const idx = items.value.findIndex(
      (s) => setKeyFromSummary(s) === key
    );
    if (idx >= 0) {
      items.value[idx].skillCount = detail.skills.length;
    }
  }

  function selectSet(key: SetKey | null) {
    selectedSetKey.value = key;
    if (key && !detailCache.value[key]) {
      const { scope, ownerLocationId, id } = parseSetKey(key);
      fetchSetDetail(id, scope, ownerLocationId);
    }
  }

  return {
    items,
    selectedSetKey,
    detailCache,
    isLoading,
    searchQuery,
    scopeFilter,
    filteredItems,
    selectedDetail,
    fetchSets,
    fetchSetDetail,
    createSet,
    updateSet,
    deleteSet,
    addSkillToSet,
    removeSkillFromSet,
    selectSet,
  };
});
```

### Step 3c: Update router to use setKey param

- [ ] **Step 3: Update router.ts**

Change the sets route param from `:setId` to `:setKey`:

```typescript
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
```

### Step 3d: Update SetsView.vue

- [ ] **Step 4: Update SetsView.vue to use setKey**

Update `selectSet` to use composite key:

```typescript
import { setKeyFromSummary } from "@/utils/setKey";

function selectSet(set: SetSummary) {
  const key = setKeyFromSummary(set);
  setsStore.selectSet(key);
  router.push(`/sets/${encodeURIComponent(key)}`);
}
```

Update the template references:
- `:key="item.id"` → `:key="setKeyFromSummary(item)"`
- `:selected="item.id === setsStore.selectedSetId"` → `:selected="setKeyFromSummary(item) === setsStore.selectedSetKey"`
- `@click="selectSet(item.id)"` → `@click="selectSet(item)"`
- `v-if="setsStore.selectedSetId"` → `v-if="setsStore.selectedSetKey"`
- For `handleCreateSet`, update the router.push to use the new key:

```typescript
if (setsStore.selectedSetKey) {
  router.push(`/sets/${encodeURIComponent(setsStore.selectedSetKey)}`);
}
```

### Step 3e: Update SetDetailView.vue

- [ ] **Step 5: Update SetDetailView.vue to use setKey**

Change the prop from `setId` to `setKey` and decode it:

```typescript
import { parseSetKey, setKeyFromSummary, type SetKey } from "@/utils/setKey";

const props = defineProps<{ setKey: string }>();

const decodedKey = computed(() => decodeURIComponent(props.setKey));
const detail = computed(() => setsStore.selectedDetail);

function loadDetail() {
  const key = decodedKey.value;
  if (key) {
    setsStore.selectSet(key);
  }
}
```

Update the `watch`:
```typescript
watch(() => props.setKey, loadDetail);
```

### Step 3f: Update SetInspector.vue

- [ ] **Step 6: Update SetInspector.vue delete to use composite key**

The `confirmDelete` already passes `props.detail.id`, `props.detail.scope`, `props.detail.ownerLocationId` — this is fine since `deleteSet` in the store takes these individual values. No changes needed here.

### Step 3g: Update SetRow.vue

- [ ] **Step 7: No changes needed to SetRow.vue**

SetRow receives a `set` prop and renders it — the `:key` is handled in the parent (SetsView).

- [ ] **Step 8: Verify**

Run: `npm run build`
Expected: No compile errors.

- [ ] **Step 9: Commit**

```text
fix: use composite set keys to prevent identity collisions (PRA-002)
```

---

## Task 4: Fix set deletion leaving stale manifests (PRA-004)

**Files:**
- Modify: `src-tauri/src/commands/sets.rs`
- Modify: `src-tauri/src/scanner.rs`
- Modify: `src-tauri/src/domain.rs`
- Modify: `src/types/index.ts`

**Fix:** After deleting the set file, iterate over all locations and remove the set ID from their manifests. Also add a `MissingSet` issue kind so the scanner flags set IDs in manifests that don't resolve to any existing set file.

- [ ] **Step 1: Add MissingSet issue kind to Rust domain**

In `src-tauri/src/domain.rs`, add `MissingSet` to the `IssueKind` enum:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    BrokenLink,
    DeclaredMissing,
    LinkedUndeclared,
    Stale,
    MissingSet,
}
```

- [ ] **Step 2: Add missing_set to TS LocationIssue kind**

In `src/types/index.ts`, update:

```typescript
export type LocationIssue = {
  kind: "broken_link" | "declared_missing" | "linked_undeclared" | "stale" | "missing_set";
  skillName: string;
  skillId: SkillId | null;
  message: string;
};
```

- [ ] **Step 3: Update scanner to detect stale set references**

In `src-tauri/src/scanner.rs`, in the `scan_location` function, after the existing set assignment block (around line 506, after the project sets loop), add stale set detection:

```rust
    // Detect manifest set references that don't resolve to any existing set file
    let all_known_set_ids: Vec<&str> = library_sets
        .iter()
        .map(|(id, _)| id.as_str())
        .chain(project_sets.iter().map(|(id, _)| id.as_str()))
        .collect();

    for manifest_set_id in &manifest_set_ids {
        if !all_known_set_ids.contains(&manifest_set_id.as_str()) {
            issues.push(LocationIssue {
                kind: IssueKind::MissingSet,
                skill_name: manifest_set_id.clone(),
                skill_id: None,
                message: format!(
                    "Set '{}' is declared in the manifest but no matching set file exists",
                    manifest_set_id
                ),
            });
        }
    }
```

- [ ] **Step 4: Update delete_set to clean up manifests**

In `src-tauri/src/commands/sets.rs`, update `delete_set` to remove the set ID from all location manifests after deleting the file. After `std::fs::remove_file(&set_path)?;` and before `drop(guard);`, add:

```rust
    // Remove the set ID from all location manifests
    for loc in &locations {
        let manifest_path = PathBuf::from(&loc.path)
            .join(".claude")
            .join("settings.json");
        if manifest_path.is_file() {
            let manifest_sets = scanner::read_manifest_sets(&manifest_path);
            if manifest_sets.contains(&set_id) {
                // Read, modify, write the manifest
                if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                    if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(obj) = value.as_object_mut() {
                            if let Some(serde_json::Value::Array(sets)) = obj.get_mut("sets") {
                                sets.retain(|v| v.as_str() != Some(&set_id));
                                if let Ok(json) = serde_json::to_string_pretty(&value) {
                                    let _ = std::fs::write(&manifest_path, json);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
```

- [ ] **Step 5: Update delete confirmation text**

In `src/views/SetDetailView.vue:299`, the message already says "and unlink it from all locations" — this is now accurate. No change needed.

- [ ] **Step 6: Verify**

Run: `cd src-tauri && cargo check && npm run build`
Expected: No errors.

- [ ] **Step 7: Commit**

```text
fix: clean up manifests on set deletion and detect stale sets (PRA-004)
```

---

## Task 5: Fix "Copy Pull Command" not copying (PRA-005)

**Files:**
- Modify: `src/views/SettingsView.vue`

`copy_repo_pull_command` returns a string but the frontend ignores it and immediately shows a toast.

**Fix:** Use the returned string to write to clipboard.

- [ ] **Step 1: Update copyPullCommand to use the returned string**

In `src/views/SettingsView.vue`, replace the `copyPullCommand` function:

```typescript
async function copyPullCommand() {
  try {
    const cmd = await invoke<string>("copy_repo_pull_command");
    await navigator.clipboard.writeText(cmd);
    appStore.toast("Pull command copied to clipboard", "success");
  } catch {
    // Fallback: construct the command manually
    const branch = repoStatus.value?.branch ?? "main";
    const cmd = `cd "${preferencesStore.libraryRoot}" && git pull origin ${branch}`;
    try {
      await navigator.clipboard.writeText(cmd);
      appStore.toast("Pull command copied to clipboard", "success");
    } catch (err) {
      appStore.toast(
        err instanceof Error ? err.message : "Failed to copy",
        "error"
      );
    }
  }
}
```

- [ ] **Step 2: Verify**

Run: `npm run build`
Expected: No errors.

- [ ] **Step 3: Commit**

```text
fix: write pull command to clipboard on success (PRA-005)
```

---

## Task 6: Fix repo status banner race condition (PRA-006)

**Files:**
- Modify: `src/components/layout/AppShell.vue`

`AppShell` checks `needsSetup` on mount, but bootstrap hasn't finished yet so `needsSetup` is still `false` (its initial value). The repo status call can fire before preferences are loaded.

**Fix:** Watch `isBootstrapped` and only fetch repo status after bootstrap completes.

- [ ] **Step 1: Update AppShell.vue to use a watcher**

Replace the `onMounted` with a `watch`:

```typescript
import { ref, watch } from "vue";
// ... rest of imports unchanged

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
```

Remove the `onMounted` import and the existing `onMounted` block.

- [ ] **Step 2: Verify**

Run: `npm run build`
Expected: No errors.

- [ ] **Step 3: Commit**

```text
fix: defer repo status check until after bootstrap (PRA-006)
```

---

## Task 7: Hide Usage feature until tracking exists (PRA-007)

**Files:**
- Modify: `src/components/layout/SidebarNav.vue`
- Modify: `src/router.ts`

Usage data is read and displayed but never written. Rather than wire up a complex tracking system, hide the feature.

- [ ] **Step 1: Comment out the Usage nav item**

In `src/components/layout/SidebarNav.vue`, remove or comment out the Usage entry from the nav items array:

```typescript
// Remove this line:
// { label: "Usage", subtitle: "Skill analytics", to: "/usage", icon: "chart" },
```

- [ ] **Step 2: Comment out the Usage route**

In `src/router.ts`, remove or comment out:

```typescript
// Remove this block:
// {
//   path: "/usage",
//   name: "usage",
//   component: () => import("@/views/UsageView.vue"),
// },
```

- [ ] **Step 3: Verify**

Run: `npm run build`
Expected: No errors.

- [ ] **Step 4: Commit**

```text
fix: hide non-functional Usage feature (PRA-007)
```

---

## Task 8: Remove redundant backend refetch on search (PRA-008)

**Files:**
- Modify: `src/views/SkillsView.vue`

The Skills view watches `searchQuery` and `filterArchived` and calls `fetchItems()` on every change. Since the store already has a `filteredItems` computed that filters client-side, the refetches are redundant and expensive.

**Fix:** Remove the watchers. The `filteredItems` computed already handles filtering.

- [ ] **Step 1: Remove the watch imports and watchers**

In `src/views/SkillsView.vue`, remove the `watch` import and both watchers:

Remove `watch` from the import line:

```typescript
import { onMounted } from "vue";
```

Remove both `watch()` blocks (lines 32-44).

- [ ] **Step 2: Verify**

Run: `npm run build`
Expected: No errors.

- [ ] **Step 3: Commit**

```text
fix: remove redundant backend refetch on search/filter (PRA-008)
```

---

## Task 9: Fix clippy lints (PRA-009)

**Files:**
- Modify: `src-tauri/src/commands/assignment.rs`
- Modify: `src-tauri/src/commands/sets.rs`
- Modify: `src-tauri/src/tray.rs`

Fix `ptr_arg` (use `&Path` instead of `&PathBuf`) and `needless_borrows_for_generic_args` warnings.

- [ ] **Step 1: Fix assignment.rs ptr_arg**

In `src-tauri/src/commands/assignment.rs`, change the `update_manifest_skills_and_sets` function signature:

```rust
fn update_manifest_skills_and_sets(
    location_path: &Path,
    // ... rest unchanged
```

Add `use std::path::Path;` at the top if not already imported (it's already there via `PathBuf`; just add `Path` to the existing import).

Update the import line from:
```rust
use std::path::PathBuf;
```
to:
```rust
use std::path::{Path, PathBuf};
```

- [ ] **Step 2: Fix sets.rs ptr_arg**

In `src-tauri/src/commands/sets.rs`, update `resolve_set_path` signature:

```rust
fn resolve_set_path(
    set_id: &str,
    scope: &SetScope,
    owner_location_id: Option<&str>,
    library_root: &Path,
    locations: &[SavedLocation],
) -> Result<PathBuf, AppError> {
```

And `build_assigned_locations`:

```rust
fn build_assigned_locations(
    set_id: &str,
    locations: &[SavedLocation],
    library_root: &Path,
    library_skills: &[SkillMeta],
    library_sets: &[(String, SetDefinition)],
) -> Vec<SavedLocationSummary> {
```

Add `use std::path::Path;` — update the import:

```rust
use std::path::{Path, PathBuf};
```

- [ ] **Step 3: Fix tray.rs unwrap() calls**

In `src-tauri/src/tray.rs`, replace `.unwrap()` calls on state locks with `.ok()` or proper error handling:

Line 62: `let guard = state.lock().unwrap();` → `let guard = match state.lock() { Ok(g) => g, Err(_) => return Ok(menu) };`

Line 127: `let guard2 = state.lock().unwrap();` → `if let Ok(guard2) = state.lock() {`

Line 194: `let mut guard = state.lock().unwrap();` → `let mut guard = match state.lock() { Ok(g) => g, Err(_) => return };`

Line 196-197: `let canonical = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());` — this is already fine (uses `unwrap_or_else`).

- [ ] **Step 4: Run clippy**

Run: `cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings`
Expected: No errors (or only unrelated warnings).

- [ ] **Step 5: Commit**

```bash
fix: resolve clippy warnings for ptr_arg and unwrap (PRA-009)
```

---

## Task 10: Propagate lastSyncedAt after sync/apply (PRA-010)

**Files:**
- Modify: `src/stores/locationsStore.ts`
- Modify: `src/stores/assignmentStore.ts`

After sync/apply operations, the frontend store updates counts but not `lastSyncedAt`, leaving the inspector display stale.

**Fix:** The backend returns `LocationDetail` which doesn't include `lastSyncedAt` (it's only on `SavedLocationSummary`). The simplest fix: update `lastSyncedAt` on the summary with the current timestamp after a successful sync/apply.

- [ ] **Step 1: Update locationsStore.syncLocation**

In `src/stores/locationsStore.ts`, in `syncLocation`, add `lastSyncedAt` to the summary update:

```typescript
async function syncLocation(id: LocationId) {
  const detail = await invoke<LocationDetail>("sync_location", {
    id,
  });
  detailCache[id] = detail;
  const idx = locationList.value.findIndex((l) => l.id === id);
  if (idx >= 0) {
    locationList.value[idx] = {
      ...locationList.value[idx],
      issueCount: detail.issues.length,
      installedSkillCount: detail.skills.length,
      installedSetCount: detail.sets.length,
      lastSyncedAt: new Date().toISOString(),
    };
  }
}
```

- [ ] **Step 2: Update assignmentStore.apply**

In `src/stores/assignmentStore.ts`, in the `apply` function, also set `lastSyncedAt`:

```typescript
if (idx >= 0) {
  locations.locationList[idx] = {
    ...locations.locationList[idx],
    issueCount: detail.issues.length,
    installedSkillCount: detail.skills.length,
    installedSetCount: detail.sets.length,
    lastSyncedAt: new Date().toISOString(),
  };
}
```

- [ ] **Step 3: Verify**

Run: `npm run build`
Expected: No errors.

- [ ] **Step 4: Commit**

```text
fix: propagate lastSyncedAt to location summary after sync/apply (PRA-010)
```

---

## Task 11: Fix editor command parsing for paths with spaces (PRA-011)

**Files:**
- Modify: `src-tauri/src/commands/external.rs`
- Modify: `src-tauri/Cargo.toml`

`split_whitespace()` breaks editor commands like `"/Applications/My Editor.app/Contents/MacOS/editor" --wait`.

**Fix:** Use the `shell-words` crate for POSIX-compatible shell quoting.

- [ ] **Step 1: Add shell-words dependency**

In `src-tauri/Cargo.toml`, add:

```toml
shell-words = "1"
```

- [ ] **Step 2: Update open_path_in_editor**

In `src-tauri/src/commands/external.rs`, replace the parsing logic:

```rust
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tauri::State;

use crate::commands::AppError;
use crate::scanner;
use crate::state::SharedState;

#[tauri::command]
pub fn open_path_in_editor(path: String, editor_command: String) -> Result<(), AppError> {
    let resolved = PathBuf::from(&path);
    if !resolved.exists() {
        return Err(AppError::new(format!("Path does not exist: {}", path)));
    }

    let parts = shell_words::split(&editor_command)
        .map_err(|e| AppError::new(format!("Invalid editor command '{}': {}", editor_command, e)))?;
    if parts.is_empty() {
        return Err(AppError::new("Editor command is empty"));
    }

    let program = &parts[0];
    let mut cmd = Command::new(program);
    for arg in &parts[1..] {
        cmd.arg(arg);
    }
    cmd.arg(&path);

    cmd.spawn()
        .map_err(|e| AppError::new(format!("Failed to open editor '{}': {}", editor_command, e)))?;

    Ok(())
}
```

- [ ] **Step 3: Verify**

Run: `cd src-tauri && cargo check`
Expected: No errors.

- [ ] **Step 4: Commit**

```bash
fix: use shell-words for editor command parsing (PRA-011)
```

---

## Final Verification

- [ ] **Step 1: Run full build and clippy**

```bash
cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings
npm run build
```

- [ ] **Step 2: Run type check**

```bash
npx vue-tsc --noEmit
```

Expected: Both pass with no errors.

- [ ] **Step 3: Final commit if any remaining changes**

```text
chore: final production readiness fixes
```

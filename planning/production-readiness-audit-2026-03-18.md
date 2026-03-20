# Production Readiness Audit

Date: 2026-03-18
Repository: `kit`
Overall risk: blocker

## Executive Summary

The app currently builds and the core Rust/TypeScript code compiles, but it is not production ready yet.

The biggest blockers are:

1. Project-scoped sets cannot be created from the current UI.
2. Sets are not uniquely identified in the frontend, so duplicate set IDs across scopes/projects can corrupt selection, caching, and deletion behaviour.
3. The usage analytics feature is effectively non-functional because usage data is read and displayed but never written anywhere in the codebase.
4. The release pipeline is not clean: `cargo clippy -D warnings` fails, and `npm run tauri build` fails during DMG bundling.

## Verification Run

| Command | Result | Notes |
|---|---|---|
| `npm run build` | pass | Frontend production build succeeded |
| `cd src-tauri && cargo check` | pass | Rust code compiles |
| `cd src-tauri && cargo test` | pass | 0 tests ran |
| `cd src-tauri && cargo clippy --all-targets --all-features -- -D warnings` | fail | 5 warnings promoted to errors |
| `npm run tauri build` | fail | Release binary built, DMG bundling failed |

`npm run tauri build` also warned that the bundle identifier ends with `.app` (`src-tauri/tauri.conf.json:5`), which is not recommended on macOS.

## Issue Register

| ID | Confidence | Severity | File:Line | Finding | Recommendation | Effort |
|---|---|---|---|---|---|---|
| PRA-001 | certain | blocker | `src/views/SetsView.vue:47`, `src-tauri/src/commands/sets.rs:216` | Project-scoped set creation cannot work from the UI because the view always passes `undefined` for `ownerLocationId`, while the backend hard-requires it for project scope. | Add project selection to the dialog or remove the project option until ownership can be supplied. | M |
| PRA-002 | certain | blocker | `src/router.ts:42`, `src/stores/setsStore.ts:14`, `src/stores/setsStore.ts:61`, `src/stores/setsStore.ts:113`, `src/views/SetsView.vue:85` | Set identity is keyed only by `setId` in routing, cache, and deletion logic, but the backend supports both global and per-project sets with overlapping IDs. Duplicate IDs will collide in cache, route selection, and local list updates. | Use a composite key based on `scope + ownerLocationId + setId` everywhere in the frontend and routing layer. | L |
| PRA-003 | certain | major | `src/stores/setsStore.ts:70`, `src-tauri/src/commands/sets.rs:199`, `src/components/domain/SetRow.vue:18` | `create_set` returns `SetDetail`, but the store treats the response as `SetSummary` and pushes it straight into the sidebar list. Newly created rows therefore lack summary fields such as `skillCount`. | Return `SetSummary` from the command, or normalise the returned `SetDetail` into a `SetSummary` before inserting it. | S |
| PRA-004 | certain | major | `src/views/SetDetailView.vue:299`, `src/components/domain/SetInspector.vue:94`, `src-tauri/src/commands/sets.rs:526`, `src-tauri/src/scanner.rs:464` | Delete confirmation promises the set will be unlinked from all locations, but the backend only deletes the set file. Manifest references are left behind, and the scanner does not surface missing set IDs as issues. | Remove deleted set IDs from all affected manifests and add explicit stale/missing-set issue detection. | M |
| PRA-005 | certain | major | `src/views/SettingsView.vue:129`, `src-tauri/src/commands/repo.rs:275` | “Copy Pull Command” does not copy anything on the success path. The frontend ignores the returned string and immediately shows a success toast. | Return the string and always write it to the clipboard in the frontend, or copy to clipboard in Rust and rename the command accordingly. | S |
| PRA-006 | likely | major | `src/stores/appStore.ts:24`, `src/components/layout/AppShell.vue:15`, `src/views/OnboardingView.vue:91` | The repo-status banner check races bootstrap. `needsSetup` starts as `false`, so `AppShell` may call `get_skills_repo_status` before preferences are loaded, swallow the failure, and never retry after onboarding/bootstrap completes. | Trigger repo status after bootstrap settles, or watch the configured repo path and refresh when it becomes available. | S |
| PRA-007 | certain | blocker | `src-tauri/src/state.rs:18`, `src-tauri/src/commands/usage.rs:10` | Usage analytics are read and rendered, but I found no code anywhere under `src-tauri/src` that mutates `guard.inner.usage` or increments `use_count_30d`. The feature currently has no writer path. | Either wire real usage ingestion into the app or hide the Usage feature until tracking exists. | M |
| PRA-008 | certain | major | `src/views/SkillsView.vue:28`, `src/views/SkillsView.vue:32`, `src-tauri/src/commands/library.rs:22` | Search/filter changes in the Skills view trigger a full backend refetch, and `list_library_items` rescans every location for every skill and set. This will scale poorly and cause UI lag as the library grows. | Keep search/filter purely client-side after the initial fetch, or redesign the backend call to compute counts in one pass. | M |
| PRA-009 | certain | major | `src/commands/assignment.rs:369`, `src/commands/sets.rs:36`, `src/commands/sets.rs:111`, `src/tray.rs:192`, `src/tray.rs:229` | The Rust codebase does not pass the repo’s own strict clippy gate. `cargo clippy --all-targets --all-features -- -D warnings` fails on `ptr_arg` and `needless_borrows_for_generic_args`. | Fix the reported lints and make clippy part of pre-release verification. | S |
| PRA-010 | certain | minor | `src-tauri/src/commands/locations.rs:205`, `src/stores/locationsStore.ts:82`, `src/components/domain/LocationInspector.vue:18`, `src/stores/assignmentStore.ts:108` | Sync/apply operations update `last_synced_at` in Rust, but the frontend store updates only counts. The inspector’s “Last synced” display can remain stale after successful operations. | Propagate `lastSyncedAt` back into the cached location summary after sync/apply. | S |
| PRA-011 | certain | minor | `src-tauri/src/commands/external.rs:17` | Editor command parsing uses `split_whitespace()`, so quoted executables/arguments and paths containing spaces will break. | Store editor executable and args separately, or use a shell-aware parser/config format. | S |

## Detailed Notes

### PRA-001: Project-scoped sets are dead from the current UI

The new-set dialog offers a `Project` scope, but `handleCreateSet()` always calls `setsStore.createSet(..., undefined, ...)` in `src/views/SetsView.vue:47-51`.

The backend rejects project-scoped creation unless `owner_location_id` is present in `src-tauri/src/commands/sets.rs:216-224`.

Impact:

- The scope selector exposes a path that cannot succeed.
- Failures happen on a core content-management workflow.

### PRA-002: Set identity is not unique

The backend models sets by:

- `set_id`
- `scope`
- `owner_location_id` for project sets

The frontend collapses that down to `selectedSetId`, `detailCache[setId]`, `/sets/:setId`, and `v-for :key="item.id"`.

Impact:

- Two project sets with the same filename in different locations will collide.
- A project set can collide with a global set of the same ID.
- Deleting one set can remove all rows with that ID from the in-memory list because `items.value = items.value.filter((s) => s.id !== id)` only filters by `id`.

### PRA-004: Set deletion silently leaves stale manifests

The confirm dialogs explicitly promise unlinking, but the delete command only removes the `.set.json` file.

Because the scanner only reconstructs assigned sets from currently existing set definitions, stale manifest references disappear from the UI instead of being flagged. That makes the data loss/inconsistency silent.

### PRA-007: Usage feature is not wired through

The persisted state contains a `usage` map and the Usage page renders it, but there is no evidence of any update path. I searched the repo for mutations and only found type definitions and read paths.

Impact:

- `Usage` page can show empty or misleading data forever.
- `SkillDetail` usage panel is similarly untrustworthy.

### PRA-008: Search path is doing expensive backend work on every edit

The Skills view already has a local `filteredItems` computed in the store, but the view also refetches on search/filter changes. The backend then rescans all locations repeatedly inside nested loops.

Impact:

- Poor responsiveness as data volume increases.
- Unnecessary filesystem churn for what should be an in-memory filter operation.

## Additional Risks / Gaps

- There are currently no Rust tests covering the command layer, scanner behaviour, manifest mutation logic, or set lifecycle.
- `npm run tauri build` produced a release app binary but failed during DMG bundling, so the distribution pipeline is not yet trusted.
- `src-tauri/src/tray.rs` and `src-tauri/src/state.rs` still contain `unwrap()` / `expect()` calls on app-critical paths, which are avoidable crash surfaces for production.

## Recommended Fix Order

1. Fix set identity and project-set creation (`PRA-001`, `PRA-002`, `PRA-003`).
2. Fix set deletion consistency and manifest cleanup (`PRA-004`).
3. Fix repo command UX bug and bootstrap timing (`PRA-005`, `PRA-006`).
4. Decide whether Usage ships now or is hidden until tracking exists (`PRA-007`).
5. Remove performance regressions and clean release gates (`PRA-008`, `PRA-009`).
6. Tidy smaller correctness issues (`PRA-010`, `PRA-011`).

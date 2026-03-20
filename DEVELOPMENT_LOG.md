# Kit Development Log

## Cycle: 2026-03-20 23:59
- App: Kit
- Items completed:
  - [Quality] Health check dashboard for all locations (P2/M) — new HealthView with summary cards (healthy/warning/error counts), severity-grouped issue list, fix-broken-links action, location filter via query param. Backend: `run_health_check` scanner function aggregates issues from all locations, `fix_broken_links` command removes broken symlinks.
  - [Distribution] Skill library sharing via export/import (P3/M) — `export_skill_bundle` creates .kit-bundle.zip with SKILL.md files and kit-bundle.json manifest, `preview_import_bundle` shows conflicts, `import_skill_bundle` extracts skills with overwrite option. Uses zip crate (deflate compression).
  - [Innovation] Project-type detection with skill recommendations (P3/M) — `detect_project_types` checks 20+ framework markers (package.json, Cargo.toml, etc.), `recommend_skills` matches skill names/descriptions against detected types. Badges shown on location detail, recommendations listed above skill list.
  - [UX/UI] Inline skill content preview in library view (P2/S) — expand-on-click preview panel in SkillsView sidebar shows raw SKILL.md content via `read_skill_content` command. Truncated at 4000 chars. Toggle via eye icon.
  - [Performance] Filesystem watcher for live skill library updates (P2/M) — `notify` crate with `notify-debouncer-mini` (2s debounce), watches library root recursively for SKILL.md changes, emits `library-changed` Tauri event. Watcher auto-starts on bootstrap. Store listens for events and refreshes library.
  - [UX/UI] Skill usage statistics visible on skill cards (P3/S) — `LibraryListItem` gains `useCount30d`, `lastUsedAt`, `isUnusedEverywhere` fields populated from state.json usage map. Badge shown on skill rows in library sidebar. Sort-by-usage segmented control added.
  - [UX/UI] Drag-and-drop skill reordering within sets (P3/S) — HTML5 drag-and-drop on skill rows in SetDetailView with drag handle, visual feedback (opacity + border), and Alt+Arrow keyboard alternative. Persists new order via `update_set` command.
  - [Feature] Quick-assign action for skills from library view (P2/S) — "+" button on skill rows in SkillsView sidebar when an active location is selected. Single click invokes `apply_assignment` with the skill and shows success toast. Already-assigned skills show "assigned" badge.
  - [UX/UI] Location dashboard showing health status (P2/S) — compact dashboard header above location detail content with skill count, issue count, health badge (healthy/warning/error), last scan time, and detected project type badges. Health badge click navigates to HealthView filtered to that location.
  - [Quality] Unused skill detection across all locations (P2/S) — `isUnusedEverywhere` field computed from `linkedLocationCount === 0`. Library store gains `filterUnused` toggle and `unusedCount` computed. Filter shown as checkbox with count badge.
  - [Feature] Skill version tracking with update notifications (P3/S) — `skill_hashes` map in state.json records DJB2 hash of SKILL.md at assignment time. `get_skill_versions` command compares current vs recorded hash. `SkillVersionInfo` type added. `trackSkillVersions` preference (default true).
- Items attempted but failed: none
- Branch: main
- Tests passing: yes (cargo test 21/21, cargo clippy clean, vue-tsc clean)
- Build status: compiles (cargo check, vue-tsc --noEmit)
- Notes: Eleven items implemented in a single batch. New Rust dependencies: zip (deflate), notify + notify-debouncer-mini (filesystem watching). New command modules: health.rs, sharing.rs, watcher.rs. New frontend stores: healthStore.ts, watcherStore.ts. New route: /health. New view: HealthView.vue. Significant changes to domain.rs (8 new types), scanner.rs (4 new functions), lib.rs (9 new commands registered), types/index.ts (9 new types), SkillsView.vue (rewritten with usage/preview/quick-assign), LocationDetailView.vue (dashboard header + recommendations), SetDetailView.vue (drag-and-drop reordering).

## Cycle: 2026-03-20 23:30
- App: Kit
- Items completed:
  - [UX/UI] Add skill search and filtering in library view (P2/S) — enhanced existing search to match against frontmatter tags in addition to name/description; added clear button to SearchField component; improved empty state with contextual message and "Clear search" link; tags displayed as compact badges on skill rows
  - [Quality] Add SKILL.md frontmatter validation with actionable error reporting (P2/S) — validate_skill_md() produces structured ValidationIssue objects with severity (error/warning), field name, message, and fix suggestion; checks for missing frontmatter delimiters, unclosed blocks, missing name, and missing description; skills with broken frontmatter now included in library list (not silently dropped) with error/warning badges in sidebar
  - [Quality] Add set integrity validation ensuring all referenced skills exist (P2/S) — set items in library view show broken skill count badge when referenced skills are missing from library; set detail view marks missing skills with "missing" badge and strikethrough styling; SetSkillEntry gains `missing: bool` field
- Items attempted but failed: none
- Branch: feature/search-validation-set-integrity
- Tests passing: yes (cargo test 29/29, cargo clippy clean, vue-tsc clean)
- Build status: success (Kit.app + DMG bundled)
- Notes: All three items are P2/S and share backend types (ValidationIssue, tags on SkillMeta/LibraryListItem). Tags parsing handles comma-separated values on the `tags:` frontmatter line. 8 new Rust tests added covering tag parsing and validation scenarios. SearchField clear button is available to all consumers, not just SkillsView.

## Cycle: 2026-03-19 14:00
- App: Kit
- Items completed:
  - [Quality] Fix build pipeline failures (Clippy warnings and DMG bundling) — `cargo clippy -D warnings` passes clean, `npm run tauri build` produces valid Kit.app and Kit_1.0.0_aarch64.dmg
  - [Quality] Ensure project-scoped sets can be created from the UI — set creation dialog now includes a location/project selector dropdown when scope is "project", with validation preventing creation without selecting a location
- Items attempted but failed: none
- Branch: feature/build-fixes-and-project-scoped-sets
- Tests passing: yes (21/21 Rust tests)
- Build status: success (Kit.app + DMG bundled)
- Notes: Both items were substantially implemented in prior uncommitted work on main. This cycle verified all changes, ran full test suite and production build, and confirmed acceptance criteria are met. The build artefact was copied to ~/Desktop/TauriBuilds/kit/. Additional changes in the branch include: atomic file writes via `state::atomic_write()`, manifest cleanup on set deletion, SetKey-based routing for disambiguating project-scoped vs global sets, new SkillsRepoValidation/SkillsRepoStatus domain types, and various icon regenerations.

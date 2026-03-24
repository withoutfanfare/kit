# Kit Development Log

## Cycle: 2026-03-24 22:00
- App: Kit
- Items completed:
  - [UX/UI] Add skill activation toggle to temporarily disable skills without unlinking (P2/S) — `toggle_skill_activation` command flips disabled state in state.json `disabled_skills` HashSet (keyed by "locationId:skillId"), removes/adds skill from manifest. New `disabled: bool` field on `SkillAssignment`. Frontend: toggle button on linked skill rows, disabled state shown with reduced opacity and "disabled" badge. SkillRow gains action buttons for toggle and diff.
  - [Quality] Add skill body content validation for SKILL.md size and structure (P2/S) — `validate_skill_body()` in scanner.rs checks total file size (warning >30KB, error >50KB), empty body after frontmatter, and very short content (<50 chars). `get_skill_body_validation` command exposes via IPC. Results shown in SkillPeekPanel "Content quality" section with severity badges.
  - [UX/UI] Add skill content diff viewer comparing current version against assignment-time snapshot (P2/S) — Content snapshots stored in state.json `skill_snapshots` HashMap during `apply_assignment`. `get_skill_content_diff` command returns assigned vs current content. SkillDiffModal component with side-by-side columns, triggered from "view changes" button on skill rows. Handles missing snapshots gracefully.
- Items attempted but failed: none
- Branch: feature/activation-toggle-validation-diff
- Tests passing: yes (cargo clippy clean, vue-tsc clean)
- Build status: compiles (cargo check, vue-tsc --noEmit)
- Notes: Three P2/S quality items implemented. New files: commands/activation.rs (3 commands + manifest helpers), components/domain/SkillDiffModal.vue. Modified: state.rs (disabled_skills HashSet, skill_snapshots HashMap), domain.rs (disabled field, SkillContentDiff type), scanner.rs (validate_skill_body, extract_body), assignment.rs (snapshot recording, disabled enrichment), locations.rs (disabled enrichment), lib.rs (3 new commands), types/index.ts (disabled field, SkillContentDiff type), locationsStore.ts (toggleSkillActivation), SkillRow.vue (action buttons), SkillList.vue (event passthrough), SkillPeekPanel.vue (body validation section), LocationDetailView.vue (toggle handler, diff modal).

## Cycle: 2026-03-22
- App: Kit
- Items completed:
  - [UX/UI] Add keyboard shortcuts for library and location navigation (P2/S) — `useKeyboardShortcuts` composable with Cmd+1-5 view switching (Locations, Skills, Sets, Changelog, Health), j/k list navigation, Enter for detail, / for search focus, Cmd+/ shortcut help overlay. Input suppression for focused text fields. ShortcutHelpOverlay modal with grouped shortcuts.
  - [UX/UI] Add skill changelog showing recent modifications across the library (P2/S) — Rust `get_skill_changelog` command scans library skills by filesystem modification time, accepts optional `days` filter, returns `Vec<ChangelogEntry>` sorted by most recent. Frontend: changelogStore with day filter + search, ChangelogView with date range segmented control (All/7/30/90 days), time-ago display, size formatting, click-to-navigate. New /changelog route and sidebar nav link.
  - [Feature] Add bulk skill assignment to multiple locations in one operation (P2/S) — Rust `bulk_assign_skills` command iterates location IDs, creates symlinks, updates manifests, records version hashes per location. Returns per-location success/error results. Frontend: bulkAssignStore with location multi-select, select-all/deselect-all, BulkAssignModal with checkbox list showing location label/path/skill count and result badges. "Assign to locations" button on SkillDetailView header.
- Items attempted but failed: none
- Branch: main
- Tests passing: yes (cargo test 29/29, cargo clippy clean, vue-tsc clean)
- Build status: compiles (cargo check, vue-tsc --noEmit)
- Notes: Three P2/S items implemented. New files: composables/useKeyboardShortcuts.ts, components/domain/ShortcutHelpOverlay.vue, components/domain/BulkAssignModal.vue, stores/changelogStore.ts, stores/bulkAssignStore.ts, views/ChangelogView.vue, commands/changelog.rs. Modified: domain.rs (ChangelogEntry + BulkAssignResult types), assignment.rs (bulk_assign_skills command), lib.rs (2 new commands registered), types/index.ts (2 new types), router.ts (/changelog route), SidebarNav.vue (Changelog nav item), AppShell.vue (BulkAssignModal + ShortcutHelpOverlay), SkillDetailView.vue (bulk assign button).

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

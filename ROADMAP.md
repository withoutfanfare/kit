# Kit Roadmap

Desktop skill loadout manager for Claude Code — organise, assign, and manage skills across projects.

## Completed

### [Quality] Fix build pipeline failures (Clippy warnings and DMG bundling)
- **Priority:** P1 (critical)
- **Size:** S (< 1hr)
- **Added:** 2026-03-19
- **Status:** completed
- **Completed:** 2026-03-19
- **Description:** The production readiness audit (PRA-009) identified that both `cargo clippy -D warnings` and `npm run tauri build` (DMG bundling) fail. A broken build pipeline blocks all distribution and makes it impossible to ship updates. This must be resolved before any other work can be delivered to users.
- **Acceptance criteria:**
  - `cargo clippy -D warnings` passes with zero warnings
  - `npm run tauri build` completes successfully and produces a valid DMG
  - Build tested on a clean checkout (no local-only dependencies)
  - CI-equivalent build steps documented in README or CONTRIBUTING

### [Quality] Ensure project-scoped sets can be created from the UI
- **Priority:** P1 (critical)
- **Size:** S (< 1hr)
- **Added:** 2026-03-19
- **Status:** completed
- **Completed:** 2026-03-19
- **Description:** The audit (PRA-001) found that project-scoped sets cannot be created from the UI because the set creation dialog is missing a location/project selector. Sets are a core feature — grouping skills into reusable loadouts — and being unable to scope them to a project defeats the purpose for users managing multiple projects with different skill needs.
- **Acceptance criteria:**
  - Set creation dialog includes a location/project selector dropdown
  - User can create sets scoped to a specific project or globally
  - Existing global set creation flow unchanged
  - Set scope displayed in the sets list view
  - Duplicate set ID validation works correctly across scopes (PRA-002)

## Pending

### [Feature] Add skill dependency resolution and conflict detection
- **Priority:** P3 (nice-to-have)
- **Size:** L (3-8hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Tag:** Needs review
- **Description:** When assigning skills to a project, there's no visibility into whether skills conflict with each other (e.g. competing hook configurations) or depend on other skills. Surfacing dependency information from skill manifests and warning about potential conflicts during assignment would prevent subtle configuration issues that are hard to debug.
- **Acceptance criteria:**
  - Skills with declared dependencies show dependency badges in the library view
  - Assignment preview warns if a required dependency is not in the target loadout
  - Conflicts between skills (e.g. same hook event, competing settings) highlighted during preview
  - Warnings are advisory only — user can override and proceed
  - Dependency/conflict data sourced from SKILL.md frontmatter

### [UX/UI] Add skill search and filtering in library view
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-19
- **Status:** completed
- **Completed:** 2026-03-20
- **Description:** As the skill library grows beyond a handful of entries, finding a specific skill by scrolling becomes inefficient. A search/filter bar in the library view that matches against skill name, description, and tags would let users locate skills quickly, especially when assigning skills to a new project and browsing unfamiliar libraries.
- **Acceptance criteria:**
  - Search input at the top of the library view with instant filtering
  - Matches against skill name, description text, and frontmatter tags
  - Results update as the user types (debounced, no submit required)
  - Empty state shown when no skills match the filter
  - Clear button to reset the filter; filter state preserved during session

### [Quality] Add health check dashboard for all locations
- **Priority:** P2 (important)
- **Size:** M (1-3hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Users managing skills across multiple projects have no single view to spot issues. Broken symlinks, outdated manifests, skill conflicts, and missing dependencies can silently degrade Claude Code sessions. A health check dashboard that scans all registered locations and surfaces issues at a glance would help users maintain their skill configurations proactively.
- **Acceptance criteria:**
  - Dashboard view accessible from the main navigation
  - Scans all registered locations for: broken symlinks, manifest/filesystem mismatches, duplicate skill IDs
  - Issues grouped by location with severity indicators (error, warning, info)
  - Each issue includes a description and suggested fix action
  - "Fix all" button for safe auto-fixable issues (e.g. remove broken symlinks)
  - Scan completes within 3 seconds for 20 locations with 50 skills each

### [Distribution] Add skill library sharing via export/import
- **Priority:** P3 (nice-to-have)
- **Size:** M (1-3hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Teams using Claude Code with shared skill conventions currently have no structured way to distribute curated skill sets. Exporting a skill set (with all referenced skills and their SKILL.md manifests) as a portable bundle and importing it on another machine would enable team-wide skill standardisation without manual file copying.
- **Acceptance criteria:**
  - Export a set (or selection of skills) as a .zip bundle containing SKILL.md files and set definition
  - Import a bundle into the library root, with conflict detection for existing skills
  - Preview dialog before import showing what will be added/overwritten
  - Imported skills retain their original metadata and directory structure
  - Bundle format documented for manual creation by advanced users

### [Innovation] Add project-type detection with skill recommendations
- **Priority:** P3 (nice-to-have)
- **Size:** M (1-3hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** When users register a new project location, Kit has no awareness of what kind of project it is — a Laravel app, a Tauri app, a Python ML project — and therefore cannot suggest relevant skills. Scanning the project directory for framework markers (composer.json, Cargo.toml, package.json scripts, pyproject.toml) and recommending skills from the library that match the detected stack would reduce the skill assignment friction from "browse and guess" to "review and confirm".
- **Acceptance criteria:**
  - Location scan detects project type from common framework files (Laravel, Tauri, Node.js, Python, Go, etc.)
  - Detected project type displayed as a badge on the location card
  - Skill recommendations surfaced when viewing a location with no or few assigned skills
  - Recommendations based on skill frontmatter tags matching detected frameworks
  - Recommendations are advisory only — user always confirms before assignment

### [UX/UI] Add inline skill content preview in library view
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** Users browsing the skill library can see skill names and descriptions from frontmatter, but cannot read the full SKILL.md content without navigating to the skill detail view. An inline preview (expand-on-click or hover panel) showing the SKILL.md content with markdown rendering would let users evaluate skills quickly during browsing and assignment workflows, especially when exploring unfamiliar skills.
- **Acceptance criteria:**
  - Expand-on-click or side panel preview showing rendered SKILL.md content
  - Preview loads content from the filesystem on demand (no eager loading)
  - Markdown rendered with syntax highlighting for code blocks
  - Preview dismissible via Escape or clicking outside
  - Preview available in both the library view and the assignment preview dialog
  - Content truncated with "Show more" for very long skill files (> 200 lines)

### [Performance] Add filesystem watcher for live skill library updates
- **Priority:** P2 (important)
- **Size:** M (1-3hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Kit scans the skill library on launch and on manual refresh, but changes made to skills on disk (editing SKILL.md files, adding new skills, removing directories) are invisible until the user triggers a rescan. A filesystem watcher on the library root that detects changes and incrementally updates the library view would keep the UI in sync with reality, which is particularly important when users are actively developing skills in their editor alongside Kit.
- **Acceptance criteria:**
  - Filesystem watcher active on the library root directory (recursive, watching for SKILL.md changes)
  - New skills detected and added to the library view within 2 seconds
  - Removed skill directories reflected in the library view (with broken-link detection for assigned skills)
  - Modified SKILL.md frontmatter changes reflected in the skill card (name, description, tags)
  - Watcher debounced to handle rapid file saves (e.g. editor auto-save)
  - Watcher status indicator in the UI (active/paused/error)

### [Quality] Add SKILL.md frontmatter validation with actionable error reporting
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-19
- **Status:** completed
- **Completed:** 2026-03-20
- **Description:** Kit's scanner hand-parses YAML frontmatter from SKILL.md files but performs no validation beyond extraction. Missing required fields (name, description), invalid field values, or malformed YAML silently result in skills appearing with blank metadata or being skipped entirely. Validating frontmatter during scan and surfacing clear, fixable error messages would help skill authors maintain correct manifests and catch issues before they cause confusion in downstream assignment workflows.
- **Acceptance criteria:**
  - Frontmatter validated for: required fields (name, description), valid YAML syntax, field type correctness
  - Validation errors surfaced per-skill in the library view with error badge
  - Error detail shows: field name, expected format, actual value, and fix suggestion
  - Skills with validation errors still displayed in the library (not silently dropped)
  - Validation results included in the health check dashboard
  - Quick-fix link to open SKILL.md in the configured editor

### [UX/UI] Add skill usage statistics visible on skill cards
- **Priority:** P3 (nice-to-have)
- **Size:** S (< 1hr)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Kit tracks skill usage counters in state.json (per-skill invocation counts stored via the usage tracking system) but this data is not surfaced in the UI. Displaying usage frequency and last-used date on skill cards in the library view would help users identify their most valuable skills, spot unused skills that could be cleaned up, and make more informed assignment decisions. This turns invisible backend data into actionable user insight.
- **Acceptance criteria:**
  - Skill cards show usage count badge and "last used" relative timestamp
  - Usage data sourced from existing state.json usage tracking
  - Skills with zero usage show a subtle "unused" indicator
  - Sortable by usage count in the library view (most used / least used)
  - Usage stats visible in both the library view and the assignment preview dialog
  - No additional tracking overhead — reads existing persisted counters only

### [Quality] Add set integrity validation ensuring all referenced skills exist
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** completed
- **Completed:** 2026-03-20
- **Description:** Set definition files (*.set.json) reference skills by ID, but there is no validation that all referenced skills actually exist in the library. If a skill is renamed, moved, or deleted, any sets referencing it will silently contain broken references — the assignment preview may show incomplete loadouts without explanation. Validating set integrity on load and surfacing broken references with clear fix suggestions would prevent silent skill gaps in deployed loadouts.
- **Acceptance criteria:**
  - Set files validated on load: all referenced skill IDs checked against the library index
  - Broken references surfaced per-set with the missing skill ID and suggested action (remove from set, re-link)
  - Set card shows a warning badge when integrity issues are detected
  - Validation results included in the health check dashboard
  - Quick-fix action to remove broken references from a set
  - Validation runs on library rescan (manual or via filesystem watcher)

### [UX/UI] Add drag-and-drop skill reordering within sets
- **Priority:** P3 (nice-to-have)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** Skills within a set are listed in the order they appear in the *.set.json file, but there is no way to reorder them from the UI. When sets represent a prioritised loadout (e.g. core skills first, optional skills last), the order matters for readability and intent. Drag-and-drop reordering would let users organise their sets semantically without manually editing JSON files.
- **Acceptance criteria:**
  - Drag handle visible on each skill row within the set detail view
  - Drag-and-drop reorders skills with smooth animation feedback
  - New order persisted to the *.set.json file on drop
  - Order preserved across app restarts and library rescans
  - Keyboard alternative: move selected skill up/down with Alt+Arrow
  - Reorder action logged in the set's modification timestamp

### [Feature] Add quick-assign action for skills from library view
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** Assigning a skill to a project currently requires navigating through the full assignment workflow — selecting a location, previewing changes, and confirming. For the common case of adding a single skill to the currently active location, this multi-step process is unnecessarily heavy. A quick-assign action on each skill card in the library view would reduce the most common assignment workflow to a single click, with the full preview still available for complex multi-skill assignments.
- **Acceptance criteria:**
  - "Assign to [location name]" quick action visible on skill cards when an active location is selected
  - Single click creates the symlink and updates the manifest without a separate preview dialog
  - Success confirmation toast showing the skill name and target location
  - Action disabled (with tooltip explanation) if the skill is already assigned to the active location
  - Quick-assign respects the same validation rules as the full assignment workflow (duplicate detection, path safety)
  - Full assignment workflow remains available for batch operations and cross-location assignments

### [UX/UI] Add location dashboard showing assigned skill count, health status, and last scan time
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** When selecting a location in Kit's master-detail layout, the detail panel shows the list of assigned skills but lacks a summary overview. Users managing many locations need to quickly assess a location's state — how many skills are assigned, whether there are broken links or manifest issues, and when the location was last scanned. A compact dashboard header above the skill list showing these key metrics would provide instant situational awareness without navigating to the separate health check view.
- **Acceptance criteria:**
  - Location detail panel shows a summary header with: assigned skill count, broken link count, last scan timestamp
  - Health status badge (healthy/warning/error) based on scan results
  - Click on health badge navigates to the health check dashboard filtered to that location
  - Summary data sourced from existing scanner results (no additional filesystem operations)
  - Summary updates automatically when skills are assigned, removed, or the location is rescanned

### [Quality] Add unused skill detection across all locations
- **Priority:** P2 (important)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** As the skill library grows, some skills may become orphaned — present in the library but never assigned to any location. These unused skills clutter the library view and may indicate outdated or superseded capabilities that should be archived or removed. Detecting skills that are not symlinked from any registered location and surfacing them in a "Not assigned anywhere" filter or badge would help users maintain a clean, intentional skill library.
- **Acceptance criteria:**
  - Library view supports an "Unused" filter showing skills not assigned to any registered location
  - Unused skill count displayed as a badge in the filter bar
  - Unused detection considers all registered locations (not just the active one)
  - Detection runs as part of the existing library scan (no separate scan required)
  - Skills recently added (< 24 hours) excluded from the unused indicator to avoid false positives during setup
  - Results included in the health check dashboard under a "Library hygiene" section

### [Feature] Add skill version tracking with update notifications when library skills change
- **Priority:** P3 (nice-to-have)
- **Size:** S (< 1hr)
- **Added:** 2026-03-20
- **Status:** pending
- **Description:** When a skill's SKILL.md is modified in the library (updated description, changed behaviour, new version), locations that have that skill assigned are unaware of the change. The symlink points to the updated file, so behaviour changes silently — which may be desirable but can also introduce unexpected changes. Tracking a content hash of each skill at assignment time and comparing it on subsequent scans would let Kit notify users when an assigned skill has been updated since it was linked, helping them review changes deliberately rather than being surprised by altered behaviour.
- **Acceptance criteria:**
  - SKILL.md content hash recorded in state.json when a skill is assigned to a location
  - Subsequent scans compare current hash against recorded hash
  - "Updated since assignment" badge shown on skill cards where hashes differ
  - Badge click shows a summary of what changed (date assigned vs current modification date)
  - Notification toast when scanning reveals updated skills (with count)
  - Hash tracking opt-in via settings (default: enabled) to avoid noise for users who always want latest

## Design System Adoption

These items implement the Scooda design system (derived from the Dalil app styleguide) to achieve premium visual uniformity across all Tauri applications. Items are ordered by dependency — foundation must complete before migration, migration before polish.

### [Foundation] Integrate @stuntrocket/ui shared component library and design tokens
- **Priority:** P1 (critical)
- **Size:** M (1-3hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Kit uses Vue 3 + Tailwind with design tokens in assets/tokens.css supporting light/dark via prefers-color-scheme. Adopting the Scooda design system requires installing @stuntrocket/ui from the local Verdaccio registry, replacing the current token system with Scooda shared tokens, and switching from prefers-color-scheme to class-based dark mode (.dark on html). Kit's macOS-native aesthetic (system-ui font, small text) needs to shift to Poppins with the Scooda type scale.
- **Acceptance criteria:**
  - .npmrc configured with @stuntrocket:registry=http://localhost:4873
  - @stuntrocket/ui installed as a dependency
  - assets/tokens.css replaced with Scooda tokens.css import
  - Dark mode implementation changed from prefers-color-scheme to class-based (.dark)
  - Dark mode toggle and localStorage persistence added
  - Poppins font loaded as primary sans font (replacing system-ui)
  - Colour palette, spacing, and typography aligned to Scooda values

### [UI Migration] Replace bespoke components with @stuntrocket/ui shared components
- **Priority:** P1 (critical)
- **Size:** XL (8hrs+)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** Replace all locally-defined UI components in src/components/base/ with @stuntrocket/ui equivalents. The master-detail layout pattern, skill library view, location panels, set management views, and assignment workflow all need to be rebuilt with shared components. Kit's nested route structure should be preserved while the visual layer is replaced.
- **Acceptance criteria:**
  - All src/components/base/ generic UI primitives replaced with @stuntrocket/ui imports
  - Layout shell (toolbar, panels) rebuilt with @stuntrocket/ui page layout pattern
  - Skill cards use @stuntrocket/ui Card variants with correct surface/shadow
  - Location and set list views use @stuntrocket/ui list card pattern
  - Assignment preview dialog uses @stuntrocket/ui Modal pattern
  - All form controls (settings, search) use @stuntrocket/ui form components
  - Toast/notification system uses @stuntrocket/ui Toast
  - No locally-defined UI primitive components remain

### [Polish] Achieve full Scooda styleguide visual conformance
- **Priority:** P2 (important)
- **Size:** L (3-8hrs)
- **Added:** 2026-03-19
- **Status:** pending
- **Description:** After component migration, apply the remaining Scooda specifications: ambient background blobs, custom scrollbars, micro-animations, macOS titlebar integration, z-index layering, and accessibility compliance. Kit's small-text-size aesthetic needs particular attention during the typography migration to ensure readability at Scooda's slightly larger base size.
- **Acceptance criteria:**
  - Ambient background blobs with Scooda colours and drift animations
  - Custom scrollbars with accent-tinted thumb
  - Micro-animations on all interactive elements per Scooda timing scale
  - macOS titlebar with drag region and 78px traffic light padding
  - Z-index layering matches Scooda scale
  - prefers-reduced-motion respected
  - Focus rings on all interactive elements
  - Typography visually balanced at Scooda's 15px base (verify density modes still work)
  - Visual side-by-side comparison with Dalil passes review

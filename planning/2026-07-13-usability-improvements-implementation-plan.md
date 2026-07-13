# Kit Usability Improvements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Kit understandable and usable at its supported window sizes by adapting list/detail layouts, clarifying state and actions, and turning search, recommendations and Health into complete decision flows.

**Architecture:** Keep the existing Vue 3, Pinia and Tauri structure. Put responsive behaviour in the shared shell and split-pane components; keep view-specific state in the existing Pinia stores; extend the Rust/TypeScript IPC types only where the UI needs data it cannot derive safely. Reuse `@stuntrocket/ui` controls and existing assignment, confirmation and toast flows. Add no dependencies and do not redesign the visual system.

**Tech Stack:** Vue 3 Composition API, TypeScript, Pinia, Vue Router, Tauri 2, Rust, existing `@stuntrocket/ui` components and CSS design tokens.

**Source review:** `planning/usability-review-2026-07-13.md`

## Global constraints

- Preserve the existing minimum window size of 800 × 500 in `src-tauri/tauri.conf.json`; responsive behaviour, not a larger minimum, must solve the 900 × 700 layout failure.
- Use British English in all user-facing copy.
- Keep Rust types in `src-tauri/src/domain.rs` and TypeScript types in `src/types/index.ts` synchronised in the same task.
- Do not add a frontend test framework as part of this work. Each frontend slice must pass `npm run build`; backend slices must also pass `cargo test` and `cargo clippy` from `src-tauri/`.
- Use `SButton`, `SBadge`, `SSearchInput`, `SSegmentedControl`, `SModal`, `SConfirmDialog` and `SRowActionMenu` where they already fit. Do not create duplicate primitives.
- Preserve the current routes for bookmarks and internal navigation. Labels and entry points may change without renaming `/skills`, `/sets`, `/compare` or `/changelog`.
- Keep the existing dark visual language and tokens in `src/assets/tokens.css`. Only add component-level responsive and interaction styles required by the tasks below.
- Do not touch the user-owned README or app-icon changes already present in the worktree.

---

## Phase 1 — Responsive foundation and in-place clarity

### Task 1: Make the shared shell and list/detail layout adaptive

**Files:**

- Modify: `src/components/layout/SplitPaneLayout.vue`
- Modify: `src/components/layout/SidebarNav.vue`
- Modify: `src/assets/global.css`
- Modify: `src/views/LocationsView.vue`
- Modify: `src/views/SkillsView.vue`
- Modify: `src/views/SetsView.vue`

**Interface:**

Extend `SplitPaneLayout` with this public contract:

```ts
type CompactPane = "list" | "main" | "detail";

defineProps<{
  showInspector?: boolean;
  compactPane?: CompactPane;
  backLabel?: string;
}>();

defineEmits<{
  back: [];
}>();
```

At widths above 960px, retain the current list, main and optional inspector arrangement. At 960px and below:

- `compactPane="list"` renders only the list column.
- `compactPane="main"` renders only the main slot without a Back row; use this for a first-run or empty workspace.
- `compactPane="detail"` renders the main slot, a visible `Back to …` action, and no list column.
- The inspector becomes a dismissible overlay opened by a `Details` button, rather than consuming permanent width.

At 1120px and below, collapse the primary sidebar to 64px by changing `--sidebar-width` and hiding `.nav-text`, `.nav-subtitle` and `.footer-label`; retain accessible names on every link.

**Steps:**

- [x] Add the `compactPane`, `backLabel` and `back` interface to `SplitPaneLayout.vue`.
- [x] Add an internal `inspectorOpen` ref, a `Details` toggle and a close action; reset it when `showInspector` becomes false.
- [x] Add the 960px media rules for list/main/detail switching and the inspector overlay. Do not use JavaScript window-width listeners.
- [x] Add the 1120px sidebar-collapse rules in `global.css` and component styles in `SidebarNav.vue`.
- [x] Add `aria-label` values to collapsed navigation links so icon-only mode remains understandable.
- [x] In `LocationsView.vue`, compute `compactPane` as `detail` when `route.params.locationId` exists, `list` when locations exist, otherwise `main`; handle `@back` with `router.push('/locations')` and clear the selected location.
- [x] Apply the same rule in `SkillsView.vue` and `SetsView.vue`, using `/skills` and `/sets` as their Back targets.
- [x] Confirm the primary sidebar, secondary list and detail never compete for width at 900 × 700.

**Verification:**

- [x] Run `npm run build` from the repository root. Expected: `vue-tsc --noEmit && vite build` exits 0.
- [ ] Run `npm run tauri dev`, resize to 1430 × 971 and 900 × 700, and verify all three list/detail views retain selection, Back navigation and inspector access.
- [ ] Keyboard-check the collapsed sidebar, Back action, Details toggle and inspector close action.

**Commit:** `feat: make list detail layouts adaptive`

### Task 2: Replace ambiguous dots, counts and icon-only controls with explicit meaning

**Files:**

- Create: `src/utils/statusLabels.ts`
- Create: `src/components/domain/SkillStatusLegend.vue`
- Modify: `src/components/domain/LocationRow.vue`
- Modify: `src/components/domain/LocationOverviewCard.vue`
- Modify: `src/components/domain/SkillRow.vue`
- Modify: `src/components/domain/SkillList.vue`
- Modify: `src/views/SkillsView.vue`
- Modify: `src/views/CompareLocationsView.vue`

**Interface:**

Create one shared mapping so the same state is never labelled differently across screens:

```ts
import type { LinkState } from "@/types";

export const linkStateLabels: Record<LinkState, string> = {
  linked: "Assigned",
  declared_only: "Missing",
  local_only: "Local only",
  broken_link: "Broken link",
};
```

`SkillStatusLegend.vue` must render the four text labels with their existing colours and the heading `Status`. Colour reinforces the text; it never replaces it.

**Steps:**

- [x] Add `linkStateLabels` and a typed `linkStateBadgeVariant(state)` helper to `statusLabels.ts`.
- [x] Use those helpers in `SkillRow.vue` and `CompareLocationsView.vue`; remove their local state-label switches.
- [x] Replace the bare location badge with `{{ issueCount }} issue` / `{{ issueCount }} issues` in `LocationRow.vue`.
- [x] Replace the Skills-view amber unused dot with a compact `Unused` badge and change the filter count to `{{ unusedCount }} unused`.
- [x] Label overview values as `Assigned`, `Local only` and `Broken links`, including zero values where the comparison matters.
- [x] Render `SkillStatusLegend` immediately above status-heavy skill lists and beside the unused filter in the library.
- [x] Give every icon-only preview, quick-assign, diff, activation and menu trigger both `title` and `aria-label`; use the skill name in the accessible label where available.
- [x] Ensure row menus remain visible on `:focus-within`, not only hover.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0 with no implicit-`any` errors in the shared helpers.
- [ ] In the running app, verify a user can identify `Broken link`, `Missing`, `Local only`, `Assigned` and `Unused` without opening Help.
- [ ] Turn on VoiceOver or inspect the accessibility tree and verify every icon-only action has a meaningful name.

**Commit:** `fix: clarify skill states and row actions`

### Task 3: Consolidate each location's actions and make its next step obvious

**Files:**

- Modify: `src/components/domain/LocationHeader.vue`
- Modify: `src/components/domain/LocationInspector.vue`
- Modify: `src/views/LocationDetailView.vue`
- Modify: `src/views/CompareLocationsView.vue`

**Behaviour:**

The header is authoritative:

1. `Add skills` remains the primary action.
2. `Sync` and `Reveal` remain visible secondary actions.
3. `More` contains `Open manifest` when available, `Compare with…`, and `Remove location…`.
4. The inspector contains metadata only: path, manifest status and last sync time.
5. When issues exist, the Health summary becomes a labelled action: `Resolve 1 issue` or `Resolve N issues`.

**Steps:**

- [x] Replace the header Remove button with an `SRowActionMenu`-style More menu containing the three actions above.
- [x] Move `openManifest()` from `LocationInspector.vue` to `LocationHeader.vue`; use the existing editor command preference.
- [x] Remove Sync, Open Manifest and Remove Location controls and their unused imports/functions from `LocationInspector.vue`.
- [x] Change the clickable Health badge in `LocationDetailView.vue` to a keyboard-operable button with explicit issue copy.
- [x] Route `Compare with…` to `{ path: '/compare', query: { locationA: detail.id } }`.
- [x] In `CompareLocationsView.vue`, read `route.query.locationA` on mount, validate that it exists in `locationList`, and preselect it without starting a comparison.
- [x] Stack or wrap the location header actions below 720px of content width so the location name and path remain readable.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Verify Sync and Remove appear in exactly one place.
- [ ] Verify `Resolve N issues` opens Health with the location filter and `Compare with…` preselects the current location.

**Commit:** `refactor: consolidate location actions`

---

## Phase 2 — Decision workflows

### Task 4: Turn Health into a grouped, filterable triage workspace

**Files:**

- Modify: `src-tauri/src/domain.rs`
- Modify: `src-tauri/src/scanner.rs`
- Modify: `src-tauri/src/commands/health.rs`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src/types/index.ts`
- Modify: `src/stores/healthStore.ts`
- Modify: `src/views/HealthView.vue`

**IPC additions:**

Keep existing `HealthIssue` fields and extend the result with per-location units:

```ts
export type HealthLocationSummary = {
  locationId: LocationId;
  locationLabel: string;
  errorCount: number;
  warningCount: number;
  infoCount: number;
  brokenLinkCount: number;
};

export type BrokenLinkRemovalPreview = {
  locationId: LocationId;
  locationLabel: string;
  paths: string[];
};

export type HealthCheckResult = {
  issues: HealthIssue[];
  locations: HealthLocationSummary[];
  locationCount: number;
  healthyCount: number;
  warningCount: number;
  errorCount: number;
  scannedAt: string;
};
```

Add two Tauri commands with matching Rust structs:

```ts
preview_broken_link_removal(locationIds: LocationId[]): Promise<BrokenLinkRemovalPreview[]>
remove_broken_links(locationIds: LocationId[]): Promise<HealthCheckResult>
```

The preview must rescan each selected location and return only current broken symlink paths. The apply command must remove only the paths that still resolve as broken symlinks at apply time, then run a fresh health check. Do not accept arbitrary paths from the frontend.

**Steps:**

- [x] Add the Rust and TypeScript types together.
- [x] Populate `HealthCheckResult.locations` inside `scanner::run_health_check`; derive `healthyCount` from entries whose three issue counts are zero.
- [x] Add scanner tests proving location counts and issue counts use the documented units.
- [x] Implement and register `preview_broken_link_removal` and `remove_broken_links`.
- [x] Add a Rust test proving the apply path ignores a link that is no longer broken between preview and confirmation.
- [x] In `healthStore.ts`, add `severityFilter: 'all' | 'healthy' | 'warning' | 'error'`, `selectedLocationIds`, grouped issue data and preview/apply methods.
- [x] Render the summary blocks as buttons labelled `N healthy locations`, `N warnings` and `N errors`; set `aria-pressed` for the active filter.
- [x] Group issues by location, make the location heading a real router link, and show cause-specific rows within each group.
- [x] Replace `Fix` with `Remove broken link` or `Remove N broken links`.
- [x] Allow selection only on groups with safe broken-link fixes. `Remove selected…` opens a preview listing location, count and paths, followed by an `SConfirmDialog`.
- [x] After apply, clear selection, replace the result with the fresh backend result and toast the actual number removed.

**Verification:**

- [x] Run `cd src-tauri && cargo test`. Expected: all tests pass, including new health tests.
- [x] Run `cd src-tauri && cargo clippy -- -D warnings`. Expected: exit 0.
- [x] Run `npm run build`. Expected: exit 0 and IPC types match.
- [ ] Manually verify summary filtering, location links, preview/cancel/apply and a stale preview where a link is repaired before confirmation.

**Commit:** `feat: add grouped health triage and safe bulk fixes`

### Task 5: Make global search real and distinguish it from local filters

**Files:**

- Create: `src/components/domain/GlobalSearchResults.vue`
- Modify: `src/components/layout/WindowToolbar.vue`
- Modify: `src/stores/appStore.ts`
- Modify: `src/composables/useKeyboardShortcuts.ts`
- Modify: `src/views/SkillsView.vue`
- Modify: `src/views/SetsView.vue`
- Modify: `src/components/domain/AssignmentSheet.vue`

**Store contract:**

Add only UI state to `appStore`:

```ts
const isGlobalSearchOpen = ref(false);

function openGlobalSearch(): void;
function closeGlobalSearch(): void;
```

`globalSearchQuery` already persists while the app is running; do not clear it on navigation. `GlobalSearchResults.vue` derives results from `locationsStore.locationList`, `libraryStore.items` and `setsStore.items`, grouped as Locations, Skills and Sets, with a maximum of six results per group.

**Steps:**

- [x] Wrap the top search in a combobox-style container with a visible `⌘K` hint.
- [x] Make `⌘K`/`Ctrl+K` focus the global field and open results even when another input is focused; retain `/` for the nearest local filter only.
- [x] On first global-search focus, fetch library items and sets if their stores are empty; do not refetch on every keystroke.
- [x] Filter name, path, summary and tags case-insensitively and show an explicit empty result.
- [x] Add arrow-key traversal, Enter to navigate, Escape to close and click-outside dismissal. Keep the query after navigation.
- [x] On selection, update the relevant store selection before routing to `/locations/:id`, `/skills/:id` or the encoded `/sets/:setKey`.
- [x] Rename local placeholders to `Filter skills`, `Filter sets` and `Filter skills and sets` in Skills, Sets and Assignment Sheet respectively.
- [x] Remove the current ambiguous selector that includes sets in `SkillsView`; that screen becomes the Skills tab in Task 8.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Search for one location, skill and set using keyboard only; verify grouped results, correct detail navigation and query preservation after Back.
- [ ] Verify typing in a local filter does not open or overwrite global search.

**Commit:** `feat: add grouped global search`

### Task 6: Turn recommendations into a selectable, dismissible flow

**Files:**

- Modify: `src-tauri/src/domain.rs`
- Modify: `src-tauri/src/scanner.rs`
- Modify: `src/types/index.ts`
- Create: `src/composables/useDismissedRecommendations.ts`
- Modify: `src/stores/assignmentStore.ts`
- Modify: `src/views/LocationDetailView.vue`

**Data contract:**

Replace the inferred reason grouping with explicit backend data:

```ts
export type SkillRecommendation = {
  skillId: SkillId;
  skillName: string;
  projectType: string;
  reason: string | null;
};
```

The scanner sets `projectType` to the matched detected project type. Set `reason` only when it adds information beyond `Recommended for {projectType}`; otherwise return `null` so repeated boilerplate is not rendered.

Persist dismissed recommendation IDs per location in local storage under `kit.dismissedRecommendations.v1`:

```ts
type DismissedRecommendations = Record<LocationId, SkillId[]>;
```

Expose `isDismissed`, `dismiss`, `restore` and `restoreAll(locationId)`. Malformed storage must fall back to `{}` without breaking the location view.

**Steps:**

- [x] Add `project_type` and optional `reason` to the Rust type and update the TypeScript mirror.
- [x] Update `scanner::recommend_skills` and add tests for technology grouping, archived/assigned exclusion and non-repeated reasons.
- [x] Implement the local-storage composable with immutable array updates so Vue reacts correctly.
- [x] Extend `assignmentStore.open(forLocationId, initialSkillIds: SkillId[] = [])` to seed selection and fetch a preview when initial IDs are supplied.
- [x] In `LocationDetailView.vue`, group visible recommendations by `projectType` and keep a local `Set<SkillId>` for checkboxes.
- [x] Add `Select all`, `Add selected` and per-row `Dismiss` actions; `Add selected` must open the existing Assignment Sheet with the selected skills and its before/apply preview.
- [x] Render a `N dismissed` disclosure with `Restore` actions so dismissal is reversible.
- [x] Clear selected IDs that disappear after assignment or a location rescan.

**Verification:**

- [x] Run `cd src-tauri && cargo test`. Expected: recommendation tests pass.
- [x] Run `cd src-tauri && cargo clippy -- -D warnings`. Expected: exit 0.
- [x] Run `npm run build`. Expected: exit 0.
- [ ] Dismiss a recommendation, restart the app and verify it remains hidden; restore it and verify multi-selection opens the normal assignment preview.

**Commit:** `feat: make skill recommendations actionable`

### Task 7: Make the modified-files view honest and informative

**Files:**

- Modify: `src-tauri/src/domain.rs`
- Modify: `src-tauri/src/commands/changelog.rs`
- Modify: `src/types/index.ts`
- Modify: `src/views/ChangelogView.vue`

**Decision:**

Kit does not currently store historical snapshots for every edit, link and unlink event. Do not fabricate change types or a before/after diff. Keep the `/changelog` route for compatibility but label the destination and screen `Recently modified` until an event log exists.

Extend each entry with the locations currently affected:

```ts
assignedLocations: Array<{
  id: LocationId;
  label: string;
}>;
```

**Steps:**

- [x] Replace `assignedLocationCount` in both IPC types with `assignedLocations`; derive the count in Vue.
- [x] Populate the array while scanning saved locations in `get_skill_changelog`.
- [x] Change the page title, loading copy and empty state from Changelog/changes to Recently modified/modified skills.
- [x] Use `SKILL.md edited` as the row summary instead of repeating the skill ID.
- [x] Show exact local date and time in the visible row and list up to two affected location names plus `+N more`.
- [x] Keep row navigation explicit with an `Open skill` label or chevron; do not imply an activity-detail screen.

**Verification:**

- [x] Run `cd src-tauri && cargo test && cargo clippy -- -D warnings`. Expected: both exit 0.
- [x] Run `npm run build`. Expected: exit 0.
- [x] Verify every visible claim is supported by current filesystem data and that no row claims a diff is available.

**Commit:** `fix: rename changelog to recently modified`

---

## Phase 3 — Information architecture and secondary workflows

### Task 8: Consolidate primary navigation around Locations, Library, Health and Recently modified

**Files:**

- Create: `src/components/domain/LibraryTabs.vue`
- Modify: `src/components/layout/SidebarNav.vue`
- Modify: `src/views/SkillsView.vue`
- Modify: `src/views/SetsView.vue`
- Modify: `src/composables/useKeyboardShortcuts.ts`
- Modify: `src/components/domain/ShortcutHelpOverlay.vue`

**Navigation contract:**

- Primary destinations: Locations → `/locations`; Library → `/skills`; Health → `/health`; Recently modified → `/changelog`.
- `Library` is active for both `/skills` and `/sets`.
- `LibraryTabs.vue` renders router links for `Skills` and `Sets` at the top of both Library sidebars.
- Compare remains at `/compare` but is entered contextually from the location More menu added in Task 3.

**Steps:**

- [x] Replace the six-item `navItems` array with the four destinations above and remove unused Compare/Set icons.
- [x] Make `isActive` accept the Library route group (`/skills` or `/sets`).
- [x] Add `LibraryTabs.vue` to both views, before their local filter controls.
- [x] In `SkillsView.vue`, fix `libraryStore.filterKind` to `skill` on mount and remove the All/Skills/Sets segmented control.
- [x] Update shortcuts to `⌘1 Locations`, `⌘2 Library`, `⌘3 Health`, `⌘4 Recently modified`; retain `⌘/` for shortcut help.
- [x] Update `ShortcutHelpOverlay.vue` labels and destinations to match exactly.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Verify all old routes still load directly, Library remains highlighted on both tabs, and Compare is reachable from a location.
- [ ] Verify the displayed shortcut help and actual keyboard behaviour agree.

**Commit:** `refactor: simplify primary navigation`

### Task 9: Reduce controls and improve editing in Set detail

**Files:**

- Modify: `src/views/SetDetailView.vue`

**Behaviour:**

- Keep drag-and-drop reordering.
- Replace always-visible up, down and remove controls with one row menu: `Move up`, `Move down`, `Remove from set`.
- Each focused row supports `Alt+ArrowUp` and `Alt+ArrowDown`.
- Replace the small section-header `Add` action with an `Add skills` button.
- Show description text plus an explicit `Edit description` action; editing uses the existing `SInlineTextField` and saves through `setsStore.updateSet`.

**Steps:**

- [x] Make each skill row focusable and add the two keyboard handlers with boundary checks.
- [x] Expose the two row shortcuts with `aria-keyshortcuts`.
- [x] Replace `.reorder-buttons` and `.remove-button` with `SRowActionMenu`; disable unavailable Move actions at the first and last positions.
- [x] Preserve drag handles and make their tooltip `Drag to reorder {skill name}`.
- [x] Replace the `SSectionHeader` action with a proper header row and `SButton size="sm"` labelled `Add skills`.
- [x] Add `isEditingDescription`; show `No description` and `Edit description` when empty, or the saved text and `Edit description` when present.
- [x] Ensure menu actions do not trigger the row's skill preview.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Reorder the first, middle and last rows using drag, menu and keyboard; verify the saved order survives reopening the set.
- [ ] Edit, clear and restore a description and verify the explicit editing state is understandable.

**Commit:** `fix: simplify set detail controls`

### Task 10: Make onboarding and empty states follow one clear path

**Files:**

- Modify: `src/views/OnboardingView.vue`
- Modify: `src/components/domain/LocationList.vue`
- Modify: `src/views/LocationsView.vue`
- Modify: `src/views/SetsView.vue`

**Onboarding flow:**

Use four explicit states:

```ts
type OnboardingStep = "library" | "project" | "scan" | "review";
```

1. Library: choose and validate the skills repository, then save it before continuing.
2. Project: add the first project location.
3. Scan: show progress while `addLocation`/`fetchDetail` completes.
4. Review: show the skill and issue counts. The primary action is `Resolve N issues` when issues exist, otherwise `Open location`.

Each state has one primary action; Back is secondary and must not discard a saved valid library silently.

**Steps:**

- [x] Save `libraryRoot` immediately after a valid repository is confirmed, before calling `locationsStore.addLocation`.
- [x] Split project selection, scan progress and review into the states above.
- [x] Route `Resolve N issues` to `/health?locationId=…`; route `Open location` to the new location detail.
- [x] Keep the editor command optional and move it to the library step as a secondary field.
- [x] Hide `LocationList.vue`'s Add Location footer when the list is empty so `LocationsView.vue` owns the sole empty-state primary action.
- [x] Hide the Sets sidebar `New Set` footer when there are no sets so the workspace empty state owns `Create Set`.
- [x] Change empty-state copy to one action and one outcome; do not repeat drag and button instructions in separate panes.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] With a temporary empty Kit state, complete library → project → scan → review and verify no project scan occurs against the previous library root.
- [ ] Verify Locations and Sets each show only one primary add/create action when empty.

**Commit:** `fix: streamline onboarding and empty states`

### Task 11: Tighten Settings and support the system appearance

**Files:**

- Modify: `src/composables/useTheme.ts`
- Modify: `src/views/SettingsView.vue`

**Theme contract:**

```ts
export type ThemePreference = "system" | "light" | "dark";

return {
  theme,          // stored ThemePreference
  resolvedTheme,  // "light" | "dark"
  setTheme,
};
```

Store the preference under the existing `kit.theme` key. Existing `light` and `dark` values remain valid; an absent or invalid value becomes `system`. Create one module-scope `window.matchMedia('(prefers-color-scheme: dark)')` listener for the app lifetime so mounting Settings repeatedly cannot add duplicate listeners.

**Steps:**

- [x] Implement `system` resolution and media-query change handling in `useTheme.ts`; keep one shared listener rather than adding one per Settings mount.
- [x] Replace Dark mode's checkbox with an `SSegmentedControl` labelled System, Light and Dark.
- [x] Rename `Select Backup` to `Restore from Backup…`.
- [x] Change its description to `Choose a Kit backup to preview changes before replacing or merging current library data.`
- [x] Keep the existing restore preview and confirmation; make overwrite wording explicitly state which current files can change.
- [x] Constrain `.settings-view` to `max-width: 760px; width: 100%; margin: 0 auto;` while retaining responsive side padding.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Verify System follows a live macOS appearance change and explicit Light/Dark ignore it.
- [ ] Restart Kit in each mode and verify the preference persists without a theme flash.
- [ ] Open restore preview and confirm no current data changes before the final Restore action.

**Commit:** `feat: add system theme and clearer restore settings`

### Task 12: Make Help contextual and align it with the updated UI

**Files:**

- Modify: `src/views/HelpView.vue`
- Modify: `src/components/domain/ShortcutHelpOverlay.vue`

**Steps:**

- [x] Add a compact `Start here` group with router links to Locations, Library, Health and Settings.
- [x] Turn references such as “set your skill library root in Settings” into real `RouterLink` controls.
- [x] Replace the colour-only Help legend with the same text labels used by `SkillStatusLegend`: Assigned, Local only, Missing and Broken link.
- [x] Update Skills/Sets terminology to Library tabs and replace Changelog wording with Recently modified.
- [x] Remove instructions for controls moved or renamed in Tasks 3, 8 and 11.
- [x] Keep Help as a reference, but shorten repeated paragraphs where the relevant view now explains the action in place.
- [x] Recheck shortcut documentation against `useKeyboardShortcuts.ts`.

**Verification:**

- [x] Run `npm run build`. Expected: exit 0.
- [ ] Follow every Help link and verify it lands on the named destination.
- [x] Search Help for the obsolete strings `Changelog`, `Select Backup`, `Dark mode`, and colour-only `dot =`; expected: no stale instructional copy.

**Commit:** `docs: make in-app help contextual`

---

## Phase 4 — Integrated verification and release gate

### Task 13: Run cross-workflow usability and regression checks

**Files:**

- Modify: `planning/2026-07-13-usability-improvements-implementation-plan.md` (tick completed checks and record only genuine follow-up blockers below the relevant task)

**Automated verification:**

- [x] Run `npm run build`. Expected: TypeScript checking and Vite production build exit 0.
- [ ] Run `cd src-tauri && cargo fmt -- --check`. Expected: exit 0.
- [x] Run `cd src-tauri && cargo test`. Expected: all tests pass.
- [x] Run `cd src-tauri && cargo clippy -- -D warnings`. Expected: exit 0.
- [x] Run `npm run tauri build` when all preceding checks pass. Expected: a distributable app is produced without changing source files.

Follow-up: `cargo fmt -- --check` remains blocked by pre-existing formatting drift in `src/bin/kit.rs`, `src/commands/{activation,assignment,backup,bootstrap,comparison,external,health,library,locations,manifest,repo,sets}.rs`, `src/{lib,linker,scanner,state,tray,watcher}.rs`. Task 13 deliberately did not mass-format unrelated code.

**Manual acceptance matrix:**

- [ ] At 1430 × 971, verify the wide split-pane layout remains efficient and inspectors can be collapsed and restored.
- [ ] At 900 × 700 and the supported 800 × 500 minimum, verify Locations, Skills and Sets show list or detail with a Back action and no clipped primary action.
- [ ] Use only the keyboard to navigate primary destinations, global search, local filters, list rows, set row menus, recommendation selection and Health fixes.
- [ ] Verify counts always include units: issues, unused, recommendations, healthy locations, warnings and errors.
- [ ] Verify status meaning never depends on colour alone and all icon-only actions have accessible names.
- [ ] Verify one location can be added, synced, compared, assigned skills, taken to Health and removed without encountering duplicate authoritative actions.
- [ ] Verify dismissed recommendations and theme preference survive an app restart.
- [ ] Verify bulk broken-link removal always previews paths and requires confirmation.
- [ ] Verify `Recently modified` makes no unsupported claim about diffs or event history.
- [ ] Verify each first-run and empty screen has one primary action.
- [ ] Verify System/Light/Dark, backup preview, restore copy and Settings width at both target window sizes.
- [ ] Verify Help links, terms and keyboard shortcuts match the implemented interface.

**Regression checks:**

- [ ] Assign and remove individual skills and sets through Assignment Sheet; verify preview and manifest update behaviour is unchanged.
- [ ] Reorder a set and reopen it; verify persistence.
- [ ] Open skill peek and diff panels from all retained entry points.
- [ ] Start the app with an existing persisted state created before these changes; verify missing recommendation-dismissal and theme values use safe defaults.
- [ ] Confirm direct visits to `/skills`, `/sets`, `/compare` and `/changelog` still work.

**Commit:** `chore: verify usability improvements`

## Review coverage

| Review recommendation | Implemented by |
| --- | --- |
| Adaptive list/detail views, Back navigation, collapsed primary sidebar and retained minimum-size guard | Task 1 |
| Genuine global search, labelled local filters, visible `⌘K` and preserved query | Task 5 |
| Text status labels, labelled counts, inline legend, colour reinforcement, tooltips and accessible names | Task 2 |
| One location action hierarchy, state-aware issue action and collapsible inspector | Tasks 1 and 3 |
| Technology-grouped, multi-select and dismissible recommendations | Task 6 |
| Explicit Health units, summary filters, grouping, precise fixes, preview/confirmation and location links | Task 4 |
| Honest modified-file history and affected-location context | Task 7 |
| Four primary destinations, Library tabs and contextual Compare | Task 8 |
| Quieter Set rows, keyboard reordering, clear Add skills and explicit description editing | Task 9 |
| Single-action empty states and library → project → scan → review onboarding | Task 10 |
| System/Light/Dark, clearer restore wording and constrained Settings width | Task 11 |
| Contextual Help links and guidance moved into status-heavy views | Tasks 2 and 12 |

## Delivery boundaries

This plan intentionally does not include:

- a dashboard;
- a design-system rewrite;
- a new dependency or frontend test framework;
- a fabricated activity log or historical diff model;
- an increased minimum window size as a substitute for responsive behaviour.

A future Activity implementation should start only after Kit has a persisted event model containing event type, timestamp, actor/source, affected locations and before/after data. At that point `/changelog` can be migrated deliberately; it is not part of this usability release.

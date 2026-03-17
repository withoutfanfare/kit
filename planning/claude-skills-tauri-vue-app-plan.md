# Kit v1 Product Specification

This document defines the implementation target for v1.

## 1. Product Definition

`Kit` is a native-feeling macOS desktop app for managing Claude skills and sets across a skill library and multiple project locations.

The product is not a dashboard and not a generic agent control plane. It is a calm, Finder-style loadout manager that answers three practical questions fast:

- What is active in this location?
- Where is this skill used?
- Which sets should I create, edit, and assign?

### Core product stance

- Skills are the atomic unit.
- Sets are first-class loadout objects, but they remain file-backed JSON.
- Locations consume both explicit skills and assigned sets.
- Resolved links are derived from those declarations and shown clearly.
- Sets must remain CLI-friendly and editable outside the app.

### V1 promise

- Understand a location’s current loadout in seconds.
- Create and configure sets in-app without friction.
- Assign skills or sets to locations with a safe preview before applying.
- Separate declaration actions from link actions so the UI is trustworthy.
- Use inline editing for light tasks and excellent external editor flows for heavy tasks.
- Let teams commit project sets to the repository and share them through git like other project files.

### Product boundary

- `Kit` manages skills, sets, manifests, declarations, and links.
- `Kit` does not run agents, orchestrate commands, or become a terminal replacement.
- Future agent or command control can be layered later, but must not dilute the loadout-management UX in v1.

## 2. Core User Stories And Jobs To Be Done

### Primary jobs

- When I open a project, I want to see its actual loadout immediately so I can trust what is active and why.
- When I notice repeated skill combinations, I want to turn them into a reusable set quickly.
- When I manage many projects, I want to assign and unassign sets with confidence instead of recreating skill combinations manually.
- When a skill appears in a location, I want to know whether it came from an explicit skill declaration, a set, or both.
- When something is broken or drifting, I want the app to explain the state and give me the right next action.

### Core user stories

- As a user, I can register one skill library root and a saved list of project locations.
- As a user, I can create as many sets as I want directly inside the app.
- As a user, I can choose whether a set is global or project-scoped.
- As a user, I can edit a set’s name, description, and skill membership without leaving the app.
- As a user, I can assign a set to a location and preview exactly which skills and manifest entries will change.
- As a user, I can inspect a location and see assigned sets, explicit skills, resolved skills, and issues separately.
- As a user, I can inspect a skill and see which sets include it and which locations use it.
- As a user, I can inspect a set and see which skills it contains and which locations it is assigned to.
- As a user, I can commit project-scoped sets to the repo so the team shares them through normal version control.
- As a user, I can use separate actions for `Add to Manifest`, `Remove from Manifest`, `Link`, and `Unlink` instead of one ambiguous button.
- As a user, I can open raw files in my editor when the task is structural or long-form.

## 3. V1 Feature Set

### In scope

- One skill library root.
- Saved project locations managed by the app.
- First-class set management in the app:
  - sets remain JSON files on disk
  - global sets and project-scoped sets are both supported
  - create set
  - rename set
  - edit set description
  - add skills to set
  - remove skills from set
  - reorder skills inside a set
  - delete set
- Location loadout management:
  - assign set to location
  - unassign set from location
  - add explicit skill to location
  - remove explicit skill from location
  - preview resolved link and manifest changes before apply
- Read and render:
  - skills
  - sets
  - location declarations
  - resolved links
  - archive state
  - usage summaries
- Clear loadout provenance:
  - explicit skill
  - provided by set
  - explicit skill plus set
  - local only
  - broken link
- Separate quick actions for declaration vs filesystem state:
  - `Add to Manifest`
  - `Remove from Manifest`
  - `Link`
  - `Unlink`
  - `Assign Set`
  - `Unassign Set`
- Inline editing for:
  - saved location label
  - saved location notes
  - set name
  - set description
- In-app set composition UI for membership editing.
- External actions:
  - open `SKILL.md`
  - open set file
  - open manifest file
  - reveal location
  - reveal library item
- Settings for:
  - library root
  - editor opener preference
  - show archived by default
  - default view

### Explicitly out of scope for v1

- Running agents from the app.
- Running arbitrary shell commands from the app.
- Live terminal embedding.
- Multi-user collaboration.
- Cloud sync.
- Git-aware workflows.
- Nested sets.
- Conditional or parameterised sets.
- Database-only sets or opaque app-owned set formats.
- Bulk apply across multiple locations in one action.
- Matrix/grid overview across every skill and every location.
- Background file watching and instant auto-refresh.
- Full rich-text or markdown editing inside the app.
- Deleting library skills from disk.

### Deferred but likely v1.1+

- Duplicate set
- Archive set
- Batch assignment across multiple locations
- Watchers and refresh-on-focus
- Better usage analytics
- Read-only agent and command metadata if the ecosystem grows in that direction

## 4. Information Architecture And Route/View Structure

The app should feel like one workspace with strong object navigation, not several mini-products.

### Top-level navigation

- `Locations`
- `Skills`
- `Sets`
- `Usage`
- `Settings`

`Locations` is the default landing view.

### Route map

```text
/                    -> redirect to /locations
/locations
/locations/:locationId
/skills
/skills/:skillId
/sets
/sets/:setId
/usage
/settings
```

### View structure

#### `/locations`

Primary operational screen.

- Sidebar: saved locations
- Main pane: selected location or empty state
- Inspector: metadata and actions for the selected object

#### `/locations/:locationId`

Main day-to-day loadout view.

- Toolbar:
  - search
  - segmented scope filter
  - primary button: `Edit Loadout`
- Content sections:
  - Overview
  - Assigned Sets
  - Explicit Skills
  - Resolved Skills
  - Issues
- Inspector:
  - path
  - manifest status
  - sync state
  - quick actions

#### `/skills`

Skill library view.

- Searchable skill list
- Filters: archived, unused, linked
- Detail pane for selected skill

#### `/skills/:skillId`

Skill relationship view.

- Header: name, archive state, source path
- Sections:
  - summary
  - included in sets
  - linked locations
  - usage
  - actions

#### `/sets`

Set management view.

- Searchable set list
- Scope filter: `All`, `Global`, `Project`
- Quick create affordance
- Detail pane for selected set

#### `/sets/:setId`

Set detail and editing view.

- Header: name, description, scope, set size
- Sections:
  - included skills
  - assigned locations
  - quick actions
- Membership editing happens here, not in an external editor by default

#### `/usage`

Supporting view, not a dashboard.

- Most used skills
- Recently used skills
- Unused skills
- Suggestions

#### `/settings`

- General
- Library
- Editor
- Advanced

## 5. Primary UX Flows

### Flow 1: First launch

1. User opens `Kit`.
2. App asks for:
   - skill library root
   - first project location
   - preferred editor opener
3. App lands on that location detail view.

### Flow 2: Edit a location loadout

1. User opens a location.
2. User presses `Edit Loadout`.
3. A sheet opens with two tabs:
   - `Skills`
   - `Sets`
4. User selects explicit skills and assigned sets.
5. Preview explains:
   - declarations to add or remove
   - links to create or remove
   - duplicates ignored
   - warnings
6. User presses `Apply`.
7. Location view updates and changed rows are highlighted briefly.

### Flow 3: Create a set

1. User opens `Sets`.
2. User presses `New Set`.
3. User chooses scope:
   - `Global`
   - `Project`
4. If `Project` is chosen, the user selects the owning location or current project context is prefilled.
5. User enters name and optional description.
6. Set detail opens immediately in edit mode.
7. User adds skills through an in-app membership picker.

This flow should take seconds, not feel like file authoring.

### Flow 4: Edit a set

1. User opens a set.
2. User updates name or description inline.
3. User adds, removes, or reorders skills in the membership list.
4. App saves immediately or on explicit save, depending on the final implementation choice.

### Flow 5: Inspect provenance in a location

1. User opens a location.
2. In `Resolved Skills`, each row shows provenance badges:
   - `Explicit`
   - `From Set: <name>`
   - `Explicit + Set`
   - `Local Only`
   - `Broken`
3. User can tell instantly why a skill is present.

### Flow 6: Quick row actions are explicit

Location rows must never collapse declaration and link operations into one ambiguous button.

Valid actions appear separately, for example:

- `Add to Manifest`
- `Remove from Manifest`
- `Link`
- `Unlink`
- `Assign Set`
- `Unassign Set`

The UI only shows actions that make sense for the current state.

### Flow 7: Open heavy content externally

1. User clicks `Open in Editor` on a skill, set, or manifest.
2. App opens the file or folder via configured editor command or system default.
3. User returns to `Kit` and refreshes manually in v1 if needed.

## 6. Component Architecture

Views should compose a small reusable system instead of duplicating view markup.

### Shell and layout components

- `AppShell`
- `WindowToolbar`
- `SidebarNav`
- `SplitPaneLayout`
- `InspectorPanel`
- `EmptyState`

### Base UI components

- `SearchField`
- `SegmentedControl`
- `PrimaryButton`
- `SecondaryButton`
- `IconButton`
- `InlineTextField`
- `Badge`
- `StatusBadge`
- `RowActionMenu`
- `SectionHeader`
- `ListRow`
- `SheetPanel`
- `ConfirmDialog`
- `NoticeBanner`

### Domain components

- `LocationList`
- `LocationRow`
- `LocationHeader`
- `LocationOverviewCard`
- `AssignedSetList`
- `ExplicitSkillList`
- `ResolvedSkillList`
- `ResolvedSkillRow`
- `IssueList`
- `LocationInspector`
- `SkillList`
- `SkillInspector`
- `SetList`
- `SetRow`
- `SetInspector`
- `SetEditor`
- `SetMembershipList`
- `SetMembershipPicker`
- `LoadoutSheet`
- `LoadoutPreview`
- `LinkedLocationsList`
- `UsageSummaryPanel`

### View composition

#### Location detail

- `AppShell`
- `WindowToolbar`
- `SplitPaneLayout`
- `LocationList`
- `LocationHeader`
- `LocationOverviewCard`
- `AssignedSetList`
- `ExplicitSkillList`
- `ResolvedSkillList`
- `IssueList`
- `LocationInspector`
- `LoadoutSheet`

#### Set detail

- `AppShell`
- `WindowToolbar`
- `SplitPaneLayout`
- `SetList`
- `SetEditor`
- `SetInspector`

## 7. Rust Backend Commands And Contracts

The Rust layer owns filesystem truth, parsing, diffing, set expansion, and mutations. The frontend should not implement path or symlink logic.

### Core contract types

```ts
type LocationId = string
type SkillId = string
type SetId = string
type SetScope = "global" | "project"

type LinkState =
  | "linked"
  | "declared_only"
  | "local_only"
  | "broken_link"

type SkillProvenance = {
  explicit: boolean
  setIds: SetId[]
}

type SavedLocationSummary = {
  id: LocationId
  label: string
  path: string
  issueCount: number
  assignedSetCount: number
  explicitSkillCount: number
  resolvedSkillCount: number
  lastSyncedAt: string | null
}

type LocationDetail = {
  id: LocationId
  label: string
  path: string
  manifestPath: string | null
  notes: string | null
  assignedSets: SetAssignment[]
  explicitSkills: ExplicitSkillAssignment[]
  resolvedSkills: ResolvedSkillAssignment[]
  issues: LocationIssue[]
  stats: {
    assignedSetCount: number
    explicitSkillCount: number
    resolvedSkillCount: number
    brokenCount: number
  }
}

type SetSummary = {
  id: SetId
  name: string
  description: string | null
  scope: SetScope
  ownerLocationId: LocationId | null
  skillCount: number
  assignedLocationCount: number
  path: string
}

type SetDetail = {
  id: SetId
  name: string
  description: string | null
  scope: SetScope
  ownerLocationId: LocationId | null
  path: string
  skills: Array<{ id: SkillId; name: string; archived: boolean }>
  assignedLocations: SavedLocationSummary[]
}

type LoadoutPreview = {
  locationId: LocationId
  declarationAdds: PreviewChange[]
  declarationRemoves: PreviewChange[]
  linkAdds: PreviewChange[]
  linkRemoves: PreviewChange[]
  warnings: string[]
}
```

### Manifest model

For v1, `Kit` should treat the location manifest as two first-class declaration arrays:

- `skills`: explicit skills
- `sets`: assigned sets

Resolved links for a location are:

- `skills`
- plus expanded skills from all assigned `sets`

This is the basis for reliable provenance and safe unassignment.

### Set storage model

Sets remain plain JSON files and must stay compatible with command-line workflows.

Recommended storage layout:

- global sets: `<libraryRoot>/sets/*.set.json`
- project sets: `<location>/.claude/sets/*.set.json`

Rules:

- `Kit` edits these files in place.
- The app does not keep a separate hidden database representation of sets.
- Project sets are intended to be committed to the repo and shared through git.
- The CLI should be able to read the same set files without app-specific translation.
- Project-scoped sets are visible when working inside that project context.
- Global and project sets should both be assignable to locations.

### Required Tauri commands

#### Bootstrap and settings

- `get_app_bootstrap() -> AppBootstrap`
- `update_preferences(input: UpdatePreferencesInput) -> Preferences`

#### Locations

- `list_locations() -> SavedLocationSummary[]`
- `add_location(input: { path: string; label?: string }) -> SavedLocationSummary`
- `update_location(input: { id: LocationId; label?: string; notes?: string | null }) -> SavedLocationSummary`
- `remove_location(input: { id: LocationId }) -> SavedLocationSummary[]`
- `get_location_detail(input: { id: LocationId }) -> LocationDetail`
- `sync_location(input: { id: LocationId }) -> LocationDetail`

#### Skills

- `list_skills(input: { query?: string; includeArchived?: boolean }) -> SkillSummary[]`
- `get_skill_detail(input: { id: SkillId }) -> SkillDetail`
- `archive_skill(input: { id: SkillId }) -> SkillDetail`
- `unarchive_skill(input: { id: SkillId }) -> SkillDetail`

#### Sets

- `list_sets(input: { query?: string; scope?: "all" | "global" | "project"; locationId?: LocationId }) -> SetSummary[]`
- `create_set(input: { name: string; description?: string | null; scope: SetScope; ownerLocationId?: LocationId }) -> SetDetail`
- `get_set_detail(input: { id: SetId }) -> SetDetail`
- `update_set(input: { id: SetId; name?: string; description?: string | null; skillIds?: SkillId[] }) -> SetDetail`
- `delete_set(input: { id: SetId }) -> { ok: true }`

#### Loadout management

- `preview_assignment(input: { locationId: LocationId; addSkillIds?: SkillId[]; removeSkillIds?: SkillId[]; addSetIds?: SetId[]; removeSetIds?: SetId[] }) -> LoadoutPreview`
- `apply_assignment(input: { locationId: LocationId; addSkillIds?: SkillId[]; removeSkillIds?: SkillId[]; addSetIds?: SetId[]; removeSetIds?: SetId[] }) -> LocationDetail`

#### Explicit row actions

- `add_skill_to_manifest(input: { locationId: LocationId; skillId: SkillId }) -> LocationDetail`
- `remove_skill_from_manifest(input: { locationId: LocationId; skillId: SkillId }) -> LocationDetail`
- `assign_set_to_location(input: { locationId: LocationId; setId: SetId }) -> LocationDetail`
- `unassign_set_from_location(input: { locationId: LocationId; setId: SetId }) -> LocationDetail`
- `link_skill(input: { locationId: LocationId; skillId: SkillId }) -> LocationDetail`
- `unlink_skill(input: { locationId: LocationId; skillId: SkillId }) -> LocationDetail`

#### Usage

- `get_usage_summary(input: { days?: number }) -> UsageSummary`

#### External actions

- `open_path_in_editor(input: { path: string }) -> { ok: true }`
- `reveal_in_finder(input: { path: string }) -> { ok: true }`
- `open_with_default_app(input: { path: string }) -> { ok: true }`

### Backend rules

- All mutations return updated domain objects, not just `ok`.
- Set expansion happens in Rust.
- Preview logic happens in Rust.
- The frontend never creates or removes symlinks directly.
- Removing a set from a location must not remove a skill link that is still required by another set or explicit skill declaration.

## 8. Frontend State Model

Use Pinia with domain stores, not per-view state blobs.

### Stores

#### `appStore`

- bootstrap data
- global search query
- global loading and error state

#### `preferencesStore`

- library root
- editor command
- default view
- show archived

#### `locationsStore`

- location list
- selected location id
- detail cache by id

#### `skillsStore`

- skill list
- selected skill id
- detail cache by id
- filters

#### `setsStore`

- set list
- selected set id
- detail cache by id
- active scope filter
- create and delete state

#### `loadoutStore`

- sheet open state
- active location id
- selected explicit skills
- selected assigned sets
- selected removals
- latest preview
- apply in-flight state

#### `usageStore`

- summary payload
- time range

### State principles

- Server state is cached but re-fetchable.
- Selection state is separate from fetched data.
- Optimistic updates are acceptable for small inline edits.
- Mutations that affect filesystem truth revalidate from Rust after completion.

## 9. Editing Model

V1 should keep lightweight tasks in-app, especially for sets.

### Inline-editable

- saved location label
- saved location notes
- set name
- set description

### Edited in-app, but not as free-form text

- set membership
- set ordering
- location loadout assignment
- manifest declaration actions

These are structural interactions and should use explicit controls rather than raw text editing.

### Opens externally

- `SKILL.md`
- raw manifest file
- raw set JSON file if the user wants direct file control
- any long-form markdown content
- folder-level file operations

### Rule of thumb

- If the task is scalar and low risk, keep it inline.
- If the task is structured but central to the product, build a dedicated in-app editor.
- If the task is free-form, multiline, or file-oriented, open externally.

## 10. First-Pass Visual Design System

The app should feel like a quiet macOS utility, not a web dashboard inside a shell.

### Design direction

- Native window chrome with transparent title bar on macOS.
- System font stack with SF Pro behaviour.
- Finder-like sidebar, Settings-like grouped content, restrained inspector.
- Calm, minimal, text-first UI.

### Visual principles

- Prefer grouped lists and inset sections over loud cards.
- Use colour mostly for state and warnings.
- Keep set editing surfaces clean and list-driven, not form-heavy.
- Avoid novelty animation.

### Tokens

#### Typography

- `font-sans`: system-ui, -apple-system, BlinkMacSystemFont, sans-serif
- `text-xs`: 11px
- `text-sm`: 12px
- `text-md`: 13px
- `text-lg`: 15px
- `text-xl`: 20px

#### Spacing

- `space-1`: 4px
- `space-2`: 8px
- `space-3`: 12px
- `space-4`: 16px
- `space-5`: 20px
- `space-6`: 24px

#### Radius

- `radius-sm`: 6px
- `radius-md`: 10px
- `radius-lg`: 14px

#### Surfaces

- `surface-app`
- `surface-sidebar`
- `surface-panel`
- `surface-hover`
- `surface-selected`

#### Roles

- `text-primary`
- `text-secondary`
- `text-tertiary`
- `border-subtle`
- `accent`
- `success`
- `warning`
- `danger`

### Control styling

- Toolbar height: compact, around 44px below title bar
- Sidebar rows: 28-32px
- List rows: 32-36px
- Pills and provenance badges: compact and text-first
- Inspector width: fixed and calm
- Set membership editor: two-pane picker or grouped checklist, not modal chaos

### Motion

- 150-200ms ease for fades, sheet transitions, and row insertion
- No bounce-heavy or playful motion

## 11. Open Questions, Risks, And Decisions Still To Make

### Decisions made for v1

- `Locations` is the default landing view.
- Sets are first-class in v1.
- Sets remain JSON on disk and are not app-owned records.
- Project-scoped sets live in the repository.
- Set creation and membership editing happen in-app.
- Manifest declaration and link actions stay separate in the UI.
- One library root is enough for v1.
- No nested sets in v1.
- Agent and command execution remain out of scope.

### Open questions

- Should set descriptions stay optional, or should every set have one-line intent text?
- Should set order matter only in the editor, or also in location presentation?
- Do we want a duplicate-set action in v1, or can it wait?
- Should archived skills remain selectable inside sets, or be warned against?
- How visible should provenance badges be before the UI becomes noisy?
- How should global and project set ID collisions behave?

### Risks

- If set assignment and explicit skill assignment are not modelled separately in the manifest, unassignment logic will be fragile.
- If the preview is inaccurate even once, trust in the product drops sharply.
- Too much raw manifest editing would blur the boundary between manager and editor.
- Set editing can become cluttered if the composer UI is not disciplined.
- If the CLI and app diverge on set JSON shape or storage paths, team sharing will break down quickly.

### Implementation priority

1. Finalise manifest and provenance rules in Rust, including separate `skills` and `sets` declarations.
2. Finalise the set JSON schema and storage paths for global and project-scoped sets.
3. Implement set CRUD and set detail editing against real JSON files.
4. Replace the current assignment concept with a real `LoadoutSheet`.
5. Build explicit row actions for declaration vs link state.
6. Add usage and settings polish last.

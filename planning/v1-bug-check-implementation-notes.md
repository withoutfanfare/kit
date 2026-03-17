# Kit V1 Bug Check: Implementation Notes

Date: 2026-03-17

This document captures the current high-priority implementation issues found during a static review of the v1 app.

Review basis:

- `npm run build`
- `cargo check`
- source review across Vue stores/views/components and Tauri commands

Build status:

- Frontend build passes
- Rust compile check passes
- One CSS build warning is present

## Release Assessment

Current status: not ready for v1 release

Reason:

- one primary v1 flow is not rendered
- settings persistence is wired incorrectly
- set assignment is exposed in UI but not implemented end-to-end
- usage time-range controls are misleading

## Priority Order

1. Fix settings persistence contract mismatch
2. Mount and complete the `Add Skills` flow
3. Either implement set assignment fully or remove it from v1
4. Make usage filtering honest or functional
5. Fix broken Settings external-path actions
6. Remove or redesign invalid manifest editing surface
7. Clean up CSS selector warning

## Issue Register

### KIT-BUG-001

Severity: blocker

Title: Settings updates send partial data to a command that expects a full preferences object

Frontend:

- `src/stores/preferencesStore.ts`

Backend:

- `src-tauri/src/commands/bootstrap.rs`

Problem:

- The frontend calls `update_preferences` with a partial object such as `{ showArchived: true }`.
- The Rust command expects a full `Preferences` payload.
- Runtime updates to settings will fail once invoked from the UI.

Why it matters:

- Settings are core to first-run and ongoing app configuration.
- This breaks library root updates, editor command updates, default view updates, and archive visibility updates.

Recommended implementation:

- Choose one contract and make both sides match.
- Recommended: change the backend command to accept a partial update type and merge into stored preferences in Rust.
- Alternative: keep backend strict and have the frontend send the full preferences object every time.

Acceptance criteria:

- Changing any single preference from Settings succeeds without requiring unrelated fields.
- Existing preferences remain intact after a partial change.
- Invalid library roots still fail with a clear error.

Verification:

- Change `defaultView`
- Toggle `showArchived`
- Change `editorCommand`
- Change `libraryRoot`
- Restart app and confirm values persist

### KIT-BUG-002

Severity: blocker

Title: `Add Skills` is not mounted, so the primary v1 workflow does nothing visible

Relevant files:

- `src/components/domain/LocationHeader.vue`
- `src/views/LocationsView.vue`
- `src/views/LocationDetailView.vue`
- `src/components/domain/AssignmentSheet.vue`

Problem:

- Clicking `Add Skills` only updates assignment store state.
- `AssignmentSheet` is not rendered anywhere in the active locations flow.
- Users cannot access the core assignment UI.

Why it matters:

- This is the signature interaction of the app.
- Without it, the main product promise is not deliverable.

Recommended implementation:

- Mount `AssignmentSheet` at the locations workspace level so it can overlay the detail view reliably.
- Recommended placement: inside `LocationsView.vue`, alongside the main split-pane layout.
- Confirm sheet state survives navigation within the locations route if that is desired.

Acceptance criteria:

- Clicking `Add Skills` opens the sheet immediately.
- The sheet closes cleanly on cancel and after apply.
- The sheet always has access to the selected location detail.

Verification:

- Open a location
- Click `Add Skills`
- Confirm sheet appears
- Select items
- Cancel
- Re-open and apply

### KIT-BUG-003

Severity: major

Title: Set selection is exposed in the UI but not implemented in preview/apply

Relevant files:

- `src/stores/assignmentStore.ts`
- `src/components/domain/AssignmentSheet.vue`
- `src-tauri/src/commands/assignment.rs`

Problem:

- The UI supports selecting sets.
- The store tracks `selectedSetIds`.
- Preview and apply only send skill add/remove arrays.
- Rust commands do not accept set IDs.

Why it matters:

- Users can select a set and reasonably expect it to work.
- The current behaviour is misleading and incomplete.
- Sets are now a first-class product requirement, not an optional extra.

Recommended implementation:

- Implement sets end-to-end in v1.
- Keep sets JSON-backed and file-based so they remain CLI-friendly and repo-shareable.
- Add `setIdsToAdd` to preview/apply contracts.
- Add `setIdsToRemove` as well.
- Expand sets in Rust, not Vue.
- Preview should show expanded skill link and manifest effects.
- Apply should create the resulting skill links and manifest updates deterministically.
- Support both global sets and project-scoped sets stored in real JSON files.

Acceptance criteria:

- Selecting a set results in predictable preview output.
- Applying a set produces actual linked skills.
- Duplicate skills are ignored cleanly.
- The assigned set is persisted as JSON-backed state, not only in app memory.

Verification:

- Select one set with multiple skills
- Preview expanded result
- Apply and inspect location detail
- Re-open from CLI or raw files and confirm the same set definitions still work

### KIT-BUG-004

Severity: major

Title: Usage time-range control is cosmetic and does not affect backend results

Relevant files:

- `src/views/UsageView.vue`
- `src/stores/usageStore.ts`
- `src-tauri/src/commands/usage.rs`

Problem:

- The view allows switching between `7d`, `30d`, and `90d`.
- The store does not send the selected range to Rust.
- The backend always reports from the same `use_count_30d` field.

Why it matters:

- The UI implies analytics precision that does not exist.
- This undermines trust in the Usage screen.

Recommended implementation:

- Make a strict v1 choice:
  - either remove the segmented control and label the page as 30-day usage
  - or implement time-range-aware usage properly
- Recommended for v1: remove the fake control unless real ranged data already exists.

Acceptance criteria:

- The UI only shows controls that affect real output.
- Labels and backend behaviour match exactly.

Verification:

- If keeping ranges, confirm changing range changes payload and rendered results.
- If simplifying, confirm no range switch is visible and all copy says 30 days.

### KIT-BUG-005

Severity: major

Title: Settings editor test and app-data reveal use unexpanded `~` paths

Relevant files:

- `src/views/SettingsView.vue`
- `src-tauri/src/commands/external.rs`

Problem:

- Settings sends `~/.kit` to Tauri commands.
- Rust uses `PathBuf::from` and `exists()` directly.
- `~` is not expanded, so these actions fail on normal systems.

Why it matters:

- The Settings screen includes actions that appear to verify editor integration and reveal app data.
- Both currently give false negatives or fail unexpectedly.

Recommended implementation:

- Resolve the app data path in Rust from `dirs::home_dir()` or shared state helpers.
- Do not send shell shorthand paths from the frontend.
- Consider a dedicated command such as `get_app_data_path()` if the UI needs to display it.

Acceptance criteria:

- `Test` works against a real existing path.
- `Reveal` opens the real app data folder.
- UI displays the resolved absolute path instead of `~/.kit`.

Verification:

- Click `Test`
- Click `Reveal`
- Confirm both work on a clean machine

### KIT-BUG-006

Severity: medium

Title: `ManifestEntryEditor` contract does not match the backend command

Relevant files:

- `src/components/domain/ManifestEntryEditor.vue`
- `src-tauri/src/commands/manifest.rs`

Problem:

- The component behaves like a generic inline text editor.
- The backend command only supports action strings of `"add"` or `"remove"` for a skill entry.
- If this component is mounted, ordinary edits will fail.

Why it matters:

- This creates a latent bug in a component intended for lightweight manifest editing.
- The UI model and backend model disagree.

Recommended implementation:

- Decide what v1 manifest editing actually is.
- Recommended for v1:
  - remove this component from active use until a narrow manifest-editing model is defined
  - or replace it with explicit add/remove controls instead of free-form text editing

Acceptance criteria:

- No inline manifest editor is exposed unless its contract is correct.
- Any visible manifest edit action maps cleanly to backend behaviour.

Verification:

- If removed: confirm no broken manifest text editing surface remains
- If kept: confirm add/remove interaction works without free-form invalid states

### KIT-BUG-007

Severity: medium

Title: Invalid deep selector causes a CSS build warning and unreliable separators

Relevant files:

- `src/components/domain/SkillList.vue`

Problem:

- The production build reports:
  - `'deep' is not recognized as a valid pseudo-class`
- The rule `.section-group > :deep(*) + :deep(*)` is not valid for the current build tooling.

Why it matters:

- This is not a release blocker by itself.
- It does mean shipped styling may not match the intended grouped-list appearance.

Recommended implementation:

- Replace the selector with a supported scoped pattern.
- Prefer a simpler DOM structure if needed over relying on fragile deep selectors.

Acceptance criteria:

- `npm run build` completes without this CSS warning.
- Section row separators render consistently.

## Suggested Implementation Sequence

### Phase 1: unblock core product

- Fix `update_preferences`
- Mount `AssignmentSheet`
- Decide and implement or remove set support

### Phase 2: remove misleading UX

- Simplify or implement usage ranges
- Fix Settings external actions
- Remove or narrow manifest inline editing

### Phase 3: polish and harden

- Clean CSS warning
- Add error toasts/inline messages where runtime failures are currently silent
- Add targeted tests around contracts and assignment flows

## Suggested Test Coverage To Add

### Frontend

- preferences store update path with single-field changes
- assignment store preview/apply payloads
- locations flow showing `AssignmentSheet`
- usage view behaviour for chosen time range model

### Rust

- preferences merge/update behaviour
- assignment preview with skills
- assignment preview/apply with sets if implemented
- external path handling for app-data paths
- manifest update command contract

## Definition Of Done For Bug Pass

This bug pass is complete when:

- Settings changes persist successfully
- `Add Skills` opens and applies changes visibly
- Set behaviour is either fully implemented or removed
- Usage controls are honest
- Settings external actions work on a normal machine
- No dead or misleading manifest editing path remains
- Frontend build is free of the current deep-selector warning

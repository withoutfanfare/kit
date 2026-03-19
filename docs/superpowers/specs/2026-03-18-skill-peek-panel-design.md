# Skill Peek Panel — Design Spec

Date: 2026-03-18

## Problem

Skills appear in five different places across the app (location detail, set detail, usage view, assignment sheet, skills view), but clicking a skill has inconsistent behaviour: sometimes it navigates away, sometimes it does nothing. Users need a fast, non-disruptive way to see what a skill is from wherever they encounter it.

## Solution

A global slide-over panel that shows skill details when clicking any skill name outside of the dedicated `/skills` view. The panel slides in from the right edge of the app window, overlaying the current content without navigating away.

## Component: SkillPeekPanel

New file: `src/components/domain/SkillPeekPanel.vue`

Mounted once in `AppShell.vue`. Teleported to `body` for clean stacking.

### Content (top to bottom)

- Close button (X) top-right
- Skill name (heading) + archived badge if applicable
- Source path (monospace, truncated)
- Description/summary if present
- Linked locations (compact list of location labels)
- Included in sets (compact list of set names)
- Usage (30-day count)
- Actions: "Open in Editor" and "Reveal in Finder" buttons

### States

- **Loading**: subtle spinner centred in the panel while `get_skill_detail` is in flight
- **Error**: if the fetch fails (e.g. skill deleted from disk), show a brief message: "Could not load skill details" with a "Close" button. No actions are shown in this state.
- **Loaded**: full content as described above

### Visual treatment

- Width: 320px, anchored to right edge, full height below the toolbar
- Background: `var(--surface-panel)` with `var(--shadow-sheet)` on left edge
- Backdrop: `rgba(0, 0, 0, 0.08)` — subtle, lighter than SheetPanel's 0.25
- Slide animation: 200ms ease-out in, 150ms ease-default out
- Z-index: 250 (above SheetPanel's 200 so it works from inside AssignmentSheet)
- Close on: click outside, Escape key, X button
- The panel simply overlays content — no responsive breakpoint needed. The minimum app window width is 800px (set in `tauri.conf.json`), and the panel is a temporary overlay that the user dismisses quickly.

### Backdrop click propagation

The peek backdrop must call `event.stopPropagation()` on click so that dismissing the peek panel does not also trigger `SheetPanel`'s `@click.self` close handler when both are open simultaneously. Both are teleported to `body` with independent backdrops; without `stopPropagation`, closing the peek could inadvertently close the sheet behind it.

### Focus

Focus moves to the panel on open. On close, focus returns to the element that triggered the peek. This keeps keyboard navigation predictable without requiring full focus trapping.

## Store: useSkillPeekStore

New file: `src/stores/skillPeekStore.ts`

Uses Composition API setup syntax: `defineStore("skillPeek", () => { ... })`, consistent with all other stores in the project.

### State

- `peekSkillId: ref<SkillId | null>(null)`
- `detail: ref<SkillDetail | null>(null)`
- `isLoading: ref<boolean>(false)`
- `error: ref<string | null>(null)`
- `cache: ref<Record<SkillId, SkillDetail>>({})`

### Actions

- `peek(id: SkillId)` — sets `peekSkillId`, clears `error`, checks cache, fetches via `get_skill_detail` if needed. On failure, sets `error`.
- `close()` — sets `peekSkillId` to null, keeps cache

### Computed

- `isOpen` — `computed(() => peekSkillId.value !== null)`

### Cache

The cache is valid for the lifetime of the app session. It is cleared when `libraryStore.fetchItems()` is called (which happens on archive/unarchive and when the AssignmentSheet opens), ensuring the peek panel reflects current state after mutations.

## Integration points

### 1. LocationDetailView

`SkillList` already emits `@selectSkill` but nothing handles it. Wire both `SkillList` instances to call `skillPeekStore.peek(skillId)`.

### 2. SetDetailView — skill rows

Skill rows in the set's skills section gain a click handler calling `skillPeekStore.peek(skill.id)`. These rows already have hover styles.

### 3. SetDetailView — skill picker sheet

The "Add skills to set" `SheetPanel` lists available skills. Add an info icon button (same pattern as AssignmentSheet below) so users can peek at a skill before adding it. Clicking the row still adds the skill to the set.

### 4. UsageView

Replace `router.push` to `/skills/:skillId` with `skillPeekStore.peek(skillId)`. Users stay on the Usage page.

### 5. AssignmentSheet — library skills list

Add a small info icon button next to each skill name in the library skills list. Clicking the icon peeks at the skill. Clicking the row still toggles the checkbox. This avoids conflicting click targets on the same element.

### 6. AssignmentSheet — installed skills list

The "Installed" removable skills section also gets the info icon button. Same pattern: icon peeks, row toggles the remove checkbox.

### 7. SkillsView — no change

The dedicated skills page keeps its current full navigation to `/skills/:skillId` with the inspector panel. The peek panel is for everywhere else.

## Style change: SkillRow cursor

`SkillRow.vue` currently has `cursor: default`. Change to `cursor: pointer` so that clickable skill rows communicate their affordance visually.

## Files to create

- `src/stores/skillPeekStore.ts`
- `src/components/domain/SkillPeekPanel.vue`

## Files to modify

- `src/components/layout/AppShell.vue` — mount `SkillPeekPanel`
- `src/components/domain/SkillRow.vue` — change cursor to pointer
- `src/views/LocationDetailView.vue` — handle `@selectSkill` events
- `src/views/SetDetailView.vue` — add click handler to skill rows + info icon in skill picker
- `src/views/UsageView.vue` — replace `router.push` with `peek()`
- `src/components/domain/AssignmentSheet.vue` — add info icon button per skill in both library and installed sections

## No backend changes

The existing `get_skill_detail` Tauri command provides all the data needed. No new Rust commands required.

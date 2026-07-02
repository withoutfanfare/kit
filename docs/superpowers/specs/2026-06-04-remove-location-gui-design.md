# Remove Location from the GUI — Design

**Date:** 2026-06-04
**Status:** Approved

## Problem

The backend command (`remove_location`), the store action (`locationsStore.removeLocation`), and even a UI component (`LocationInspector.vue`) for removing a saved location all exist — but `LocationInspector` is orphaned (never rendered), so the GUI offers no way to remove a location.

Removal is non-destructive: it only unregisters the location from `~/.kit/state.json`. No project files or symlinks are touched, and a removed location can be re-added at any time.

## Decisions

- **Entry points (user-selected):** detail view header button, list-row action menu, and the resurrected `LocationInspector` panel.
- **Row trigger:** `SRowActionMenu` ("⋯" hover button) rather than a native right-click context menu — uses the design system, no new pattern.
- **Confirmation:** one shared `SConfirmDialog` (the established pattern from SetDetailView's Delete Set), `danger` styled.

## Design

One shared flow, three triggers:

1. **`src/composables/useRemoveLocation.ts`** — module-level `pendingRemoval = ref<SavedLocationSummary | null>` plus `requestRemoveLocation(loc)`. Mirrors the existing `showShortcutHelp` module-ref pattern in `useKeyboardShortcuts.ts`.

2. **`LocationsView.vue`** — renders the single `SConfirmDialog`:
   - Title: `Remove location?`
   - Message: `Kit will forget '<label>'. No files or symlinks in the project are touched.`
   - Confirm label: `Remove`, `danger`
   - On confirm: `locationsStore.removeLocation(id)`; if the active route is `/locations/<that id>`, `router.push("/locations")`; success toast. Failure → error toast.
   - Also renders `LocationInspector` in `SplitPaneLayout`'s `#inspector` slot when `locationsStore.selectedDetail` is set (mirrors SkillsView/SetsView).

3. **`LocationRow.vue`** — `SRowActionMenu` with a single danger action `Remove…` → `requestRemoveLocation(location)`. Click on the menu must not trigger row selection.

4. **`LocationDetailView.vue`** — `SButton` (secondary, sm) `Remove Location` in the dashboard header → `requestRemoveLocation`, mirroring SetDetailView's Delete Set button.

5. **`LocationInspector.vue`** — replace its direct, unconfirmed `removeLocation()` handler with `requestRemoveLocation(...)`. Sync and Open Manifest remain unchanged.

## Error handling

The store only mutates local state after a successful `invoke`, so a failed removal leaves the UI consistent. The dialog closes either way; failures surface as an error toast.

## Testing

No frontend test infrastructure exists in this project. Verification: `npx vue-tsc --noEmit`, then drive the running dev app — remove via each of the three entry points, confirm the row disappears, the route returns to `/locations`, and re-adding works.

## Noted trade-off

The inspector panel duplicates some information already shown in the detail dashboard (path, manifest status). Accepted — it is what the component was originally built for, and matches the sibling views' layout.

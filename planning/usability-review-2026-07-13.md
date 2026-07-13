# Kit usability review

Date: 13 July 2026

Scope: read-only review of the native macOS app at 1430 × 971 and 900 × 700. Screens reviewed: Locations, Skills, Sets, Compare, Changelog, Health, Settings, and Help.

## Summary

Kit already has the right basic shape for a desktop management tool: persistent navigation, a compact library list, and a detail workspace. The main usability problem is that important meaning is hidden behind terse icons, colours, and repeated controls. The interface becomes difficult to use at smaller window sizes because the fixed panes clip rather than adapt.

The best first release is not a redesign. Make the current structure adaptive, remove duplicated actions, and make status and next steps self-evident.

## Highest-impact changes

### 1. Make list/detail views adaptive

At 900 × 700, selecting a skill leaves most navigation and list context clipped. The detail remains laid out as if the window were much wider.

- Keep the current split view at wide sizes.
- Below a practical width, show either the list or detail, not both.
- Opening an item should push to a detail view with a clear Back action.
- Collapse the primary sidebar to icons or hide it behind a sidebar control before reducing the useful list width.
- Give the window a minimum size only as a final guard, not as the main solution.

### 2. Consolidate search

The permanent top search and the Skills/Sets list search compete with one another.

- Make the top field a genuine global search or command palette, with grouped results for locations, skills, and sets.
- Keep the list field only as a local filter and label it clearly, for example `Filter skills`.
- Add a visible shortcut hint such as `⌘K` to global search.
- Preserve the query when opening an item and returning to the list.

### 3. Explain status in place

Amber dots, issue counts, eye icons, plus icons, and labels such as `38` require the user to infer their meaning or visit Help.

- Use short text labels for important states: `Broken link`, `Missing`, `Local only`, `Assigned`, and `Unused`.
- Label counts where they appear: `15 issues`, `38 unused`, `22 recommendations`.
- Put a compact status legend beside the relevant filter, not only in Help.
- Reserve colour as reinforcement; do not use it as the only explanation.
- Add tooltips and accessible names to every icon-only action.

### 4. Give each location one action hierarchy

The selected location shows Sync and Remove both in the header and the right inspector. The duplicate controls make it unclear which area is authoritative.

- Keep one header toolbar: `Add skills` as the primary action, followed by Sync, Reveal, and a More menu.
- Put Remove Location in the More menu or the inspector's destructive section, but not both.
- Make the main next step respond to state, for example `Resolve 15 issues` when a location has problems.
- Keep the right inspector for metadata and secondary actions; allow it to collapse.

### 5. Turn recommendations into a decision flow

The location view presents 22 recommendations as a long list with the same explanation repeated. It tells the user what Kit found but offers no obvious way to act on the result.

- Group recommendations by detected technology.
- Allow multi-selection and provide one `Add selected` action.
- Explain why each recommendation is relevant only when the reason differs.
- Let users dismiss a recommendation so repeatedly rejected suggestions stop creating noise.

### 6. Make Health a triage workspace

The summary mixes units: healthy appears to count locations, while warnings and errors count issues. Repeated `Fix` buttons do not say what will happen.

- Label the units explicitly: `7 healthy locations`, `15 warnings`, `12 errors`.
- Make the summary blocks filters rather than decorative cards.
- Group repeated issues by location or cause.
- Replace generic `Fix` with the actual operation, such as `Remove broken link`.
- Support selecting safe repeated fixes and applying them together, with a preview and confirmation.
- Link each location name back to its detail view.

### 7. Make Changelog describe changes

The current Changelog reads like a recently modified inventory. Each row repeats the skill name as its subtitle and clicking a row opens the normal skill detail, so the user still cannot tell what changed.

- Show the change type and summary: added, removed, edited, linked, or unlinked.
- Include the affected locations and exact time.
- Open a before/after diff or an activity detail when a row is selected.
- If historical diffs are unavailable, rename the screen to `Recently modified` so it does not promise more than it contains.

## Medium-impact improvements

### Simplify navigation

Use four primary destinations: Locations, Library, Health, and Activity. Skills and Sets can be tabs within Library. Compare works better as a contextual action from Locations or multi-selection than as a permanent top-level destination.

### Reduce row-level control noise in Sets

Set rows show a drag handle, two reorder arrows, and remove on every line.

- Keep drag-and-drop plus keyboard-accessible Move Up/Down actions in a row menu.
- Reveal destructive controls on selection or hover while preserving keyboard access.
- Turn the small `Add` link into an `Add skills` button.
- Make `Add a description…` an explicit editable field with a visible Edit action.

### Improve first-run and empty states

Use one primary action in each empty state. The first-run path should be: choose a library, add a project, review the scan, then resolve issues. Avoid duplicating the add action in both the list footer and workspace empty state.

### Tighten Settings

- Offer System, Light, and Dark appearance choices rather than a Dark mode switch.
- Rename `Select Backup` to `Restore from Backup…` and explain that it changes current data.
- Constrain settings content width so labels and controls are not separated by the full window.

### Make Help contextual

The Help screen is a long reference page. Keep it, but add links to the relevant setting or screen and surface the small pieces of guidance where users need them. The status legend is the clearest example: it belongs beside status-heavy lists.

## Quick wins

1. Label every bare count and status dot.
2. Replace `Fix` with the operation it performs.
3. Remove duplicate Sync and Remove controls from location detail.
4. Rename Changelog to `Recently modified` until diffs exist.
5. Rename `Select Backup` to `Restore from Backup…`.

## Suggested implementation order

1. Adaptive list/detail navigation at smaller window sizes.
2. Status labels, count labels, tooltips, and clearer action copy.
3. Location action consolidation and Health grouping.
4. Search consolidation.
5. Actionable recommendations and meaningful activity history.
6. Navigation consolidation only after the workflows above are settled.

## Deliberately skipped

- No new dashboard: Locations is already a sensible default workspace.
- No design-system rewrite: the calm dark visual language is suitable; clarity and responsive behaviour are the problems.
- No new dependencies: the improvements can use the existing Vue, CSS, and Tauri structure.

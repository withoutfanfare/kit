# Design

<!-- impeccable:design-from-build seed=aa45fed1 -->

Recorded from the built interface, not from intentions. Where this disagrees
with the code, the code is right and this file is stale.

## The system

Built to the craft level of Linear, with Kit's own identity. The previous
"Panel Schedule" world was discarded: it dressed the app in a metaphor and
never detailed it. What replaced it is a product design system.

Four rules hold it together:

1. **One family.** Inter, everywhere. No display face in UI labels — that was
   the single loudest wrong note in the previous pass.
2. **Surfaces are alpha layers**, not solid greys. Every raised surface is the
   ground plus a white (or black) overlay, so nesting stays coherent.
3. **Every interactive thing has all its states**: default, hover, focus,
   active, disabled, selected. Half a set is what makes an interface feel cheap.
4. **Definition comes from rings and elevation**, not 1px hairlines everywhere.

### Colour

Kit's accent is the terracotta from its own app icon, so the app reads as
itself rather than as a copy of the tools it learned its craft from.

| Role | Light | Dark |
|---|---|---|
| Ground | `#F7F7F8` | `#0A0B0D` |
| Raised | `#FFFFFF` | `#101114` |
| Accent | `#B5622F` | `#E08A5F` |
| Text (4 steps) | `#16181C` → `#8B9098` | `#F2F3F5` → `#62666D` |

Warning is decisively yellower than the accent and danger decisively pinker,
so neither can be mistaken for brand colour. Colour never carries meaning
alone — every state has a glyph or a word.

**Light is declared before dark.** `:root` and `.dark` have the same
specificity, so whichever is written last wins. Reversing that order silently
disables dark mode.

### Type

Inter Variable, self-hosted. Fixed scale at a 1.15–1.2 ratio (11/12/13/14/16/
20/26/34), with 13px as the workhorse for rows, labels and controls. Variable
weights (510, 590) rather than 400/700 jumps. Negative tracking on display
sizes or they read loose. Tabular numerals wherever figures are compared.

### Space, radius, motion

4px base. Radius 3/4/6/8/12 — 6px for controls, 8px for panels. Transitions
120–240ms; motion conveys state, never decoration.

### Primitives

`.btn` (primary/secondary/ghost/danger, all states), `.input`, `.badge`,
`.row`, `.panel`, `.label`, `.meter`, `.skeleton`. Nothing in the app should
roll its own button. Every list row carries `.row` explicitly — a `.rows > li`
descendant selector was silently dropped by the CSS pipeline once already.

## Status

Built and carrying the world:

- `panel.css` (incl. the shared section/row vocabulary), `global.css`,
  `index.html` (direction contract)
- `PanelIcon`, `LinkStateMark`, `SkillStatusLegend`
- `SidebarNav` (the bus), `WindowToolbar` (rating plate + watcher lamp)
- `PanelView` (new; the front door), `LoadoutView` (the schedule)
- `LocationRow`, `LocationList`, `SkillRow`, `SkillList`, `SetRow`, `SetList`
- `LocationDetailView` (data plate), `SkillsView` (schedule rows, one state mark
  instead of seven pills), `HealthView` (filter strip, plate causes),
  `UsageView` (column heads), `ChangelogView`, `HelpView`, `OnboardingView`,
  `SettingsView`
- `SkillDetailView`, `SetDetailView` and the three inspectors
  (`SkillInspector`, `SetInspector`, `LocationInspector`)

Uppercase section labels were normalised to the plate treatment across fifteen
files, so the app has one heading voice rather than sixteen hand-rolled ones.

**The inspector rule.** An inspector carries only what the main pane cannot:
where the thing physically lives, its state, and the actions that act on the
file itself. It previously repeated the summary, linked locations, sets and
usage that the main pane already showed in full, so every detail screen said
everything twice and neither copy was authoritative.

Inheriting the world but not structurally reworked — they take the palette,
type scale, spacing and heading treatment, and read consistently, but their
composition is the previous design:

`CompareLocationsView`, `AssignmentSheet`, `SelectionPreview`, `IssueList`,
`LocationOverviewCard`, `LocationHeader`, `SkillPeekPanel`, `BulkAssignModal`,
`SkillDiffModal`, `GlobalSearchResults`, `ShortcutHelpOverlay`,
`ManifestEntryEditor`, `LibraryTabs`, `LinkedLocationsList`,
`UsageSummaryPanel`.

## One backend change

`DefaultView` gained a `Panel` variant and new installs default to it, because
the root route now honours the Default view preference instead of hard-coding a
destination — the setting previously changed nothing. Existing `state.json`
values still deserialise unchanged.

## Designing without the backend

`KIT_UI_FIXTURES=1 npm run dev` swaps the Tauri IPC for `src/dev/fixtureBackend.ts`
and serves the UI at `localhost:1420`. The fixture data is deliberately messy —
a broken link, a globally vetoed skill, a missing location, unverifiable account
packs — because designing against tidy data produces interfaces that cannot show
the truth. The alias is off unless the env var is set, so it cannot reach a
production build.

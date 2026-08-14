# Design

<!-- impeccable:design-from-build seed=aa45fed1 -->

Recorded from the built interface, not from intentions. Where this disagrees
with the code, the code is right and this file is stale.

## The world: Panel Schedule

Kit is drawn as a distribution board. That is not a metaphor laid over the top —
it is the artefact that already solves Kit's problem, and the mappings are
structural:

| Panel | Kit |
|---|---|
| Main panel | Global (`~/.claude/skills`) — feeds every session |
| Sub-panel | A project location |
| Circuit | A skill |
| Rated load | Token cost |
| Panel capacity | The session's context budget |
| Upstream breaker | `skillOverrides` — cuts the circuit whatever the sub-panel says |
| Lock-out tag | A skill a hook runs from; must not be thrown |
| Legend plate | Every label |
| Schedule card | The Loadout view |
| Hatched, off-schedule | Sources Kit cannot verify — outside every total |

**The one discipline:** take the panel's *grammar* (plate, rating, position,
tag, bus), never its *textures*. No bevels, no brushed metal, no screws, no
faux-industrial chrome. A reviewer who finds a gradient standing in for metal
has found a defect.

## Files

- `src/assets/panel.css` — the whole world: palette (light + dark), type,
  spacing, motion, and the panel's parts. Redefines the `--color-*` layer that
  `@stuntrocket/ui` and every Kit component read.
- `src/assets/global.css` — imports the above, plus short-name aliases.
- `@stuntrocket/ui` is **never modified**; Kit overrides locally only.

## Colour

Restrained: neutrals plus one accent. The visitor came to operate.

| Role | Light | Dark |
|---|---|---|
| Ground | `#D9DCD8` cool machine grey | `#141715` panel enamel |
| Surface | `#E7E9E5` | `#1C201D` |
| Accent (oxide green) | `#2F5D45` | `#6FA98A` |
| Bus (copper) | `#8A5524` | `#C2793A` |
| Caution / lock-out | `#8A6210` | `#D2A044` |

**Never warm the ground.** Cream or paper tones read as stationery, not panel;
the ground is deliberately cool in both themes.

**Copper is structure, never decoration.** It appears on the sidebar bus, the
Global main-panel mark, the watcher lamp, and the project's segment on the load
strip. It is never a button, a link, or an accent for emphasis.

**The load ramp** (`--load-1` … `--load-6`) is hand-picked, not mixed: six
sources sit side by side on one strip and a generated ramp collapses into mud in
the middle. Global is always `--load-1` and the current project always
`--load-2` (copper), so those two keep their identity across every screen
whatever the sort order; everything else walks down the ramp in rank order.

**Colour never carries meaning alone.** Every state has a glyph or a word
beside it. See `LinkStateMark.vue`.

## Type

- **Interface:** the system face (`-apple-system` / SF). This has to read as a
  Mac app; SF is what a Mac app is set in.
- **Plates and ratings:** Archivo Narrow, self-hosted via `@fontsource`,
  uppercase, tracked `0.13em`, small. This is how engraved legend plates and
  rating stamps are lettered. No font CDN — a desktop app must not depend on one.
- **Numerals are tabular everywhere.** Ratings, counts and positions are read
  down columns and compared.
- Scale is deliberately small (`--text-md` is 12.5px). A tool is read at arm's
  length on a large display, and every extra pixel of leading costs a row.
- `--text-rating` (38px) is the only genuinely large type in the app, used for
  the session draw figure. Everything else on that screen qualifies it.

## The panel's parts

Defined in `panel.css`, used everywhere:

- `.plate` — engraved legend plate: inset ink on a cut face. Not a bevel.
- `.plate-bare` — the same lettering with no face, for quiet section markers.
- `.rating` / `.rating-unit` — a load figure and its unit.
- `.position` — the number stamped beside a breaker. A schedule's positions are
  its addresses, so the sequence genuinely carries information (this is the one
  earned exception to "no section numbers").
- `.tag-lockout` — the one saturated element. Hung on a circuit that must not be
  thrown.
- `.bus` — copper spine.
- `.unsurveyed` — diagonal hatching. Anything inside it is outside every total.
- `.rule` / `.rule-strong` — hairlines at two weights.

## Components

- `PanelIcon.vue` — the whole icon set, one weight (1.25 stroke on a 16 grid),
  one cap style. Each is the schematic symbol for its thing: the loadout icon is
  a bus with branches, lock-out is a padlock on a hasp. No unicode glyphs, no
  emoji, no mixed fills.
- `LinkStateMark.vue` — how link state is drawn everywhere. Glyph is the signal,
  colour agrees, label says it in words.
- `SkillRow.vue` — carries the breaker: a quiet housing with a handle that moves.
  The housing stays neutral; a column of saturated blocks is the loudest thing
  on a screen and says nothing the handle's position doesn't.

## Motion

One authored moment per surface, damped and physical — the way a switch throws,
never a spring. The load strip energises left to right on the Panel; the
sidebar's branch tick draws when current reaches it. `--ease-default` and
`--ease-out` only; **there is no overshoot easing in this system** and adding one
is a defect. All motion is disabled under `prefers-reduced-motion`.

## Corners and depth

Corners are near-square (`--radius-md` is 3px): a panel is cut, not moulded.
Panels sit flat. Only things that genuinely float — sheets, menus, toasts —
cast a shadow, and shadows carry both offset and blur.

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

# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

Delivered as a Tauri v2 desktop application for macOS. The rendering surface is
a webview, so web design rules apply, but the product must read as a Mac app:
window chrome, keyboard conventions, and density follow desktop expectations
rather than page expectations.

## Users

Developers who use Claude Code and have accumulated more skills than they can
hold in their head. The primary user runs several projects at once and keeps a
central library of skill folders that different projects draw on.

Confirmed: Kit is being built as a real product for other developers, not as a
single-operator tool. A second user arriving without a walkthrough must be able
to understand what a library, a location, a set and a link state are from the
interface itself. Onboarding, Help and empty states carry real product weight.

## Product Purpose

Kit answers "what is actually loading in this project, and what is it costing
me?" and then lets the user change the answer safely.

Claude Code loads skills from folders and symlinks scattered across a machine:
a global folder that applies to every session, per-project folders, installed
plugins, and account-level packs. The truth is spread over the filesystem and
several JSON files, and the failure modes are silent — a broken symlink, a
skill switched off globally while a project still links it, a manifest naming a
skill that no longer exists. None of it surfaces until a session behaves oddly.

Success is a user who can see the real loadout in seconds, trust the number,
and make a change without hand-editing symlinks or JSON.

## Positioning

Kit reports what will actually load, not what is on disk. It applies Claude
Code's own resolution rules — the global veto in `skillOverrides`, project
folders layering on top of global ones, installed plugin versions rather than
cached ones — and shows the result with an estimate of the context cost.

A file manager can show the folders. A terminal can show the symlinks. Neither
can tell you that a skill you can see is not loading, or that a skill you are
paying context for has not been used in three months. That resolution work,
plus honest reporting of what it cannot know, is the product.

## Operating Context

- **macOS desktop.** Light and dark, resizable window, transparent title bar.
  Runs alongside an editor and a terminal, frequently in a second window.
- **Two session shapes, roughly equally common.** A sub-minute visit to check
  or fix one thing, and a several-minute tidying session: reviewing what is
  linked where, pruning, assigning skills to a project. The design needs a
  glanceable layer that opens into real working surfaces.
- **A companion CLI (`kit`)** shares the same state, so the app is not the only
  way anything happens; the filesystem can change underneath it.
- **The user's real filesystem is the database.** State is derived by scanning
  on each visit. There is no server and nothing is cached between runs beyond
  `~/.kit/state.json`.
- **The stakes are asymmetric.** Adding a skill is trivially reversible.
  Removing one can break session start-up, and the breakage only appears in the
  *next* session, not at the moment of the click.

## Capabilities and Constraints

**Core objects.** Library (a directory of skill folders, each with a
`SKILL.md`), Locations (Global — `~/.claude/skills`, always first, never
removable — and Projects the user adds), Skills, Sets (named groups stored as
`*.set.json`), Manifests (a project's `.claude/settings.json`).

**Link states**, which the interface must distinguish: linked, local-only,
declared-only, broken link.

**Other sources that reach a session** but which Kit does not manage: installed
plugins, and account-level packs delivered by the desktop app. These are read
and reported, not controlled.

**Views:** Locations (+ detail), Skills (+ detail), Sets (+ detail), Loadout,
Usage, Health, Compare, Settings, Help, Changelog, Onboarding.

**Two rules the product enforces**, because both fail silently otherwise:
never write a manifest for Global (the neighbouring `settings.json` is the
user's live Claude Code configuration), and never unlink a skill a hook runs
from.

**Honest uncertainty is a feature.** Kit must distinguish "no data" from
"zero" everywhere it reports: absent usage logs are not proof a skill is
unused, and an unreadable settings file is not proof nothing is switched off.
Anything Kit cannot verify is shown as unverified rather than counted.

**Token cost is an estimate**, roughly four characters per token over each
skill's name and description. It must never be presented with false precision.

**Technical constraints.** Vue 3 (`<script setup>`), TypeScript, Pinia, Vue
Router, Tailwind v4, Vite. Rust backend over Tauri `invoke()`. All filesystem
work happens in Rust. `@stuntrocket/ui` supplies the component library and base
tokens and is shared with the author's other projects: Kit may redefine the
token layer locally, but must not change the shared library.

## Brand Commitments

Name: **Kit**. An existing app icon ships at `src-tauri/icons/`.

Voice, as established in the current interface copy and to be preserved: plain
British English, second person, short sentences. It says what a thing means
rather than naming it and moving on, and it admits what it does not know
("That's missing data, not zero use"). No exclamation marks, no cheerleading,
no invented certainty.

## Evidence on Hand

Real, in-repo: the README's concept documentation, a working CLI with its own
docs (`docs/CLI.md`), a changelog, and a test suite over the resolution rules.

Not on hand, and not to be fabricated: user counts, testimonials, screenshots
of other people's setups, performance benchmarks, pricing, or any claim about
adoption. Kit is not published as a signed app; installation currently requires
building from source and clearing a macOS Gatekeeper warning.

## Product Principles

1. **Report what loads, not what exists.** Every number on screen is the result
   of applying the runtime's real rules, or it is labelled as something less.
2. **Name the uncertainty.** Missing data is shown as missing. An estimate is
   shown as an estimate. Kit never rounds a gap up into a fact.
3. **Make the reversible easy and the irreversible deliberate.** Preview before
   apply; the destructive path is always the slower one.
4. **Teach the vocabulary in place.** Library, location, set, link state and
   context cost are unavoidable terms; the interface explains them where they
   are used rather than in a manual.
5. **Answer on arrival.** The user should learn the state of things before
   deciding anything, in both the sub-minute visit and the long tidy.

## Accessibility & Inclusion

Light and dark must both be first-class, following the system setting.
Meaning is never carried by colour alone — link state, health and conflict all
need a second cue. Keyboard operation matters: the app already ships shortcuts
and a shortcut help overlay, and the redesign must not regress them.

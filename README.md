# Kit

<p align="center">
  <img src="src-tauri/icons/128x128@2x.png" alt="Kit app icon" width="144" height="144">
</p>

**Skill loadout manager for Claude Code.**

Kit is a native macOS desktop app for managing Claude Code skills across a central library and multiple project locations. Instead of hand-editing symlinks and manifests, you get a Finder-style interface: see what's active, assign skills in seconds, and catch broken links before they bite.

There's also a companion command-line tool (`kit`) for scripting and terminal workflows, documented in [docs/CLI.md](./docs/CLI.md).

<!-- screenshot -->

## Contents

- [Quick start](#quick-start) — clone, run, and get the app into `/Applications`
- [Features](#features)
- [How it works](#how-it-works) — the concepts you need before assigning skills
- [Development](#development) — commands, project structure, architecture
- [Tech stack](#tech-stack)

---

## Quick start

**You need:** macOS, Node.js 18+, npm, and the Rust toolchain ([rustup.rs](https://rustup.rs)). The Tauri CLI comes in as a dev dependency, so you don't install it separately.

**1. Clone and install:**

```bash
git clone <repository-url>
cd kit
npm install
```

**2. Run it in development:**

```bash
npm run tauri dev
```

This starts the Vite frontend and the Tauri Rust backend together, and opens the app window.

**3. First-run setup.** Kit needs two things on first launch:

- **A skills library root:** the directory holding your skill folders. Kit tries to auto-detect this from your `~/.claude/skills/` symlink targets; if it can't, set it in Settings.
- **Your first project location:** a directory where Claude Code skills live (usually symlinks under `.claude/skills/`). Add it from the Locations view.

Editor command is optional and defaults to `code`.

### Install the app on your Mac

`npm run tauri dev` is for development. To run Kit as a normal app from your Applications folder, build it once and copy it across.

**1. Build the release app:**

```bash
npm run tauri build
```

The first build compiles the Rust backend and takes a few minutes. Later builds are faster.

**2. Copy Kit into Applications:**

```bash
cp -R "src-tauri/target/release/bundle/macos/Kit.app" /Applications/
```

Prefer the installer? Open `src-tauri/target/release/bundle/dmg/Kit_1.0.0_*.dmg` and drag **Kit** into the Applications folder shown in the window.

**3. Launch** from Spotlight or Launchpad.

**First launch warning.** The app isn't signed with an Apple Developer certificate, so macOS blocks it the first time. Right-click **Kit** in Applications, choose **Open**, then click **Open** in the dialogue box. One-time only.

To update, rebuild and copy over the old `Kit.app`.

### The typical workflow

1. Open **Locations** and select a project.
2. **Review the loadout:** which skills are linked, which are local-only, and any issues.
3. **Edit the loadout:** add or remove skills and sets, with a live preview of the changes.
4. **Apply:** Kit creates the symlinks, updates the manifest, and confirms the result.

---

## Features

- **Location loadouts:** see exactly which skills are active in each project, how they got there, and whether anything is broken.
- **Skill library:** browse, search, and archive skills from a single library root.
- **Sets:** group skills into reusable loadouts, scoped globally or per-project, stored as plain JSON that works with git and the CLI.
- **Safe assignment:** preview every symlink and manifest change before applying, with add and remove clearly separated.
- **Issue detection:** broken symlinks, missing declarations, undeclared links, and stale entries surfaced automatically.
- **Usage insights:** which skills get used most, which are unused, and suggestions for tidying up.
- **macOS-native feel:** transparent title bar, system fonts, light/dark mode, compact Finder-style layout.

---

## How it works

These are the concepts Kit is built around. Read this before your first assignment.

### Library

A single directory of skill folders. Each folder holds a `SKILL.md` with YAML frontmatter (`name`, `description`, `version`, `archived`). Kit scans this directory to build the skill catalogue.

### Locations

The project directories Kit tracks. Each has its own installed skills (symlinks or local folders) and a manifest. Kit scans a location to work out what's active and flag issues.

### Skills

The atomic unit: a folder with a `SKILL.md`. Within a location, a skill can be in one of four link states.

| Link state | Meaning |
|---|---|
| **Linked** | Valid symlink pointing to the library |
| **Local only** | Directory present, but not from the library |
| **Declared only** | Listed in the manifest, but no matching folder |
| **Broken link** | Symlink target no longer exists |

### Sets

Named groups of skills, stored as `*.set.json` files, in two scopes:

- **Global:** in `<libraryRoot>/sets/`, available everywhere.
- **Project:** in `<location>/.claude/sets/`, committed to the repo and shared via git.

Sets stay plain JSON, so the CLI can read them and you can edit them outside Kit.

### Manifests

Each location's `.claude/settings.json`. It holds `skills` and `sets` arrays declaring what should be active. Kit reads and writes this during assignment.

---

## Development

### Commands

| Task | Command |
|---|---|
| Run full app (frontend + backend) | `npm run tauri dev` |
| Frontend only (no Tauri shell) | `npm run dev` |
| Production build | `npm run tauri build` |
| Type-check frontend | `npx vue-tsc --noEmit` |
| Rust check | `cd src-tauri && cargo check` |
| Rust lint | `cd src-tauri && cargo clippy` |
| Rust tests | `cd src-tauri && cargo test` |

### Project structure

```text
kit/
├── src/                        # Vue 3 frontend
│   ├── assets/                 # CSS tokens and global styles
│   ├── components/
│   │   ├── base/               # Generic UI (buttons, badges, search)
│   │   ├── domain/             # Skill, location, and set components
│   │   └── layout/             # App shell, sidebar, panels, toolbar
│   ├── stores/                 # Pinia stores (one per domain)
│   ├── views/                  # Route-level views
│   ├── types/index.ts          # TypeScript types (mirrors Rust domain)
│   └── router.ts               # Vue Router config
├── src-tauri/                  # Rust backend
│   └── src/
│       ├── lib.rs              # Tauri builder and command registration
│       ├── state.rs            # Persisted state (~/.kit/state.json)
│       ├── domain.rs           # Shared types (serde camelCase)
│       ├── scanner.rs          # Filesystem scanning and frontmatter parsing
│       ├── linker.rs           # Symlink creation and removal
│       └── commands/           # Tauri command handlers by domain
├── planning/                   # Product specs and implementation notes
├── vite.config.ts
├── tsconfig.json
└── package.json
```

### Architecture notes

**Frontend and backend talk over Tauri's `invoke()` IPC.** Rust types use `#[serde(rename_all = "camelCase")]` so they serialise straight to the TypeScript types in `src/types/index.ts`. Those two files must stay in sync.

**All filesystem work happens in Rust:** symlink creation, manifest editing, directory scanning. The frontend never touches the filesystem directly.

**State lives in `~/.kit/state.json`,** loaded on startup. There's no database; Kit derives truth from the filesystem on each scan.

---

## Tech stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://v2.tauri.app) |
| Frontend | [Vue 3](https://vuejs.org) (Composition API, `<script setup>`) |
| State management | [Pinia](https://pinia.vuejs.org) |
| Routing | [Vue Router](https://router.vuejs.org) |
| Build tool | [Vite](https://vite.dev) |
| Backend | Rust |
| Styling | CSS custom properties with light/dark mode |

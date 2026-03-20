# Kit

**Skill loadout manager for Claude Code.**

Kit is a native macOS desktop app that manages Claude Code skills across a central library and multiple project locations. It replaces manual symlink juggling and manifest editing with a calm, Finder-style interface where you can see what's active, assign skills in seconds, and catch problems before they bite.

<!-- screenshot -->

---

## Features

- **Location loadouts** — see exactly which skills are active in each project, how they got there, and whether anything is broken
- **Skill library** — browse, search, and archive skills from a single library root
- **Sets** — group skills into reusable loadouts, scoped globally or per-project, stored as plain JSON files that work with git and the CLI
- **Safe assignment** — preview every symlink and manifest change before applying, with clear add/remove separation
- **Issue detection** — broken symlinks, missing declarations, undeclared links, and stale entries surfaced automatically
- **Usage insights** — see which skills are used most, which are unused, and get suggestions for tidying up
- **macOS-native feel** — transparent title bar, system fonts, light/dark mode, compact Finder-style layout

---

## Prerequisites

- **macOS** (the only supported platform for v1)
- **Node.js** (v18+) and **npm**
- **Rust** toolchain ([rustup.rs](https://rustup.rs))
- **Tauri CLI** — installed automatically via the project's dev dependencies
- A **skills library** — a directory of skill folders, each containing a `SKILL.md` with YAML frontmatter

---

## Installation

Clone the repository and install dependencies:

```bash
git clone <repository-url>
cd kit
npm install
```

---

## Getting Started

### Run the app

```bash
npm run tauri dev
```

This starts both the Vite frontend dev server and the Tauri Rust backend. The app window opens automatically.

### First-run setup

On first launch, Kit needs two things:

1. **Skills library root** — the directory containing your skill folders. Kit tries to auto-detect this from `~/.claude/skills/` symlink targets. If it can't, you'll set it in Settings.

2. **Your first project location** — a directory where Claude Code skills are installed (typically via symlinks in `.claude/skills/`). Add it from the Locations view.

Optionally, configure your preferred editor command in Settings (defaults to `code`).

### Typical workflow

1. **Open Locations** and select a project
2. **Review the loadout** — see which skills are linked, which are local-only, and any issues
3. **Edit the loadout** — add or remove skills and sets with a live preview of what will change
4. **Apply** — Kit creates symlinks, updates the manifest, and confirms the result

---

## Core Concepts

### Library

A single directory containing skill folders. Each skill folder has a `SKILL.md` file with YAML frontmatter (`name`, `description`, `version`, `archived`). Kit scans this directory to build the skill catalogue.

### Locations

Saved project directories that Kit tracks. Each location has its own set of installed skills (as symlinks or local folders) and a manifest file. Kit scans locations to determine what's active and detect issues.

### Skills

The atomic unit. A skill is a folder with a `SKILL.md` file. Skills in a location can be:

| Link state | Meaning |
|---|---|
| **Linked** | Valid symlink pointing to the library |
| **Local only** | Directory present but not from the library |
| **Declared only** | Listed in the manifest but no matching folder |
| **Broken link** | Symlink target no longer exists |

### Sets

Named groups of skills stored as `*.set.json` files. Sets come in two scopes:

- **Global** — stored in `<libraryRoot>/sets/`, available everywhere
- **Project** — stored in `<location>/.claude/sets/`, committed to the repo and shared via git

Sets remain plain JSON, readable by the CLI and editable outside Kit.

### Manifests

Each location's `.claude/settings.json` file. Contains `skills` and `sets` arrays declaring what should be active. Kit reads and writes these during assignment.

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

**Frontend–backend communication** happens via Tauri's `invoke()` IPC. Rust types use `#[serde(rename_all = "camelCase")]` so they serialise directly to the TypeScript types in `src/types/index.ts`. These two files must stay in sync.

**All filesystem operations** (symlink creation, manifest editing, directory scanning) happen in Rust. The frontend never touches the filesystem directly.

**State** is persisted to `~/.kit/state.json` and loaded on startup. There is no database — Kit derives truth from the filesystem on each scan.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop shell | [Tauri 2](https://v2.tauri.app) |
| Frontend | [Vue 3](https://vuejs.org) (Composition API, `<script setup>`) |
| State management | [Pinia](https://pinia.vuejs.org) |
| Routing | [Vue Router](https://router.vuejs.org) |
| Build tool | [Vite](https://vite.dev) |
| Backend | Rust |
| Styling | CSS custom properties with light/dark mode |

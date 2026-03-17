# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What is Kit?

Kit is a Tauri v2 desktop app for managing Claude Code skill loadouts. It tracks a library of skills (folders with SKILL.md files), lets users assign them to project locations via symlinks, group them into sets, and detect issues like broken links or manifest mismatches. State persists to `~/.kit/state.json`.

## Commands

```bash
# Development (starts both Vite frontend and Rust backend)
npm run tauri dev

# Build production app
npm run tauri build

# Frontend only (no Tauri shell)
npm run dev

# Type-check frontend
npx vue-tsc --noEmit

# Rust checks (from src-tauri/)
cd src-tauri && cargo check
cd src-tauri && cargo clippy
cd src-tauri && cargo test
```

## Architecture

**Tauri v2 app** with a Vue 3 frontend and Rust backend. No database — all state is filesystem-based (symlinks, JSON files, SKILL.md frontmatter parsing).

### Backend (src-tauri/src/)

- `lib.rs` — Tauri builder, registers all commands and plugins (shell, dialog, window-state)
- `state.rs` — `AppState` loaded from `~/.kit/state.json`, wrapped in `Mutex<AppState>` as Tauri managed state. Contains preferences, saved locations, and per-skill usage counters
- `domain.rs` — All shared types. Uses `#[serde(rename_all = "camelCase")]` so Rust snake_case maps to JS camelCase over the IPC bridge
- `scanner.rs` — Core logic: scans library root for skills (via SKILL.md frontmatter) and sets (`*.set.json`), scans locations for symlinks/issues, builds summaries. Hand-parses YAML frontmatter (no yaml crate)
- `linker.rs` — Symlink creation/removal with safety checks (verifies targets exist, refuses to delete non-symlinks)
- `commands/` — Tauri command handlers grouped by domain: `bootstrap`, `locations`, `library`, `sets`, `assignment`, `manifest`, `usage`, `external`

### Frontend (src/)

- **Vue 3 + Composition API + `<script setup>`**, Pinia stores, Vue Router, TypeScript
- `@` path alias maps to `./src/`
- `types/index.ts` — TypeScript mirrors of Rust domain types (must stay in sync)
- Stores call backend via `invoke()` from `@tauri-apps/api/core`
- `appStore` — bootstrap, global error/toast state
- `locationsStore` / `libraryStore` / `setsStore` / `assignmentStore` — domain CRUD with detail caches
- Components: `base/` (generic UI), `domain/` (skill/location/set-specific), `layout/` (shell, panels, toolbar)
- Views follow master-detail pattern with nested routes (e.g. `/locations/:locationId`)
- Design tokens in `assets/tokens.css` — CSS custom properties with light/dark mode via `prefers-color-scheme`. Targets macOS-native look (system-ui font, small text sizes)

### Key concepts

- **Library root** — a directory containing skill folders (each with SKILL.md). Auto-detected from `~/.claude/skills/` symlink targets, or configured in Settings
- **Location** — a saved project directory. Kit scans its `.claude/skills/` for symlinks back to library skills
- **Link states** — `linked` (valid symlink to library), `local_only` (dir, not from library), `declared_only` (in manifest but no file), `broken_link` (dead symlink)
- **Manifest** — `.claude/settings.json` in each location, contains `skills` and `sets` arrays
- **Sets** — groups of skills defined in `*.set.json` files. Global sets live in `<libraryRoot>/sets/`, project sets in `<location>/.claude/sets/`
- **Assignment** — the workflow of adding/removing skill symlinks and updating the manifest, with preview before apply

### IPC bridge

Rust types use `#[serde(rename_all = "camelCase")]` and enums use `#[serde(rename_all = "snake_case")]`. The TS types in `types/index.ts` must mirror the Rust `domain.rs` types exactly. When adding a new command, register it in `lib.rs`'s `invoke_handler`.

# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the Vue 3 frontend: `components/` is split into `base/`, `domain/`, and `layout/`; `views/` holds route-level screens; `stores/` contains Pinia state; `types/index.ts` mirrors backend domain types. `src/assets/` defines global styles and design tokens. `src-tauri/src/` contains the Rust backend, with Tauri commands grouped under `commands/` plus shared modules such as `domain.rs`, `scanner.rs`, `linker.rs`, and `state.rs`. Planning notes live in `planning/`.

## Build, Test, and Development Commands
Use `npm run tauri dev` to run the desktop app with the Vite frontend and Tauri shell. Use `npm run dev` for frontend-only work and `npm run build` for a production frontend build plus TypeScript checking. Backend verification lives in `src-tauri/`: run `cargo check`, `cargo clippy`, and `cargo test` from that directory. Use `npm run tauri build` when validating a distributable desktop build.

## Coding Style & Naming Conventions
TypeScript uses Vue Composition API with `<script setup lang="ts">`, double quotes, and 2-space indentation in Vue files. Name Vue components and views in PascalCase, for example `SkillInspector.vue`; keep stores and composables in camelCase files such as `appStore.ts`. Rust follows standard `rustfmt` conventions: snake_case modules/functions and grouped Tauri commands under `src-tauri/src/commands/`. Prefer small, domain-focused modules over generic helpers. Keep design changes aligned with `src/assets/tokens.css`.

## Testing Guidelines
There is no dedicated frontend test runner configured yet, so every change should at minimum pass `npm run build` and relevant Rust checks. For backend changes, run `cd src-tauri && cargo test`. When adding tests, place Rust tests near the affected module or in `src-tauri/tests/`; keep test names descriptive, for example `sync_location_ignores_missing_manifest`.

## Commit & Pull Request Guidelines
This repository currently has no local commit history, so there is no existing message format to copy. Use short, imperative subjects and keep them scoped, for example `feat: add set detail inspector` or `fix: guard broken symlink scan`. PRs should include a concise summary, linked issue or task when available, manual verification steps, and screenshots for UI changes.

## Agent-Specific Notes
The IPC boundary is strict: Rust types in `src-tauri/src/domain.rs` and TypeScript types in `src/types/index.ts` must stay in sync. When adding a new Tauri command, register it in `src-tauri/src/lib.rs` and update the relevant Pinia store or view together.

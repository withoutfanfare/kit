# Kit Roadmap Log

## Cycle: 2026-03-22 (implementation)
- **Items completed:**
  - [UX/UI] Add keyboard shortcuts for library and location navigation (P2, S)
  - [UX/UI] Add skill changelog showing recent modifications across the library (P2, S)
  - [Feature] Add bulk skill assignment to multiple locations in one operation (P2, S)
- **Items added:** none
- **Items archived:** none
- **Observations:** Three P2/S items implemented in a single batch. Keyboard shortcuts bring Kit in line with the portfolio standard. Changelog provides library-wide modification visibility. Bulk assignment eliminates repetitive per-location assignment for shared skills. Kit now has 4 pending functional items (diff viewer P2/S, location comparison P2/S, library backup P3/S, dependency resolution P3/L) + 3 design system items.

## Cycle: 2026-03-23 09:00
- **Items added:**
  - [Distribution] Add one-click skill library backup and restore for machine migration and disaster recovery (P3, S)
- **Items archived:** none
- **Observations:** Added one item filling the Distribution category gap — Kit had no pending Distribution items after the export/import feature was completed. Library backup/restore addresses a durability gap that the export/import feature only partially covers (individual sets vs full library state). Kit has 7 functional pending items + 3 design system = 10 total. The keyboard shortcuts (P2, S) and skill content diff viewer (P2, S) pair would deliver the highest daily-use improvement — shortcuts for navigation speed, diff viewer for informed skill update decisions.

## Cycle: 2026-03-23 03:00
- **Items added:**
  - [Feature] Add location skill comparison view showing assignment differences between locations (P2, S)
- **Items archived:** none
- **Observations:** Kit has 7 pending functional items + 3 design system = 10 total. Added location comparison to address a skill standardisation gap — users managing Claude Code skills across 10+ projects must manually switch between locations and visually compare assignments to ensure consistency. A side-by-side view showing unique and shared skills (with version difference flagging via the completed content hash tracking) would make loadout standardisation a one-screen operation. This complements the completed quick-assign and unused skill detection features by adding a comparison perspective. Bulk assignment (P2, S) and keyboard shortcuts (P2, S) remain the highest-priority items. The dependency resolution item (P3, L) remains tagged as "Needs review".

## Cycle: 2026-03-22 21:00
- **Items added:**
  - [Feature] Add bulk skill assignment to multiple locations in one operation (P2, S)
  - [UX/UI] Add keyboard shortcuts for library and location navigation (P2, S)
- **Items archived:** none
- **Observations:** Kit has 5 pending functional items + 3 design system = 8 total. Bulk assignment (P2, S) addresses the most repetitive workflow for users managing many locations — when a new universal skill is created, assigning it to 10+ locations one by one is tedious and error-prone. Keyboard shortcuts (P2, S) bring Kit in line with the portfolio standard — every other app has shortcuts implemented or planned, but Kit (which targets the most keyboard-driven audience: Claude Code power users) has none. Both are small and build on existing infrastructure (assignment workflow, existing UI patterns). The diff viewer (P2, S) and changelog (P2, S) form the strongest pair for skill evolution awareness. The dependency resolution item (P3, L, Needs review) remains the most speculative pending feature.

## Cycle: 2026-03-22 15:00
- **Items added:**
  - [UX/UI] Add skill changelog showing recent modifications across the library (P2, S)
- **Items archived:** none
- **Observations:** Kit has 3 pending functional items + 3 design system = 6 total. Added a library changelog (P2, S) to provide a macro-level view of skill evolution. The completed version tracking item flags per-assignment changes, and the diff viewer (P2, S) shows what changed in individual skills — but neither provides a library-wide "what's been modified recently" view. Skill authors iterating on multiple skills and project owners consuming from shared libraries need this temporal overview. The item builds on existing filesystem metadata and content hashes with no additional tracking overhead. The diff viewer (P2, S) and changelog (P2, S) together form a strong "skill evolution awareness" pair. The dependency resolution item (P3, L, Needs review) remains the most speculative item.

## Cycle: 2026-03-22 09:00
- **Items added:**
  - [UX/UI] Add skill content diff viewer comparing current version against assignment-time snapshot (P2, S)
- **Items archived:** none
- **Observations:** Kit has 2 pending functional items + 3 design system = 5 total pending. The existing functional item is skill dependency resolution (P3, L, Needs review). Added a content diff viewer (P2, S) to close the gap in the version tracking workflow: the completed version tracking item records content hashes and flags when skills change, but users can only see that something changed — not what. When a skill's SKILL.md is updated, users need to review actual content differences before accepting the change into their project. This extends state.json to store the assignment-time content snapshot alongside the hash. The diff viewer (P2, S) is now the highest-priority functional item, ahead of the dependency resolution feature (P3, L).

## Cycle: 2026-03-20 23:59
- **Items completed:**
  - [Quality] Health check dashboard for all locations (P2, M)
  - [Distribution] Skill library sharing via export/import (P3, M)
  - [Innovation] Project-type detection with skill recommendations (P3, M)
  - [UX/UI] Inline skill content preview in library view (P2, S)
  - [Performance] Filesystem watcher for live skill library updates (P2, M)
  - [UX/UI] Skill usage statistics visible on skill cards (P3, S)
  - [UX/UI] Drag-and-drop skill reordering within sets (P3, S)
  - [Feature] Quick-assign action for skills from library view (P2, S)
  - [UX/UI] Location dashboard showing health status (P2, S)
  - [Quality] Unused skill detection across all locations (P2, S)
  - [Feature] Skill version tracking with update notifications (P3, S)
- **Items archived:** none
- **Observations:** Eleven items completed in a single batch, clearing all pending functional items except the L-sized skill dependency resolution (tagged "Needs review") and the Design System Adoption section. Kit is now at 4 pending items (1 functional + 3 design system). Backend additions: health check scanner, project-type detector (20+ framework markers), skill content hashing (DJB2), filesystem watcher (notify + debouncer), export/import commands (zip crate). Frontend additions: HealthView with summary cards and severity-grouped issues, watcher store with auto-refresh on library-changed events, library store gains unused filter and sort-by-usage, SkillsView gets inline preview + quick-assign + usage badges, LocationDetailView gets dashboard header with health/scan/project-type badges and skill recommendations, SetDetailView gets drag-and-drop reordering with keyboard alt+arrow alternative.

## Cycle: 2026-03-19 08:00
- **Items added:**
  - [Quality] Fix build pipeline failures — Clippy warnings and DMG bundling (P1, S)
  - [Quality] Ensure project-scoped sets can be created from the UI (P1, S)
  - [Feature] Add skill dependency resolution and conflict detection (P3, L)
- **Items archived:** none
- **Observations:** Initial roadmap seeding. Kit is functionally complete but blocked by build pipeline failures (PRA-009) and a core UX gap in project-scoped set creation (PRA-001). These two P1 items must be resolved before the app can be shipped. The remaining 8 audit issues (set ID uniqueness, usage analytics writer, copy-to-clipboard) should be tracked in future cycles.

## Cycle: 2026-03-19 15:00
- **Items added:** none
- **Items archived:** none
- **Observations:** No new items added. Kit's build pipeline is broken (P1) and project-scoped sets are blocked (P1) — adding more items before these foundational blockers are resolved would dilute focus. The set ID uniqueness validation (PRA-002) is already captured in the acceptance criteria of the project-scoped sets item. The remaining audit issues (usage analytics writer, copy-to-clipboard, and others) should be added once the two P1s are cleared and the app can actually be built and tested.

## Cycle: 2026-03-21 08:00
- **Items added:** none
- **Items archived:** none
- **Observations:** Kit is at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. Two P1 items completed (build pipeline fix, project-scoped sets), unblocking the app for further development. The roadmap expanded significantly since the last log entry with items across all categories — skill search, health check dashboard, library sharing, project-type detection, inline preview, filesystem watcher, frontmatter validation, usage statistics, set integrity validation, drag-and-drop reordering, and quick-assign. Reviewed P3 items: skill dependency resolution (L, "Needs review" tag) is the most speculative — if the "Needs review" conclusion is that it's not worth pursuing, it should be the first archival candidate to make room. The P2 cluster (skill search, health check, inline preview, filesystem watcher, frontmatter validation, set integrity, quick-assign) forms a strong execution batch.

## Cycle: 2026-03-20 20:00
- **Items added:**
  - [UX/UI] Add location dashboard showing assigned skill count, health status, and last scan time (P2, S)
  - [Quality] Add unused skill detection across all locations (P2, S)
  - [Feature] Add skill version tracking with update notifications when library skills change (P3, S)
- **Items archived:** none
- **Observations:** Five items completed (build pipeline, project-scoped sets, skill search, SKILL.md validation, set integrity validation), showing strong execution momentum. The three additions address library management maturity: the location dashboard (P2, S) provides instant situational awareness when selecting a location, unused skill detection (P2, S) helps maintain a clean library by identifying orphaned skills, and version tracking (P3, S) alerts users when assigned skills change — preventing silent behaviour changes from updated SKILL.md files. All three are small (S) and build on existing scanner infrastructure. Kit is now at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. The P2 cluster (health check, inline preview, filesystem watcher, quick-assign, location dashboard, unused detection) is the strongest functional batch.

## Cycle: 2026-03-21 14:00
- **Items added:** none
- **Items archived:** none
- **Observations:** Kit remains at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. Five completed items (build pipeline, project-scoped sets, skill search, SKILL.md validation, set integrity validation) show strong execution momentum. The P2 cluster (health check, inline preview, filesystem watcher, quick-assign, location dashboard, unused detection) forms a cohesive "library management maturity" batch. Recommend starting with inline skill content preview (P2, S) as it directly improves the browsing and assignment workflow that users encounter most frequently. The skill dependency resolution item (P3, L, "Needs review") remains the most speculative item — should be validated before committing the L-sized effort. No additions until execution reduces the pending count.

## Cycle: 2026-03-20 08:14
- **Items added:** none
- **Items archived:** none
- **Observations:** Kit remains at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. Five completed items (build pipeline, project-scoped sets, skill search, SKILL.md frontmatter validation, set integrity validation) show strong execution momentum. Reviewed P3 items for archival: skill dependency resolution (L, "Needs review" tag) is the most speculative and largest non-design-system item — it depends on skills declaring dependencies in frontmatter, which few currently do. If validated as not worth pursuing, it should be the first archival candidate. Other P3 items (library sharing M, project-type detection M, usage statistics S, drag-and-drop reordering S, version tracking S) are all defensible. The P2 cluster (health check, inline preview, filesystem watcher, quick-assign, location dashboard, unused detection) provides a cohesive "library management maturity" batch. Recommend starting with the inline preview (P2, S) as it directly improves the skill browsing and assignment workflow.

## Cycle: 2026-03-20 22:30
- **Items added:** none
- **Items archived:** none
- **Observations:** Kit remains at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. No new completions since last cycle. Five completed items show the strongest execution-to-pending ratio in the portfolio. The skill dependency resolution item (P3, L, "Needs review" tag) continues to be the prime archival candidate — it requires skills to declare dependencies in frontmatter, which is not yet a convention. Recommend the inline preview (P2, S) and quick-assign (P2, S) pair as the starting point — together they streamline the most common workflow: browsing skills and assigning them to a project. No additions until execution reduces the pending count.

## Cycle: 2026-03-20 20:30
- **Items added:** none
- **Items archived:** none
- **Observations:** Kit remains at 15 pending items (12 functional + 3 design system) — at the rebalancing threshold. Five completed items (build pipeline, project-scoped sets, skill search, SKILL.md validation, set integrity validation) show the strongest execution-to-pending ratio. The skill dependency resolution item (P3, L, "Needs review") remains the prime archival candidate for a future cycle when space is needed. The inline preview (P2, S) and quick-assign (P2, S) pair remains the recommended starting point for the next development session. No additions until execution reduces the pending count.

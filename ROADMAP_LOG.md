# Kit Roadmap Log

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

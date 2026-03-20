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

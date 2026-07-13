<script setup lang="ts">
import { linkStateLabels } from "@/utils/statusLabels";
</script>

<template>
  <div class="help-view">
    <h1 class="page-title">Help</h1>
    <p class="page-subtitle">Learn how to use Kit to manage your Claude skills</p>

    <section class="settings-section">
      <h2 class="section-title">Start here</h2>
      <nav class="settings-group start-links" aria-label="Start here">
        <RouterLink to="/locations" class="start-link">
          <span class="help-title">Locations</span>
          <span class="help-description">Add and review projects</span>
        </RouterLink>
        <RouterLink to="/skills" class="start-link">
          <span class="help-title">Library</span>
          <span class="help-description">Browse skills and sets</span>
        </RouterLink>
        <RouterLink to="/health" class="start-link">
          <span class="help-title">Health</span>
          <span class="help-description">Find and resolve issues</span>
        </RouterLink>
        <RouterLink to="/settings" class="start-link">
          <span class="help-title">Settings</span>
          <span class="help-description">Configure Kit</span>
        </RouterLink>
      </nav>
    </section>

    <!-- Getting Started -->
    <section class="settings-section">
      <h2 class="section-title">Getting started</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">What is Kit?</span>
          <p class="help-description">
            Kit is a skill loadout manager for Claude Code. It helps you see which
            skills are active in each project, add or remove skills safely, and spot
            issues like broken links or missing declarations.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Setting up</span>
          <p class="help-description">
            Choose your skills repository in <RouterLink to="/settings" class="help-link">Settings</RouterLink>,
            then add the project folders where skills are linked in
            <RouterLink to="/locations" class="help-link">Locations</RouterLink>.
          </p>
        </div>
      </div>
    </section>

    <!-- Locations -->
    <section class="settings-section">
      <h2 class="section-title">Locations</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">What are locations?</span>
          <p class="help-description">
            Locations are project folders where you work with Claude. Kit scans each
            location's <code>.claude/skills/</code> directory to discover which skills
            are linked, local-only, or broken.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Adding a location</span>
          <p class="help-description">
            Open <RouterLink to="/locations" class="help-link">Locations</RouterLink>,
            choose Add Location and select a project folder. Kit scans it and shows
            the current skill loadout.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Skill states</span>
          <ul class="help-description status-list">
            <li><strong>{{ linkStateLabels.linked }}</strong> &mdash; linked from the library.</li>
            <li><strong>{{ linkStateLabels.local_only }}</strong> &mdash; present outside the library.</li>
            <li><strong>{{ linkStateLabels.declared_only }}</strong> &mdash; declared but not installed.</li>
            <li><strong>{{ linkStateLabels.broken_link }}</strong> &mdash; linked to a target that cannot be found.</li>
          </ul>
        </div>
        <div class="help-row">
          <span class="help-title">Issues</span>
          <p class="help-description">
            <RouterLink to="/health" class="help-link">Health</RouterLink> groups
            missing skills, broken links and manifest issues, with a review step before
            actions that change files.
          </p>
        </div>
      </div>
    </section>

    <!-- Library -->
    <section class="settings-section">
      <h2 class="section-title">Library</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Skills and sets</span>
          <p class="help-description">
            Use the Library's <RouterLink to="/skills" class="help-link">Skills</RouterLink>
            and <RouterLink to="/sets" class="help-link">Sets</RouterLink> tabs to
            browse or search each type.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Skill detail</span>
          <p class="help-description">
            Select a skill in the <RouterLink to="/skills" class="help-link">Skills tab</RouterLink>
            to see its linked locations, sets and usage over the past 30 days.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Archiving</span>
          <p class="help-description">
            Archive skills you no longer actively use. Archived skills are hidden by
            default; change that in <RouterLink to="/settings" class="help-link">Settings</RouterLink>.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Recently modified</span>
          <p class="help-description">
            <RouterLink to="/changelog" class="help-link">Recently modified</RouterLink>
            lists SKILL.md files by modification time and shows affected locations.
          </p>
        </div>
      </div>
    </section>

    <!-- Managing Skills -->
    <section class="settings-section">
      <h2 class="section-title">Managing Skills</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Adding skills to a project</span>
          <p class="help-description">
            Choose a project in <RouterLink to="/locations" class="help-link">Locations</RouterLink>
            and select Add skills. Search or select several skills and sets, then
            review the preview before applying the changes.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Removing skills</span>
          <p class="help-description">
            Open Add skills for the location, select installed skills to remove and
            review the preview before applying the changes.
          </p>
        </div>
      </div>
    </section>

    <!-- Keyboard & Tips -->
    <section class="settings-section">
      <h2 class="section-title">Keyboard &amp; Tips</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Keyboard shortcuts</span>
          <p class="help-description">
            Press <kbd>&#8984;/</kbd> to open the current shortcut reference.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Settings</span>
          <p class="help-description">
            Use <RouterLink to="/settings" class="help-link">Settings</RouterLink> to
            choose the skills repository and editor, show archived skills, change
            appearance, or back up and restore the library.
          </p>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.help-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow-y: auto;
  padding: var(--space-5) var(--space-6);
}

.page-title {
  font-family: var(--font-sans);
  font-size: var(--text-xl);
  font-weight: var(--weight-semibold);
  color: var(--text-primary);
  margin: 0 0 var(--space-1);
}

.page-subtitle {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  margin: 0 0 var(--space-6);
  line-height: 1.5;
}

/* Sections — mirrors SettingsView */
.settings-section {
  margin-bottom: var(--space-6);
}

.section-title {
  font-family: var(--font-sans);
  font-size: var(--text-xs);
  font-weight: var(--weight-semibold);
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  margin: 0 0 var(--space-2);
}

.settings-group {
  background: var(--surface-panel);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.start-links {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 1px;
  background: var(--border-subtle);
}

.start-link {
  min-width: 0;
  padding: var(--space-3) var(--space-4);
  background: var(--surface-panel);
  text-decoration: none;
}

.start-link:hover {
  background: var(--surface-hover);
}

.start-link:focus-visible,
.help-link:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: -2px;
}

/* Help rows */
.help-row {
  padding: var(--space-3) var(--space-4);
}

.help-row + .help-row {
  border-top: 1px solid var(--border-subtle);
}

.help-title {
  display: block;
  font-family: var(--font-sans);
  font-size: var(--text-md);
  font-weight: var(--weight-medium);
  color: var(--text-primary);
  margin-bottom: 4px;
}

.help-description {
  font-family: var(--font-sans);
  font-size: var(--text-sm);
  color: var(--text-secondary);
  line-height: 1.55;
  margin: 0;
}

.help-link {
  color: var(--accent);
  font-weight: var(--weight-medium);
  text-decoration: none;
}

.help-link:hover {
  text-decoration: underline;
}

.status-list {
  padding-left: var(--space-4);
}

.status-list li + li {
  margin-top: 2px;
}

.help-description kbd {
  font-family: var(--font-mono, monospace);
  font-size: var(--text-xs);
  background: var(--surface-hover);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  padding: 1px 4px;
  color: var(--text-primary);
}

.help-description code {
  font-family: var(--font-mono, monospace);
  font-size: var(--text-xs);
  background: var(--surface-hover);
  padding: 1px 4px;
  border-radius: var(--radius-sm);
  color: var(--text-primary);
}

@media (max-width: 720px) {
  .start-links {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>

<script setup lang="ts">
</script>

<template>
  <div class="help-view">
    <h1 class="page-title">Help</h1>
    <p class="page-subtitle">Learn how to use Kit to manage your Claude skills</p>

    <!-- Getting Started -->
    <section class="settings-section">
      <h2 class="section-title">Getting Started</h2>
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
            Start by setting your skill library root in Settings. This is the folder
            where your master skill files live (each skill has its own folder with a
            SKILL.md file). Then add project locations &mdash; these are the folders
            where skills get symlinked.
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
            Click "Add Location" in the sidebar and choose a project folder. Kit will
            scan it and show you the current skill loadout.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Skill states</span>
          <p class="help-description">
            Green dot = linked from library. Blue dot = local only. Amber dot =
            declared but missing. Red dot = broken link.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Issues</span>
          <p class="help-description">
            Kit detects problems like broken symlinks, skills declared in the manifest
            but not installed, and linked skills missing from the manifest. Each issue
            has action buttons to resolve it.
          </p>
        </div>
      </div>
    </section>

    <!-- Skills -->
    <section class="settings-section">
      <h2 class="section-title">Skills</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Skill library</span>
          <p class="help-description">
            The Skills view shows everything in your library root. You can filter by
            skills, sets, or search by name.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Skill detail</span>
          <p class="help-description">
            Select a skill to see where it's used across your projects, which sets
            include it, and usage statistics.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Archiving</span>
          <p class="help-description">
            Archive skills you no longer actively use. Archived skills are hidden by
            default but can be shown via Settings.
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
            Open a location, click "Add Skills". The assignment sheet lets you search
            and multi-select skills and sets. The preview panel shows exactly what will
            change before you apply.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Removing skills</span>
          <p class="help-description">
            In a location's skill list, use the issue actions or the assignment sheet
            to unlink skills.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Sets</span>
          <p class="help-description">
            Sets are named groups of skills. Adding a set installs all its constituent
            skills at once. Sets are defined as <code>.set.json</code> files in your
            library root.
          </p>
        </div>
      </div>
    </section>

    <!-- Usage -->
    <section class="settings-section">
      <h2 class="section-title">Usage</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Usage tracking</span>
          <p class="help-description">
            The Usage view shows which skills are most active, recently used, and
            unused. This helps you identify skills to archive or promote.
          </p>
        </div>
      </div>
    </section>

    <!-- Keyboard & Tips -->
    <section class="settings-section">
      <h2 class="section-title">Keyboard &amp; Tips</h2>
      <div class="settings-group">
        <div class="help-row">
          <span class="help-title">Refresh</span>
          <p class="help-description">
            Kit scans the filesystem on each view. Close and reopen a location to
            refresh its state.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Open externally</span>
          <p class="help-description">
            Use "Open in Editor" or "Reveal in Finder" buttons to jump to skill files
            or project folders in your preferred tools.
          </p>
        </div>
        <div class="help-row">
          <span class="help-title">Settings</span>
          <p class="help-description">
            Click the gear icon at the bottom of the sidebar to configure your library
            root, default editor, and display preferences.
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

.help-description code {
  font-family: var(--font-mono, monospace);
  font-size: var(--text-xs);
  background: var(--surface-hover);
  padding: 1px 4px;
  border-radius: var(--radius-sm);
  color: var(--text-primary);
}
</style>

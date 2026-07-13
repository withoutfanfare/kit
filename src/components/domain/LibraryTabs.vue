<template>
  <nav class="library-tabs" aria-label="Library sections">
    <RouterLink to="/skills" class="library-tab">Skills</RouterLink>
    <RouterLink to="/sets" class="library-tab">Sets</RouterLink>
  </nav>
</template>

<style scoped>
.library-tabs {
  display: flex;
  padding: var(--space-2) var(--space-3) 0;
  border-bottom: 1px solid var(--border-subtle);
  flex-shrink: 0;
}

.library-tab {
  flex: 1;
  padding: var(--space-2);
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--weight-medium);
  text-align: center;
  text-decoration: none;
}

.library-tab:hover {
  color: var(--text-primary);
}

.library-tab.router-link-active {
  border-bottom-color: var(--accent);
  color: var(--accent);
}

.library-tab:focus-visible {
  border-radius: var(--radius-sm);
  outline: 2px solid var(--accent);
  outline-offset: -2px;
}
</style>

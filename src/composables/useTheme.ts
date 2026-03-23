import { ref, watch } from "vue";

type Theme = "dark" | "light";

const STORAGE_KEY = "kit.theme";

const theme = ref<Theme>(loadTheme());

function loadTheme(): Theme {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === "light" || stored === "dark") return stored;
  } catch {
    // localStorage may be unavailable in some contexts
  }
  return "dark";
}

function applyTheme(value: Theme) {
  document.documentElement.classList.toggle("dark", value === "dark");
  document.documentElement.classList.toggle("light", value === "light");
}

// Apply on load
applyTheme(theme.value);

// Persist and apply on change
watch(theme, (value) => {
  applyTheme(value);
  try {
    localStorage.setItem(STORAGE_KEY, value);
  } catch {
    // Ignore storage errors
  }
});

export function useTheme() {
  function toggle() {
    theme.value = theme.value === "dark" ? "light" : "dark";
  }

  function setTheme(value: Theme) {
    theme.value = value;
  }

  return {
    theme,
    toggle,
    setTheme,
  };
}

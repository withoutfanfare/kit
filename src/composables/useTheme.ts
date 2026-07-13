import { computed, ref, watch } from "vue";

export type ThemePreference = "system" | "light" | "dark";
type ResolvedTheme = "light" | "dark";

const STORAGE_KEY = "kit.theme";
const systemTheme = window.matchMedia("(prefers-color-scheme: dark)");
const systemThemeIsDark = ref(systemTheme.matches);

const theme = ref<ThemePreference>(loadTheme());
const resolvedTheme = computed<ResolvedTheme>(() =>
  theme.value === "system"
    ? systemThemeIsDark.value ? "dark" : "light"
    : theme.value
);

function loadTheme(): ThemePreference {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === "light" || stored === "dark") return stored;
  } catch {
    // localStorage may be unavailable in some contexts
  }
  return "system";
}

function applyTheme(value: ResolvedTheme) {
  document.documentElement.classList.toggle("dark", value === "dark");
  document.documentElement.classList.toggle("light", value === "light");
}

systemTheme.addEventListener("change", (event) => {
  systemThemeIsDark.value = event.matches;
});

applyTheme(resolvedTheme.value);

watch(
  theme,
  (value) => {
    try {
      localStorage.setItem(STORAGE_KEY, value);
    } catch {
      // Ignore storage errors
    }
  },
  { immediate: true }
);

watch(resolvedTheme, applyTheme);

export function useTheme() {
  function setTheme(value: ThemePreference) {
    theme.value = value;
  }

  return {
    theme,
    resolvedTheme,
    setTheme,
  };
}

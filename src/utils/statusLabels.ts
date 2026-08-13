import type { LinkState } from "@/types";

/**
 * The four states a skill can be in at a location, in the user's language.
 *
 * The old labels described the plumbing — "Local only", "Broken link" — which
 * says what the filesystem looks like rather than what it means for you. Each
 * term now leads with the consequence and keeps one short line explaining it,
 * so the same words appear everywhere the state does.
 */
export type LinkStateTerm = {
  /** Short enough for a badge or a table cell. */
  label: string;
  /** One sentence, plain: what this actually means. */
  meaning: string;
};

export const linkStateTerms: Record<LinkState, LinkStateTerm> = {
  linked: {
    label: "From your library",
    meaning:
      "A shortcut to the shared copy in your library. Edit it once there and every project using it follows.",
  },
  local_only: {
    label: "Not in your library",
    meaning:
      "This skill exists only in this project. Move it into your library so other projects can use it — and so deleting the project doesn't lose it.",
  },
  declared_only: {
    label: "Listed but not installed",
    meaning:
      "This project's settings ask for the skill, but the folder isn't there, so it won't load.",
  },
  broken_link: {
    label: "Points at nothing",
    meaning:
      "A shortcut whose target has been moved, renamed or deleted. It loads nothing and is safe to remove.",
  },
};

/** Badge and legend text. */
export const linkStateLabels: Record<LinkState, string> = {
  linked: linkStateTerms.linked.label,
  local_only: linkStateTerms.local_only.label,
  declared_only: linkStateTerms.declared_only.label,
  broken_link: linkStateTerms.broken_link.label,
};

/** The hover explanation that goes with a label. */
export function linkStateMeaning(state: LinkState): string {
  return linkStateTerms[state].meaning;
}

const linkStateBadgeVariants: Record<
  LinkState,
  "success" | "warning" | "accent" | "error"
> = {
  linked: "success",
  declared_only: "warning",
  local_only: "warning",
  broken_link: "error",
};

export function linkStateBadgeVariant(state: LinkState) {
  return linkStateBadgeVariants[state];
}

/**
 * Terms that aren't link states but are just as opaque on first read. Kept
 * beside the others so the product has one vocabulary, not several.
 */
export const glossary = {
  global: {
    label: "Global",
    meaning:
      "The skills folder Claude Code reads in every session, whichever project you're in.",
  },
  library: {
    label: "Library",
    meaning:
      "Your master collection of skills. Nothing here loads on its own — projects point at it.",
  },
  set: {
    label: "Set",
    meaning: "A named group of skills you can add to a project in one go.",
  },
  contextCost: {
    label: "Context cost",
    meaning:
      "Every available skill puts its name and description into each session. This is the rough size of that, in tokens.",
  },
  commandOnly: {
    label: "Command only",
    meaning:
      "Runs when you type its slash command, but isn't offered to the model, so it costs nothing until you call it.",
  },
  override: {
    label: "Switched off globally",
    meaning:
      "A skillOverrides entry in your Claude Code settings. It beats every shortcut, so the skill stays off even where a project asks for it.",
  },
  accountPack: {
    label: "Account pack",
    meaning:
      "Skills delivered by the Claude desktop app. Kit can see the cached copy but can't switch these on or off.",
  },
} as const satisfies Record<string, LinkStateTerm>;

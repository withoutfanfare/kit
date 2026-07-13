import type { LinkState } from "@/types";

export const linkStateLabels: Record<LinkState, string> = {
  linked: "Assigned",
  declared_only: "Missing",
  local_only: "Local only",
  broken_link: "Broken link",
};

const linkStateBadgeVariants: Record<
  LinkState,
  "success" | "warning" | "accent" | "error"
> = {
  linked: "success",
  declared_only: "warning",
  local_only: "accent",
  broken_link: "error",
};

export function linkStateBadgeVariant(state: LinkState) {
  return linkStateBadgeVariants[state];
}

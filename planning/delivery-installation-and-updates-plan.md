# Kit Delivery, Installation, And Update Plan

Date: 2026-03-17

This plan covers how `Kit` should be delivered to internal users, how first-run setup should work, how the app should auto-update, and how it should detect that the shared skills repository has changed.

## 1. Goal

Make it genuinely easy for an internal teammate to:

- install `Kit`
- point it at their cloned skills repository
- get productive in under two minutes
- keep the app itself up to date
- notice when the shared skills repository has moved ahead

The app should not require terminal fluency for the happy path, but it must stay compatible with existing git and CLI workflows.

## 2. Assumptions

- Initial users are internal team members on macOS.
- They will clone the main skills repository themselves or via existing team setup docs.
- The skills repository may live in different places on different machines.
- The skills repository should remain a normal git repository on disk.
- Sets and other shareable files should remain file-backed and repo-friendly.

## 3. Product Decisions

### Decision 1: app distribution

`Kit` should ship as a signed and notarized macOS desktop app distributed outside the Mac App Store.

Recommended deliverables per release:

- signed `.dmg` for first-time install
- signed updater artifact for in-app updates
- GitHub release notes for internal visibility

### Decision 2: skills repo configuration

The app must not assume a fixed path for the main skills repository.

Instead:

- first run asks the user to locate the skills repository
- the chosen path is validated and saved
- the user can change it later in Settings

### Decision 3: app auto-update

Use Tauri 2’s updater plugin for app updates.

### Decision 4: skills repo update detection

Do not make a GitHub token the primary mechanism for repo freshness checks.

Recommended order:

1. Primary: inspect the local git clone and compare it with its configured remote
2. Optional fallback: GitHub API check using a user-provided fine-grained token stored securely

Reason:

- the repo is already cloned locally
- the local clone already knows its remote and branch
- many users will already have working git or SSH credentials
- forcing a token up front creates avoidable setup friction

## 4. Distribution Model

### Release channel strategy

Start simple:

- `stable`
- optional later `beta`

Internal team users should default to `stable`.

### Recommended release pipeline

1. Build signed macOS release artifacts in CI
2. Sign update bundles with Tauri updater signing key
3. Notarize the macOS app
4. Publish artifacts and release metadata
5. Publish updater manifest for the latest stable version

### Recommended hosting

Best practical options:

- dedicated private update endpoint or bucket
- or GitHub Releases plus a stable update manifest endpoint

Recommendation:

- keep installer downloads and release notes in GitHub Releases
- keep the updater manifest under a controlled HTTPS endpoint

This keeps release management familiar while avoiding overly coupling the updater flow to GitHub API quirks.

## 5. First-Run Installation Experience

### Happy path

1. User downloads `Kit.dmg`
2. User installs the app
3. User opens `Kit`
4. First-run setup asks for:
   - skills repository path
   - optional default editor command
   - optional first project location
5. App validates the skills repo and lands on the main workspace

### First-run screen design

The first-run experience should be one compact guided flow, not a wizard maze.

Recommended screens:

#### Step 1: Welcome

- short explanation of what `Kit` manages
- primary button: `Choose Skills Repository`

#### Step 2: Locate skills repository

- directory picker
- inline validation after selection
- detected information shown immediately:
  - valid git repo or not
  - expected skill structure found or not
  - branch name if available

#### Step 3: Confirm setup

- show chosen repo path
- show optional editor command
- optional button: `Add First Project`
- primary button: `Open Kit`

### Validation rules for skills repo

The selected directory must be checked for:

- directory exists
- readable
- appears to be a git repository
- contains expected skill structure

Validation should return plain-language messages such as:

- `This looks like a valid skills repository`
- `Git metadata was not found here`
- `No skills were found in this folder`

### Nice first-run touches

- offer to auto-detect common paths first
- remember the most recent successful path
- allow skipping project-location setup
- allow reopening onboarding from Settings

## 6. Ongoing Setup And Settings

Settings should include a dedicated `Skills Repository` section with:

- current path
- branch name
- repo status
- last checked time
- `Change Location`
- `Reveal in Finder`
- `Open in Editor`
- `Check for Repository Updates`

This should not be buried in generic advanced settings.

## 7. App Auto-Update Plan

### Recommendation

Implement app auto-update in v1 using Tauri 2 updater.

### Why this is viable

Based on current official Tauri 2 docs:

- updater support is available via the updater plugin
- update signatures are required and cannot be disabled
- updates can be served from static JSON or a dynamic endpoint
- custom request headers are supported, including `Authorization`

### Product behaviour

Recommended user experience:

- app checks for updates on launch and then once per day
- if an update exists:
  - show a quiet banner or Settings badge
  - let the user choose `Download and Install`
- do not force-install immediately
- after install:
  - prompt for restart when appropriate

### Update channel state in preferences

Add:

- update channel: `stable` by default
- auto-check enabled: `true` by default
- last app update check time
- skipped version if needed later

### Security requirements

- updater signing key managed outside the repo
- public verification key shipped in app config
- no embedded private update credentials inside the app bundle

### GitHub token for app updates?

Technically possible, but not recommended as the default.

Reason:

- Tauri updater can send authenticated requests
- but requiring every user to configure a GitHub token just to receive app updates is poor onboarding
- embedding a shared org token in the app would be a security mistake

Recommendation:

- do not require a user GitHub token for app auto-update in the default path
- use a controlled update endpoint for app artifacts instead
- only use authenticated headers if distribution absolutely must remain private and there is no better internal endpoint

## 8. Skills Repository Update Detection

This is different from app auto-update and should be treated as a separate feature.

### What the app should detect

For the configured skills repository, the app should be able to tell the user:

- repo is up to date
- local repo is behind remote
- local repo is ahead of remote
- local repo has uncommitted changes
- repo status could not be checked

### Recommended primary approach

Use the local git clone as the source of truth.

Recommended implementation:

- inspect `.git`
- read current branch and upstream
- run a fetch/update check with a timeout
- compare local HEAD to upstream
- surface ahead/behind counts

This is the right default because it reflects the actual checkout the user is working with.

### Why local git is better than a token-first GitHub API approach

- it works with existing SSH or credential-manager auth
- it respects whichever remote the user actually cloned
- it handles branches, forks, and internal remotes more naturally
- it avoids forcing another auth setup step

### Recommended product UX

Show repository freshness in one calm place:

- small repo-status indicator in Settings
- optional subtle banner when the repo is behind

Recommended copy:

- `Skills repository is up to date`
- `Skills repository is 4 commits behind origin/main`
- `Skills repository has local changes`

Recommended actions:

- `Reveal Repository`
- `Open in Editor`
- `Copy Pull Command`
- `Recheck`

Important:

- `Kit` should notify, not auto-pull
- automatic pulls are risky because they can overwrite user context or trigger merge conflicts

### Where to surface it

Primary surface:

- Settings > Skills Repository

Secondary surface:

- subtle top-level notice when the repo is behind and the app has successfully confirmed that status

Do not make this a loud dashboard widget.

## 9. Optional GitHub API Fallback

### When to use it

Only as an optional advanced fallback if:

- git is unavailable
- local auth is unavailable
- the team explicitly wants GitHub-based status checks

### Token model

If this fallback is implemented:

- use a fine-grained personal access token
- request the minimum repository read permissions needed
- store it in the OS keychain, not plain app config
- make it optional, never mandatory for first-run success

### What it can support

- checking the latest release for app update metadata
- comparing repo freshness against GitHub when the remote is GitHub-hosted

### What it should not do

- become the only supported repo-status path
- require an org-wide shared token
- be stored in the repo or app bundle

## 10. Backend Work Needed

### Preferences and onboarding

- store `skillsRepositoryPath`
- store `editorCommand`
- store `updateChannel`
- store `lastRepoCheckAt`
- store `lastAppUpdateCheckAt`

### Tauri commands

#### Onboarding and repo setup

- `validate_skills_repository(path) -> SkillsRepoValidation`
- `save_skills_repository(path) -> Preferences`
- `get_skills_repository_status() -> SkillsRepoStatus`
- `recheck_skills_repository_status() -> SkillsRepoStatus`

#### App updates

- `check_for_app_update() -> AppUpdateStatus`
- `download_and_install_app_update() -> UpdateInstallResult`

#### External actions

- `reveal_skills_repository() -> { ok: true }`
- `open_skills_repository_in_editor() -> { ok: true }`
- `copy_repo_pull_command() -> { command: string }`

### Suggested contract shapes

```ts
type SkillsRepoValidation = {
  valid: boolean
  path: string
  isGitRepo: boolean
  detectedBranch: string | null
  skillCount: number
  issues: string[]
}

type SkillsRepoStatus = {
  path: string
  branch: string | null
  upstream: string | null
  state: "up_to_date" | "behind" | "ahead" | "diverged" | "dirty" | "unavailable"
  aheadBy: number
  behindBy: number
  hasUncommittedChanges: boolean
  lastCheckedAt: string | null
  message: string
}

type AppUpdateStatus = {
  checkedAt: string
  available: boolean
  currentVersion: string
  latestVersion: string | null
  notes: string | null
}
```

## 11. Frontend Work Needed

### New UI areas

- first-run setup screen
- Settings > Skills Repository section
- update banner or Settings badge for app updates
- repo-status indicator and action row

### UX rules

- first-run setup must succeed without a GitHub token
- repo-update status should be informative, not alarming
- app-update prompts should be quiet and reversible
- errors should explain the likely cause:
  - missing git
  - auth problem
  - no upstream configured
  - invalid repo path

## 12. Recommended Implementation Phases

### Phase 1: onboarding foundation

- add configurable skills repo path
- add validation command
- build first-run setup flow
- surface repo path in Settings

### Phase 2: app delivery and updater

- integrate Tauri updater plugin
- add release signing and update manifest flow
- build in-app update check and install UX

### Phase 3: skills repo status

- implement local git-based freshness checks
- surface repo status in Settings
- add subtle behind-remote notification

### Phase 4: optional advanced fallback

- add optional GitHub token support only if needed
- store token in keychain
- support GitHub-based checks where local git is not viable

## 13. Open Questions

- Will the internal team always have git installed, or do we need a friendlier fallback?
- Is the skills repository private on GitHub, and if so, do all users already authenticate via SSH or GitHub Desktop?
- Do we want the app installer and updater artifacts to be private or simply internal-by-convention?
- Do we want one central internal update endpoint, or is GitHub Releases sufficient for v1 delivery?
- Should the app offer a `Clone Skills Repository` action later, or stay focused on selecting an existing clone?

## 14. Recommendation Summary

Recommended v1 approach:

- ship a signed, notarized macOS app
- use Tauri updater for app updates
- do not require GitHub tokens for first-run or normal updates
- let users choose the skills repo path during onboarding
- detect skills repo freshness via the local git clone
- notify users when the repo is behind, but never auto-pull
- keep GitHub token support as an optional advanced fallback only

## 15. External References

These recommendations were checked against current official documentation:

- [Tauri 2 updater plugin](https://v2.tauri.app/plugin/updater/)
- [GitHub REST releases API](https://docs.github.com/rest/releases/releases)
- [GitHub personal access tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token)
- [GitHub API credential security guidance](https://docs.github.com/en/rest/authentication/keeping-your-api-credentials-secure)

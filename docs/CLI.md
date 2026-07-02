# Kit CLI

**The one-sentence version:** Kit CLI applies named groups of skills from your skill library into a project's `.claude/skills/` folder, as symlinks.

That's it. Everything below is detail.

---

## The mental model

You have three things:

1. **A skill library** — a directory full of skill folders (each folder is one Claude Code skill).
   Typical location: `~/Ai/Assets/Claude/Skills/`.

2. **Sets** — plain text files listing skill names, one per line.
   A set is just a named loadout: *"these are the skills I want active together."*
   Typical location: `~/.kit/sets/my-default.txt`.

3. **Projects** — any folder where you want skills to be available.
   When you run Kit CLI, it creates symlinks at `<project>/.claude/skills/<skill-name>` pointing at the real skill folders in your library.

Kit CLI is the bridge between (1) and (3), driven by (2).

```text
Library                      Project
~/Ai/Assets/Claude/Skills/   ~/some-project/.claude/skills/
├── bug-triage/              ├── bug-triage/       ────╮
├── laravel-specialist/      ├── code-review/      ──╮ │
├── code-review/             └── laravel-specialist/─│─│─╮
├── vue-nuxt-specialist/                             │ │ │
├── ...lots more             Set: my-web-default.txt │ │ │
│                            ─────────────────────── │ │ │
│                            bug-triage    ──────────╯ │ │
│                            code-review    ───────────╯ │
│                            laravel-specialist ─────────╯
│
└──── (symlink targets) ←────────────────────────────────┘
```

The skills stay in the library. They're just *reachable* from the project via symlinks.

---

## The five commands

There are two groups: set-based (apply a whole loadout) and skill-based (add/remove individual skills).

**Set-based:** `kit apply`, `kit sets`
**Skill-based:** `kit link`, `kit unlink`
**Read-only:** `kit list`

For everyday per-project tweaking, `kit link` and `kit unlink` are the ones you'll reach for most.

### `kit sets` — list available sets

Shows every `.txt` file in your sets directory.

```console
$ kit sets
Sets directory: /Users/you/.kit/sets
  - my-web-default (4 skills) [/Users/you/.kit/sets/my-web-default.txt]
  - laravel-full (12 skills) [/Users/you/.kit/sets/laravel-full.txt]
```

### `kit apply --set <name> --project <path>` — symlink a set into a project

Creates symlinks in `<project>/.claude/skills/` for every skill in the named set.

```console
$ kit apply --set my-web-default --project ~/Development/my-new-app
  + laravel-specialist
  + vue-nuxt-specialist
  + code-review
  + test-writer
Applied set 'my-web-default' to /Users/you/Development/my-new-app: 4 symlinked, 0 missing.
```

Idempotent — safe to re-run. If a symlink already exists for one of the skills, Kit removes it and recreates it pointing at the library. Symlinks for skills *not* in the set are left alone.

If a skill listed in the set doesn't exist in your library, it's reported as missing and the command exits `1`. The other skills still get symlinked.

### `kit link <skill> [<skill> ...] --project <path>` — symlink individual skills

When you want *this one skill* in a project, without applying a whole set. Multiple skills in one invocation are fine.

```console
$ kit link laravel-specialist code-review --project ~/some/app
  + laravel-specialist
  + code-review
Added 2 skill(s) to /Users/you/some/app.
```

Idempotent — if a symlink for the skill already exists, it's replaced. Skills not in the library are reported as missing and the command exits `1`, but the ones that *are* in the library still get linked.

### `kit unlink <skill> [<skill> ...] --project <path>` — remove individual skills

The inverse of `link`. Deletes the symlink at `<project>/.claude/skills/<skill>`.

```console
$ kit unlink code-review --project ~/some/app
  - code-review
Removed 1 skill(s) from /Users/you/some/app.
```

**Safety:** Only removes entries that are actual symlinks. If something at that path is a real directory or file (e.g., someone hand-rolled it), `kit unlink` refuses to delete it and exits `1`. No `--force` flag in v1.

### `kit list --project <path>` — see what's active in a project

Shows active skills and (if present) the team badge file.

```console
$ kit list --project ~/Development/my-new-app
Project: /Users/you/Development/my-new-app

Team badge: (none — not registered)

Active skills:
  - laravel-specialist -> /Users/you/Ai/Assets/Claude/Skills/laravel-specialist
  - vue-nuxt-specialist -> /Users/you/Ai/Assets/Claude/Skills/vue-nuxt-specialist
  - code-review -> /Users/you/Ai/Assets/Claude/Skills/code-review
  - test-writer -> /Users/you/Ai/Assets/Claude/Skills/test-writer
```

---

## Flags and environment variables

Defaults are sensible on most setups — you usually don't need these.

| Flag (per-call) | Env var (persistent) | Default | What it sets |
|---|---|---|---|
| `--library PATH` | `KIT_LIBRARY` | Read from Kit GUI's `~/.kit/state.json`, or `~/.claude/skills/` | Where skill folders live |
| `--sets-dir PATH` | `KIT_SETS_DIR` | `~/.kit/sets/` | Where `.txt` set files live |
| `--json` | — | off | Emit JSON instead of human output |

---

## Worked examples

### Create your first set

```bash
mkdir -p ~/.kit/sets

cat > ~/.kit/sets/my-web-default.txt <<EOF
# My default web-development skills
laravel-specialist
vue-nuxt-specialist
code-review
test-writer
EOF

kit sets
# → should now list "my-web-default (4 skills)"
```

Comments (lines starting with `#`) and blank lines are ignored. Trailing whitespace is stripped.

### Put those skills into a project

```bash
kit apply --set my-web-default --project ~/Development/my-new-app
```

Now open Claude Code in that project — it'll pick up the symlinked skills.

### Inspect a project

```bash
kit list --project ~/Development/my-new-app
```

### Add a single skill to a project (no set needed)

The most common daily task. No need to define a set — just link the skill directly.

```bash
kit link laravel-debugger --project .
```

Multiple at once:

```bash
kit link laravel-debugger tailwind-system ui-design --project ~/Development/my-app
```

### Remove a single skill

```bash
kit unlink laravel-debugger --project .
```

### Update a set and re-apply

```bash
# Add another skill
echo "docs-updater" >> ~/.kit/sets/my-web-default.txt

# Re-run — idempotent, only new symlinks appear
kit apply --set my-web-default --project ~/Development/my-new-app
```

### Remove a project's skills entirely

Kit CLI doesn't have a `remove` command (yet). Just delete the folder:

```bash
rm -rf ~/Development/my-new-app/.claude/skills
```

The library isn't touched — only the symlinks.

### Machine-readable output

```bash
kit --json sets
kit --json apply --set my-web-default --project ~/Development/my-new-app
kit --json list --project ~/Development/my-new-app
```

Useful for piping to `jq` or invoking from other scripts.

---

## Relationship to the Kit app (GUI)

Kit CLI is a **companion** to the Kit macOS app that lives in this same repository. The app is the full feature set — browse your library, create sets visually, see which skills are assigned where, catch broken links, etc. The CLI is the minimum-viable subset for use from a shell.

| Concern | CLI | GUI |
|---|---|---|
| Library root | Reads `preferences.libraryRoot` from `~/.kit/state.json` | Manages `~/.kit/state.json` — change it here |
| Sets | Plaintext `.txt` files in `~/.kit/sets/` | Kit-native JSON sets stored in app state |
| Symlinks | `kit apply` | Assignment workflow with previews |

**Both share the same library root** — if you change it in the GUI, the CLI picks it up on next run.

**Sets are not yet shared.** The CLI reads plaintext files; the GUI uses its own JSON format. If you want the same sets visible in both, you'll need to duplicate them for now. Future work may unify the two formats.

---

## Relationship to the Teams system

If you use the [Teams orchestration system](/Users/dannyharding/The%20Team/) (a separate thing that lives at `~/Teams/`), it maintains its own sets at `~/Teams/_shared/bin/sets/` and calls Kit CLI from its SOPs.

To make Kit CLI default to those sets (so `kit sets` and `kit apply` "just work" from anywhere), **pick one**:

```bash
# Option A — symlink (recommended, simplest)
ln -s ~/Teams/_shared/bin/sets ~/.kit/sets

# Option B — environment variable in your shell rc
echo 'export KIT_SETS_DIR="$HOME/Teams/_shared/bin/sets"' >> ~/.zshrc
```

Kit CLI itself doesn't know anything about Teams. It just reads sets from wherever you point it.

---

## Installation

From this repo:

```bash
./scripts/install-cli.sh
```

This runs `cargo build --bin kit --release` and symlinks the resulting binary to `~/bin/kit`. Make sure `~/bin` is on your `PATH`.

Re-run the script whenever you change the CLI source to rebuild and re-link.

---

## Exit codes

| Code | Meaning |
|---|---|
| `0` | Success |
| `1` | Operational failure — e.g., some skills in the set aren't in the library. Other skills still get symlinked; this code signals "job partially done, check output". |
| `2` | Usage error — bad flags or missing required args |

---

## What Kit CLI does NOT do (yet)

- Create or edit sets interactively. Use a text editor or redirect to append.
- Detect or repair broken symlinks. The Kit GUI handles this in its health check.
- Read Kit's native JSON sets. Plaintext `.txt` only for now.
- Manage or register teams (that's the Teams system's job, via a separate shell script).

---

## Troubleshooting

### `Sets directory does not exist: /Users/.../.kit/sets`

The default is `~/.kit/sets/`, which starts empty. Either:

- Create it: `mkdir -p ~/.kit/sets` and add some `.txt` set files
- Point Kit somewhere that already has sets: `export KIT_SETS_DIR=/path/to/sets`
- Symlink: `ln -s ~/your/sets ~/.kit/sets`

### `N missing` after `kit apply`

One or more skills listed in the set aren't present in your library. Either:

- Check `kit sets` and open the `.txt` file — is the skill name exactly right (case-sensitive)?
- Check `~/Ai/Assets/Claude/Skills/<name>/` exists
- Remove the offending entry from the set file if the skill is genuinely gone

### `kit list` shows skills that aren't symlinks

The `.claude/skills/` directory may have real folders (created by the Kit GUI, another tool, or by hand). `kit apply` only touches the symlinks it creates — it won't delete real directories.

### `kit --version` works but `kit` doesn't

`~/bin/` isn't on your `PATH`. Add this to `~/.zshrc`:

```bash
export PATH="$HOME/bin:$PATH"
```

### The binary at `~/bin/kit` is stale

The symlink points at `target/release/kit` in the Kit repo. Rebuild:

```bash
cd /path/to/kit && ./scripts/install-cli.sh
```

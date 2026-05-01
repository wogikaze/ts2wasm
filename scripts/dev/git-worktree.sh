#!/usr/bin/env bash
set -euo pipefail
R="$(git rev-parse --show-toplevel 2>/dev/null || echo "$(cd "$(dirname "$0")/../.." && pwd)")"
W="$R/_worktrees"
u() { cat >&2 <<'EOU'; }
c() { local b="$1" base="${2:-HEAD}" p="$W/$b"; mkdir -p "$W"; git worktree add -b "$b" "$p" "$base"; }
l() { git worktree list --porcelain | awk '/^worktree /{w=$2} /^branch /{print w, substr($2,12)}'; }
r() { local b="$1" f="${2:-}" p; p="$(git worktree list --porcelain | awk -v br="refs/heads/$b" '/^worktree /{w=$2} /^branch /&&$2==br{print w;exit}')"; [ -z "$p" ] && p="$W/$b"; [ -d "$p" ] || exit 1; if [ "$f" != "--force" ] && [ -n "$(cd "$p" && git status --porcelain)" ]; then exit 1; fi; git worktree remove "$p" 2>/dev/null || git worktree remove --force "$p"; rmdir "$W" 2>/dev/null || true; }
p() { git worktree prune; rmdir "$W" 2>/dev/null || true; l; }
case "${1:-h}" in c|create) shift; c "$@";; l|list) l;; r|remove) shift; r "$@";; p|cleanup) p;; *) exit 1;; esac

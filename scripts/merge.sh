#!/usr/bin/env bash
# Fast-forwards main in the main checkout to a branch whose tip scripts/check.sh passed on a clean
# tree, and with --push pushes it:
#   scripts/merge.sh my-branch          # merge only
#   scripts/merge.sh --push my-branch   # merge, then push main
# It refuses if the main checkout isn't on main (other sessions switch it), if the tip wasn't
# checked, or if main didn't end up at the tip.
set -euo pipefail

push=false
if [[ ${1:-} == --push ]]; then
    push=true
    shift
fi
branch=${1:?usage: scripts/merge.sh [--push] BRANCH}
common=$(cd "$(git rev-parse --git-common-dir)" && pwd)
cd "$common/.."

tip=$(git rev-parse --verify "$branch^{commit}")
if ! grep -qx "$tip" "$common/checked-commits" 2>/dev/null; then
    echo "$branch ($tip) hasn't passed scripts/check.sh on a clean tree" >&2
    exit 1
fi
current=$(git branch --show-current)
if [[ $current != main ]]; then
    echo "the main checkout is on $current, not main" >&2
    exit 1
fi
git merge --ff-only --quiet "$tip"
if [[ $(git rev-parse main) != "$tip" ]]; then
    echo "main is at $(git rev-parse main), not $tip" >&2
    exit 1
fi
echo "main is at $tip"
if $push; then
    git push --quiet origin main
    echo "pushed"
fi

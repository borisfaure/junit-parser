#!/usr/bin/env bash
# Open a GitHub issue per RustSec advisory found in a `cargo audit --json` report.
# Requires: gh (authenticated via $GH_TOKEN), jq, and $RUN_URL in the environment.
set -euo pipefail

report=${1:-audit.json}

jq -c '.vulnerabilities.list // [] | .[]' "$report" | while read -r item; do
    id=$(jq -r '.advisory.id' <<<"$item")
    title=$(jq -r '.advisory.title' <<<"$item")
    pkg=$(jq -r '.package.name' <<<"$item")
    version=$(jq -r '.package.version' <<<"$item")
    url=$(jq -r '.advisory.url // ""' <<<"$item")
    desc=$(jq -r '.advisory.description' <<<"$item")
    patched=$(jq -r '.versions.patched // [] | join(", ")' <<<"$item")
    unaffected=$(jq -r '.versions.unaffected // [] | join(", ")' <<<"$item")

    # Dedup: skip if an open issue already references this advisory id.
    if [ -n "$(gh issue list --state open --search "$id in:title" --json number --jq '.[0].number // empty')" ]; then
        echo "issue for $id already open, skipping"
        continue
    fi

    body=$(printf '| Details |  |\n| --- | --- |\n| Package | `%s` |\n| Version | `%s` |\n| URL | %s |\n| Patched Versions | %s |\n| Unaffected Versions | %s |\n\n%s\n\n_Reported by `cargo audit` in [this run](%s)._\n' \
        "$pkg" "$version" "$url" "$patched" "$unaffected" "$desc" "$RUN_URL")

    gh issue create --title "$id: $title" --body "$body"
done

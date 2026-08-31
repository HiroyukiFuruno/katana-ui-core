#!/usr/bin/env bash
set -euo pipefail

HOOK_NAME="${1:-${GIT_HOOK_NAME:-}}"
PUSH_REMOTE_NAME="${2:-}"
if [[ -z "$HOOK_NAME" ]]; then
    echo "KUC hook dispatcher: missing hook name" >&2
    exit 1
fi

PUSH_UPDATES=""
if [[ "$HOOK_NAME" == "pre-push" ]]; then
    PUSH_UPDATES="$(cat)"
fi

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

DEFAULT_BRANCH="$(git symbolic-ref -q --short refs/remotes/origin/HEAD 2>/dev/null | sed 's#origin/##' || true)"
if [[ -z "$DEFAULT_BRANCH" ]]; then
    DEFAULT_BRANCH="$(git remote show origin 2>/dev/null | awk '/HEAD branch/ { print $NF }' || true)"
fi
if [[ -z "$DEFAULT_BRANCH" ]]; then
    DEFAULT_BRANCH="master"
fi

CURRENT_BRANCH="$(git branch --show-current)"

trace() {
    if [[ "${KUC_HOOK_TRACE:-}" == "1" ]]; then
        echo "KUC hook trace: $*" >&2
    fi
}

has_repo_issue_reference() {
    local message="$1"
    local remote_url=""

    if printf '%s' "$message" | grep -Eq "#[0-9]+"; then
        return 0
    fi

    remote_url="$(git remote get-url origin 2>/dev/null || true)"
    if [[ -n "$remote_url" ]] && printf '%s' "$message" | grep -Eq "github.com/.*/.*issues/[0-9]+"; then
        return 0
    fi

    return 1
}

is_dependency_update_path() {
    local path="$1"

    case "$path" in
        Cargo.lock|docs/dependency-policy.md|docs/release/*.md)
            return 0
            ;;
        Cargo.toml|crates/*/Cargo.toml|crates/*/Cargo.lock)
            return 0
            ;;
    esac

    return 1
}

has_downstream_dependency_update_paths() {
    local source="${1:-}"
    local path

    while IFS= read -r path; do
        if [[ -z "$path" ]]; then
            continue
        fi
        if is_dependency_update_path "$path"; then
            return 0
        fi
    done <<< "$source"

    return 1
}

has_dependency_evidence() {
    local evidence="$1"
    local lower
    lower="$(printf '%s' "$evidence" | tr '[:upper:]' '[:lower:]')"

    [[ "$lower" == *"upstream"* ]] || return 1
    [[ "$lower" == *"published"* ]] || return 1
    [[ "$lower" == *"version"* ]] || return 1
    [[ "$lower" == *"api migration"* ]] || return 1
    [[ "$lower" == *"manifest"* ]] || return 1
    [[ "$lower" == *"lock"* ]] || return 1
    [[ "$lower" == *"verification"* ]] || return 1

    return 0
}

issue_evidence_for_message() {
    local message="$1"
    local issue_number=""
    local evidence=""
    local found=0

    while IFS= read -r issue_number; do
        [[ -n "$issue_number" ]] || continue
        found=1
        evidence="$(gh issue view "$issue_number" --json body,comments --jq '[.body, (.comments[].body)] | join("\n")' 2>/dev/null)" || {
            echo "KUC hook policy: repository issue #$issue_number could not be inspected." >&2
            return 1
        }
        if has_dependency_evidence "$evidence"; then
            return 0
        fi
    done < <(printf '%s' "$message" | grep -Eo '#[0-9]+' | tr -d '#' | sort -u)

    if [[ "$found" == "0" ]]; then
        echo "KUC hook policy: dependency update has no inspectable repository issue." >&2
    else
        echo "KUC hook policy: referenced issues lack upstream published version/API migration/manifest/lock/verification evidence." >&2
    fi
    return 1
}

dispatch_commit_msg() {
    local commit_msg_file="$1"
    local message=""
    local staged_changes=""

    message="$(cat "$commit_msg_file")"

    if [[ -n "$CURRENT_BRANCH" && "$CURRENT_BRANCH" != "$DEFAULT_BRANCH" ]]; then
        if ! has_repo_issue_reference "$message"; then
            echo "KUC hook policy: non-default branch commit message must reference a repository issue." >&2
            exit 1
        fi
    fi

    staged_changes="$(git diff --name-only --cached)"
    if [[ -n "$staged_changes" ]] && has_downstream_dependency_update_paths "$staged_changes"; then
        if ! issue_evidence_for_message "$message"; then
            echo "KUC hook policy: downstream dependency updates require evidence in the referenced repository issue." >&2
            exit 1
        fi
    fi
}

commit_has_dependency_updates() {
    local commit_sha="$1"
    local changed

    if [[ -z "$commit_sha" ]]; then
        return 1
    fi

    changed="$(git diff-tree --root --name-only --no-commit-id -r "$commit_sha" || true)"
    has_downstream_dependency_update_paths "$changed"
}

dispatch_pre_push() {
    local local_sha=""
    local remote_sha=""
    local commit_list=""
    local zero_sha="0000000000000000000000000000000000000000"

    while read -r _local_ref local_sha _remote_ref remote_sha; do
        trace "push local=$local_sha remote=$remote_sha"
        if [[ -z "$local_sha" || "$local_sha" == "$zero_sha" ]]; then
            continue
        fi

        if [[ "$remote_sha" == "$zero_sha" ]]; then
            if [[ -n "$PUSH_REMOTE_NAME" ]]; then
                commit_list="$(git rev-list "$local_sha" --not --remotes="$PUSH_REMOTE_NAME")"
            else
                commit_list="$(git rev-list "$local_sha" --not --remotes)"
            fi
        else
            commit_list="$(git rev-list "$remote_sha".."$local_sha")"
        fi
        trace "commits=$commit_list"

        while IFS= read -r commit_sha; do
            if [[ -z "$commit_sha" ]]; then
                continue
            fi

            if commit_has_dependency_updates "$commit_sha"; then
                trace "dependency commit=$commit_sha"
                local commit_message=""
                commit_message="$(git log -n 1 --pretty=%B "$commit_sha")"

                if ! has_repo_issue_reference "$commit_message"; then
                    echo "KUC hook policy: downstream dependency updates on pushed commits need repository issue reference." >&2
                    echo "commit: $commit_sha" >&2
                    exit 1
                fi

                if ! issue_evidence_for_message "$commit_message"; then
                    echo "KUC hook policy: downstream dependency updates require evidence in the referenced repository issue." >&2
                    echo "commit: $commit_sha" >&2
                    exit 1
                fi
            fi
        done <<< "$commit_list"
    done <<< "$PUSH_UPDATES"

    return 0
}

case "$HOOK_NAME" in
    commit-msg)
        dispatch_commit_msg "${2:?missing commit message path}"
        ;;
    pre-push)
        dispatch_pre_push
        ;;
    pre-commit)
        :
        ;;
esac

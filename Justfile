# ============================================================
# katana-ui-core - Development Justfile
# ============================================================
# Stable local, CI/CD, and release task entrypoint.
# Usage:
#   just
#   just <recipe>
#   just VERSION=vX.Y.Z release-check
# ============================================================

set shell := ["bash", "-uc"]

REPO_ROOT := justfile_directory()
RTK := env_var_or_default("RTK", `command -v rtk 2> /dev/null || true`)
RTK_CMD := if RTK == "" { "" } else { RTK + " " }
JOBS := env_var_or_default("JOBS", "2")
CARGO := env_var_or_default("CARGO", RTK_CMD + "cargo")
VERSION := env_var_or_default("VERSION", `awk -F '"' '/^version = / { print $2; exit }' Cargo.toml`)
VERSION_BARE := replace(VERSION, "v", "")
COVERAGE_MIN_LINES := env_var_or_default("COVERAGE_MIN_LINES", "64")
RELEASE_REPO := env_var_or_default("RELEASE_REPO", "HiroyukiFuruno/katana-ui-core")
KAL_VERSION := env_var_or_default("KAL_VERSION", "0.5.1")

default: help

# Show this help
help:
    @just --list --unsorted

# Apply Rust formatting
fmt:
    {{CARGO}} fmt --all

# Check Rust formatting
fmt-check:
    {{CARGO}} fmt --all -- --check

# Check workspace type safety
check-types:
    {{CARGO}} check --workspace --locked

# Run strict Clippy checks
lint:
    RUSTFLAGS="-D warnings" {{CARGO}} clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings -D clippy::unwrap_used -D clippy::expect_used -D clippy::todo -D clippy::unimplemented -D clippy::dbg_macro -D clippy::panic -D clippy::wildcard_imports

# Install shared KatanA AST lint CLI from crates.io
ast-lint-install:
    {{CARGO}} install katana-ast-lint --version {{KAL_VERSION}} --locked --force

# Run shared KatanA Rust syntax checks
ast-lint:
    kal check
    python3 scripts/assert-kuc-state-ownership.py
    python3 scripts/assert-storybook-page-layout.py

# Run katana-ui-core specific guardrails
kuc-guardrails:
    python3 scripts/test_kuc_guardrails.py
    python3 scripts/assert-kuc-guardrails.py
    bash scripts/assert-core-dependency-boundary.sh
    bash scripts/assert-core-public-api-neutral.sh
    python3 scripts/assert-root-plan-task-drift.py

# Backward-compatible alias for older local workflows
kuw-guardrails: kuc-guardrails

# Run Storybook page structure checks
storybook-ast-lint:
    python3 scripts/assert-storybook-page-layout.py

# Reject direct Floem overlay lifecycle calls outside the shared guard.
overlay-lifecycle-lint:
    bash scripts/assert-overlay-lifecycle.sh

# Check MenuButton placement and close behavior contracts.
menu-button-contract:
    bash scripts/assert-menu-button-contract.sh

# Run workspace tests
unit-test:
    {{CARGO}} test --workspace --all-targets --all-features --locked

# Alias used by KML-style workflows
test: unit-test

# Run coverage as a release confidence gate
coverage:
    {{CARGO}} llvm-cov --workspace --all-features --locked --summary-only --fail-under-lines {{COVERAGE_MIN_LINES}}

# Run the local quality gate
check: fmt-check check-types lint unit-test ast-lint kuc-guardrails overlay-lifecycle-lint menu-button-contract
    @echo "checks passed"

# Sweep old build artifacts locally (older than 7 days)
sweep:
    @{{CARGO}} sweep --time 7 || true

# Remove build artifacts
clean: sweep
    {{CARGO}} clean

# Update dependency crates to latest compatible versions
update-safe:
    {{RTK_CMD}}cargo update

# Upgrade all dependency requirements, then update Cargo.lock
update:
    {{RTK_CMD}}cargo upgrade -i
    {{RTK_CMD}}cargo update

# Run the visible KUC Storybook panel window
storybook:
    RUSTFLAGS="-D warnings" {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-window 0

# Print the KUC Storybook validation summary without opening a window
storybook-summary:
    RUSTFLAGS="-D warnings" {{CARGO}} run -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked

# Run the visible KUC Storybook panel plus modal native window
storybook-modal:
    RUSTFLAGS="-D warnings" {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --open-modal-window 0

# Check that Storybook compiles without running
storybook-check:
    RUSTFLAGS="-D warnings" {{CARGO}} check -p katana-ui-core-storybook --all-targets --locked

# Render the KUC Storybook panel to a PNG snapshot.
storybook-visual-snapshot:
    RUSTFLAGS="-D warnings" {{CARGO}} run --release -p katana-ui-core-storybook --bin katana-ui-core-storybook --locked -- --visual-snapshot target/storybook-panel.png

# Check Storybook pages can build without opening visible windows.
storybook-smoke:
    bash scripts/storybook-headless-smoke.sh

# Verify VERSION follows the published release line
release-target-check:
    bash scripts/release/verify-version.sh "{{VERSION}}"
    GH_TOKEN="${GH_TOKEN:-$(gh auth token 2>/dev/null || true)}" python3 scripts/release/verify-release-target.py --target-version "{{VERSION}}" --repo "{{RELEASE_REPO}}"

# Verify KUC v0.1.0 DoD is actually complete before release gates continue.
release-readiness-check:
    python3 scripts/assert-kuc-release-readiness.py

# Verify package metadata and dry-run the publishable crate
release-verify: check coverage
    bash scripts/release/verify-version.sh "{{VERSION}}"
    {{CARGO}} package -p katana-ui-core --locked --allow-dirty
    {{CARGO}} publish -p katana-ui-core --dry-run --locked --allow-dirty
    bash scripts/release/verify-primary-adapter-release.sh "{{VERSION}}"

# Verify release branch readiness before merging
release-check: release-target-check release-readiness-check release-verify
    bash scripts/release/assert-crate-not-published.sh "{{VERSION}}"

# Show recent Release workflow runs
release-status:
    gh run list --repo {{RELEASE_REPO}} --workflow Release --limit 5

# Check Storybook overlay/action pages in an opened state, not only initial mount.
storybook-interaction-smoke:
    bash scripts/storybook-interaction-smoke.sh

# Check Storybook pages against requirement scenarios, not only crash-free launch.
storybook-requirement-gate:
    bash scripts/storybook-requirement-gate.sh

# Run all Rust tests with warnings denied.
cargo-test:
    RUSTFLAGS="-D warnings" cargo test --workspace --all-targets

# Run the full Storybook regression gate used before publishing.
storybook-regression: cargo-test storybook-check ast-lint kuc-guardrails overlay-lifecycle-lint menu-button-contract storybook-smoke storybook-interaction-smoke storybook-requirement-gate

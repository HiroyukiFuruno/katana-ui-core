# KUC AST Lint Remediation Plan

## Decision

KUC release gates keep `kal check` enabled. The current 469 findings are not
suppressed, excluded, or threshold-adjusted. The prior parser panic on a
large integer literal is removed by using an equivalent byte construction, so
the full finding set is observable.

## Baseline

The 2026-08-26 baseline contains 469 findings:

| Rule | Findings | Required response |
| --- | ---: | --- |
| magic-numbers | 378 | Extract domain, layout, color, and raster constants into named KUC-local tokens. |
| file-length | 33 | Split by cohesive renderer, input, model, routing, or evidence responsibility. |
| type-separation | 21 | Move public data contracts from mixed implementation modules into dedicated type modules. |
| pub-free-fn | 13 | Move public operations to a namespaced type while preserving compatibility through associated methods. |
| comment-style | 12 | Convert comments to documentation or a concise WHY annotation. |
| nesting-depth | 4 | Extract routing and paint decisions to early-return helpers. |
| horizontal-layout | 3 | Use the required alignment primitive. |
| conditional-frame | 2 | Preserve inactive-frame geometry with the required button primitive. |
| error-first | 2 | Return or propagate errors instead of nesting success paths. |
| process-command | 1 | Route external process execution through KUC's cross-platform process service. |

The largest ownership clusters are `tab_strip_retained` (137),
`diagnostics_list` (70), `status_bar` (66), and `source_address_strip` (52).
They are generic KUC UI modules, not KLE code.

## Boundaries

- KUC owns reusable raster, input, focus, layout, accessibility, and command
  surface behavior.
- KLE composes opaque KUC projections and must not receive KUC implementation
  types or Katana-specific actions.
- Refactoring may not add an emoji, font, or platform fallback. Construction
  and rendering failures stay typed and propagate to the caller.
- Splits preserve public contracts with re-exports only where the current API
  requires compatibility. No compatibility shim may reintroduce `Default`,
  `expect`, `unwrap`, or a hidden fallback for the fallible raster boundary.

## Execution Order

1. Split `source_address_strip`, `status_bar`, and `diagnostics_list` into
   contract, raster, paint, interaction, and retained-state modules; replace
   literal layout/raster values with named tokens and retain their existing
   Japanese, VS16, and AccessKit contracts.
2. Split `tab_strip_retained` by retained state, route table, paint, overlay,
   and interaction; retain opaque host proposal behavior and fail-closed stale
   route handling.
3. Move public free functions and mixed contracts in text-command root,
   sanitized projection, and text-raster modules into focused types.
4. Resolve remaining comment, layout, conditional-frame, and process-service
   rules without semantic substitutions.
5. After each cluster, run its focused Rust tests, strict clippy, `kal check`,
   and the Storybook runtime evidence. The complete KUC gate remains
   `just check`; no release proceeds until it passes.

## First Extraction Design

The first batch is deliberately limited to the three editor-adjacent generic
surfaces. It changes module ownership, not behavior.

| Current module | Destination modules | Public boundary |
| --- | --- | --- |
| `source_address_strip` | `source_address_strip/types`, `raster`, `paint`, `interaction`, `adapter` | Re-export the existing public receipts, styles, error, adapter, and output from the module root. |
| `status_bar` | `status_bar/types`, `paint`, `accessibility`, `adapter` | Re-export the existing public style, receipt, error, adapter, and output. |
| `diagnostics_list` | `diagnostics_list/types`, `identity`, `paint`, `accessibility`, `adapter` | Replace free identity functions with associated functions on `DiagnosticsTargetIdentity`; keep deprecated-free compatibility only when a downstream public compatibility test proves it is required. |

`types` owns serializable/frame contracts and named visual constants. `raster`
owns KUC text-raster calls and error conversion. `paint` owns pure plan
construction. `interaction` or `accessibility` owns physical egui input and
AccessKit publication. `adapter` owns retained state and composition only.
No destination module imports KLE or KatanA host code.

### Completed Batch: Source Address

`source_address_strip` now uses the planned `types`, `raster`, `paint`,
`interaction`, and `adapter` modules behind the original module-root API. The
cluster has zero `kal check` findings. Its focused contract still verifies
physical input, one-shot submission, AccessKit, Japanese labels, and distinct
`⭐` / `⭐️` raster evidence. This batch does not make the overall KUC AST gate
green; the remaining clusters stay release blockers.

### Completed Batch: Status Bar

`status_bar` now separates public contracts, retained adapter state, rendering,
paint-plan assembly, popover lifecycle, and accessibility publication while
retaining its original module-root API. The cluster has zero `kal check`
findings. Its focused contract verifies Japanese and emoji rasterization,
pointer and AccessKit actions, and one-shot progress popover behavior. This
does not make the overall KUC AST gate green; the remaining clusters stay
release blockers.

### Completed Batch: Diagnostics List

`diagnostics_list` now separates public contracts, opaque target identity,
paint, accessibility, and retained adapter composition. Display text remains
out of the identity boundary: `DiagnosticsTargetIdentity` exposes associated
identity constructors rather than restoring the former public free functions.
The focused contract passes all ten cases, including Japanese and emoji raster
records, pointer/keyboard/AccessKit activation, retained scrolling, stale
target rejection, and opaque identity stability. The diagnostics path has zero
`kal check` findings.

### Completed Batch: Retained Tabs And Scenarios

The generic retained tab surface is split into retained state, interaction,
label paint, overlay, and shared support modules without adding host semantics.
All imports are explicit so the strict wildcard-import gate remains active.
The tab lease contract passes all three cases. The generic command-surface
scenario has likewise been separated into responsibility-specific modules; its
twelve end-to-end scenario contracts pass. Both paths have zero `kal check`
findings.

After these batches, a live `just ast-lint` run reported 129 remaining
findings.

### Completed Batch: Generic Event And Raster Transports

`root_event`, `sanitized_search_event`,
`sanitized_command_projection_adapter`, and `tab_strip_text_raster` now have
zero findings in their respective paths. The event transports retain their
opaque contracts, and the tab raster path retains its Unicode and VS16
handling. A private router type alias replaces a complex function-pointer
declaration without weakening clippy. After this batch, the adapter library
passes the strict library clippy command and all 203 library tests; a live
`just ast-lint` run reports 92 remaining findings.

### Completed Batch: Interaction And Accessibility Layers

`interaction_locator`, `accesskit_evidence`, and
`closeable_tab_strip_adapter` now have zero findings in their respective
paths. Interaction routing remains opaque and fails closed for stale routes;
the AccessKit ledger keeps its pointer, keyboard, and accessibility evidence
contracts. The closeable tab surface has a separate renderer without acquiring
host semantics. A fresh strict adapter-library clippy run and all 203 library
tests pass after this batch. A live `just ast-lint` run reports 71 remaining
findings.

### Completed Batch: Floating Paint And Motion Evidence

`command_chrome_floating_paint` retains the generic floating command surface's
emoji, keyboard, and AccessKit behavior with seven focused contracts passing.
`motion_artifact_writer` now separates types, ffmpeg work, and shared helpers;
its nine focused tests and feature-enabled compilation pass. Both paths have
zero findings. A live `just ast-lint` run reports 61 remaining findings.

The full AST gate is therefore still a release blocker; no threshold or
exclusion has been changed.

### Gate Closure

The remaining generic surfaces and their split-module guardrails are now
covered by the same strict source checks. A live `just ast-lint` run passes
the Storybook UI harness, `kal check`, state-ownership checks, generic KUC
guardrails, and page-layout checks with zero AST findings. `just check` passes
all 2,187 workspace tests, and `just storybook-regression` passes the 77-page
headless surface, 403 live interactions, requirement gate, and interaction
smoke. This closes AST remediation only; publication, immutable version pinning,
and real KatanA-host parity remain separate release gates.

## Evidence Required Before KLE Release

- `just check` is green on a clean KUC candidate.
- Unicode evidence distinguishes `⭐` from `⭐️` in the KUC raster path.
- Storybook runtime artifacts exercise command controls, find/replace,
  Markdown authoring controls, source address, diagnostics, tabs, and
  accessibility actions through actual KUC events.
- The published KUC version and commit are immutable inputs to KLE's source
  closure capture.

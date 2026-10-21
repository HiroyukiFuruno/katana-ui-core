## Implementation

- [x] Preserve Issue #85 changes and implement independent code raster builder on the same release branch.
- [x] Add public regression tests for body isolation, code drawing / wrapping / hit equivalence and scroll cache invalidation.
- [x] PR #90 P1: invalidate plain-line wrapping cache when code raster config/face changes; cache keyへrenderer設定identityを含めるunit regression、およびplain code-roleのdraw/wrap contractが通過。
- [x] Verify focused tests, format and strict Clippy: public code config 5 tests + Card 8 tests passed; just fmt-check and raster-host lib/integration strict Clippy passed.
- [ ] Parent release workflow: review, version, full release checks, Draft PR, review/fixes, Ready, CI, merge and public release.

## Release-preflight diagnostic (2026-10-21)

- [/] PR #90のrelease-preflight 39522454714はfull coverage実行器の子process終了を記録しないまま失敗した。検査対象やcoverage基準を下げず、childごとの状態記録・失敗時log artifact・hosted runnerの同時thread抑制を追加して再実行する。

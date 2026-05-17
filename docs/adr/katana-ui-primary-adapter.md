# ADR: katana-ui-core primary adapter 選定

ステータス: Proposed
作成日: 2026-05-17

## 目的

`katana-ui-core` の primary adapter を選ぶための比較基準を固定する。
この ADR が Accepted になるまで、`katana-ui-core-floem` は primary adapter 候補として扱う。

## 候補

| 候補 | 内容 |
| --- | --- |
| A | Floem を primary adapter にする |
| B | GPUI を primary adapter にする |
| C | egui を短期 primary adapter にする |
| D | primary を確定せず adapter agnostic で進める |

## 比較基準

| 基準 | 見ること |
| --- | --- |
| API 安定度 | adapter public API が短期で壊れにくいか |
| エディタ系適合 | text input、keyboard、focus、multi-window を扱いやすいか |
| 移行コスト | 既存 KatanA / KUC code からの移行量 |
| Phase 5 整合 | KatanA 統合時の責務境界と合うか |
| 外部利用者向け魅力 | KUC を単体利用する人にとって選びやすいか |
| Storybook / release gate 維持コスト | core と同等の gate を継続できるか |

## 現時点の運用

- `katana-ui-core-floem` を primary adapter 候補として先に整備する。
- `katana-ui-core-egui` と `katana-ui-core-gpui` は互換 adapter として compile test を最低 gate にする。
- Storybook は `katana-ui-core` の core-only 確認だけを必須にし、primary adapter 経由にはしない。
- primary が切り替わる場合、旧 primary は [`compat-adapters.md`](../compat-adapters.md) の互換 adapter rule に降格する。

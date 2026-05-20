# Compatibility adapters policy

作成日: 2026-05-17
対象: `katana-ui-core-egui`, `katana-ui-core-gpui`, primary に選ばれなかった adapter

## 目的

互換変換層（compatibility adapter）が何を保証し、何を保証しないかを明確にする。
この文書は repo 外の release policy を読ませないための KUC repo-local policy である。

## Adapter 一覧

| adapter crate | 役割 | 初期対応 widget | runtime / window / surface | release blocking |
| --- | --- | --- | --- | --- |
| `katana-ui-core-egui` | 後続候補。現時点では skeleton のみ | Text / Button / Row / Column | skeleton のみ | 現在の primary release gate には含めない |
| `katana-ui-core-gpui` | 後続候補。現時点では skeleton のみ | Text / Button / Row / Column | skeleton のみ | 現在の primary release gate には含めない |
| `katana-ui-core-floem` | primary に選ばれなかった場合の Floem 互換 adapter | primary 実装済み範囲 | primary 実装済み範囲 | primary 降格後は compatibility rule に従う |

TextArea の複数行 IME は core DTO を正本にする。
各 adapter は現時点では compile-gate stub として、`input_kind=Multiline`、`phase`、`preedit`、`commit_text`、`caret` を `ImeRequest` に写すことを保証する。
framework-native な候補ウィンドウや実描画は後続 scope とし、core public API を壊さないことを先に固定する。

Virtualization の row 測定は adapter / consumer 側の責務とする。
adapter は実測した row height を KUC の `RowHeightOverride` として返し、KUC core は `VirtualizationPlanner` で visible range、overscan、aria-setsize / aria-posinset、scroll offset 補正を計算する。
adapter は独自の global scroll state を持たず、component ごとの `VirtualizationConfig` と `VirtualRange` をそのまま反映する。

## 最低品質 gate

互換 adapter crate を作る段階では最低限以下を通す。

- adapter crate の compile test
- Text / Button / Row / Column の `UiTree` 変換 skeleton test
- core crate dependency leak guard

Storybook smoke は `katana-ui-core` の core-only 確認だけを必須にする。
互換 adapter ごとの Storybook smoke は行わず、最低品質 gate にも含めない。
現時点の優先順位は KUC core と primary adapter 候補の確立であり、egui / GPUI 互換 crate は framework-native 実装までは行わない。

primary adapter と同等の full Storybook regression は行わない。

## Release blocking rule

互換 adapter failure は、次のいずれかに該当する場合だけ primary release を止める。

- core public API compatibility を壊している。
- selected primary adapter の build / test に影響している。
- release package から core crate を壊す dependency leak を発生させている。

上記に該当しない場合、failure は compatibility scope として報告し、primary release を止めない。

## SemVer policy

- 互換 adapter の対応 widget 追加は minor release で扱う。
- 互換 adapter の未対応機能は README と本ファイルに明記する。
- 既存対応 widget の削除、public type の削除、挙動互換の破壊は breaking change として扱う。
- primary adapter から compatibility adapter へ降格した crate は、降格時点の対応範囲を本ファイルに記録する。

## Documentation requirement

各 adapter は README または crate-level docs に以下を持つ。

- 対応 widget 一覧
- 未対応機能一覧
- fallback behavior
- runtime / window / surface 対応範囲
- Storybook 確認ページ
- release blocking rule へのリンク

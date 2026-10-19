## Context

Card の子原点は描画とhit metricsの各 helper で16px加算される。scroll測定もこのhelperを利用する。

## Goals / Non-Goals

Goals: Cardごとに0pxや任意の非負pxを設定し、描画・測定・hit/scrollを一致させる。
Non-Goals: 外部repoの変更、既存Cardの既定変更、他のnode kindへの新しい挙動。

## Decisions

- UiCommonProps.child_indent: Option<u16> を追加する。None は既存16px、Some は指定値。builder は child_indent(u16)。paddingとは別加算。
- serde(default) で旧JSONを受け入れる。Cardをlayout container扱いにするとラベル等の互換性まで変わるため採用しない。
- coreとStorybookの既存helperへ同じ条件を反映し、専用AccordionやTreeのインデントは維持する。
- 新規依存は不要。cargo updateで直接・推移的依存の互換更新を評価する。egui-winit 0.37 はegui/eframe全体のmajor移行を要するため、本patchでは0.36系列を維持する。

## Risks / Trade-offs

- public structの全フィールド指定リテラルは追加フィールドを要する → Default更新構文とbuilderを推奨する。
- 四つのhelperの乖離 → core/Storybook双方の数値回帰と公開API統合テストで検出する。

## Migration Plan

v0.4.2公開後、CardのcommonへUiCommonProps::default().child_indent(0)を渡す。旧モデルは変更不要。

## Open Questions

なし。

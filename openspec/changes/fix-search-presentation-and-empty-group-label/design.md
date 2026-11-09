## Context

SearchResultModeは外部consumerがstruct literalで構築する公開structであり、項目の直接追加はソース互換性を壊す。SelectionDialogProjectionLeaseはprivate fieldsを持つ公開型で、既存のquery Arrow policyもmode targetをキーにしたbuilder/mapを使っている。retained search inputはTextArea/TextSurfaceをmode/fieldごとに保持するため、hintとinvalid状態の再同期でinputを作り直すとfocus/IME状態を失う。

TabStripTextRasterizerはTabStrip専用だが内部で汎用PlatformTextRasterizerを呼び出す。汎用rasterizerのEmptyText拒否は共通契約であり、empty inputの検査を無効化してはならない。現在のgroup paintはlabel bounds/操作/AccessKitを作る前にrasterizeを呼ぶため、空文字がroot全体を失敗させる。

## Goals / Non-Goals

**Goals:**
- 外部公開struct literalの互換性を保つmode別検索presentation builderを追加する。
- hint、意味的validity、非操作履歴見出しをKUCのretained rootでtheme描画し、host所有の判定処理を維持する。
- 空TabStrip labelをcomponent境界で空表示として描画し、非ゼロの操作bounds、正確な空文字AX値、後続入力を保つ。
- 公開rootでmode切替/再投影/入力/host event、および日本語→VS16 emoji→空名→継続入力を回帰する。

**Non-Goals:**
- 検索/regex判定、履歴保管、入力widget、色値のhost指定、placeholder glyphの代入。
- PlatformTextRasterizerのvalidation変更・無効化。
- KLEまたは他repositoryの変更。

## Decisions

1. `SearchResultMode`にはfieldを追加せず、公開`SearchModePresentation`と`SelectionDialogProjectionLease::search_mode_presentation(target, value)`を追加する。理由はstruct literal利用の互換性を保つため。候補はSearchResultModeへのfield追加だが破壊的なので採用しない。hintsは`Option<String>`、各filter validityもOptionにして未指定時の既存描画を維持する。mode targetの存在はbuilderで検査する。
2. Retained inputは現在のTextSurfaceとTextAreaを維持し、内部同期メソッドでplaceholder/invalidだけを更新する。value/focus/selection/compositionを再初期化しない。hintは操作名/AX labelとは独立させ、accessibility labelは従来の`mode.label`/controls labelを保つ。invalidは既存TextArea theme stateへ写像し、valid/unspecifiedは非invalid表示にする。
3. 履歴見出しは既存painterのnon-interactive label経路に描き、`mode.history`へ挿入しない。空見出しはraster requestを発生させない。
4. `TabStripTextRasterizer`は空文字に対して汎用rasterizerを呼ばず、0x0/empty pixelsのopaque component rasterを返す。paintはtexture operationを生成せず、背景/marker、最低限のpadding由来bounds、route/hit-test、AX値をそのまま処理する。代替glyphを使わない。汎用PlatformTextRasterizerのEmptyText validationは保持する。
5. Dependency updateは候補ごとに既存品質gateで評価し、egui-winit 0.37移行を「minorだから安全」と推定しない。IMEを含む全条件で品質を保てない候補は採用しない。

## Risks / Trade-offs

- [Risk] input optionの同期がIME composition/focusを壊す → hint/validityだけ変更する保持状態回帰とIME中の変更回帰を追加する。
- [Risk] empty rasterを0x0 texture operationへ漏らす → 全てのTabStrip raster callsiteでempty時のpaint operation/texture生成を省略する。
- [Risk] SearchModePresentationが誤targetへ紐付く → builderで投影内targetを検査し、stale lease拒否・mode switch/resyncをfull-rootで検証する。
- [Risk] egui-winit更新がnative IMEへ影響する → registry版と既存IME/全quality gatesで評価し、gate未達なら更新を採らない。

## Migration Plan

Additive builder/API。未設定modeは従来表示のまま。公開crate consumerでregistry release後のcompile/input regressionを行う。自動Release失敗時はmain merge commitから修正release branchを作り、公式Release workflowを再実行する。手動tag/crate publishは行わない。

## Open Questions

なし。

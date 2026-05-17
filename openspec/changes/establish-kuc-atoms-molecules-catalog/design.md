## Context

KUC は画面部品だけでなく、起動、窓、描画面、テーマ、イベント、描画モデルを持つ UI Core として再定義済みである。
ただし、旧個別 change の 01〜24 は「旧 Storybook で確認した」という履歴を多く含み、KUC の純 Rust model、内部 state、部品カタログ、自動品質ゲートの契約にはまだ移し切れていない。

現在の目標は、利用側が最小部品（atoms）と組み合わせ部品（molecules）を組み合わせて UI を構築できることに置く。
画面（pages）だけを利用側が実装すればよい最終形や、画面ひな形（templates）はこの段階の計画対象にしない。
一方で、Storybook 自身を構成するために大きな構成部品（organisms）相当が必要になる場合は、内部実装として許可する。

## Goals / Non-Goals

**Goals:**

- 01〜24 の UI 要件を KUC の atoms / molecules 実装タスクへ再分類する。
- core 基盤の theme / font / text / input / event / state / layout 契約を先に固定する。
- Storybook を KUC 部品カタログとして設計し、TreeView、Tabs、preview、settings を KUC 自身の部品で構成する。
- 各 UI の option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページを完了条件にする。
- Storybook 目視ではなく、自動テストと guard を CI/CD 品質ゲートの主根拠にする。
- 古い OpenSpec / docs の完了扱いを、履歴、移管済み、archive 候補に整理する。

**Non-Goals:**

- この change 作成作業では Rust 実装を開始しない。
- adapter の本実装は MVP に含めない。
- 公開 API として organisms / templates / pages を計画しない。
- archive 済み 01〜24 をそのまま復帰しない。
- KatanA 本体や sibling repository を変更しない。

## Decisions

### 1. 次フェーズの正本はこの change にする

`establish-kuc-atoms-molecules-catalog` を、01〜24 を KUC 独自実装へ移すための正本 change にする。
`ui-core-root-plan` は親設計として残すが、UI 実装完了の根拠にはしない。

代替案として `ui-core-root-plan` に追記し続ける方法もあるが、親設計と部品実装の責務が混ざるため採用しない。

### 2. widget 層は同 crate 内の公開階層として開始する

初期版では `widget::atoms` と `widget::molecules` を公開境界にする。
既存の `atom` / `molecule` は core 内部の model 実装として整理し、利用側が参照する説明は `widget` 階層に寄せる。
atoms の UI 別契約は `atoms-contract.md` に固定する。
molecules の UI 別契約は `molecules-contract.md` に固定する。

別 crate 化は将来の選択肢として残すが、MVP では workspace と release の複雑さを増やさない。

### 3. Storybook は部品カタログであり品質判定の主役ではない

Storybook は、画面上で部品の見た目と操作感を確認するための部品カタログである。
左ペインの一覧は TreeView、preset 切替は Tabs、各部品ページは preview と settings を持つ。
画面構成の詳細は `storybook-catalog-contract.md` に固定する。

ただし、部品が正しく動くことは Storybook 目視ではなく、単体テスト、レイアウト回帰、画像回帰、入力回帰、guard で判定する。
品質ゲートの詳細は `quality-gates-contract.md` に固定する。

### 4. core 基盤を部品実装より先に固定する

theme、font、text、input、event、state、layout は全部品に影響するため、atoms / molecules より先に契約を固定する。
特に日本語入力（IME）、OS 絵文字、英日混在の上下中央揃えは KUC の基盤要件として扱う。
詳細な実装契約は `core-foundation-contract.md` に固定する。
この契約は `UiCoreFacade`、`ThemeSnapshot`、font role、`UiStateId`、layout regression の受け渡しを対象にする。

### 5. 古い change は削除より先に移管する

`katana-widget-parity-backlog` と `ui-core-interaction-visual-parity` は、要件をこの change に移したうえで superseded と明記する。
`18-accordion`、`23-color-picker-complete-parity`、`24-code-diff` は、個別要件を移したうえで archive 候補にする。
archive 済み 01〜17、19〜22 は履歴として残し、復帰しない。

## Risks / Trade-offs

- [Risk] 旧 change の完了チェックが新基準の完了に見える → [Mitigation] README、docs、OpenSpec README、各 superseded change に新changeへの移管を明記する。
- [Risk] Storybook が再び品質ゲートの代替になる → [Mitigation] `kuc-quality-gates` で自動テスト中心の完了条件を必須化する。
- [Risk] organisms / templates を早く公開しすぎる → [Mitigation] Storybook 内部実装としては許可し、公開 API は atoms / molecules に限定する。
- [Risk] docs と OpenSpec の正本が割れる → [Mitigation] docs は説明と索引、OpenSpec change は実装契約と tasks に分ける。

## Migration Plan

1. 新changeへ core / widget / Storybook / quality gate の契約を作る。
2. 01〜24 の対応表と完了条件を `owned-ui-task-map.md` と tasks に移す。
3. 旧完了記録を `ui-core-parity-gap.md` で旧基準として整理する。
4. README、directory structure、widget extraction policy、OpenSpec README の矛盾を直す。
5. superseded change と archive 候補を明記する。
6. `openspec validate` と `git diff --check` で docs / OpenSpec 整理のみを検証する。

## Open Questions

現時点の未決定は実装フェーズへ持ち越さない。
organisms / templates / pages の公開 API は今回の対象外であり、将来必要になった時点で別 change として提案する。

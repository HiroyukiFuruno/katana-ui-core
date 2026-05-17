## ADDED Requirements

### Requirement: dependency leak guard

KUC MUST 中核 crate（core crate）が `floem`, `gpui`, `egui`, または Katana domain crates に依存したとき失敗する checks を含める。
checks MUST local quality gate の一部にする。

#### Scenario: forbidden framework dependency is added

- **WHEN** `floem`, `gpui`, or `egui` を `crates/katana-ui-core` に追加する
- **THEN** 依存漏れ検査（dependency leak guard）が失敗する
- **AND** dependency は変換層 crate（adapter crate）へ移す

#### Scenario: forbidden domain dependency is added

- **WHEN** `katana-*` domain crate を `crates/katana-ui-core` に追加する
- **THEN** 依存漏れ検査（dependency leak guard）が失敗する
- **AND** integration は consumer-facing crate または変換層境界（adapter boundary）へ移す

### Requirement: Storybook and tests follow core boundaries

Storybook、compile tests、snapshot tests、smoke tests MUST 中立 core と adapter output を分けて検証する。
Storybook の担当は中立 core の catalog 検証に限定し、adapter output は adapter crate の compile / unit test で検証する。
Storybook MUST `katana-ui-core` core model だけを使い、Floem / GPUI / egui / adapter crate 経由にしてはならない。

#### Scenario: component verification is run

- **WHEN** KUC atom、molecule、runtime、window、surface feature を実装する
- **THEN** core model tests が neutral DTO / trait behavior を validate する
- **AND** `katana-ui-core` Storybook が core model、state identity、catalog coverage を validate する

#### Scenario: Storybook panel theme gate runs

- **WHEN** `storybook-requirement-gate` を実行する
- **THEN** Storybook は `katana-ui-core::panel::Panel` で root、navigation、preview を構成していることを検査する
- **AND** 各表示枠（panel）に見た目テーマ（theme）が設定され、`panel_theme_configured=true` が出る場合だけ成功する
- **AND** story root に後付け見た目設定（style）が解決され、`styled_story_roots` が必須 story 数と一致する場合だけ成功する

#### Scenario: Storybook crate owns the verification model

- **WHEN** Storybook gate を実行する
- **THEN** `crates/katana-ui-core-storybook` が catalog、panel、requirements を所有する
- **AND** ルート直下の旧 `storybook/` は主検証経路として扱わない

#### Scenario: Storybook visual snapshot is rendered

- **WHEN** `storybook-requirement-gate` または `storybook-visual-snapshot` を実行する
- **THEN** `crates/katana-ui-core-storybook` は KUC の表示枠（panel）tree を PNG snapshot として描画する
- **AND** HTML export ではなく、KUC Storybook の描画 surface から生成された画像を確認対象にする

### Requirement: UI state ownership guard

UI ごとの状態（state）を component 内部で一意管理する制約 MUST repo-local ast-lint gate で検査する。
この検査 MUST `kal` 本体や `kal.json` へ追記せず、KUC repo 内の script と `just ast-lint` で管理する。

#### Scenario: state ownership check runs

- **WHEN** `just ast-lint` を実行する
- **THEN** `scripts/assert-kuc-state-ownership.py` が走る
- **AND** `UiStateId`、一意な state id 生成、外部公開されない atom state、重複 UI の state identity test を検査する

### Requirement: release dry-run covers core and selected adapters

Release verification MUST 中核 crate（core crate）と選択済み主系変換層 crate（primary adapter crate）を含める。
Floem adapter release coverage MUST Floem が最初の primary candidate である間は存在する。

#### Scenario: release check runs

- **WHEN** release verification を実行する
- **THEN** core package dry-run が走る
- **AND** 選択済み primary adapter package dry-run または compile-equivalent gate が走る

### Requirement: docs and task IDs stay aligned with the repo-local root plan

KUC repo docs MUST task IDs を `docs/architecture/ui-separation/root-plan-source.md` と揃える。
KUC-specific OpenSpec tasks MUST root plan task IDs を保持する。

#### Scenario: repo-local root task ID changes

- **WHEN** `docs/architecture/ui-separation/root-plan-source.md` の task ID が追加、削除、rename される
- **THEN** `docs/ui-separation-plan.md` とこの OpenSpec change を drift について確認する
- **AND** implementation runner は `root-plan-source.md` を先に更新せず local-only task を作らない

### Requirement: implementation status uses OpenSpec evidence

この change の tasks MUST implementation と verification evidence が存在するまで unchecked のままにする。
lint 通過だけで task 完了にしてはならず、その task の intended architectural boundary も満たさなければならない。

#### Scenario: task is marked done

- **WHEN** task checkbox を `[ ]` から `[x]` または `[/]` に変える
- **THEN** 対応する implementation、test、または documented evidence が存在する
- **AND** evidence は root UI separation boundary が満たされたことを示す

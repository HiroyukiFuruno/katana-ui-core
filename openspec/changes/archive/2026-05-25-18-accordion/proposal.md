## Why

> Archive candidate: Accordion の KUC 実装要件は `openspec/changes/establish-kuc-atoms-molecules-catalog/` へ移管する。この change は要件移管後に archive 候補として扱う。

設定パネル / FAQ / セクション分割で「ヘッダクリックで本体を折り畳む」UI が要る。
root plan 適用後は repo 外の実装を直接読まず、必要な挙動を KUC の中立 model と自動契約へ移して実装する。
完了根拠は Storybook の目視ではなく、state / event / action contract、render props、入力回帰、Storybook requirement gate とする。

## What Changes

- `molecule::Accordion` に expanded、disabled、controlled、indicator、trigger area、tree mode、reduced motion、body border を持たせる。
- `molecule::AccordionGroup` に single / multiple の同時展開制御を持たせる。
- indicator は `Leading` / `Trailing` / `None` を扱う。
- trigger area は icon only / text only / icon + text / full row を扱う。
- 高さアニメーションは reduced motion で抑制できる render contract として扱う。

## Capabilities

### New Capabilities

- `widget-accordion`: セクションの折り畳みコンテナ。chevron 配置、trigger area、tree mode、controlled request、group 展開制御を統一。

## Impact

- 設定 UI、ヘルプセクション、リスト項目の詳細展開などで利用。

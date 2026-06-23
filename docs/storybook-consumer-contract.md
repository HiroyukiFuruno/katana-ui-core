# Storybook Consumer Contract

## 目的

Storybook page の `ready` は、見た目、手動操作、ログ出力だけで判断しない。
外部利用者が `katana-ui-core` を使うときと同じ契約が、typed かつ自動検証可能な形で揃っているときだけ `ready` とする。

## Ready 条件（必須）

1. `public API`
   `widget::atoms` / `widget::molecules` の公開契約で同じ部品が構築できる。
2. `typed props / options`
   settings 変更が文字列表示ではなく typed option の before/after で追跡できる。
3. `typed state`
   state id と component state が page 固有ではなく component ownership で管理される。
4. `typed action`
   操作後に action が空ではなく、対象 state への更新として記録される。
5. `typed event / log`
   event log は target、before、after を含む typed callback log で追跡できる。
6. `layout bounds`
   panel、preview、inspector、scrollbar の bounds/hit target が契約テストで検証される。
7. `hit target`
   クリック可能領域は表示だけでなく hit rect と入力回帰で検証される。
8. `rendering contract`
   非空描画、theme 差分、操作後差分を数値で検証する。
9. `fallback 禁止`
   required page で generic fallback を使わない（`required_ui_fallbacks=0`）。
10. `Storybook 専用状態の禁止`
    Storybook 専用の簡易 state を `ready` 根拠にしない。core の component state 契約を使う。

## 禁止事項

- Storybook の目視確認、スクリーンショット、静的な表示追加を完了根拠にしない。
- fallback を置いただけの page を `ready` にしない。
- action/event が空のまま `ready` にしない（受動部品は受動契約の自動テストを必須とする）。
- Storybook の都合だけで作った state を、利用者向け契約の代わりにしない。

## ガード運用

- `scripts/assert-storybook-consumer-contract.py` で、この文書、tasks、既存 gate の接続を確認する。
- `tasks.md` は「全 Storybook page の readiness audit」を最優先の未完了タスク（`- [ ]`）として残し、audit 完了前に `ready` 扱いへ戻さない。
- `just kuc-guardrails` で self-test と本検査を毎回実行する。

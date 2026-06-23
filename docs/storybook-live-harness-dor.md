# Storybook Live Harness DoR

作成日: 2026-06-13

## 結論

Storybook の修正作業は、解析レーンと実作業レーンを分離する。
実作業は、この文書に固定した DoR を満たした P0 から着手する。

## レーン定義

### 解析レーン

解析レーンは、実装前に次を固定する。

- UI ごとの要件、公開 API、option、state、action、event、callback。
- Storybook が実部品を動かしているか、Storybook 専用の絵や state で代替していないか。
- 実操作検証の入口と、その入口が何を保証していないか。
- P0/P1/P2 の修正順序。

解析レーンでは、UI 本体や Storybook の挙動を変更しない。

### 実作業レーン

実作業レーンは、解析レーンで固定済みの P0 を対象にする。
実装は回帰テストまたは guard と同時に行う。

## 現時点の監査結果

### 検証入口

| 入口 | 現状 | 完了根拠としての扱い |
| --- | --- | --- |
| `storybook-reflection-audit` | page / requirement 接続を検査する | 実部品の動作保証ではない |
| `storybook-requirement-gate` | headless scenario の集計値を検査する | 実操作の網羅保証ではない |
| `storybook-interaction-smoke` | `storybook-requirement-gate` を呼ぶだけ | interaction smoke として未成立 |
| checkbox / radio targeted test | direct click 経路を検査する | 実画面クリック経路の保証として不足 |

### P0 不足

- checkbox / radio は、実画面で state が変わらない報告がある。
- 既存 targeted test は通っているため、テストが実画面経路を十分に固定していない。
- `storybook-interaction-smoke` は名前と実態がずれている。

## 実作業レーンへ進む条件

P0 実装へ進む前に、次を満たす。

- `storybook-interaction-smoke` の責務を「実操作 smoke」に戻す作業単位を切る。
- checkbox / radio について、表示される mark、label、row、disabled、focus、reset、複数 instance の state 分離を検査対象にする。
- native window 経路または同等の座標変換経路を通る回帰テストを作る。
- 実装完了判定を screenshot や手動目視に置かない。

## P0 作業順

1. `storybook-interaction-smoke` に live interaction audit を追加する。
2. checkbox の実画面クリック経路を回帰テストで固定する。
3. radio の実画面クリック経路を回帰テストで固定する。
4. 実装修正後、state / action / event / preview の同期を確認する。

## 禁止事項

- Storybook 専用 state だけを動かして実部品の修正扱いにする。
- Inspector の表示だけ変えて動作完了扱いにする。
- Katana 固有仕様を KUC の public API に混ぜる。
- screenshot を完了根拠にする。

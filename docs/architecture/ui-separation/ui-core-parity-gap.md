# UI Core parity gap

## 結論

`ui-core-root-plan`、`katana-widget-parity-backlog`、`ui-core-interaction-visual-parity` の完了記録だけでは、01〜24 が KUC の実用 atoms / molecules として完了したとは扱わない。
これらの記録は旧基準の証跡であり、現在の完了条件は `establish-kuc-atoms-molecules-catalog` に移す。

## 旧基準で確認済みだったこと

| 項目 | 旧基準の扱い |
| --- | --- |
| root architecture | framework-neutral core、adapter 境界、runtime / window / surface の親設計 |
| 最低構造 | 必須 story、最低 node 構造、state id 衝突の確認 |
| Storybook visual | 旧 panel screenshot、light / dark、操作後差分、modal window 証跡 |
| interaction report | story selection、theme switch、operation sequence、callback log |
| visual fallback | generic fallback を完了根拠にしない方針 |

上記は履歴としてだけ扱う。新基準では部品ごとの option / action / event / state / preset / preview / settings / 自動テスト / 数値化された layout / rendering contract / Storybook ページを満たす必要がある。

## 新基準で未完了扱いに戻す理由

- Storybook は静的見本帳ではなく、選択中 UI の layout / option / action / event / state / rendering / panel 独立 scroll を実画面で扱うフィードバック用の画面である。
- Storybook は部品の正しさを単独で証明しない。
- 自動テスト、数値化された layout / rendering contract、input regression、guard を CI/CD 品質ゲートにする必要がある。
- 日本語入力（IME）、OS 絵文字、英日混在テキストの上下中央揃えは core 基盤の契約として検証する必要がある。
- 旧個別 change の完了チェックは現在の KUC 公開 API 形状を保証しない。

## 移管先

| 旧記録 | 移管先 |
| --- | --- |
| 01〜24 の部品要件 | `establish-kuc-atoms-molecules-catalog/tasks.md` |
| Storybook 操作面 | `specs/kuc-storybook-catalog/spec.md` |
| 自動品質ゲート | `specs/kuc-quality-gates/spec.md` |
| core theme / font / text / input / event / state / layout | `specs/kuc-core-foundation/spec.md` |
| atoms / molecules 公開境界 | `specs/kuc-widget-layer/spec.md` |

## 次の判定

次に完了を判断する時は、次の順で見る。

1. `openspec validate establish-kuc-atoms-molecules-catalog --strict`
2. core / atoms / molecules の契約テスト
3. 数値化された layout / rendering contract
4. input regression
5. state / event / action contract
6. guard

Storybook は release readiness の根拠にしない。要件行から 1〜6 のいずれかの自動検査へ追跡できない場合は、テストシナリオ漏れとして扱う。

## Storybook 未反映監査

`just storybook-reflection-audit` は、required page が Storybook の固有画面へ反映されているかを監査する。
この監査は、ページが存在するだけ、汎用 renderer へ落ちるだけ、汎用 preset / 汎用 interaction spec へ逃げるだけの状態を未反映として扱う。

監査対象:

- required page が `dedicated.rs` の page 固有 surface に接続されていること。
- required page が page 固有 preset label を持つこと。
- required page が option / action / event / state の明示 spec を持つこと。

現在の用途:

- `just check` の代替ではなく、Storybook 完成度の不足を一覧化するための明示監査とする。
- v0.1.0 release readiness へ昇格する前に、この監査の `missing-*` を 0 にする。

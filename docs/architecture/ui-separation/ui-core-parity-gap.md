# UI Core parity gap

## 結論

`ui-core-root-plan`、`katana-widget-parity-backlog`、`ui-core-interaction-visual-parity` の完了記録だけでは、01〜24 が KUC の実用 atoms / molecules として完了したとは扱わない。
これらの記録は旧基準の証跡であり、現在の完了条件は `establish-kuc-atoms-molecules-catalog` に移す。

## 旧基準で確認済みだったこと

| 項目 | 旧基準の扱い |
| --- | --- |
| root architecture | framework-neutral core、adapter 境界、runtime / window / surface の親設計 |
| 最低構造 | 必須 story、最低 node 構造、state id 衝突の確認 |
| Storybook visual | panel screenshot、light / dark、操作後差分、modal window 証跡 |
| interaction report | story selection、theme switch、operation sequence、callback log |
| visual fallback | generic fallback を完了根拠にしない方針 |

上記は有用な履歴だが、新基準では部品ごとの option / action / event / state / preset / preview / settings / 自動テスト / 画像回帰 / Storybook ページを満たす必要がある。

## 新基準で未完了扱いに戻す理由

- Storybook は部品カタログであり、部品の正しさを単独で証明しない。
- 自動テスト、layout regression、visual regression、input regression、guard を CI/CD 品質ゲートにする必要がある。
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
3. layout regression
4. visual regression
5. input regression
6. guard
7. Storybook 部品カタログの実画面確認

Storybook のスクリーンショットは最後の補助証跡であり、1〜6 の代替にしない。

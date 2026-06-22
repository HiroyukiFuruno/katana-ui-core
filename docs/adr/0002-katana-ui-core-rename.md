# ADR-0002: `katana-ui-widget` を `katana-ui-core` にリネームし、UI Core の責務を明示する

ステータス: Accepted
決定日: 2026-05-17
ローカル根拠: `docs/architecture/ui-separation/root-plan-source.md`, `docs/ui-separation-plan.md`

## コンテキスト

`katana-ui-widget` は当初「Adapter 向け共通 UI widget の共通基盤」として設計された。
今回の UI 分離構想で以下を整理した結果、本 repo の責務が「画面部品（widget: atoms / molecules）の集合」を超えていることが明らかになった。

本 repo の実体は **フレームワーク非依存（framework-neutral）な UI Core** であり、以下を所有する。

- Component model / render model (`UiTree` / `UiNode` 等)
- theme token
- event model
- external runtime contract (framework-specific UI / native が消費する中立 trait)
- **window / runtime / surface API** (`Application::new().window(...).run()` のような entry point)
- atoms / molecules (widget primitive はその一部)

「widget」という名称はこの全体像を表現しきれず、以下のリスクを生む。

- 利用側が「atoms / molecules しか入っていない crate」と誤認する。
- window / runtime API の追加が「widget の範囲外」と判断され開発が遅れる。
- 外部利用者の発見性が下がる。UI Core を探す人は `core` を検索する。

## 決定

1. GitHub repo 名を `katana-ui-widget` から `katana-ui-core` にリネームする。
2. Cargo crate 名も `katana-ui-core` に変更する。
3. external runtime / renderer crate は KUC active workspace の管理対象外にする。
4. storybook crate 名を `katana-ui-core-storybook` に変更する。
5. 略語を `KUW` から `KUC` に変更する。新規 docs では KUC を使う。
6. UI 分離構想の Phase 1 で **window / runtime / surface module** を core に追加する。

window API の中立化（neutral）粒度は「中」とする。
標準 API に含める範囲は title / size / close / focus / fullscreen / multi-window / icon。
platform menu / IME / drag & drop は標準 API には入れず、external runtime 経由の逃がし口（escape hatch）で扱う。

## 理由

- 未公開段階なので rename cost が小さい。
- 命名と実体を合わせることで、将来の保守時に責務を誤解しにくくなる。
- KUC 略語は「core」を反映しており、KDV / KLE / KCF / KDR / KMM / KCU と同じ形式で読める。
- window / runtime / surface を core に置くことで、external runtime / renderer は共通の neutral API を消費する責務に集中できる。

## 代替案

### 案 A: `katana-ui-widget` のまま runtime / window module を追加する

却下理由: 名前と実体のズレが大きくなり、外部利用者の発見性も悪化する。

### 案 B: `katana-ui-runtime` と `katana-ui-widget` に分割する

却下理由: runtime と widget を分けると、利用側の dependency 数と設計判断が増える。
1 crate に集約しても責務境界は module と external runtime contract で表現できる。

### 案 C: window API の neutral 化粒度を「最小」または「最大」にする

却下理由:

- 最小では editor 系 product で実用上必要な fullscreen / multi-window が不足する。
- 最大では platform menu / IME / drag & drop まで neutral 化が必要となり、framework-specific UI の差異が大きく破綻しやすい。
- 「中」は一般的な desktop runtime の共通サポート範囲とほぼ一致し、特殊機能は external runtime contract 拡張で逃がせる。

## 影響

### 直ちに影響するもの

- `docs/ui-separation-plan.md`
- `docs/architecture/ui-separation/root-plan-source.md`
- `openspec/changes/ui-core-root-plan/`
- `README.md`
- `Cargo.toml`
- `Justfile`
- `scripts/`
- `storybook/Cargo.toml`
- `docs/directory-structure.md`
- `docs/release.md`

### P0-B 系タスクとして実施するもの

- GitHub repo 名の `katana-ui-core` 化。
- Cargo.toml の crate 名変更。
- adapter / storybook crate 名の変更。
- Justfile / scripts / release / publish dry-run / release target verify の参照更新。
- README / docs / OpenSpec 新規 change の KUC 表記化。
- 関連 repo の dependency 表記追従は KUC repo 内では実装しない。追従が必要な場合は `docs/external-followups.md` に repo 名、変更対象ファイル、期待する dependency 表記、KUC 側の根拠 task ID を記録する。

### 過去 OpenSpec changes / 履歴文書

既存の `openspec/changes/archive/` 配下と過去 PR / handoff / tmp 文書は触らない。
`katana-ui-widget` 表記は歴史的事実として残す。
新規作成する OpenSpec changes は `katana-ui-core` 表記とする。

## 検証

- [x] 新規 docs と新規 OpenSpec change で `katana-ui-widget` 表記を増やしていない。ただし履歴説明は除外する。
- [x] 略語表に KUC が登録され、新規 docs で KUW を使っていない。
- [x] window / runtime / surface module が `root-plan-source.md` と `ui-core-root-plan` tasks に含まれている。
- [x] P0-B に rename 実施タスクが明記されている。

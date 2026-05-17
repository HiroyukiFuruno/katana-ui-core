# Widget extraction policy

作成日: 2026-05-17
対象: KUC に取り込む画面部品（widget）と UI model

## 目的

`katana-ui-core` に入れてよい UI と、入れてはいけない UI を明確にする。
KUC は Floem 専用 crate ではなく、フレームワーク非依存（framework-neutral）な UI Core である。

## 採用条件

画面部品（widget）または UI model は、次の条件をすべて満たす場合だけ KUC に入れる。

1. **Framework-neutral**: public API が Floem View / GPUI Element / egui Ui を返さない。
2. **Domain-neutral**: Katana の document、editor、linter、chat session、workspace model に依存しない。
3. **Render-model first**: `UiTree` / `UiNode` / `UiProps` で表現できる。
4. **Theme-token based**: 色、余白、角丸、影、z-index を theme token 経由で扱う。
5. **Adapter-ready**: Floem / GPUI / egui adapter が変換できる DTO / trait 境界を持つ。
6. **Repo-local evidence**: repo 外の実装を直接読まず、`docs/inventory/*.md` または OpenSpec change にコピー済みの根拠から実装できる。

## 除外条件

以下は KUC core には入れない。

| 対象 | 除外理由 | 置き場所 |
| --- | --- | --- |
| Markdown rendering panel | KMM / document model に依存する | consumer crate または document viewer |
| Chat composer | vendor UI protocol と chat session に依存する | chat domain |
| Linter result list | lint domain object に依存する | consumer crate |
| Workspace file tree | project model に依存する | application UI |
| Editor gutter / ruler | editor document model に依存する | language editor adapter |
| Language server status | LSP integration に依存する | consumer crate |
| File diff approval UI | file path、approve / reject、multi-file navigation を含む | application UI |

ただし、domain を持たない表示専用の `CodeDiff` のように、2つの文字列を比較するだけの部品は KUC に入れてよい。

## Reference inputs

実装 runner は repo 外を直接読まない。
既存実装の挙動が必要な場合、先に repo 内へ根拠をコピーする。

許可する入力:

- `docs/inventory/*.md`: 既存 UI 挙動のコピー済み要約
- `openspec/changes/*/design.md`: 明示された設計
- `openspec/changes/*/specs/*/spec.md`: 受け入れ条件
- `storybook/`: repo 内にある画面確認用 sample

新しい widget が repo 外の挙動を参考にする場合は、先に `docs/inventory/<widget>.md` を作り、以下を記録する。

- 画面上でどう見えるか
- 何を操作するものか
- 入力 props
- 出力 event
- 状態遷移
- adapter で再現すべき見た目
- KUC core には入れない domain 要素

## 互換 adapter との関係

KUC core は neutral DTO / trait を提供する。
Floem / GPUI / egui 固有の描画は adapter crate に置く。
対応範囲と未対応機能は [`docs/compat-adapters.md`](compat-adapters.md) に記録する。

## 旧KUW由来の扱い

旧 `katana-ui-widget` / `KUW` 時代の文書や OpenSpec archive は履歴として残す。
新規作業では `katana-ui-core` / `KUC` と framework-neutral 方針を使う。

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
7. **Storybook-ready**: preview、settings、preset、state、event、action 履歴を Storybook で表現できる。静的見本ではなく、layout、option、action、event、state、rendering、panel 独立 scroll を実画面で扱える。
8. **Test-gated**: Storybook や手動操作ではなく、自動テスト、数値化された layout / rendering contract、入力回帰、guard で完了判定できる。

ドラッグ&ドロップ（drag and drop）系の UI は、domain 固有のファイルツリーや chat 添付 model を持ち込まず、`DragData`、`DropTarget`、`DragHandle`、`DropIndicator`、`DragPreview` の neutral DTO として表現できる場合だけ採用する。
OS ファイル、URL、テキストは adapter が `os/file-list`、`os/url`、`os/text` に変換し、core は payload の中身を解釈しない。

ContextMenu は、pointer 起動、node 起動、仮想矩形起動を `ContextMenuAnchor` として core 側で扱う。
右クリック位置や window 座標の取得は adapter 責務、項目内容や command の意味づけは consumer 責務とし、core は anchor、placement、item kind、callback log の契約だけを持つ。

Tabs は segmented な切替 UI として扱い、close button、dirty 表示、pin、group、drag reorder、overflow menu を持たせない。
それらが必要な場合は CloseableTabStrip を使い、workspace、document、chat session、file path の意味は consumer 側 state に閉じ込める。

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
| App shell | 画面全体の構造であり organisms / templates に近い | consumer crate |
| Title bar / window chrome | OS window / adapter 固有責務を含む | consumer crate / adapter |
| Splash screen template | 起動 lifecycle と branding を含む | consumer crate |
| Chat root / message thread / composer | chat session / vendor protocol に依存する | chat domain |

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

## Atomic design の扱い

MVP の公開対象は最小部品（atoms）と組み合わせ部品（molecules）である。
大きな画面単位（organisms）、画面ひな形（templates）、画面（pages）は今は公開対象にしない。

ただし、Storybook 自身を構成する shell、navigation、preview workspace、settings inspector、panel scroll state は内部構成部品として許可する。
Storybook は中央本文に全 UI のカード一覧を並べる静的見本帳ではなく、左 TreeView で選んだ UI の操作、状態、描画差分を扱う画面として扱う。
これらを公開 widget API へ昇格させる場合は、別 OpenSpec change で目的、対象、利用側 API を定義する。

## 旧個別 change の扱い

旧文書や OpenSpec archive は履歴として残す。
新規作業では `katana-ui-core` / `KUC` と framework-neutral 方針を使う。
01〜24 の現在の実装正本は `openspec/changes/establish-kuc-atoms-molecules-catalog/` である。

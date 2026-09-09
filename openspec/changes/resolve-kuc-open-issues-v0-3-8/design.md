## Context

KUC v0.3.7 は `raster-host` と四辺別 grid border を公開したが、KDV の exact registry consumer は canonical crop 91/95 であり、Issue #35 は再オープンしている。Issue #37 の API 実装は公開済みだが、KDV consumer の registry-only acceptance が未完了である。加えて、#40 は固定8 scenario だけを扱う artifact API、#43 は logical/physical scale の不整合、#44 は session projection に保存されない search state を問題としている。

この変更は KUC が所有する contract を修正する。KDV と KLE は公開済み registry crate を消費して evidence を作るが、KUC の raster、RawInput、search visibility、renderer callback、KatanA 固有 semantics を再実装しない。

## Goals / Non-Goals

**Goals:**

- KUC #35、#37、#40、#43、#44 を v0.3.8 の同一 public contract で解決する。
- raster の physical texture と logical layout / paint / hit bounds を同じ変換で扱い、document typography も同一の最終 metrics を使用する。
- consumer-defined generic leaf を KUC-owned stage と opaque artifact evidence に結合し、KLE が RawInput や renderer を所有しない。
- retain一回・毎frame synchronizeで search の close/reopen と保存対象の操作状態を決定的に投影する。
- registry-only KDV consumer の canonical crop 95/95 と grid border acceptance を、閾値や reference を変えずに検証する。

**Non-Goals:**

- KatanA、KDV、KLE の domain type、Markdown、path/URL、座標、renderer callback を KUC public API に追加しない。
- KDV 固有の倍率補正、reference の置換、score threshold の緩和、consumer の独自 raster/hit-test は行わない。
- fixed `FullTextCommandSurfaceScenarioId`、既存 `write_opaque`、uniform `UiBorder` の互換動作を変更しない。

## Decisions

### 1. 一つの release line と一つの統合ゲート

全変更は `release/v0.3.8` に積み上げ、個別の作業単位では全 workspace quality gate を実行しない。コードと focused regression を追加した後、統合 HEAD で format、lint、workspace test、consumer contract、coverage、release-check、3 OS CI、Draft PR review、registry consumer acceptance を順に実行する。

v0.3.7 からの API は additive とし、既存 API の削除・型変更が必要と分かった場合は実装を止めて version を再評価する。

### 2. raster metrics は KUC 内で一度だけ変換する

texture allocation と raster request は physical pixel を使い、layout reservation、`UiRect`、clip、hit bounds は `ceil(physical / scale)` の logical extent を使う private helper に統一する。`max_width_px` は logical width として request に渡し、text raster layout だけが physical width への変換を行う。

TextSurface gutter / placeholder、Diagnostics、同じ helper を使わない StatusBar / CommandChrome を監査対象とする。raster host の document typography は role ごとの final font size、line height、baseline を render、wrap、layout、hit-test に渡し、KDV で検出した 91/95 の差は KUC の numerical regression と registry consumer acceptance の両方で再現・解消を確認する。

### 3. consumer artifact plan は opaque で KUC が stage を発行する

`ConsumerArtifactPlanV1` は opaque leaf ID、閉じた `GenericInteractionClass`、閉じた `GenericEffectClass`、opaque host-projected token、KUC-issued stage binding だけを受け取る。v1 の interaction class は text input、IME commit、selection、scroll、toolbar activation、floating toolbar、search、context menu、accessibility activation、viewport resize に限定する。consumer が独自 class、RawInput、renderer callback、document content、座標を渡す経路は提供しない。

issuer は同じ retained root を stage 間で保持し、順序付き one-shot stage だけを発行する。KUC writer は stage ごとの numbered PNG、decode/pixel/root-record hash、current-frame AccessKit hash、`⭐️` の color glyph・measurement・hit-test evidence、one-shot forwarding receipt を artifact manifest へ結合する。opaque token と host target は Debug、Serde、manifest に出力しない。

空 plan、未知 schema / class、不完全 plan、重複 leaf・binding・stage、stale revision、token/root mismatch、stage skip / reorder、receipt reuse / cross-bind、既存 media、hash・PNG・AccessKit・Unicode evidence mismatch は typed error で fail-closed にする。既存の固定 scenario と variable viewport writer は互換維持する。

### 4. scenario session は表示状態を含む KUC-owned projection とする

`ScenarioSessionState` に search visibility、options、replace mode、result position を持たせ、`ScenarioSessionUpdate` が対応する event を保存する。close は次 revision の presentation を `search: None` にし、明示的な `reopen_search()` だけが visible state を戻す。query と replace value は保存するが、navigation / replace request の外部効果、検索結果、文書本文を session が推測して変更しない。

これにより session は KLE に visible flag、callback、child setter を要求せず、#40 の stage machine も同じ root projection を使える。

### 5. 下流受入は公開後の exact registry artifact で行う

KUC release 後、KDV Issue #48 の既存 consumer worktree で `katana-ui-core = "=0.3.8"` を registry-only で解決し、path / git override なしの boundary check、per-side border regression、canonical crop 95/95 を実行する。KLE は同じ公開版だけから consumer artifact plan の 3 OS evidence を生成する。下流の実装・lockfile更新は上流 artifact の確認後に行う。

## Risks / Trade-offs

- [KDV crop の差が KUC 以外にある] → KUC の numerical regression と KDV exact registry run を同じ reference / crop convention で照合し、consumer固有補正を導入しない。
- [HiDPI helper の横断適用で既存 layout が変わる] → TextSurface、Diagnostics、StatusBar、CommandChrome を scale 1/1.5/2 の個別 regression で監査し、同じ logical bounds を paint と hit-test に使用する。
- [artifact plan が domain API を漏らす] → public foreign-consumer compile test と AST guard で KLE/KatanA型、RawInput construction、renderer callback、payload accessor を拒否する。
- [session が外部効果まで保存する] → navigation / replace の receipt は保存しても、result / document state を session state に入れない regression を追加する。
- [公開後の下流受入が失敗する] → KUC #35/#37 は close せず、失敗した exact artifact、score、boundary evidence を Issue に記録して KUC側の修正へ戻る。

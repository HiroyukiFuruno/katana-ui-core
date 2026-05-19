# Tasks — 04-add-rich-popover-and-hover-card

## 1. 設計確定

- [ ] 1.1 `HoverCard` の trigger / delay / pointer follow / slot 構成を確定する。
- [ ] 1.2 `Popover` の追加 option（arrow / slots / focus_management / keep_open_on_inner_focus / auto_flip_priority）を確定する。
- [ ] 1.3 共通 `PlacementRequest` / `PlacementResult` を確定する。

## 2. 共通 placement engine

- [ ] 2.1 `interaction/placement.rs` に純関数 `resolve_placement` を実装する。
- [ ] 2.2 priority list flip、viewport clamp、arrow offset 計算を実装する。
- [ ] 2.3 単体テストで anchor 3 種類 × placement 8 種類 × viewport edge ケースを検証する。

## 3. HoverCard 実装

- [ ] 3.1 `molecule/disclosure/hover_card.rs` を新設し、option / action / event / state を実装する。
- [ ] 3.2 open / close delay を `state` に持たせ、timer 切替えを純関数 step として実装する。
- [ ] 3.3 pointer follow を `anchor` の Pointer mode に統合する。
- [ ] 3.4 actions slot の interactive node にフォーカスが入った場合の keep_open を実装する。
- [ ] 3.5 `widget::molecules` の re-export に `HoverCard` を追加する。

## 4. Popover 拡張

- [ ] 4.1 `Popover` の option 構造体に arrow / slots / focus_management / keep_open_on_inner_focus / auto_flip_priority を追加する。
- [ ] 4.2 open 時の focus 移動と close 時の focus return を実装する。
- [ ] 4.3 arrow 描画 model を実装する（actual paint は adapter）。
- [ ] 4.4 既存 Popover preset を破壊しないよう、追加 option はデフォルト値で互換性を保つ。

## 5. 既存 molecule の placement 移行

- [ ] 5.1 `Tooltip` を共通 placement engine に切替える。
- [ ] 5.2 `ContextMenu` を共通 placement engine に切替える。
- [ ] 5.3 `Menu` / `MenuButton` のパネル配置を共通 placement engine に切替える。
- [ ] 5.4 `SelectBox` / `ComboBox` のドロップダウンを共通 placement engine に切替える。

## 6. 自動テスト

- [ ] 6.1 HoverCard の open / close delay が正しく state を遷移することを検証する。
- [ ] 6.2 pointer 移動が card 本体に入ると close delay が一時停止することを検証する。
- [ ] 6.3 actions slot に focus が入ると keep_open になることを検証する。
- [ ] 6.4 Popover の focus_management = FirstInteractive が最初の interactive 要素にフォーカスすることを検証する。
- [ ] 6.5 close 時に元 focus holder にリターンすることを検証する。
- [ ] 6.6 arrow offset が panel 端 clamp を尊重することを検証する。
- [ ] 6.7 共通 placement engine が既存 molecule の placement test を回帰なくパスすることを検証する。

## 7. 画像回帰

- [ ] 7.1 HoverCard の placement 8 種類、arrow 表示、slot 構成（heading / body / footer / actions）を回帰する。
- [ ] 7.2 Popover の slots 構成、arrow、focus_management 動作を回帰する。
- [ ] 7.3 Tooltip / ContextMenu / Menu の placement 回帰（共通 engine 切替え後の差分）を回帰する。
- [ ] 7.4 light / dark theme での arrow tone を回帰する。

## 8. Storybook ページ

- [ ] 8.1 `Disclosure > HoverCard` ノードを catalog に追加する。
- [ ] 8.2 HoverCard preset「default」「pointer follow」「focus trigger」「rich content」「actions」を実装する。
- [ ] 8.3 Popover 既存ページに「arrow」「slots」「focus_management」preset を追加する。
- [ ] 8.4 settings で delay / placement / arrow / focus / slot を切替えできるようにする。

## 9. ドキュメント

- [ ] 9.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に HoverCard 行を追加する。
- [ ] 9.2 共通 placement engine の責務を `docs/widget-extraction-policy.md` に明記する。

## 10. 品質ゲート

- [ ] 10.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 10.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 10.3 `openspec validate 04-add-rich-popover-and-hover-card --strict` をパスする。
- [ ] 10.4 画像回帰 / 入力回帰の CI gate をパスする。

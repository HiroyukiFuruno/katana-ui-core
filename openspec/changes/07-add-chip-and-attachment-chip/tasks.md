# Tasks — 07-add-chip-and-attachment-chip

## 1. 設計確定

- [ ] 1.1 Chip の option（variant / tone / size / interactive / selected / disabled / dismissible / slots）を確定する。
- [ ] 1.2 AttachmentChip の `kind` enum と `status` enum、progress 制御を確定する。
- [ ] 1.3 ChipGroup の overflow strategy（None / Menu / ScrollHorizontal）を確定する。

## 2. 中核実装

- [ ] 2.1 `atom/chip.rs` を新設し `Chip` atom を実装する。
- [ ] 2.2 `molecule/attachment_chip.rs` を新設し `AttachmentChip` molecule を実装する。
- [ ] 2.3 `molecule/chip_group.rs` を新設し `ChipGroup` molecule を実装する。
- [ ] 2.4 `widget::atoms` / `widget::molecules` の re-export を更新する。

## 3. 連携

- [ ] 3.1 ChipGroup overflow=Menu に既存 `Menu` molecule を使う。
- [ ] 3.2 ChipGroup の reorder opt-in に `02-add-drag-drop-primitive` を使う。
- [ ] 3.3 AttachmentChip の status=Error 時の retry action は `Button` atom を子に持つ。

## 4. 自動テスト

- [ ] 4.1 Chip の dismissible + Backspace / Delete で `ChipDismissed` が発火することを検証する。
- [ ] 4.2 Chip の disabled が press / dismiss を抑止することを検証する。
- [ ] 4.3 AttachmentChip の status 遷移（Pending → Uploading → Ready / Error）が `AttachmentChipStatusChanged` を順に発火することを検証する。
- [ ] 4.4 Error → Retry action → status が Pending に戻ることを検証する。
- [ ] 4.5 ChipGroup overflow=Menu で表示幅を超えた chip が menu に集約されることを検証する。
- [ ] 4.6 ChipGroup overflow=ScrollHorizontal でスクロール量を state に持ち、event 発火されることを検証する。
- [ ] 4.7 reorder opt-in で `ChipReordered` 発火、 disable 時は drag が無視されることを検証する。

## 5. 画像回帰

- [ ] 5.1 Chip の variant × tone × size 全組合せの主要 subset を回帰する。
- [ ] 5.2 dismissible / selected / disabled / focused の状態を回帰する。
- [ ] 5.3 AttachmentChip の kind × status × progress を回帰する。
- [ ] 5.4 ChipGroup の wrap / horizontal scroll / overflow menu 展開を回帰する。
- [ ] 5.5 light / dark theme での tone を回帰する。

## 6. Storybook ページ

- [ ] 6.1 `Atom > Chip` ノードを catalog に追加する。
- [ ] 6.2 `Molecule > AttachmentChip` ノードを追加する。
- [ ] 6.3 `Molecule > ChipGroup` ノードを追加する。
- [ ] 6.4 preset「フィルタタグ」「ファイル添付」「画像添付」「URL 添付」「アップロード中」「エラー」「overflow」を実装する。
- [ ] 6.5 settings で variant / tone / size / status / progress を切替えできるようにする。

## 7. ドキュメント

- [ ] 7.1 `docs/architecture/ui-separation/owned-ui-task-map.md` に Chip / AttachmentChip / ChipGroup 行を追加する。
- [ ] 7.2 `Badge` Storybook ページに「dismiss / interactive は Chip」リンクを追記する。

## 8. 品質ゲート / DoD

- [ ] 8.1 `cargo test -p katana-ui-core` をパスする。
- [ ] 8.2 `cargo clippy -p katana-ui-core --all-targets -- -D warnings` をパスする。
- [ ] 8.3 `openspec validate 07-add-chip-and-attachment-chip --strict` をパスする。
- [ ] 8.4 画像回帰 / 入力回帰の CI gate をパスする。

# タブ操作と検索履歴の追加設定

既定の操作は維持される。設定は公開 `egui::text_command_surface` から取得でき、hostはopaque target、correlation、表示値と操作結果だけを扱う。

## グループ名

`TabStripGroupPopupPresentation::rename_policy(TabStripRenamePolicy::OnChange)` は、確定した各入力変更を `RenameGroup` として通知する。空名も通知する。preeditは確定名に含めない。既定の `OnSubmit` は従来どおりEnterで確定する。

変更時通知を採用したhostは受信値を保存して同じgroupへ再投影できる。KUCが編集中のdraftとfocusを保持する。Enterで同じ変更を再通知せず、Escapeや外側clickでも通知済みの値を巻き戻さない。

`request_inline_rename(true)` の要求だけを編集開始とは扱わない。`TabStripProposalPort::inline_rename_started` を実装し、`TabStripInlineRenameStarted` のgroupとcorrelationを確認してから要求を解除する。採用された描画passの成立を後続root frameで通知するため、要求後もroot描画を継続する。別popupで延期された要求や破棄されたpassからは成立を通知しない。既存port実装ではこの追加callbackは既定のno-opとなる。

## グループ色

`TabStripGroupDescriptor::color_presentation` へ `TabStripGroupColorPresentation::new(marker, background, collapsed_background, member)` を渡す。4値はRGBAを保持し、header marker、展開時背景、折りたたみ時背景、所属tabの色を設定する。popupのswatch候補へ現在色を追加する必要はない。未設定時は従来の表示となる。

## Drop

`TabStripProjection::drop_policy(TabStripDropPolicy::NearestGhostCenter)` で最近接挿入境界を選ぶ。既定は `Region`。KUCは開始pointerとghost中心のoffsetおよび幅を保持し、可視tab先頭、隣接間の中点、末尾から最も近い境界を選ぶ。同距離なら先の境界、可視tabがなければ挿入先なしとする。候補は実際のスクロールviewportと交差する表示領域から構成する。可視末尾より後ろに隠れたtabがある場合は、その可視tabの直後を `After` で通知し、strip全体の末尾と区別する。末尾への挿入には既存の `tab_drop_at_end_available` capabilityも必要である。hostは既存のopaque配置操作だけを処理する。

## 操作起点

`TabStripProposal::origin()` は `Option<TabStripProposalOrigin>` を返す。公開rootの操作は本体、末尾control、context menu、navigation、group header/popup、drag終端を区別する。従来の `TabStripProposal::new` で作った値は `None`。host独自の操作順位はhostに残す。

## 検索履歴

`SelectionDialogProjectionLease::query_arrow_key_policy(mode, SearchQueryArrowKeyPolicy::History)` を使う。既定は `Results` で結果行を移動する。存在しないmodeやFile投影には設定できない。

queryにfocusがある場合、Upは新しい履歴から古い履歴へ移動し、最古で止まる。Downは新しい履歴へ戻り、最後に開始時のdraftを復元する。KUCがfocus、cursor、draft、キー消費、IME調停を管理する。hostのキー監視やfocus復旧は不要である。

操作結果は `QueryHistoryNavigated { mode, target, value, direction }` で届く。draft復元ではtargetが `None` となる。通常の `QueryChanged`、明示的な行選択の `HistorySelected` と区別できる。値の変化がない操作では通知しない。

通常の文字入力、mode切替、開閉、履歴の構成・順序・label・disabled変更、policy変更、hostによるquery上書きで履歴cursorを破棄する。同じ内容と現在queryの新leaseではcursorを維持する。履歴削除・消去はqueryを保持し、次の操作ではそのqueryをdraftとする。IME preeditおよびcommit frameの矢印を履歴移動へ使わない。

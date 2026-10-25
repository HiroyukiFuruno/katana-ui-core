# タブとグループのhost通知

hostはopaque targetと表示順序を投影し、KUCは描画・入力・menu・rename・drag/dropを管理する。
確定した状態、membership、採否、保存はhostが管理する。

| 操作 | Pointer | Keyboard | AccessKit |
| --- | --- | --- | --- |
| タブ選択 | labelをprimary click | focusしたlabelでEnter/Space | 投影label/boundsのButtonをClick |
| menuを開く | labelをsecondary click | 専用のmenu開始キーは未提供 | 専用のmenu開始actionは未提供 |
| グループ開閉 | collapsibleなheaderをprimary click | focusしたheaderでEnter/Space | headerのButtonをClick |
| 名前確定 | rename項目、またはhostのinline開始要求 | inputでEnter確定、Escape取消 | 投影placeholderを持つtext input |
| 閉じる・pin・色・group操作 | 投影されたcontrol/menu項目をclick | focusしたcontrolでEnter/Space | 投影labelのButtonをClick |
| タブ・グループの移動 | dragして有効なdrop位置でrelease | Escapeでdrag取消 | 専用のdrag actionは未提供 |

`TabStripProjectionLease::with_proposal_port`へ登録したportは、`root.show`が入力を処理する中で
一回だけ呼ばれる。`TabStripProposal::correlation()`でhostの投影と照合し、
`into_operation()`で対象・destination・確定値を取り出す。改名文字列は`TabStripText::as_str()`で取得する。
`forward_events_once`でこのport通知が重複することはない。
lease内の`TabStripProposalPortHandle::forward_once`がnonceごとに一回限りの配送を管理する。

タブdragは`StartDrag(source)`で対象を保持し、`FinishDrag`のdestinationと組み合わせて
hostの順序やmembershipを更新する。`CancelDrag`で保持対象を破棄する。
`ReorderTab` / `ReorderGroup`にはsourceとdestinationが一緒に入る。
hostは自身が有効とするcorrelationとtargetを照合し、受理できなければportから`Rejected`を返す。
表示位置やlabelから対象を推測してはならない。

hostが受理した変更は、新しいrevisionのprojectionへ再投影する。root直下の`tab`と`group`は
builderの呼出し順を保ち、一つの横scroll内へ並ぶ。KUCはpin優先やmembershipを推測しない。

`request_inline_rename`はrenamableな実targetとrename placeholderを必要とする。
開始済みの要求は再描画で再開始せず、lease同期でも同じopaque targetのdraft/focusを保持する。
targetが消滅するとoverlayを閉じ、別targetへ入力を転用しない。
AccessKitの操作IDもopaque targetと操作へ結合し、削除後に同じ表示位置へ来た別targetへ再利用しない。
操作別capabilityが無効なrouteはhostへ配送しない。最終的な採否はhost portが決める。

公開rootの実入力例は`egui_tab_strip_group_drag_contract`、混在順序とboundsの回帰は
`egui_tab_strip_mixed_order_issue_contract`を参照する。

## 継続操作とlease更新

タブのドラッグ中に有効な新leaseを同期しても、開始時のportとcorrelationを終端通知まで保持する。`FinishDrag`または`CancelDrag`は`StartDrag`を受けたportへ返し、通常の選択やメニュー操作は最新leaseのportへ渡す。hostは、開始時の操作終端を処理できる配線をドラッグ終了まで維持する。

inline rename要求は、一度開いた要求をtrueのまま再投影しても繰り返さない。確定または取消の後、`request_inline_rename(false)`を同期してからtrueへ戻すと、同じgroup targetへ次の要求を発行できる。通常メニューからの名前変更は、このinline要求フラグを必要としない。

# KUC atoms / molecules OpenSpec 自己レビュー

作成日: 2026-05-20

## 結論

初回整理後の OpenSpec は大枠では KUC の atoms / molecules 境界に沿っている。
ただし、KDV / KLE / `katana` / `katana-chat-ui` が無駄なく組むには、既存 UI のうち `ScrollArea`、`SplitPane`、`SearchBox` 周辺の要件が不足していた。

本レビューで次の change を追加し、不足を補った。

- `00-add-scroll-area-contract`
- `00-add-split-pane-contract`
- `22-add-search-control-strip`

## レビュー観点

| 観点 | 判定 | 改善 |
| --- | --- | --- |
| 不足 UI はないか | 要改善 | scroll container、split pane、search control strip を追加 |
| UI はあるが要件は十分か | 要改善 | 既存 `ScrollArea` / `SplitPane` / `SearchBox` の弱い契約を change 化 |
| 4 repo が KUC だけで無駄なく組めるか | 改善済み | viewer / editor / chat root は対象外のまま、周辺操作 UI を補強 |
| Storybook 追加が明文化されているか | 改善済み | 追加 change すべてに Storybook section を追加し、`21` に自動回帰と docs を追加 |
| 曖昧さが残っていないか | 改善済み | README に横断 DoD と scope rule を追加 |
| 各 change の DoD は明確か | 改善済み | 品質ゲート / DoD の必須項目を README に固定 |

## 追加で見つかった不足

### ScrollArea

画面上では、長い内容を独立して縦横に動かす領域。
KDV / KLE / `katana` / `katana-chat-ui` の周辺 panel、検索結果、履歴、設定、Storybook inspector で必要。

既存実装はあるが、axis、offset、extent、scrollbar、event、外部 scroll command が OpenSpec 上で十分ではなかった。

### SplitPane

画面上では、左右または上下の 2 領域を境界線で分け、境界線をドラッグして比率を変える UI。
editor / preview、TOC / viewer、navigation / preview / inspector で必要。

既存実装はあるが、2 pane contract、ratio clamp、keyboard resize、persistence 境界、`CollapsiblePanel` との違いが曖昧だった。

### SearchControlStrip

画面上では、検索欄の横に match case、whole word、regex、前へ / 次へ、件数、replace controls が並ぶ UI。
`SearchBox` だけでは、KLE find / replace、KDV viewer search、`katana` search modal、chat history search を無駄なく組めない。

## 対象外として維持したもの

| UI | 理由 |
| --- | --- |
| viewer 本文 | KDV が実装する |
| editor 本文 / gutter / selection rendering | KLE が実装する |
| chat root / message thread / composer | `katana-chat-ui` が実装する |
| app shell / page template | consumer が実装する |
| command registry / search engine | consumer が実装する |

## 残リスク

既存 dirty worktree には、ClaudeCode 由来と思われる `AppShell` / `TitleBar` / `SplashScreen` などの実装差分が残っている。
今回の OpenSpec ではそれらを KUC public API に入れない方向へ整理したが、実装時には guard で公開 API から排除する必要がある。

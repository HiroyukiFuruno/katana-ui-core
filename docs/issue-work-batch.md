# KUC open issue batch

Branch: `fix/kuc-open-issues-batch`

KUC 自身が所有する API、実装、回帰試験、品質ゲートだけで完了を判定する。下流 repository の採用、表示スコア、release は本台帳の完了条件に含めない。

## Active

- [ ] #49 CI の apt 導入を timeout / retry / 共通実装で保護する。
- [/] #52 Windows の候補フォント順をKUCで決定的に適用し、有限なwrap/layout/hit契約を維持する。公開 `CandidateChain` API と候補順・有限wrapの回帰を追加済み。Windows registry-only consumerでの実測確認待ち。
- [/] #50 全量ゲートの開始条件と SHA 別の状態を自動化する。入力manifest、review thread 0、軽量preflight、開始/完了状態のローカル検証とworkflow接続を追加済み。GitHub上の実workflow通過確認待ち。
- [/] #51 fractional container の描画・hit 境界を現行契約と回帰で再監査し、未充足だけを修正した。batch gate 待ち。
- [ ] #53 P2 の修正・後続 Issue 移管・対応不要を機械的に追跡する。
- [/] #59 non-ASCII monospace の face 選択と fallback 契約を整合する。focused test は成功、batch gate 待ち。
- [/] #60 ScrollArea の viewport 外 hit 収集を打ち切る。1,000 child / content_height=40_000 の viewport collector が末尾を走査しない回帰を追加済み。全体ゲート待ち。
- [x] #61 highlight RGBA を実背景に alpha 合成する。light/dark canvas 上で alpha 0/60/255 の数値回帰を追加済み。
- [/] #62 padding 付き明示高 Text の hover wrapper advance を維持する。focused test は成功、batch gate 待ち。
- [/] #64 同一入力の release 検証結果を安全に再利用する。GitHub Actions の成功済み release-preflight artifact だけを候補にし、release workflow は GitHub API の run metadata、source SHA、全 tracked input digest、OS/arch/toolchain/font 環境、issuer、24時間期限を照合してからだけ release-check をskipする。欠損・dirty tree・不一致・期限切れは通常の release-check を実行する。GitHub 実行確認待ち。

## P2 disposition ledger

P2は `fix`（同一リリース修正）、`migrate`（後続Issue移管）、`no-action`（根拠付き対応不要）のいずれかに分類する。P1は必須修正であり、未対応のままresolveしない。各行はPRコメントと元レビューthreadへ遡及できる証跡を持つ。

| issue | disposition | reproduction | impact | acceptance | owner repository | dependency | original review thread | evidence |
|---|---|---|---|---|---|---|---|---|
| #51 | fix | PR #47のfractional container境界6ケース | 描画とnode/action hitの物理境界不一致 | fractional container各ケースの自動回帰とscale=2/scroll=0.75のfull-vs-partial pixel一致 | katana-ui-core | なし | https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994309754, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994337155, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994524088, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994627889, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994627893, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994677515, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994677520, https://github.com/HiroyukiFuruno/katana-ui-core/pull/47#discussion_r3994897522 | fixed containerのlabel/padding下端clipと、scale=2・offset=0.75のMediaFrame full-vs-partial pixel parityをfocused testで確認。batch gate待ち。 |
| #59 | migrate | 非ASCIIを含むinline-code/monospace span | 明示monospaceとfallbackの選択契約が未整合 | 非ASCII glyphのface/測定/描画/hit一致を回帰し、既存fallbackを維持する | katana-ui-core | font family/face選択契約 | https://github.com/HiroyukiFuruno/katana-ui-core/pull/58#discussion_r4000372243 | Issue #59へ再現・受入条件・担当・依存・元threadを移管し、PR threadをresolve済み |
| #60 | migrate | 長いScrollAreaのviewport外多数child hit収集 | 画面外末尾の測定・hit構築コストが増える | viewport cutoffを維持しつつfull-content geometryとvisible hitを保持し、count回帰を通す | katana-ui-core | fractional scroll/wrapped-link hit契約 | https://github.com/HiroyukiFuruno/katana-ui-core/pull/58#discussion_r4000372244 | Issue #60へ性能受入条件を移管し、PR threadをresolve済み |
| #61 | migrate | translucent text-highlight-background token | alphaがRGB化されopaque描画になる | alpha 0/60/255を実背景へ数値合成し、current-highlight優先順位を維持する | katana-ui-core | theme token / canvas blend contract | https://github.com/HiroyukiFuruno/katana-ui-core/pull/58#discussion_r4000401560 | Issue #61へ再現・pixel受入条件・担当・依存・元threadを移管し、PR threadをresolve済み |
| #62 | migrate | explicit Text height + vertical padding + hover wrapper | hover時にpaddingが再適用され後続位置が変わる | legacy/fractional/scrollでpixel/node/action位置とadvanceを不変にする | katana-ui-core | hover surface layout contract | https://github.com/HiroyukiFuruno/katana-ui-core/pull/58#discussion_r4000401562 | Issue #62へ再現・位置不変受入条件・担当・依存・元threadを移管し、PR threadをresolve済み |

## User feedback

- [/] KUC Issue を下流 repository の受入・採用・release に従属させない。KUC owner scope だけで Close を判定する。

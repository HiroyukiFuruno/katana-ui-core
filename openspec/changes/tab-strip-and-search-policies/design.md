# 設計

## 所有境界
hostはopaque target、correlation、名前、RGBA、履歴文字列を投影する。KUCはfocus、IME、popup lifecycle、hit test、drag anchor/ghost、履歴cursor/draft、AccessKit、one-shot配送を所有する。検索algorithmや文書storeは含めない。

## 互換性
追加policyは明示opt-inとする。既存constructor、Enter確定、region drop、結果行navigationは既定のまま保持する。既存proposal constructorのoriginはNoneとし、値から起点を推測しない。public struct literalの形状を変えない検索mode設定はlease builderへ追加する。

## TabStrip
名前変更は確定入力のみを扱い、変更時policyでは空文字も通知する。通知済みの変更はEscapeで巻き戻さない。開始成立は実際にeditorが表示され採用されたpassにだけ通知する。target/correlationと要求単位を保持し、他popupによる延期、要求解除、非編集化、再要求を区別する。

表示RGBAはswatch候補から独立したpresentationで投影する。headerとmemberへ適用し、開閉alphaを明示する。未指定時は従来描画を保持する。

最近接dropの候補は可視tabだけから構築する。最初の左端・隣接間中点・最後の右端の順に、ghost中心からの距離を比較し同距離は先を選ぶ。実際のスクロールviewportと親clipの交差で候補を絞り、部分表示の範囲を使う。可視末尾が論理末尾より手前ならAfter、真の末尾だけEndOfStripと末尾capabilityを使う。開始pointerとghost中心のoffset、ghost幅を一度保存する。lease更新でもanchorと開始portを保持し、終端通知は一度に限る。

proposal originはtab本体、末尾control、context menu、navigation、group popup、drag終端を型で保持する。UI経路の内部文字列や座標をhostへ漏らさない。

## 検索履歴
modeごとにResults（既定）またはHistoryをleaseへ設定する。query focusかつ操作可能時だけUp/Downを履歴へ消費する。Upは最新から最古へ移動し最古で止まり、Downは新しい方へ戻り最後に開始draftを復元する。通常query変更、明示履歴選択、履歴キー操作は別eventとする。

通常入力、mode切替、開閉、履歴の所属・順序・label・disabled変更、policy変更、host query overrideでcursorをresetする。同じactive mode・履歴・policy・KUC現在queryを新revisionで再投影した場合はcursorを維持する。stale/equal leaseは状態を変えない。削除/消去では現在queryを保持し、次の操作のdraftとする。preeditおよびcommit frameの矢印を履歴/結果移動へ誤適用しない。

## 自動検証と公開
Issueごとの実full-root入力回帰でtyped通知、host効果、数値描画、focus/AccessKit、IME、stale/duplicate/one-shotを確認する。補助unit testでedge branchを埋める。既存build/test/lint/AST/coverageを維持し、公開後はregistry版のみで同じ公開契約を検証する。7 Issueは同じDraft PRとv0.6.0へまとめる。

## Global action
文書の有無やread-onlyから独立したroot所有の入力・menu・描画経路を追加する。hostはopaque identity、label、shortcut、global scope、enabledを投影する。既存root認証とrevision・one-shot転送を利用し、他root/stale/duplicateを拒否する。IMEやfocus競合は明示policyとして維持する。

## 検索強調投影
元のUTF-8 byte一致と表示grapheme範囲を別型で保持する。非ゼロ幅一致に交差するgraphemeを全体表示し、元位置は変えない。不正UTF-8境界、範囲外、逆向きはtyped errorとし、既存highlight validatorは維持する。ゼロ幅や複数一致の扱いは公開API文書と自動回帰へ固定する。

Global shortcutの既定focus方針はAnyとし、モーダル内queryを含むfocusから同じactionで開閉できる。DocumentOnlyを明示した場合は他inputを優先する。いずれも本文・モーダル入力の継続preeditとcommit frameを抑止する。modal背面menuのpointer操作は抑止し、shortcutで閉じる経路を保持する。

DocumentOnlyは同じframe内にprimary pointer pressがある場合、focus移動を優先してshortcutを抑止する。既定Anyはこの抑止を行わない。これにより移動前focusに依存する誤発火を避け、次frameの確定focusで再判定する。

# KUC ボタン契約ハーネス

## 要因

Storybook の Button 表示は、通常ボタン、切替ボタン、無効状態、処理中状態を同じ「押下済み」に寄せて扱っていた。
そのため、見た目では押しっぱなしに見え、ボタン（Button）/ テキストボタン（Text Button）/ プリセット（preset）間の状態分離や再押下可能性が弱くなっていた。

## 固定する契約

- 通常ボタンは、押下ごとにアクション（action）/ イベント（event）を発火できる。
- 通常ボタンの押下表示は一時状態であり、ポインター解放（pointer release）相当で通常表示へ戻る。
- 押せない状態は無効（disabled）/ 処理中（loading）/ 待機中（busy）/ 連打抑止（cooldown）など、明示的な状態として扱う。
- 選択の保持が必要な場合は切替ボタン（Toggle）/ 選択状態（selected state）を使い、通常ボタン（Button）に混ぜない。
- Storybook はページ（page）+ プリセット（preset）ごとに状態を分離し、別 UI へ押下状態を伝播させない。

## 追加したハーネス

- core 契約: ボタン押下（Button press）は複数回処理でき、選択状態（selection state）に変化しないことを検証する。
- Storybook 操作契約: 同じボタン（Button）を解放（release）後に再押下できることを検証する。
- Storybook 分離契約: ボタン（Button）/ テキストボタン（Text Button）/ プリセット（preset）間で押下状態が共有されないことを検証する。
- レイアウト（layout）契約: ボタンのプレビュー（preview）の右側ステータス行（status rows）はボタン本体へめり込まない余白を持つことを検証する。

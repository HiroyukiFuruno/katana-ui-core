## ADDED Requirements

### Requirement: Modeごとのquery矢印policy
KUC SHALL preserve result navigation as default and expose opt-in history navigation for the focused query. Upは最新から最古へ進み最古で止まり、Downは新しい方へ戻り最後に開始draftを復元する。

#### Scenario: 履歴の往復
- **WHEN** query「下書き⭐️」と2件の履歴でUp、Up、Up、Down、Downを操作する
- **THEN** 最新、最古、最古、最新、元draftとなり、変化した場合だけ専用typed eventを一度通知しfocusとAccessKitを維持する

### Requirement: 履歴cursorとIMEの所有
KUC SHALL own cursor/draft and leave compositionおよびIME commit frameの矢印を変換操作へ残す。通常入力、mode切替、開閉、履歴構成変更、policy変更、host query overrideでcursorをresetする。

#### Scenario: Lease再投影と履歴編集
- **WHEN** 同じ有効履歴・policy・現在queryを新leaseで再投影、または削除・消去・query overrideを行う
- **THEN** 同一内容では往復を維持し、意味のある変更では現在queryを次のdraftとしてresetしphantom復元を起こさない

#### Scenario: 負例と配送
- **WHEN** 空履歴、別fieldのfocus、IME preedit/commit frame、stale leaseまたは重複配送がある
- **THEN** 履歴を誤変更せず、通常query変更や明示履歴選択と区別し、検索・文書へのhost効果を重複させない

#### Scenario: 同一frame内の複数キー
- **WHEN** 一つの入力frameに複数のUp/Down pressまたはrepeatが順番に到着する
- **THEN** 各入力を順序どおり一度ずつ処理し、方向をまとめたり別キーの消費を行わない

#### Scenario: 履歴キーと文字入力の混在
- **WHEN** 同じ入力frameで履歴キーと文字・caret・選択操作が混在する
- **THEN** 入力順を維持して処理し、履歴置換後の文字を失わず、通常編集でcursorをresetし、対応するtyped eventを同じ順に通知する

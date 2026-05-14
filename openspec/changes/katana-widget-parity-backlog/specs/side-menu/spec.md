## ADDED Requirements

### Requirement: SideMenu widget

SideMenu は左右配置、細いアイコンバー（icon rail）、クリック（click）固定表示、ホバー（hover）一時表示、SVGアイコン操作（SVG icon action）、アイコン（icon）からの内容パネル（panel）表示を MUST 扱えること。

#### Scenario: SVG icon action を実行する

- **WHEN** 利用者がメニューアイコン（menu icon）を押す
- **THEN** アイコンごとの処理（callback）が呼ばれる

#### Scenario: 左右の展開方向を守る

- **WHEN** SideMenu が左配置で内容パネル（panel）を表示する
- **THEN** 内容パネルはアイコンバー（icon rail）の右側へ表示される
- **WHEN** SideMenu が右配置で内容パネル（panel）を表示する
- **THEN** 内容パネルはアイコンバー（icon rail）の左側へ表示される

#### Scenario: click で固定表示する

- **WHEN** 利用者がポップ内容（pop content）を持つアイコン（icon）をクリック（click）する
- **THEN** アイコンに対応する内容パネル（panel）が固定表示される
- **AND** 同じアイコンを再度クリックすると固定表示が閉じる

#### Scenario: hover で遅延表示する

- **WHEN** 利用者がポップ内容（pop content）を持つアイコン（icon）にホバー（hover）する
- **THEN** SideMenu は短い遅延後にアイコンに対応する内容パネル（panel）を一時表示する
- **AND** ポインター（pointer）がアイコンバー（icon rail）と内容パネルの外へ出ると一時表示を閉じる
- **AND** クリック（click）直後はホバー表示へ即時復帰しない

#### Scenario: icon から pop を開く

- **WHEN** アイコン（icon）にポップ内容（pop content）が設定されている
- **THEN** 指定方式でポップ内容を表示する

#### Scenario: KatanA 風の縦 icon rail として表示する

- **WHEN** Storybook で SideMenu を表示する
- **THEN** 白い大枠ではなく、暗色の細い縦アイコンバー（icon rail）として表示される
- **AND** 上寄せアイコン（icon）群と下寄せアイコン群を分けて表示できる
- **AND** 選択中または表示中のアイコンは背景色で識別できる

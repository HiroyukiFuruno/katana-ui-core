## ADDED Requirements

### Requirement: TreeView widget

TreeView は階層データの展開、折り畳み、選択、アクティブ表示、垂直線、indent を扱えることを MUST とする。
行はボタン枠ではなく tree row として描画し、開閉アイコン、任意アイコン、ラベル、選択背景を持つことを MUST とする。
TreeView 自体は通常スクロールを内包せず、親 container がスクロールを管理できることを MUST とする。
入力は `children` を持つ nested JSON 相当の構造を主とし、階層の深さは TreeView が再帰的に算出することを MUST とする。

#### Scenario: nested data を再帰的に表示する

- **WHEN** 利用者が `children` を持つ nested data を TreeView に渡す
- **THEN** TreeView は parent / child / leaf の階層を再帰的に描画する
- **AND** 利用者は表示のためだけに indent を手計算しなくてよい

#### Scenario: item icon を表示する

- **WHEN** parent item または leaf item に SVG icon が指定される
- **THEN** 行の冒頭にその icon を表示する
- **AND** folder / file / settings などの用途別 icon を上位から差し替えられる

#### Scenario: 親 item を開閉する

- **WHEN** 利用者が parent item の開閉アイコン、または設定された開閉領域を押す
- **THEN** expanded state が切り替わる
- **AND** `on_expand` または `on_collapse` callback が呼ばれる

#### Scenario: 開閉領域を指定する

- **WHEN** 利用者が expand trigger を icon only / label only / icon + label / disabled から選ぶ
- **THEN** TreeView は指定された領域だけで parent item の開閉を行う
- **AND** disabled の場合は parent item の開閉を発火しない

#### Scenario: 全開 / 全閉じ control を表示する

- **WHEN** `show_expand_controls` が true
- **THEN** TreeView 左上に「すべて展開」と「すべて折りたたむ」を表示する
- **AND** default では表示しない

#### Scenario: parent item を選択する

- **WHEN** 利用者が selectable な parent item のラベル領域を押す
- **THEN** active item が parent item に更新される
- **AND** 子 item の expanded state は勝手に変わらない

#### Scenario: leaf item を選択する

- **WHEN** 利用者が leaf item を押す
- **THEN** active item が更新される
- **AND** `on_select` callback が呼ばれる

#### Scenario: 階層線を表示する

- **WHEN** `show_indent_lines` が true
- **THEN** 子 item の左側に垂直線と depth 由来の indent を表示する
- **AND** 明示された indent がある場合は上書きできる

#### Scenario: 展開中の水平線を表示する

- **WHEN** `show_horizontal_lines` が true
- **THEN** expanded parent item の下に水平線を表示する
- **AND** default では表示しない
- **AND** 線の種類、太さ、色は上位から指定できる

#### Scenario: scroll 親に配置する

- **WHEN** TreeView が Storybook sidebar や設定 panel のような scroll container 内に置かれる
- **THEN** TreeView 内部で二重スクロールを作らない
- **AND** 表示行の高さと横幅は親 container に追従する

#### Scenario: 大量 item を表示する

- **WHEN** item 数が多い
- **THEN** 利用側が明示的に virtualized mode を選択できる
- **AND** virtualized mode は通常の TreeView と同じ行表示、選択、開閉挙動を維持する

#### Scenario: katana 由来の見た目を再現する

- **WHEN** file explorer、TOC、settings tree のような用途で表示する
- **THEN** 開閉アイコン、folder/file などの任意 SVG、active background、hover background、垂直線が katana の実利用に近い密度で表示される

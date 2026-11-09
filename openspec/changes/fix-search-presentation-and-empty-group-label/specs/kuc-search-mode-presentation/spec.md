## ADDED Requirements

### Requirement: 検索mode別presentationを公開leaseから同期する
KUCは公開APIからmode targetに結び付くquery/include/exclude hint、include/exclude validity、履歴見出しを受け取り、retained search rootへ同期しなければならない（MUST）。これらの値は検索結果の意味やhostの検証処理を変更してはならない（MUST NOT）。既存SearchResultModeのstruct literal利用を壊してはならない（MUST NOT）。

#### Scenario: 未指定presentationは既存表示を保つ
- **WHEN** consumerが検索mode presentationを設定しないleaseを描画する
- **THEN** KUCは従来のinput hint、validation decoration、履歴行表示を保つ

#### Scenario: mode別値を投影して切り替える
- **WHEN** leaseが異なるtargetの複数modeへ異なるhint/validity/headingを設定し、hostがmodeを切り替えて再投影する
- **THEN** KUCは各modeに対応する値だけを表示し、mode labelとcontrols labelをhintやheadingで置換しない

#### Scenario: 空と日本語のhintを同期する
- **WHEN** query/include/exclude hintに空文字または日本語を設定し、値を入力せずrootを描画する
- **THEN** 空hintは表示値もAX操作名も消さず、日本語hintはplaceholderとしてのみ表示される

### Requirement: include/exclude validityをthemeで表示する
KUCはhostが指定したinclude/excludeのvalidityを各入力欄に独立して同期し、invalid状態をKUCの既存themeで描画しなければならない（MUST）。KUCは入力内容を検証してはならず（MUST NOT）、hostへ色値や描画要求を求めてはならない（MUST NOT）。

#### Scenario: includeとexcludeのvalidityを独立に切り替える
- **WHEN** hostがincludeをinvalid、excludeをvalidとして投影し、次のleaseで状態を反転する
- **THEN** KUCは同じretained root/inputのfocusを保ち、各欄のtheme状態だけを対応して更新する

#### Scenario: host検証値を変更せず描画する
- **WHEN** input文字列が同じままhostがvalidityだけを変更する
- **THEN** KUCはSearchRequested/QueryChanged/IncludeChanged/ExcludeChangedを生成せず、検索やregex判定を実行しない

### Requirement: 履歴見出しを非操作表示する
KUCは任意のlocalized history headingを履歴行より前へ非操作labelとして描画し、選択可能な履歴行・削除対象・検索結果として扱ってはならない（MUST NOT）。

#### Scenario: 見出しと履歴行を区別する
- **WHEN** headingと複数のhistory rowsを投影する
- **THEN** headingはAccessKit上Labelであり、クリックやキー操作でHistorySelected/HistoryDeletedを発行せず、行targetを持たない

#### Scenario: 履歴行が空でも見出しを表示する
- **WHEN** headingを設定してhistory rowsを空にする
- **THEN** headingだけを非操作表示し、選択可能な履歴項目を生成しない

### Requirement: retained inputを維持してpresentationを同期する
KUCはhint/validityの更新時に既存retained inputとfocus、selection、IME compositionを保持しなければならない（MUST）。stale revisionを拒否し、各frameに紐付く操作イベントのone-shot配送を維持しなければならない（MUST）。

#### Scenario: presentation再同期中も入力状態を保つ
- **WHEN** query fieldがfocus/compositionを持つ状態で新しいleaseがhintまたはvalidityだけを変更する
- **THEN** KUCはfocus/compositionを失わず、再投影自体から入力変更eventを発行しない

#### Scenario: stale revisionと重複配送を拒否する
- **WHEN** hostが古いrevisionを再送する、または同じroot frameを二度forwardする
- **THEN** stale投影は拒否され、既存eventは一度だけhostへ届く

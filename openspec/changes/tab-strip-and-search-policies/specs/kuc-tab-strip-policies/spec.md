## ADDED Requirements

### Requirement: 名前変更の通知方針と開始成立
KUC SHALL preserve Enter確定を既定とし、明示設定で確定入力の各変更を空文字を含め通知する。開始通知は実際に成立した編集に対してtargetとcorrelationへ束縛し、deferredまたはdiscarded passでは通知してはならない。

#### Scenario: 日本語入力の継続と再投影
- **WHEN** 変更時通知を選んだ編集へ日本語と正確なVS16文字列を入力し同じgroupを再投影する
- **THEN** focusとdraftを維持し、preeditを通知せず、確定変更を重複なく通知する

#### Scenario: 開始要求の延期と再要求
- **WHEN** 他popup、非編集target、消失target、要求解除または後続の明示再要求を投影する
- **THEN** 成立した編集だけを各要求につき一度通知し、異なるtargetへ配送しない

### Requirement: Group表示色を候補色から独立させる
KUC SHALL accept optional arbitrary RGBA presentation independent of popup swatches and apply it to group header and member tabs with explicit expanded/collapsed alpha. 未設定時の表示は維持する。

#### Scenario: 色とgroup構成の変更
- **WHEN** 同名の異なるgroupへ異なるRGBAを投影し、色・開閉・所属・削除を変更する
- **THEN** 対象領域だけが指定RGBAとなり、他groupと本文の描画および入力契約は維持される

### Requirement: 最近接挿入境界と安定したghost
KUC SHALL preserve existing drop behavior by default and provide an opt-in nearest visible-tab boundary policy. 候補は最初の左端、隣接間の中点、最後の右端とし、ghost中心Xから最短、同距離では先の境界を選ぶ。headerと閉じたmemberは除外する。

#### Scenario: 不均等幅と領域外のdrag
- **WHEN** pointerとghostの開始offsetを保持したdragをtab中央、隙間、strip外で終了する
- **THEN** ghost、indicator、hit test、opaque挿入先が同じ境界を示し、可視tabがなければ挿入しない

#### Scenario: Lease継続と取消
- **WHEN** drag中にleaseを更新または取消する
- **THEN** 開始portへ終端を一度だけ配送し、取消ではhostの構成を変更しない

#### Scenario: 横スクロールと部分表示の挿入境界
- **WHEN** tab列がviewportからはみ出し、水平スクロール中に最近接dropを操作する
- **THEN** 非表示tabを候補から除外し、部分表示の範囲とindicatorを一致させ、可視末尾より後ろにtabがあればAfterを返し、真の末尾だけEndOfStripと末尾capabilityを適用する

### Requirement: Proposalの明示的な操作起点
KUC SHALL expose optional typed origin distinguishing tab body, trailing control, context menu, navigation, group popup, and drag terminal. 既存constructorのoriginはNoneとし推測しない。

#### Scenario: 同じ操作の異なる起点
- **WHEN** 異なるUI経路から同じtargetとoperationのproposalを生成する
- **THEN** originを区別でき、correlationとtargetは操作時のleaseへ一度だけ束縛される

#### Scenario: 無効な継続
- **WHEN** staleまたは無効targetのmenu/drag継続が届く
- **THEN** 別targetへ付け替えず、重複したhost効果を起こさない

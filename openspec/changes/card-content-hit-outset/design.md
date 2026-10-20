## Decisions

公開API `UiNode::hit_outset_left(u16)` は対象 node の hit 左端を非負座標の範囲で拡張する。右端は変えない。既定 0 は既存挙動。拡張は node hit と node action hit の生成時に一度だけ適用し、既存 viewport / scroll clip に従う。子孫および inline link span の個別領域には継承しない。

Card 側は既存 `child_indent(16)`、本文側は `hit_outset_left(16)` とする。同一 stable node に設定するため、描画と hit の識別子を分離する wrapper は不要。描画・layout・scroll measurement は描画原点に基づくため文字折返しや後続 node の位置が変わらない。独立した hit 座標系を layout に流す設計は折返し幅を変えるため採用しない。

公開 UiNode の builder で設定し、既存 JSON は serde(default) で読み込む。

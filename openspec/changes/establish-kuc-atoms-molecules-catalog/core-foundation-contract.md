# Core foundation contract

作成日: 2026-05-18
対象: `kuc-core-foundation`

## 結論

KUC の部品実装は、見た目設定（theme）、文字設定（font）、文字描画、入力、イベント、状態、配置の基盤契約が固定された後に開始する。
この契約は中核 crate（core crate）に閉じ、framework-specific UI の型や OS 固有の font path を前提にしない。

## 3.1 外部入口（facade）

KUC は外側から UI 全体の設定を渡す入口として `UiCoreFacade` を持つ。
`UiCoreFacade` は次を保持する。

| 項目 | 契約 |
| --- | --- |
| 見た目設定（theme） | `ThemeSnapshot` として受け取る。`with_theme` で差し替え可能にする。 |
| 後付け style | `StyleSheet` として受け取る。部品生成後でも style の解決結果を差し替え可能にする。 |
| 全体状態（global state） | focus target、active overlay、modal stack など、UI 全体にまたがる状態だけを持つ。 |
| 既定 font role | 既定は `body`。`with_default_font_role` で差し替え可能にする。 |
| font 解決 | `font(role)` は role 指定を優先し、未定義時は既定 role に戻す。 |

部品ごとの値や開閉状態、入力値は `UiCoreFacade` の global state に寄せない。
各部品は自身の状態 ID を持ち、外側は action / event と facade 設定を通じて観測・制御する。
利用側が通信中、保存中、外部選択中のような app 側の状態から UI を制御したい場合は、global state に部品固有 state を直接置かない。
外側は `UiStateHandle` の `get` / `with` で短命に読み取り、`set/update` で部品内部状態へ反映する。
部品は `sync_state` でその snapshot を取り込み、描画可否や action 可否を自身の状態として判定する。

## 3.2 Katana 既定 theme

既定 theme は Katana の accent color を持つ。
`ThemeSnapshot::dark()` は Storybook と通常利用の既定値であり、少なくとも次を提供する。

| token | 契約 |
| --- | --- |
| `background` | 背景色 |
| `surface` | 通常面 |
| `panel` | Storybook panel と side surface |
| `code-background` | code 表示面 |
| `text` | 主文字色 |
| `muted` | 補助文字色 |
| `accent` | Katana accent color |
| `border` | 境界線 |
| `selection` | 選択表示 |

font role は `body` を Proportional、`code` を Monospace とする。
`shortcut` のような等幅表示 role は、専用 token が未定義なら `code` に解決してよい。

## 3.3 文字描画と上下中央揃え

文字描画は、英語、日本語、英日混在、絵文字を同じ高さの箱に置いたとき、上下中央が揃うことを契約にする。

必須サンプル:

| 種別 | サンプル |
| --- | --- |
| 英語 | `Katana UI Core` |
| 日本語 | `刀 UI 部品` |
| 英日混在 | `Katana 設定 Panel` |
| 絵文字混在 | `保存 ✅` |

自動テストは line box、baseline、ascent、descent、visual center を計測する。
既定 theme / 既定 font role の text center 差は 1px 以内を目標値にする。
環境差で 1px を超える場合は、font resolver の差分として記録し、しきい値の引き下げで通さない。

## 3.4 入力 event

KUC は次の入力を core event として扱う。

| 入力 | 契約 |
| --- | --- |
| key input | 物理 key、修飾 key、target を保持する。 |
| text commit | 確定済み文字列を保持する。 |
| 日本語入力（IME） | composition / preedit と commit を区別する。 |
| OS 絵文字入力 | emoji を通常 text commit として失わず保持する。 |
| focus | focus in / out と target を保持する。 |

TextInput 系部品は、利用側に文字列 state を外出ししなくても、入力後の内部 state を更新できる。
利用側は event log と action log から、入力前後の state を確認できる。

## 3.5 部品ごとの state

各 UI 部品インスタンスは `UiStateId` を持つ。
同じ種類、同じ label、同じ option の部品が同じ画面に複数存在しても、state は共有されない。

| 対象 | 契約 |
| --- | --- |
| atom | 表示専用であっても state id を持つか、state 不要であることを契約に明記する。 |
| molecule | 子部品の state id を失わず、親の state と区別して追跡できる。 |
| Storybook | action / event / state 履歴に target state id を表示する。 |
| global state | focus、overlay、modal など全体制御だけを扱う。部品固有 state の置き場にしない。 |
| 外部制御 state | 利用側の app state から `UiStateHandle` を通じて部品内部状態へ `set/update` する。state の所有は部品に残す。 |

state ID の生成は決定的な外部 key だけに依存しない。
重複 label による accidental sharing を品質ゲートで失敗扱いにする。

## 3.6 配置回帰（layout regression）

配置結果は自動テストで検証できる形で取り出せる必要がある。

必須検査:

| 検査 | 内容 |
| --- | --- |
| size | 幅、高さ、最小値、最大値 |
| spacing | padding、gap、border width |
| alignment | vertical center、horizontal center、baseline |
| scroll | scroll bounds、overflow、visible range |
| overlay | anchor、placement、z-index、viewport 内収まり |
| overlap | 意図しない text / control の重なり |

完了判定は layout report、数値化された rendering contract、入力回帰、guard の通過を主根拠にする。

## 3.7 Panel scroll と drag

Panel は画面全体のスクロールに従うだけではなく、Panel ごとに独立した scroll state を持つ。
Navigation、Preview、Details、TreeView preview のように入れ子になった panel でも、どの panel を操作したかを state と event で区別できる必要がある。

| 対象 | 契約 |
| --- | --- |
| panel scroll state | panel id、axis、offset、visible range、content size、viewport size を持つ。 |
| scrollbar model | thumb bounds、track bounds、visibility、overlay / reserved、always / auto / hidden を持つ。 |
| drag | drag start、drag move、drag end、cancel、pointer capture target を event として記録する。 |
| nested panel | 子 panel の scroll / drag が親 panel の scroll state を変えない。 |
| rendering contract | scrollbar の表示方式、thumb 位置、drag 後の差分、panel 間の独立性を数値化して検査する。 |

scrollbar は表示飾りではなく拡張可能な model として扱う。
将来の overlay scrollbar、常時表示 scrollbar、細幅 scrollbar、preview 用の強制表示を同じ contract で扱えるようにする。

# Live root の Unicode 数値観測

`EguiTextCommandSurfaceHostRootFrame::observe_unicode((start, end))` は、そのframeで
描画した本文の論理UTF-8範囲を観測する。範囲は空でない完全な書記素の境界で指定する。
別surfaceの描画やhostによるcrop・caret計算は不要である。
IME未確定の表示文字列を本文へ混同しないため、composition中の論理本文観測は明示拒否する。

返り値は不透明で変更できず、本文、対象scalar、計測bounds、caret、hit test、
RGBA crop、AccessKit、root/frame/artifactの識別を読み取れる。
観測は本文や選択、フォーカス、スクロール、イベントの消費状態を変更しない。

```rust,ignore
let primary = frame.observe_unicode(color_range)?; // 「⭐️」全体のUTF-8範囲
let control = frame.observe_unicode(control_range)?; // 「☆」のUTF-8範囲
let receipt = frame.forward_events_once(&mut host)?;
let (observations, crops, hashes) =
    primary.native_observations(&control, &receipt, committed_range)?;
```

`native_observations` は `native-ime-evidence` featureで利用できる。
実際に読み込んだfont identityと実AX node identityを使い、既存の
`NativeFrameObservations`、`RgbaCropObservation`、`NativeObservationHashes` を返す。
同じrootでも異なるframeの対照やreceiptは拒否する。色付き対象は正確な
`U+2B50 U+FE0F`、対照は `U+2606` とする。
本文が更新された場合は更新後のframeから改めて観測する。

この変換は入力のnative originを認定しない。OSのIME transcriptは既存verifierの
run/challenge/producer契約と独立して結合・検証する。synthetic RawInputの契約テストを
実IME測定の証拠として扱わない。

実行可能な公開API回帰は
`crates/katana-ui-core/tests/egui_host_root_facade_contract.rs` の
`live_unicode_observer_runtime_binds_native_shapes_to_one_forwarded_frame` にある。

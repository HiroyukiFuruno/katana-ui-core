## Context
Issue #104: 24px幅のcode soft wrapでcopyへLFが混入する。Issue #105: max_width=Noneが4096幅のRTL右寄せとして解釈され、短いArabic fragmentでもrun幅4097/glyph x約4140となる。

## Goals
描画、測定、wrap、rounded selection、link hit、decorationが同一geometry/source境界を使う。既存の明示改行とCRLFを保ち、soft wrapから新しいsource改行を作らない。

## Decisions
未指定幅のフォールバック値を実際のintrinsic content座標へ混入させない。内部bufferの4096px安全幅とwrap上限は維持し、幅未指定のrich textだけを左原点に配置する。bidiの字形順序・方向を強制せず、外側のbidi fragment配置へローカルboundsを返す。明示されたmax_widthの従来配置は維持する。visual lineとlogical source lineを区別する。hitだけのclampやconsumer側の座標補正は採用しない。

空白保持はcode roleまたは各code spanへ限定する。単一のinline codeを含むことを理由に通常本文全体の空白幅・行頭trimを変更しない。code spanがvisual wrapで行頭へ移った場合もsourceの空白を保持する。

## Risks
RTL/LTR/FSI/PDI、combining grapheme、明示max_widthのwrap、空段落、source adjacencyをfocused回帰と完全release-checkで確認する。

## 依存関係
`just update-safe`で互換lock更新を実施（tmp/v044-update-safe.log）。`cargo outdated --workspace`で直接・推移依存を再評価（tmp/v044-outdated.log）。egui/eframe 0.37への更新は、egui featureで公開する型を変える互換性のない移行になるため、このpatch修正では0.36.2を維持する。glifo 0.2など上流が要求する旧系列の推移依存も、互換範囲外へ単独で強制更新しない。Tokio 1.54.1等の互換更新は採用する。

CI Actionも公式release/tagを照合し、[checkout v7.0.1](https://github.com/actions/checkout/releases/tag/v7.0.1)、[upload-artifact v7.0.2](https://github.com/actions/upload-artifact/releases/tag/v7.0.2)、[download-artifact v8.0.2](https://github.com/actions/download-artifact/releases/tag/v8.0.2)、[crates-io-auth-action v1.0.6](https://github.com/rust-lang/crates-io-auth-action/releases/tag/v1.0.6)のcommit SHAへ固定した。rust-toolchainも2026-10-23に取得したmasterのSHAへ同期する。既存のstable指定、OIDCの権限・token出力、artifact名・path・ZIP検証を維持し、uploadには既定値と同じ `archive: true` を明示する。mutableなrust-cache@v2とinstall-actionのtool別tagは既存の追従方式を維持する。

## 実行調整
容量不足時の共通cleanupはrootだけが担当し、agentは重複実行せず報告する。全体の重いgateもrootが逐次回収する。2026-10-23のcleanupは一部guard/同時実行ENOENTでexit1、ただしdfで48GiBの空きを確認した。

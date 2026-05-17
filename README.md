<p align="center">
  <img src="assets/kuw-icon.png" width="128" alt="katana-ui-core icon">
</p>

<h1 align="center">katana-ui-core</h1>

<p align="center">
  Floem-first shared UI widgets for the KatanA ecosystem.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ui-core/actions/workflows/test-and-build.yml"><img src="https://github.com/HiroyukiFuruno/katana-ui-core/actions/workflows/test-and-build.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ui-core/releases/latest"><img src="https://img.shields.io/github/v/release/HiroyukiFuruno/katana-ui-core" alt="Latest Release"></a>
  <a href="https://crates.io/crates/katana-ui-core"><img src="https://img.shields.io/crates/v/katana-ui-core.svg" alt="crates.io"></a>
  <a href="https://docs.rs/katana-ui-core"><img src="https://img.shields.io/badge/docs.rs-katana--ui--widget-blue" alt="docs.rs"></a>
</p>

---

## 目的

`katana-ui-core` は、KatanAエコシステム向けの共有UI部品を管理するリポジトリです。

**Floem前提**です。`egui` 互換層や egui 固有の部品は対象外です。

## Widget 階層と依存方向

```
theme  ←  primitive  ←  composite  ←  layout
```

| 階層 | パス | 依存可能な層 |
|---|---|---|
| `theme` | `src/theme/` | なし |
| `primitive` | `src/primitive/` | theme のみ |
| `composite` | `src/composite/<category>/` | theme, primitive |
| `layout` | `src/layout/` | theme, primitive, composite |

詳細は [`docs/directory-structure.md`](docs/directory-structure.md) を参照。

## Storybook

widget を目視確認するための独立した Floem アプリです。`crates/` 外で管理し、workspace に含めません。

```bash
# Storybook を起動
just storybook

# コンパイル確認のみ（CI用）
just storybook-check
```

## 品質ゲート

```bash
# 全チェック（fmt / types / lint / ast-lint / tests）
just check

# ast-lint インストール（初回のみ）
just ast-lint-install
```

## ドキュメント

- [`docs/directory-structure.md`](docs/directory-structure.md) — 階層図と依存方向
- [`docs/widget-extraction-policy.md`](docs/widget-extraction-policy.md) — 抽出対象の判断軸
- [`docs/release.md`](docs/release.md) — リリース手順
- [`CONTRIBUTING.md`](CONTRIBUTING.md) / [`CONTRIBUTING.ja.md`](CONTRIBUTING.ja.md) — 貢献ガイド

## スキル

- `.claude/skills/kuw-workflow-guide/` — KUW 専用ワークフローガイド（Floem前提・階層ルール・Storybook規約）

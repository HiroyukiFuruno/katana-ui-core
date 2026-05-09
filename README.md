<p align="center">
  <img src="assets/kuw-icon.png" width="128" alt="katana-ui-widget icon">
</p>

<h1 align="center">katana-ui-widget</h1>

<p align="center">
  Floem-first shared UI widgets for the KatanA ecosystem.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ui-widget/actions/workflows/test-and-build.yml"><img src="https://github.com/HiroyukiFuruno/katana-ui-widget/actions/workflows/test-and-build.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ui-widget/releases/latest"><img src="https://img.shields.io/github/v/release/HiroyukiFuruno/katana-ui-widget" alt="Latest Release"></a>
  <a href="https://crates.io/crates/katana-ui-widget"><img src="https://img.shields.io/crates/v/katana-ui-widget.svg" alt="crates.io"></a>
  <a href="https://docs.rs/katana-ui-widget"><img src="https://img.shields.io/badge/docs.rs-katana--ui--widget-blue" alt="docs.rs"></a>
</p>

---

## 目的

`katana-ui-widget` は、KatanAエコシステム向けの共有UI部品を管理するリポジトリです。

このリポジトリはFloem前提です。`egui` 固有の部品をそのまま移す場所ではありません。

## 責務

- metadata表示
- unresolved表示
- copy/edit action
- tab、toolbar、badgeなどの共有UI部品

## 非責務

- KMEの文書モデル
- metadata schema本体
- KatanAアプリ本体のshell/chrome
- `egui` 互換層

## 品質ゲート

ローカル確認は次で実行します。

```bash
just check
```

共有AST lint（抽象構文木ベースの静的検査）の導入は次で行います。

```bash
just ast-lint-install
```

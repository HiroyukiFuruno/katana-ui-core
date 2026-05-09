# katana-ui-widget

KatanAエコシステム向けに、Floem前提の共有UI widget を集約するリポジトリです。

## 方針

- `egui` のみのUI実装を引きずらず、共有可能なFloemコンポーネントを管理する
- リポジトリ外部の内部ASTやmetadata実体を保持しない
- downstream側が利用しやすいDTO設計を優先する

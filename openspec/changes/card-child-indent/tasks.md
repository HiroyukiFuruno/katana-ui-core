## 1. 公開APIと座標契約

- [x] 1.1 Option<u16>の公開フィールド、builder、旧JSON互換を追加する
- [x] 1.2 core/Storybookの描画・hit helperへ同じCard設定を反映する
- [x] 1.3 0/24/defaultの原点・残幅と公開560px契約・JSON回帰を追加する

## 2. 品質検証と公開

- [ ] 2.1 依存関係を調査・互換更新し、just release-checkを通す
- [ ] 2.2 Draft PR作成、review回収、指摘修正・reply/resolve、Ready昇格、required checks確認を行う
- [ ] 2.3 masterへmergeし、Releaseと公開crateを検証する
- [ ] 2.4 Issueへ利用方法と検証条件を記録し、ローカルbranch/worktree/stashを整理する

# P2 disposition comment template

リリースPRでP2レビュー指摘を処理するときは、各指摘についてこの項目を埋め、PR本文とIssue台帳から追跡できる状態にする。

## Classification

`fix`（同一リリースで修正）、`migrate`（後続Issueへ移管）、`no-action`（対応不要の根拠を記録）のいずれか一つ。

## Required fields

- Issue: `#123`
- Reproduction: 最小再現条件
- Impact: 影響範囲と互換性への影響
- Acceptance criteria: 自動テストまたは検証可能な受入条件
- Owner repository: 修正担当repository
- Dependency: 前提となる公開版、Issue、または依存なし
- Original review thread: 元レビューthreadの完全なGitHub URL
- Evidence: 修正・移管・対応不要を裏付けるテスト、commit、Issue、または判断根拠

コメント例:

```text
P2を #123 へ migrate しました。
Reproduction: ...
Impact: ...
Acceptance criteria: ...
Owner repository: ...
Dependency: ...
Original review thread: https://github.com/OWNER/REPO/pull/1#discussion_r123
Evidence: ...
このthreadは移管済みとしてresolveします。
```

P1は従来どおり必須修正とし、未対応のままresolveしてはならない。

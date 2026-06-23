<!-- rtk-instructions v2 -->
# RTK (Rust Token Killer) - Token-Optimized Commands

## Golden Rule

**Always prefix commands with `rtk`**. If RTK has a dedicated filter, it uses it. If not, it passes through unchanged. This means RTK is always safe to use.

**Important**: Even in command chains with `&&`, use `rtk`:
```bash
# ❌ Wrong
git add . && git commit -m "msg" && git push

# ✅ Correct
rtk git add . && rtk git commit -m "msg" && rtk git push
```

## RTK Commands by Workflow

### Build & Compile (80-90% savings)
```bash
rtk cargo build         # Cargo build output
rtk cargo check         # Cargo check output
rtk cargo clippy        # Clippy warnings grouped by file (80%)
rtk tsc                 # TypeScript errors grouped by file/code (83%)
rtk lint                # ESLint/Biome violations grouped (84%)
rtk prettier --check    # Files needing format only (70%)
rtk next build          # Next.js build with route metrics (87%)
```

### Test (60-99% savings)
```bash
rtk cargo test          # Cargo test failures only (90%)
rtk go test             # Go test failures only (90%)
rtk jest                # Jest failures only (99.5%)
rtk vitest              # Vitest failures only (99.5%)
rtk playwright test     # Playwright failures only (94%)
rtk pytest              # Python test failures only (90%)
rtk rake test           # Ruby test failures only (90%)
rtk rspec               # RSpec test failures only (60%)
rtk test <cmd>          # Generic test wrapper - failures only
```

### Git (59-80% savings)
```bash
rtk git status          # Compact status
rtk git log             # Compact log (works with all git flags)
rtk git diff            # Compact diff (80%)
rtk git show            # Compact show (80%)
rtk git add             # Ultra-compact confirmations (59%)
rtk git commit          # Ultra-compact confirmations (59%)
rtk git push            # Ultra-compact confirmations
rtk git pull            # Ultra-compact confirmations
rtk git branch          # Compact branch list
rtk git fetch           # Compact fetch
rtk git stash           # Compact stash
rtk git worktree        # Compact worktree
```

Note: Git passthrough works for ALL subcommands, even those not explicitly listed.

### GitHub (26-87% savings)
```bash
rtk gh pr view <num>    # Compact PR view (87%)
rtk gh pr checks        # Compact PR checks (79%)
rtk gh run list         # Compact workflow runs (82%)
rtk gh issue list       # Compact issue list (80%)
rtk gh api              # Compact API responses (26%)
```

### JavaScript/TypeScript Tooling (70-90% savings)
```bash
rtk pnpm list           # Compact dependency tree (70%)
rtk pnpm outdated       # Compact outdated packages (80%)
rtk pnpm install        # Compact install output (90%)
rtk npm run <script>    # Compact npm script output
rtk npx <cmd>           # Compact npx command output
rtk prisma              # Prisma without ASCII art (88%)
```

### Files & Search (60-75% savings)
```bash
rtk ls <path>           # Tree format, compact (65%)
rtk read <file>         # Code reading with filtering (60%)
rtk grep <pattern>      # Search grouped by file (75%). Format flags (-c, -l, -L, -o, -Z) run raw.
rtk find <pattern>      # Find grouped by directory (70%)
```

### Analysis & Debug (70-90% savings)
```bash
rtk err <cmd>           # Filter errors only from any command
rtk log <file>          # Deduplicated logs with counts
rtk json <file>         # JSON structure without values
rtk deps                # Dependency overview
rtk env                 # Environment variables compact
rtk summary <cmd>       # Smart summary of command output
rtk diff                # Ultra-compact diffs
```

### Infrastructure (85% savings)
```bash
rtk docker ps           # Compact container list
rtk docker images       # Compact image list
rtk docker logs <c>     # Deduplicated logs
rtk kubectl get         # Compact resource list
rtk kubectl logs        # Deduplicated pod logs
```

### Network (65-70% savings)
```bash
rtk curl <url>          # Compact HTTP responses (70%)
rtk wget <url>          # Compact download output (65%)
```

### Meta Commands
```bash
rtk gain                # View token savings statistics
rtk gain --history      # View command history with savings
rtk discover            # Analyze Codex sessions for missed RTK usage
rtk proxy <cmd>         # Run command without filtering (for debugging)
rtk init                # Add RTK instructions to AGENTS.md
rtk init --global       # Add RTK to ~/.Codex/AGENTS.md
```

## Token Savings Overview

| Category | Commands | Typical Savings |
|----------|----------|-----------------|
| Tests | vitest, playwright, cargo test | 90-99% |
| Build | next, tsc, lint, prettier | 70-87% |
| Git | status, log, diff, add, commit | 59-80% |
| GitHub | gh pr, gh run, gh issue | 26-87% |
| Package Managers | pnpm, npm, npx | 70-90% |
| Files | ls, read, grep, find | 60-75% |
| Infrastructure | docker, kubectl | 85% |
| Network | curl, wget | 65-70% |

Overall average: **60-90% token reduction** on common development operations.
<!-- /rtk-instructions -->

---

# katana-ui-core repository rules

## Storybook の扱い

Storybook は、利用者や開発者が KUC の部品を実画面で触り、見た目、操作感、設定変更時の振る舞いへフィードバックするための画面である。
Storybook を「確認環境」「目視確認の場」「静的な部品一覧」として扱ってはならない。
部品の正しさ、01〜24 の完了、v0.1.0 の release readiness は、Storybook やユーザー操作ではなく、自動テスト、数値化された layout / rendering contract、入力回帰、state / event / action contract、guard で判定する。

## 01〜24 と v0.1.0 DoD

01〜24 は、見た目だけ、文字だけ、同じ骨格の preview、ログだけの変化では完了にしない。
各 UI は option、action、event、state、preset、preview、settings、対応する自動テストを持ち、要件行から検証コードへ追跡できる必要がある。
v0.1.0 の DoD は、`katana` と `katana-chat-ui` が `katana-ui-core` だけで app UI を構築できるだけの public API と contract が揃っていることとする。

## 画像証跡の扱い

画像、スクリーンショット、手動目視、Storybook 操作を完了根拠にしてはならない。
表示の品質は、可能な限り bounds、alignment、hit target、state transition、render command、theme token、font metrics などの数値化された契約で検査する。
画像を補助証跡として増やす前に、同じ要求を自動テストまたは guard へ落とし込む。

## Guard の置き場所

KUC 固有の制約はこの repository の OpenSpec、docs、`scripts/`、Rust tests に固定する。
KUC 固有の都合を `kal` 側へ追記してはならない。

## runner 停止条件

v0.1.0 release readiness が未達の間、runner は未完了の 01〜24 / Storybook / 自動テスト / guard を自律的に消化し続ける。
ローカル実装、テスト、静的検査（lint）、OpenSpec 更新、ローカル保存（commit）は、ユーザーが「続けて」と明示している作業では停止理由にしない。

停止して確認するのは、外部へ影響する送信（push）、公開（release）、破壊的操作、または repository 外の実装へ踏み出す場合だけとする。
push confirmation required / release confirmation required / destructive operation confirmation required を停止条件の合言葉として扱う。
それ以外で作業が残っている場合は、次の未完了タスクを選び、実装と自動テストへ進む。

## repository hook

`.githooks/pre-commit` は `just kuc-guardrails` を実行し、停止条件の誤り、Storybook の完了根拠化、KUC 固有 guard の欠落を commit 前に検出する。
hook が失敗した場合はユーザー確認で止まらず、失敗内容を修正して続行する。

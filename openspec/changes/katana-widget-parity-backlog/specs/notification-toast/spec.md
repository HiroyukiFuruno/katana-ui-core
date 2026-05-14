## ADDED Requirements

### Requirement: NotificationToast widget

NotificationToast は severity 付き message を一時表示し、自動消去、手動 dismiss、stack 表示を扱えることを MUST とする。

#### Scenario: toast を表示する

- **WHEN** toast が追加される
- **THEN** severity に対応する icon、message、optional action を表示する

#### Scenario: duration 後に自動消去する

- **WHEN** duration が設定されている
- **THEN** 指定時間後に toast を閉じる
- **AND** `on_dismiss` callback が呼ばれる

#### Scenario: 複数 toast を stack 表示する

- **WHEN** 複数 toast が同時に存在する
- **THEN** position と max_visible に従って stack 表示する

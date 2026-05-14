use floem::action::exec_after;
use std::time::Duration;

const ENV_INTERACTION: &str = "KATANA_UI_WIDGET_STORYBOOK_INTERACTION";
const ENV_EXPECTED_DETAIL: &str = "KATANA_UI_WIDGET_STORYBOOK_EXPECTED_DETAIL";
const ENV_EXIT_AFTER_INTERACTION: &str = "KATANA_UI_WIDGET_STORYBOOK_EXIT_AFTER_INTERACTION";
const ENV_EXIT_AFTER_MS: &str = "KATANA_UI_WIDGET_STORYBOOK_EXIT_AFTER_MS";
const LOG_PREFIX: &str = "katana-storybook-interaction";
const REPLAY_DELAY_MS: u64 = 100;
const DEFAULT_EXIT_AFTER_MS: u64 = 700;

pub fn requested(expected: &str) -> bool {
    std::env::var(ENV_INTERACTION)
        .map(|value| value == expected)
        .unwrap_or(false)
}

pub fn mark_supported(page: &str, interaction: &str) {
    eprintln!("{LOG_PREFIX}:supported page={page} interaction={interaction}");
}

pub fn mark_exercised(page: &str, interaction: &str, detail: &str) {
    eprintln!("{LOG_PREFIX}:exercised page={page} interaction={interaction} detail={detail}");
    exit_after_expected_interaction(detail);
}

pub fn open_requested(page: &str, detail: &str) -> bool {
    let should_open = requested("open");
    if should_open {
        mark_supported(page, "open");
        mark_exercised(page, "open", detail);
    }
    should_open
}

pub fn replay(
    interaction: &'static str,
    page: &'static str,
    detail: &'static str,
    action: impl Fn() + 'static,
) {
    if !requested(interaction) {
        return;
    }

    mark_supported(page, interaction);
    schedule_replay(move || {
        action();
        mark_exercised(page, interaction, detail);
    });
}

pub fn schedule_replay(action: impl Fn() + 'static) {
    exec_after(Duration::from_millis(REPLAY_DELAY_MS), move |_| {
        action();
    });
}

fn exit_after_expected_interaction(detail: &str) {
    if std::env::var(ENV_EXIT_AFTER_INTERACTION).ok().as_deref() != Some("1") {
        return;
    }

    if std::env::var(ENV_EXPECTED_DETAIL)
        .map(|expected| expected != detail)
        .unwrap_or(false)
    {
        return;
    }

    let delay_ms = std::env::var(ENV_EXIT_AFTER_MS)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_EXIT_AFTER_MS);
    exec_after(Duration::from_millis(delay_ms), move |_| {
        std::process::exit(0);
    });
}

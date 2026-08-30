//! Generic, deterministic full text-command surface scenarios.

include!("scenario/part1.rs");
include!("scenario/part2.rs");
include!("scenario/part3.rs");
include!("scenario/part4.rs");
include!("scenario/part5.rs");
include!("scenario/part6.rs");
include!("scenario/part7.rs");
include!("scenario/part8.rs");

#[cfg(test)]
#[path = "scenario_inline_tests.rs"]
mod tests;

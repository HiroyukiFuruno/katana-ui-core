use super::NativeApplication;
use std::time::Duration;

impl NativeApplication {
    pub(crate) fn ensure_before_deadline(&mut self) -> Result<(), String> {
        if deadline_has_expired(self.started.elapsed(), self.timeout) {
            self.timed_out = true;
            Err("native run deadline exceeded".into())
        } else {
            Ok(())
        }
    }
}

fn deadline_has_expired(elapsed: Duration, timeout: Duration) -> bool {
    elapsed >= timeout
}

#[cfg(test)]
mod tests {
    use super::deadline_has_expired;
    use std::time::Duration;

    #[test]
    fn deadline_comparison_rejects_equal_and_later_elapsed() {
        assert!(!deadline_has_expired(
            Duration::from_millis(9),
            Duration::from_millis(10)
        ));
        assert!(deadline_has_expired(
            Duration::from_millis(10),
            Duration::from_millis(10)
        ));
        assert!(deadline_has_expired(
            Duration::from_millis(11),
            Duration::from_millis(10)
        ));
    }
}

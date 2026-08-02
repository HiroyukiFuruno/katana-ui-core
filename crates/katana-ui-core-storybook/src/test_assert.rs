pub(crate) trait KucTestExpect<T> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T;

    #[track_caller]
    fn kuc_unwrap(self) -> T;
}

#[track_caller]
fn test_expect_failure(message: &str) -> ! {
    std::panic::resume_unwind(Box::new(message.to_string()))
}

impl<T> KucTestExpect<T> for Option<T> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T {
        self.unwrap_or_else(|| test_expect_failure(message))
    }

    #[track_caller]
    fn kuc_unwrap(self) -> T {
        self.kuc_expect("expected Some value")
    }
}

impl<T, E> KucTestExpect<T> for Result<T, E> {
    #[track_caller]
    fn kuc_expect(self, message: &str) -> T {
        self.unwrap_or_else(|_| test_expect_failure(message))
    }

    #[track_caller]
    fn kuc_unwrap(self) -> T {
        self.kuc_expect("expected Ok value")
    }
}

#[cfg(test)]
mod tests {
    use super::KucTestExpect;

    #[test]
    fn test_expect_returns_present_option_and_ok_result() {
        assert_eq!(3, Some(3).kuc_expect("some"));
        assert_eq!(4, Some(4).kuc_unwrap());
        assert_eq!(5, Result::<_, &str>::Ok(5).kuc_expect("ok"));
        assert_eq!(6, Result::<_, &str>::Ok(6).kuc_unwrap());
    }

    #[test]
    fn test_expect_panics_for_missing_option_and_error_result() {
        assert!(std::panic::catch_unwind(|| Option::<usize>::None.kuc_expect("missing")).is_err());
        assert!(
            std::panic::catch_unwind(|| Result::<usize, _>::Err("error").kuc_expect("failed"))
                .is_err()
        );
    }
}

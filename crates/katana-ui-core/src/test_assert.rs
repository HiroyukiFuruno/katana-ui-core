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
    fn option_and_result_helpers_preserve_values_and_fail_closed() {
        assert_eq!(7, Some(7).kuc_expect("value"));
        assert_eq!(8, Some(8).kuc_unwrap());
        assert_eq!(9, Result::<_, ()>::Ok(9).kuc_expect("value"));
        assert_eq!(10, Result::<_, ()>::Ok(10).kuc_unwrap());

        assert!(std::panic::catch_unwind(|| Option::<u8>::None.kuc_expect("missing")).is_err());
        assert!(std::panic::catch_unwind(|| Option::<u8>::None.kuc_unwrap()).is_err());
        assert!(
            std::panic::catch_unwind(|| Result::<u8, ()>::Err(()).kuc_expect("error")).is_err()
        );
        assert!(std::panic::catch_unwind(|| Result::<u8, ()>::Err(()).kuc_unwrap()).is_err());
    }
}

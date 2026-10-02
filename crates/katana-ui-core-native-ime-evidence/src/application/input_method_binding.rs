#[derive(Debug, Default)]
pub(super) struct InputMethodBinding {
    pinned: Option<String>,
}

impl InputMethodBinding {
    pub(super) fn observe_preedit(&mut self, input_method: String) -> Result<(), &'static str> {
        if input_method.trim().is_empty() {
            return Err("native input method lookup returned an empty identity");
        }
        match &self.pinned {
            Some(pinned) if pinned != &input_method => {
                Err("input method changed during native run")
            }
            Some(_) => Ok(()),
            None => {
                self.pinned = Some(input_method);
                Ok(())
            }
        }
    }

    pub(super) fn verify_commit(&self, input_method: &str) -> Result<(), &'static str> {
        let Some(pinned) = &self.pinned else {
            return Err("native commit arrived before input method pin");
        };
        if pinned == input_method {
            Ok(())
        } else {
            Err("input method changed during native run")
        }
    }

    pub(super) fn is_pinned(&self) -> bool {
        self.pinned.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::InputMethodBinding;

    #[test]
    fn first_preedit_binds_and_matching_commit_succeeds() {
        let mut binding = InputMethodBinding::default();
        binding
            .observe_preedit("Japanese IME".into())
            .expect("first preedit should bind");
        binding
            .verify_commit("Japanese IME")
            .expect("matching commit should succeed");
    }

    #[test]
    fn different_method_is_rejected_after_binding() {
        let mut binding = InputMethodBinding::default();
        binding.observe_preedit("Japanese IME".into()).unwrap();
        assert_eq!(
            binding.observe_preedit("English IME".into()),
            Err("input method changed during native run")
        );
        assert_eq!(
            binding.verify_commit("English IME"),
            Err("input method changed during native run")
        );
    }

    #[test]
    fn commit_before_binding_is_rejected() {
        let binding = InputMethodBinding::default();
        assert_eq!(
            binding.verify_commit("Japanese IME"),
            Err("native commit arrived before input method pin")
        );
    }
}

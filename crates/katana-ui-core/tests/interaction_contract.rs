mod interaction_contract {
    mod callback_action_contract;
    mod color_action_contract;

    #[path = "feedback_serialization_contract.rs"]
    mod action_result_is_serializable_snapshot;
    #[path = "basic_action_contract.rs"]
    mod action_targets_only_the_matching_component_state;
    #[path = "input_commit_contract.rs"]
    mod input_action_commits_ime_emoji_and_mixed_text_to_owned_state;
}

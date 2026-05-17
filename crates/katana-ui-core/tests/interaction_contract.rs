mod interaction_contract {
    mod color_action_contract;

    #[path = "feedback_serialization_contract.rs"]
    mod action_result_is_serializable_snapshot;
    #[path = "basic_action_contract.rs"]
    mod action_targets_only_the_matching_component_state;
}

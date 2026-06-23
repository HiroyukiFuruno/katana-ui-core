use super::{SkeletonClusterPreset, StoryCatalog, StoryExample, UiCallbackLog, layout, molecule};

pub(super) fn skeleton_cluster_story() -> StoryExample {
    let list = skeleton_cluster_preset("list loading", SkeletonClusterPreset::ListRow);
    let message = skeleton_cluster_preset("message loading", SkeletonClusterPreset::Message);
    let card = skeleton_cluster_preset("card loading", SkeletonClusterPreset::Card);
    let paragraph = skeleton_cluster_preset("paragraph loading", SkeletonClusterPreset::Paragraph);
    let code_block =
        skeleton_cluster_preset("code block loading", SkeletonClusterPreset::CodeBlock);
    let image_card =
        skeleton_cluster_preset("image card loading", SkeletonClusterPreset::ImageCard)
            .live_region("Loading custom image card")
            .reduced_motion(true);
    let logs = vec![
        UiCallbackLog::new(
            card.state_id().clone(),
            "skeleton_cluster_preset_apply",
            "preset=ListRow children=2 live_region=Loading list loading reduced_motion=false",
            "preset=Card children=2 live_region=Loading card loading reduced_motion=false event=skeleton_cluster_changed",
        ),
        UiCallbackLog::new(
            message.state_id().clone(),
            "skeleton_cluster_changed",
            "preset=Message children=3 live_region=Loading message loading reduced_motion=false",
            "preset=ImageCard children=3 live_region=Loading custom image card reduced_motion=true",
        ),
    ];
    StoryCatalog::interactive_story(
        "skeleton-cluster",
        layout::Column::new()
            .child(list)
            .child(message)
            .child(card)
            .child(paragraph)
            .child(code_block)
            .child(image_card),
        logs,
    )
}

fn skeleton_cluster_preset(
    label: &'static str,
    preset: SkeletonClusterPreset,
) -> molecule::SkeletonCluster {
    molecule::SkeletonCluster::new(label).preset(preset)
}

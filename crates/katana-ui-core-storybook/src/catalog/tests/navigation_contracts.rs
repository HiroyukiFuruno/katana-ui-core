use super::*;
use crate::catalog::StoryExample;

#[test]
fn tabs_story_has_interactive_selection_contract() -> Result<(), &'static str> {
    let story = story_for("tabs")?;
    let root = story.tree.root();

    assert_eq!("Tabs", root.props().label);
    assert_eq!(3, root.props().interaction.item_count);
    assert_eq!("preview", root.props().interaction.value);
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "tab_select" && it.after.contains("event=tab_changed"))
    );
    Ok(())
}

#[test]
fn breadcrumb_story_has_route_callback_contract() -> Result<(), &'static str> {
    let story = story_for("breadcrumb")?;
    let root = story.tree.root();

    assert_eq!("Breadcrumb", root.props().label);
    assert_eq!(3, root.props().interaction.item_count);
    assert_eq!("root", root.props().interaction.value);
    assert!(
        story
            .callback_logs
            .iter()
            .any(|it| it.action == "breadcrumb_click" && it.after.contains("event=route_changed"))
    );
    Ok(())
}

#[test]
fn side_menu_story_has_interactive_callback_contracts() -> Result<(), &'static str> {
    let story = story_for("side-menu")?;
    let root = story.tree.root();

    assert_eq!("Side menu", root.props().label);
    assert!(root.props().interaction.open);
    assert!(!root.props().interaction.hovered);
    assert_eq!(0, root.props().interaction.selected_index);
    assert_eq!("files", root.props().interaction.value);
    assert!(root.props().interaction.item_count >= 2);
    assert!(root.children().len() >= 2);
    for action in [
        "side_menu_state_read",
        "select_box_selected",
        "side_menu_hover_expand",
    ] {
        assert!(
            story.callback_logs.iter().any(|it| it.action == action),
            "side-menu callback log lacks {action}"
        );
    }
    assert!(
        story.callback_logs.iter().any(|it| {
            it.action == "side_menu_hover_expand" && it.after.contains("hovered=true")
        })
    );
    Ok(())
}

fn story_for(page: &str) -> Result<StoryExample, &'static str> {
    StoryCatalog
        .examples()
        .into_iter()
        .find(|it| it.page == page)
        .ok_or("navigation page missing")
}

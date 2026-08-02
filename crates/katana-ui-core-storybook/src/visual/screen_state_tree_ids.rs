pub(super) fn tree_static_id(value: &str) -> &'static str {
    match value {
        "katana/nested/b.md" => "katana/nested/b.md",
        "katana/a.md" => "katana/a.md",
        "katana" => "katana",
        "katana/nested" => "katana/nested",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::tree_static_id;

    #[test]
    fn nested_and_unknown_ids_are_total() {
        assert_eq!("katana/nested", tree_static_id("katana/nested"));
        assert_eq!("unknown", tree_static_id("outside"));
    }
}

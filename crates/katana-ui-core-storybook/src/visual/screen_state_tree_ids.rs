pub(super) fn tree_static_id(value: &str) -> &'static str {
    match value {
        "katana/nested/b.md" => "katana/nested/b.md",
        "katana/a.md" => "katana/a.md",
        "katana" => "katana",
        "katana/nested" => "katana/nested",
        _ => "unknown",
    }
}

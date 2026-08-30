fn main() {
    let app = kuc_consumer_app::ConsumerApp::new();
    let tree = app.render();
    println!(
        "KUC consumer app: root={:?} children={}",
        kuc_consumer_app::root_kind(&tree),
        tree.root().children().len()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn main_runs_in_the_binary_test_target() {
        super::main();
    }
}

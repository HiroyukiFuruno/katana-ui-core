use super::host_root_process::HostRootProcess;

impl HostRootProcess {
    pub(super) fn unicode_font_identity(&self) -> (Option<String>, String) {
        let catalog = self.root.evidence_catalog();
        (
            catalog
                .emoji_face()
                .raw_file_sha256
                .map(|hash| hash.to_hex()),
            catalog.fingerprint().to_hex(),
        )
    }
}

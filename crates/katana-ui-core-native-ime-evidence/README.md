# katana-ui-core-native-ime-evidence

This private CLI is the native producer for KUC Issue #81. It owns a real
`winit` window and forwards OS `WindowEvent::Ime` events through
`egui-winit::State::take_egui_input`. It never synthesizes IME text and never
uses clipboard, paste, fixtures, or a simulated pointer to satisfy the native
transcript.

Windows and Linux are currently hypothesis paths in this workspace: the runner
queries the active layout/engine (`CurrentInputLanguage` on Windows and
`ibus engine`/`fcitx5-remote -n` on Linux), but those hosts have not been
observed here. A missing query is an error and is recorded as an unsuccessful
run; it is not converted into a self-declared substitute.

Example invocation:

```text
cargo run --manifest-path crates/katana-ui-core-native-ime-evidence/Cargo.toml -- \
  --revision=REVISION --challenge=CHALLENGE --producer-id=PRODUCER \
  --runner-id=RUNNER --output=/tmp/kuc-native-ime.json
```

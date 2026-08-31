# rquickjs build spike

This is an isolated feasibility check for the Step 2 JavaScript runtime. It is
intentionally outside the application crate until the cross-platform build is
accepted.

Run it from the repository root:

```text
cargo check --manifest-path src-tauri/spikes/rquickjs/Cargo.toml
cargo run --manifest-path src-tauri/spikes/rquickjs/Cargo.toml
```

The program verifies basic ES evaluation, exception propagation, and the
QuickJS interrupt handler used later for the five-second script budget. Run the
same commands on Windows and Linux before promoting the dependency into
`src-tauri/Cargo.toml`.


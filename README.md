# bevy_mistUI

bevy_mistUI is an incubating Bevy UI plugin extracted from ROPE.

Current scope:
- procedural mist/smoke borders for 2D Bevy UI
- automatic ring shells that follow `ComputedNode` bounds
- a small public API centered on `SmokeBorder` and `SmokeRingPlugin`

Current public API:
- `SmokeBorder`
- `SmokeRingMaterial`
- `SmokeRingParams`
- `SmokeRingPlugin`
- `SmokeRingPadding`
- `SmokeRingBundle`

Local example:
```bash
cargo run --manifest-path plugins/bevy_mistUI/Cargo.toml --example bevy_mistUI_gallery
```

ROPE mirror scene:
```bash
cargo run --manifest-path client/bevy/Cargo.toml --bin ui_smoke_debug
```

The crate is still being prepared for eventual publication to crates.io.

Repository:
- https://github.com/UlyssesLeoLee/ROPE_CS

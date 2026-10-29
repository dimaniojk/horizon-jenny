# Progress

## Done

- Audited the HUD port (`../horizon-port`, 37 simulation tests, `target/horizon-mod/sexmod.hmod`) and left it in place.
- Cloned and pinned Horizon and Horizon Mixins to the template's `compatibility.json` revisions. Staging uses `git archive`; the Horizon checkout is not edited.
- Created `horizon-native-port` from the native template (`sexmod-native`). The injected module is the GeckoLib interpreter plus actor state: bone hierarchy, pivots, Z/Y/X rotation, scale, easing, the small Molang subset, clip fallback, and 0.05 s tick steps with partial-tick sampling.
- Replaced HUD projection with entity vertices. The old 1,024-shape cap is not applied (`MIN_CUBE_VOLUME = 0`).
- Hooked `/sexmod` (spawn, play, nude, dress, pay, stop), the player-body tick, end-crystal entity submission, and atlas build. Payment uses the real inventory and the one-stack rule.
- Standalone tests cover parsing, hierarchy, pivots, interpolation, easing, UVs, downsampling, missing files, two actors, idle fallback, camera-relative positions, and a linear rotation midpoint.

## Validation

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | passed |
| `cargo clippy --workspace --all-targets --offline -- -D warnings` | passed |
| `cargo test --offline --lib` | 15 passed |
| `python3 tools/test_template.py` | fails on purpose: the fixture only contains the example `break_block` target and expects one injection. This bundle targets the real Horizon functions. |
| `python3 tools/horizon.py stage --horizon ../horizon` | passed. Plan applied at the pinned revision. |
| `cargo test --workspace --locked --offline` | 15 native tests passed. The audit crate has no tests. |
| `cargo test -p sexmod-sim --offline` in `../horizon-port` | 37 passed |
| `python3 tools/horizon.py check --horizon ../horizon` | passed (`cargo check` exit 0) |
| `python3 tools/horizon.py test --horizon ../horizon` | passed. Filter `game::sexmod::tests`: 7 passed inside the Horizon binary (camera offset, atlas UV, downsample, two actors, missing textures, rotation midpoint, idle fallback). The other 7 library tests live under `figure::tests` and passed in the standalone crate; that filter does not select them. |
| `python3 tools/horizon.py build --horizon ../horizon` | passed again after the final source. Binary: `target/horizon-cargo/fast/horizon-client`. `--help` runs. |
| Interactive world | launched with `--resources` pointed at the v0.2.96 `26.3` tree. Jenny, Ellie, and the slime rendered with the real mesh inside the F3+B box. Singleplayer clicks go through the remote use and attack handlers; this build hooks those. The chat line has not been confirmed in game yet. |

## Not done

- Default-feature audio and a real sound-event map.
- The full sexmod-sim scene machine inside the native client.
- Server entities.
- The rest of the sexmod scene machine beyond undress, the cum bar, thrust, and the cum clip. Bed, payment, and dialogue are not played. Undress, the bar, the movement lock, and F5 have been seen in game. This build keeps the player on the feet where the sex clip started, so the head pose and Shift do not move the camera. Yaw and pitch stay on the mouse. Flat cards overlap by half a pixel in their plane. Steve quads sample the local player's atlas skin. Breast cards use GeckoLib's face UVs and cardinal normals. The chest, the free look, and the skin have not been confirmed in this launch.

# Native port integration

Pinned by `compatibility.json` and checked against the local clones:

- Horizon `028f60881678a9a0e3e198f693a82858fa475c76`
- Horizon Mixins `0500ff24aa5933fca95a877ef41125353c4b4fe3` (0.5.0, plan schema 5)

Staging copies that Horizon commit and applies the bundle. The upstream checkout is not modified.

## Why the entity pipeline

`Frame.entities` in `crates/horizon-client/src/gfx.rs` is a slice of `mesher::Vertex`. The entity shader samples one 2D HUD atlas (`entity.wgsl`), not a texture array. Each quad is four vertices. Positions are camera-relative. Depth, perspective, and the lightmap come from that pass.

A second renderer would need a new bind group and shader. Mixins cannot add crate dependencies or new pipeline state cleanly, and the functions that create the pipeline contain macros the planner will not rewrite. The port therefore appends vertices to the existing buffer and puts character texels in the existing atlas.

## Injection points

The planner rejects a target whose body contains a macro (`format!`, `matches!`, `vec!`, …), including `HEAD`. These four functions are macro-free:

| Injection | Target | Why this function |
| --- | --- | --- |
| `before-command` | `horizon_client::game::commands::Game::run_command` | Cancellable `HEAD`. Handles `/sexmod` before the vanilla dispatcher. `Game::tick` and `mob_geometry` both contain macros, so they are not targets. |
| `on-tick` | `horizon_client::game::Game::sync_player_body` | Called on every local tick, including while the world is paused only as far as this function itself runs. Advances animation time by 0.05 s and reads sneak/jump from `game.keys`. |
| `entity-geometry` | `horizon_client::game::end::Game::end_geometry` | Called once per world frame with the entity vertex buffer, after mobs and before the frame is submitted. `mob_geometry` contains `matches!`. Partial tick is `game.clock.fraction()`, the same value the render path stores as `alpha`. |
| `atlas-extend` | `horizon_client::game::UiResources::load` | Runs in the **binary** (`main.rs` owns `mod game`; `hud` is a library module and cannot see `game`). Writes the three character PNGs into the resource root before `HudAtlas::build_over`. Later pack reloads read those files. `HudAtlas::slots` contains macros, and `RETURN` on `build_over` is rejected because the signature returns `Self`. |

The support module `sexmod` is injected into `crates/horizon-client/src/game.rs` from `mixins/sexmod/runtime.rs`. Helpers are single private functions. The payload has no inner attributes, `include!`, or external `mod` files. `Cargo.toml` dependencies are not injected; the payload only uses crates Horizon already links (`serde_json`, `png`).

## Rendering path

1. `build_over` writes `assets/minecraft/textures/font/sexmod_{jenny,ellie,slime}.png` (1024×1024 RGBA, box-filtered from the source PNG). The walker names them `font/sexmod_*`. The `font/` prefix is only how a texture taller than 512 texels is accepted (`gh <= 512 || name.starts_with("font/")`). No font provider references these files.
2. Each actor samples `animation.<id>.<clip>` through the copied GeckoLib 3 interpreter. Render time is `seconds + alpha * 0.05`.
3. `visible_quads` emits world-space corners, 0..1 UVs, and a face normal. Cubes are not dropped for volume (`MIN_CUBE_VOLUME = 0`).
4. The geometry helper turns each quad into four `Vertex` values: camera-relative position, atlas UV from the sprite rect, full-white tint, world light packed as mobs pack it (`(sky_nibble * 16) | ((block_nibble * 16) << 8)`), and a signed normal with `w = 127` so `entity.wgsl` applies directional light.
5. Actors farther than 96 blocks are skipped. There is no per-frame geometry rebuild of the rest pose; the posed quads are produced from the cached figure.

## What stays client-local

Actors live in a thread-local list. They are not server entities, they are not saved, and other players cannot see them. Inventory payment reads `Game.inventory` (`ItemStack.item`, `ItemStack.count`) and only removes the cost from one stack that already holds enough, matching the original payment rule. Creative mode is `game.mode == GameMode::Creative`.

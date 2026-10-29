# Sexmod native Horizon client

18+. This repository is a native Horizon client mod. Sex Mod models, textures, and animations are not included.

This directory builds a **custom Horizon executable**. It is not an `.hmod` package and it does not install into the stock Horizon launcher.

The client draws Jenny, Ellie, and the slime as world-space entity geometry. Vertices go through Horizon's existing entity pipeline (`Frame.entities`, `entity.wgsl`), so they use the game camera, perspective, depth test, and entity lighting. Nothing is projected onto the HUD.

## What you need

- The pinned checkouts already live next to this directory: `../horizon` at `028f60881678a9a0e3e198f693a82858fa475c76` and `../horizon-mixins` at `0500ff24aa5933fca95a877ef41125353c4b4fe3`. Staging exports that Horizon commit with `git archive` and does not edit the checkout.
- Rust/Cargo and Python 3. The template records Rust 1.95; this machine's nix Cargo is newer and is what the validation commands used.
- Local model files. They are not in this repository. Set `SEXMOD_ASSETS` to a folder that contains `geo/`, `animations/`, and `textures/` from your own Sex Mod install. If the variable is unset, the client looks in `assets/sexmod` under the working directory.
- A prepared Horizon resource directory (the game's `assets/` tree), passed the same way you start an ordinary Horizon client.

## Build

From this directory:

```sh
cargo fetch --locked
cargo test --workspace --locked --offline
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
```

`check` and `build` stage the pinned engine, apply `mixins/sexmod/horizon-mixin.toml`, and compile `horizon-client` with `--no-default-features` (the template default: no audio device and no controller backend). The executable is printed by the build command. `build` uses the `fast` profile.

The first atlas build writes three 1024×1024 PNGs into the **active resource root**:

```text
assets/minecraft/textures/font/sexmod_jenny.png
assets/minecraft/textures/font/sexmod_ellie.png
assets/minecraft/textures/font/sexmod_slime.png
```

They are box-downsampled from the original 4096×4096 textures. Horizon's atlas walker drops a non-font texture taller than 512 texels, and the `font/` prefix is what lets these images through. They are not font providers. The resource root must be writable. If the files cannot be written, or `SEXMOD_ASSETS` is missing, the models sample `horizon/white`.

## Run

Start the staged binary the same way as Horizon, with your resource directory, for example:

```sh
SEXMOD_ASSETS=/path/to/sexmod/assets \
  ./target/horizon-cargo/fast/horizon-client --resources /path/to/horizon/resources
```

`--resources` is required. `--offscreen` renders without presenting a window; `--frames N` exits after N frames. This environment compiled the binary and ran `--help`. It did not open a world: the only resource tree on disk is the live `~/.local/share/horizon` install, and the first atlas build would write the three character PNGs into it.

In game, open chat and use:

- `/sexmod jenny`, `/sexmod ellie`, `/sexmod slime` — spawn that model two blocks in front of you. She stays at that world position while the camera moves.
- `/sexmod play <suffix>` — play `animation.<id>.<suffix>` (unknown clips fall back to idle).
- `/sexmod nude` / `/sexmod dress`
- `/sexmod pay` — Jenny takes 3 emeralds from one real inventory stack. Creative mode and a zero cost skip the charge.
- `/sexmod stop` — remove the local actors.
- Right-click or left-click her when the crosshair is on her body and no block is closer. Jenny and Ellie open the original button list (Blowjob, Doggy, Strip; Strip, Cowgirl). A button charges the same items as the mod, then plays that clip. Strip swaps to the nude model at the undress cue. Sex clips show the cum bar; Shift thrusts and Space at a full bar plays the cum clip. Walking, mining, and clicking are off until the scene ends. You stay where the clip started, and the mouse still turns the view. Shift does not move you. That body uses your player skin. Hold an emerald, diamond, gold ingot, ender pearl, or dye and she walks after you; otherwise she wanders. The original girl has no attack AI. Escape closes the list. Slime has no menu.

Sneak and jump, while a matching clip is playing, switch the local clip. They do not capture the mouse or replace normal movement.

## Tests

```sh
cargo test --workspace --locked --offline
python3 tools/horizon.py test --horizon ../horizon
```

The standalone library tests cover geometry, bones, pivots, interpolation, easing, UVs, textures, world placement, camera-relative positions, multiple actors, missing assets, and the idle fallback. `python3 tools/test_template.py` still targets the upstream example's single `break_block` fixture, so it does not describe this bundle. The real plan is `python3 tools/horizon.py stage`.

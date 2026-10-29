# Known limitations

## Not a sandboxed mod

The output is a custom `horizon-client` binary produced with `horizon-mixins`. It is not `sexmod.hmod`. The HUD port in `../horizon-port` is unchanged and still projects models onto the HUD with a 1,024-shape cap. This client does not use that path.

## Textures are resampled

Source PNGs are 4096×4096. The entity shader can only sample the shared HUD atlas. Non-font textures taller than 512 texels are dropped by `HudAtlas::pack`, and `RETURN` cannot append rows to `build_over` because its signature returns `Self`. The atlas hook therefore writes 1024×1024 PNGs under `textures/font/` before packing. Those are the original pixels, box-filtered, not flat face colors. The resource root has to be writable. Missing files fall back to `horizon/white`.

The files are written from `UiResources::load` in the game binary, before the first atlas build. A later pack reload reads them from disk. Library tests that call `HudAtlas::build` directly do not seed them.

## No server-authoritative entities

Spawns, clips, and payment are client-local. There is no protocol, no chunk save, and no claim that other clients see the model. Movement of the actor is limited to facing the player. The model does not collide, path, or block the world.

## Gameplay is thinner than the `.hmod`

The native runtime does not embed `sexmod-sim`. Bed slots, pregnancy timers, payment animations, and dialogue stay in `horizon-port`. A scene clip's `sound_effects` are read: `becomeNude` swaps to the nude geometry, `stripDone` returns to idle (or starts doggy/cowgirl if that was the choice while she was dressed), sex cues show the cum bar and add the original amounts, and `bjcDone` / `doggyCumDone` / `cowgirlcumDone` end the scene. An unknown or empty clip still falls back to idle. Slime files still have no `strip` or `blowjobintro` clips. Ellie's slow cowgirl clip remains `animation.ellie.cowgirlslow2`.

## Audio is not played

`tools/horizon.py` compiles with `--no-default-features`, which disables Horizon's audio backend. `Audio::play` exists as a type, but mod sound events are not registered and this build does not open a device. The native client does not spam chat with sound cues. Wiring sound needs a default-feature build and an event registry, and it is independent of the world-space renderer.

## Input

While the menu is open or a scene clip is playing, the client tick drops W, A, S, D, sprint, sneak, and jump before they are packed into the movement packet. Sneak and jump are remembered for the scene: Shift starts the thrust clip, and Space plays the cum clip once the bar is full. Clicks are swallowed for that time. `/sexmod stop` removes the actors and the lock ends with the scene.

Singleplayer joins an integrated server, so the mouse goes to the remote use and attack handlers. Those handlers raycast her box (1.0 wide, 1.95 tall, slime 0.9 by 1.85). A hit closer than a block or a real mob releases the cursor and draws Jenny's or Ellie's buttons on the right, the same labels and prices as `MenuUI`. Choosing one sends `/sexmod scene`, which removes the items on the server and plays the clip. Escape closes the list. Slime has no menu. F3+B draws the same box. She is still not a server entity, so there is no vanilla target outline.

## Rendering gaps

- The model is submitted with the end-crystal pass, which does not run on the title screen.
- `sync_player_body` does not run for a remote/dedicated session that returns earlier in `tick`.
- Quads are one-sided in the sense of `normal.w > 0`: the shader lights them, and the back face keeps the front lighting rather than a flipped normal. Winding follows the GeckoLib cube order. The engine's own back-face policy still applies.
- Lighting uses the block light at the actor anchor, packed like other mobs. There is no per-bone light.
- The first spawn parses geometry on that frame.
- Payment props stay hidden. During a scene that includes the player, the Steve torso, arms, legs, and penis are drawn and the Steve head is not, so the first-person camera is not inside the skull. Outside those scenes the rig stays hidden.
- Breast and shoulder rounds are the authored zero-thickness cards from the geo file. Face UVs follow `GeoQuad` (unmirrored vertex 0 gets `u2`). They are not a smooth mesh.
- Idle Jenny and Ellie follow while the player holds an emerald, diamond, gold ingot, ender pearl, or dye (`EntityAITempt`), and otherwise wander. The decompiled girl has no attack task and no attack animation, and she is not a server mob, so she does not deal damage.

## What a click does

Right-click and left-click open the button list. The choice pays (unless the cost is zero or the player is creative) and starts that clip: Jenny blowjob, doggy, or strip; Ellie strip or cowgirl. Doggy and cowgirl strip first when she is still dressed. The cum bar is a HUD fill driven by the clip's sound cues. During a player scene the feet stay where you were when that clip started, so the posed head does not drag the camera, and Shift does not move you. A `moveCamera` cue still applies the mod's offset once. The mouse owns yaw and pitch. The Steve rig samples the local player's skin from the HUD atlas (`skins/…`, otherwise the wide or slim default). Wide versus slim arms follow the opaque texel at (54, 24), the same test as `GirlModel`. F5 stays free. This look and this skin have not been confirmed in game yet. `/sexmod pay`, `nude`, and `play` still work.

fn before_mobs(
    game: &crate::game::Game<'_>,
    camera: glam::DVec3,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) {
    crate::game::sexmod::set_show_steve_head(!game.camera_type.first_person());
    let camera_xyz = [camera.x, camera.y, camera.z];
    let alpha = game.clock.fraction();
    let pack = |component: f32| (component.clamp(-1.0, 1.0) * 127.0).round() as i8;
    let skin_name = game
        .player_model
        .as_ref()
        .and_then(|mob| mob.skin.clone())
        .unwrap_or_else(|| "entity/player/wide/steve".to_owned());
    let (known, _, ready) = crate::game::sexmod::skin_arms();
    if known != skin_name || !ready {
        let mut wide = !skin_name.contains("slim");
        let mut settled = !skin_name.starts_with("skins/");
        if let Some(hash) = skin_name.strip_prefix("skins/") {
            let mut path = std::env::current_dir().unwrap_or_default();
            path.push("assets");
            path.push("skins");
            if hash.len() >= 2 {
                path.push(&hash[..2]);
                path.push(hash);
            }
            if let Ok(bytes) = std::fs::read(&path) {
                if let Ok((width, height, rgba)) = horizon_client::hud::decode_bytes(&bytes) {
                    settled = true;
                    if width > 54 && height > 24 {
                        let index = ((24 * width + 54) * 4 + 3) as usize;
                        if index < rgba.len() {
                            // GirlModel: opaque texel (54, 24) is the wide Steve arm.
                            wide = rgba[index] != 0;
                        }
                    }
                }
            }
        }
        crate::game::sexmod::store_skin_arms(&skin_name, wide, settled);
    }
    let skin_rect = game
        .gui
        .atlas
        .sprites
        .get(&skin_name)
        .copied()
        .or_else(|| {
            game.gui
                .atlas
                .sprites
                .get("entity/player/wide/steve")
                .copied()
        })
        .map(|rect| [rect.u0, rect.v0, rect.u1, rect.v1]);
    for quad in crate::game::sexmod::visible_quads(camera_xyz, alpha) {
        let Some(rect) = game
            .gui
            .atlas
            .sprites
            .get(&quad.sprite)
            .copied()
            .or_else(|| game.gui.atlas.sprites.get("horizon/white").copied())
        else {
            continue;
        };
        let mut atlas = [rect.u0, rect.v0, rect.u1, rect.v1];
        let mut corner_uv = quad.uv;
        // A default Steve/Alex path means the offline session has no profile
        // skin. The character sheet already has the launcher account pasted in.
        if quad.player_skin && skin_name.starts_with("skins/") {
            if let Some(skin) = skin_rect {
                atlas = skin;
                for corner in 0..4 {
                    corner_uv[corner] = crate::game::sexmod::player_skin_uv(quad.uv[corner]);
                }
            }
        }
        let light = game.world.light(
            quad.anchor[0].floor() as i32,
            quad.anchor[1].floor() as i32,
            quad.anchor[2].floor() as i32,
        );
        let light = (u32::from(light >> 4) * 16) | ((u32::from(light & 15) * 16) << 8);
        // w < 0 lights the back face with the flipped normal, so a paper card
        // is lit from the side you are looking at.
        let normal = [
            pack(quad.normal[0]),
            pack(quad.normal[1]),
            pack(quad.normal[2]),
            if quad.two_sided { -127 } else { 127 },
        ];
        for corner in 0..4 {
            out.push(horizon_client::mesher::Vertex {
                pos: crate::game::sexmod::camera_relative(quad.corners[corner], camera_xyz),
                uv: crate::game::sexmod::atlas_uv(corner_uv[corner], atlas),
                layer: 0,
                color: 0xFFFF_FFFF,
                light,
                normal,
                glint_uv: [0.0, 0.0],
            });
        }
    }
    if !game.show_hitboxes {
        return;
    }
    let Some(white) = game.gui.atlas.sprites.get("horizon/white").copied() else {
        return;
    };
    let white = [white.u0, white.v0, white.u1, white.v1];
    let uv = crate::game::sexmod::atlas_uv([0.5, 0.5], white);
    for actor in crate::game::sexmod::actor_boxes() {
        let half = actor.width * 0.5;
        let min = [
            actor.feet[0] - half,
            actor.feet[1],
            actor.feet[2] - half,
        ];
        let max = [
            actor.feet[0] + half,
            actor.feet[1] + actor.height,
            actor.feet[2] + half,
        ];
        let light = game.world.light(
            actor.feet[0].floor() as i32,
            actor.feet[1].floor() as i32,
            actor.feet[2].floor() as i32,
        );
        let light = (u32::from(light >> 4) * 16) | ((u32::from(light & 15) * 16) << 8);
        let corners = [
            [min[0], min[1], min[2]],
            [max[0], min[1], min[2]],
            [max[0], min[1], max[2]],
            [min[0], min[1], max[2]],
            [min[0], max[1], min[2]],
            [max[0], max[1], min[2]],
            [max[0], max[1], max[2]],
            [min[0], max[1], max[2]],
        ];
        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];
        let thickness = 0.015;
        for (start, end) in edges {
            let a = corners[start];
            let b = corners[end];
            let axis = if (a[0] - b[0]).abs() > 0.0 {
                0
            } else if (a[1] - b[1]).abs() > 0.0 {
                1
            } else {
                2
            };
            for expand in [(axis + 1) % 3, (axis + 2) % 3] {
                let mut quad = [a, a, b, b];
                quad[0][expand] -= thickness;
                quad[1][expand] += thickness;
                quad[2][expand] += thickness;
                quad[3][expand] -= thickness;
                let normal = [pack(0.0), pack(1.0), pack(0.0), 0];
                for corner in quad {
                    out.push(horizon_client::mesher::Vertex {
                        pos: crate::game::sexmod::camera_relative(corner, camera_xyz),
                        uv,
                        layer: 0,
                        color: 0xFFFF_FFFF,
                        light,
                        normal,
                        glint_uv: [0.0, 0.0],
                    });
                }
            }
        }
    }
}

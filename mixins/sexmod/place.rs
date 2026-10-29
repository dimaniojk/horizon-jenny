fn place_during_scene(game: &mut crate::game::Game<'_>) {
    let Some(place) = super::sexmod::player_place() else {
        return;
    };
    let eye = game
        .player
        .as_ref()
        .map(|player| f64::from(player.dimensions.eye_height))
        .unwrap_or(1.62);
    if let Some(player) = game.player.as_mut() {
        player.position.x = place.pos[0];
        player.position.y = if place.at_eye {
            place.pos[1] - eye
        } else {
            place.pos[1]
        };
        player.position.z = place.pos[2];
        player.velocity = Vec3::ZERO;
    }
    if place.lock_look {
        game.yaw = place.yaw;
    }
    if place.lock_pitch {
        game.pitch = place.pitch;
    }
    if place.release {
        super::sexmod::clear_place();
    }
}

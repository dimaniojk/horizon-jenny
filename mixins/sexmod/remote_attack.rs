fn before_remote_attack(game: &mut crate::game::Game<'_>) -> core::ops::ControlFlow<bool> {
    if super::sexmod::controls_locked() {
        return core::ops::ControlFlow::Break(true);
    }
    if game.spectator() {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(eye) = game.eye_position() else {
        return core::ops::ControlFlow::Continue(());
    };
    let dir = super::view_vector(game.yaw, game.pitch);
    let Some(hit) = super::sexmod::look(eye, dir, game.reach()) else {
        return core::ops::ControlFlow::Continue(());
    };
    if !game.before_block(hit.distance) || game.mob_hit().is_some_and(|mob| mob.1 < hit.distance) {
        return core::ops::ControlFlow::Continue(());
    }
    if super::sexmod::open_menu(hit.name) {
        super::sexmod::begin_release();
        game.capture(false);
    } else {
        let message = super::sexmod::activate(hit.name);
        game.system_message(horizon_client::text::Text::literal(message));
    }
    game.swing_arm();
    core::ops::ControlFlow::Break(true)
}

fn before_interact(
    game: &mut crate::game::Game<'_>,
) -> core::ops::ControlFlow<Result<bool>> {
    if game.spectator() {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(eye) = game.eye_position() else {
        return core::ops::ControlFlow::Continue(());
    };
    let dir = view_vector(game.yaw, game.pitch);
    let Some(hit) = sexmod::look(eye, dir, game.reach()) else {
        return core::ops::ControlFlow::Continue(());
    };
    if !game.before_block(hit.distance) || game.mob_hit().is_some_and(|mob| mob.1 < hit.distance) {
        return core::ops::ControlFlow::Continue(());
    }
    let message = sexmod::activate(hit.name);
    game.system_message(horizon_client::text::Text::literal(message));
    game.swing_arm();
    core::ops::ControlFlow::Break(Ok(true))
}

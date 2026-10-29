fn on_tick(game: &mut crate::game::Game<'_>) {
    if game.remote.is_none() {
        return;
    }
    let (sneak, jump) = if crate::game::sexmod::controls_locked() {
        crate::game::sexmod::held_controls()
    } else {
        (
            game.keys.contains(&KeyCode::ShiftLeft),
            game.keys.contains(&KeyCode::Space),
        )
    };
    let feet = game.player.as_ref().map(|player| {
        [
            player.position.x,
            player.position.y,
            player.position.z,
        ]
    });
    let tempted = game
        .held()
        .is_some_and(|stack| crate::game::sexmod::temptation(&stack.item))
        || game
            .offhand
            .as_ref()
            .is_some_and(|stack| crate::game::sexmod::temptation(&stack.item));
    crate::game::sexmod::tick(feet, sneak, jump, Some((game.yaw, game.pitch)), tempted);
}

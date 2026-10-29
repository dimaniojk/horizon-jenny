fn lock_movement(game: &mut crate::game::Game<'_>) {
    if !super::sexmod::controls_locked() {
        return;
    }
    super::sexmod::note_controls(
        game.physical_keys.contains(&KeyCode::ShiftLeft)
            || game.physical_keys.contains(&KeyCode::ShiftRight),
        game.physical_keys.contains(&KeyCode::Space),
    );
    game.keys.remove(&KeyCode::KeyW);
    game.keys.remove(&KeyCode::KeyA);
    game.keys.remove(&KeyCode::KeyS);
    game.keys.remove(&KeyCode::KeyD);
    game.keys.remove(&KeyCode::Space);
    game.keys.remove(&KeyCode::ShiftLeft);
    game.keys.remove(&KeyCode::ControlLeft);
}

fn before_mapped_key(
    game: &mut crate::game::Game<'_>,
    key: winit::keyboard::KeyCode,
    pressed: bool,
    repeat: bool,
) -> core::ops::ControlFlow<Result<()>> {
    let _ = repeat;
    if pressed && key == winit::keyboard::KeyCode::Escape && crate::game::sexmod::menu_open() {
        crate::game::sexmod::close_menu();
        game.capture(true);
        return core::ops::ControlFlow::Break(Ok(()));
    }
    core::ops::ControlFlow::Continue(())
}

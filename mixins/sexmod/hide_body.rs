fn hide_real_body(game: &crate::game::Game<'_>) -> core::ops::ControlFlow<()> {
    let _ = game;
    // DontRenderPlayersThatHaveSex: the Steve rig inside the girl is the body.
    if crate::game::sexmod::in_player_body() {
        core::ops::ControlFlow::Break(())
    } else {
        core::ops::ControlFlow::Continue(())
    }
}

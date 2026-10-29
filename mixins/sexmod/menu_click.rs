fn on_menu_click(game: &mut crate::game::Game<'_>) {
    if crate::game::sexmod::take_release() || !crate::game::sexmod::menu_open() {
        return;
    }
    let size = game
        .renderer
        .as_ref()
        .map_or([1, 1], |renderer| [renderer.config.width, renderer.config.height]);
    let gui = horizon_client::hud::HudBatch::new(size);
    if let Some(action) =
        crate::game::sexmod::menu_command_at(game.cursor.0, game.cursor.1, gui.width, gui.height)
    {
        crate::game::sexmod::close_menu();
        game.send_chat(&format!("/sexmod scene {action}"));
    }
}

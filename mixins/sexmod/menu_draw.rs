fn paint_menu(game: &mut crate::game::Game<'_>, mut batch: HudBatch) -> HudBatch {
    if crate::game::sexmod::menu_open() && game.captured {
        crate::game::sexmod::begin_release();
        game.capture(false);
    }
    let buttons = crate::game::sexmod::menu_buttons(batch.width, batch.height);
    let meter = crate::game::sexmod::meter();
    if buttons.is_empty() && !meter.shown {
        return batch;
    }
    let atlas = &game.gui.atlas;
    if meter.shown {
        let track_h = 160.0;
        let track_w = 12.0;
        let x = 8.0;
        let y = (batch.height - track_h) * 0.5;
        batch.fill(atlas, x, y, track_w, track_h, 0xC0101010);
        let fill_h = track_h * meter.fill.clamp(0.0, 1.0);
        let fill = if meter.thrusting { 0xFFFFE08A } else { 0xFFF4F4F4 };
        batch.fill(atlas, x, y + track_h - fill_h, track_w, fill_h, fill);
        batch.text(atlas, "cum", x + track_w + 6.0, y, 0xFFFFFFFF, true);
        if meter.fill >= 1.0 {
            batch.text(atlas, "Space", x + track_w + 6.0, y + track_h - 10.0, 0xFFFFFFFF, true);
        }
    }
    if buttons.is_empty() {
        return batch;
    }
    let (mouse_x, mouse_y) = game.cursor;
    for button in buttons {
        let hover = mouse_x >= button.x
            && mouse_x <= button.x + button.w
            && mouse_y >= button.y
            && mouse_y <= button.y + button.h;
        let fill = if hover { 0xFFFFFFFF } else { 0xFFC6C6C6 };
        batch.fill(atlas, button.x, button.y, button.w, button.h, fill);
        batch.fill(atlas, button.x, button.y, button.w, 1.0, 0xFF000000);
        batch.fill(
            atlas,
            button.x,
            button.y + button.h - 1.0,
            button.w,
            1.0,
            0xFF000000,
        );
        let text_w = horizon_client::hud::HudBatch::text_width(atlas, button.label);
        batch.text(
            atlas,
            button.label,
            button.x + (button.w - text_w) * 0.5,
            button.y + 6.0,
            0xFF000000,
            false,
        );
        if !button.price.is_empty() {
            let price_w = horizon_client::hud::HudBatch::text_width(atlas, button.price);
            batch.text(
                atlas,
                button.price,
                button.x - price_w - 6.0,
                button.y + 6.0,
                0xFFFFFFFF,
                true,
            );
        }
    }
    batch
}

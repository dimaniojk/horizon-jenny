fn before_command(
    game: &mut crate::game::Game<'_>,
    command: &str,
) -> core::ops::ControlFlow<bool> {
    let Some(rest) = command.strip_prefix("sexmod") else {
        return core::ops::ControlFlow::Continue(());
    };
    if !rest.is_empty() && !rest.starts_with(' ') {
        return core::ops::ControlFlow::Continue(());
    }
    let args = rest.trim();
    let feet = game
        .player
        .as_ref()
        .map(|player| [player.position.x, player.position.y, player.position.z]);
    let creative = game.mode == horizon_client::command::args::GameMode::Creative;
    let reply = match args {
        "" | "help" => crate::game::sexmod::help().to_owned(),
        "stop" => {
            crate::game::sexmod::clear();
            "She is gone.".to_owned()
        }
        "jenny" | "ellie" | "slime" => match feet {
            Some(feet) => crate::game::sexmod::spawn(args, feet, game.yaw),
            None => "You are not in a world.".to_owned(),
        },
        "nude" => crate::game::sexmod::set_model(0),
        "dress" => crate::game::sexmod::set_model(1),
        "pay" => match crate::game::sexmod::payment() {
            None => "Nobody is here.".to_owned(),
            Some((_, count)) if count == 0 || creative => crate::game::sexmod::begin_paid(),
            Some((item, count)) => {
                if let Some(index) = game.inventory.iter().position(|stack| {
                    stack
                        .as_ref()
                        .is_some_and(|stack| &*stack.item == item && stack.count >= count)
                }) {
                    let stack = game.inventory[index].as_mut().expect("stack");
                    stack.count -= count;
                    if stack.count == 0 {
                        game.inventory[index] = None;
                    }
                    crate::game::sexmod::begin_paid()
                } else {
                    format!("You need {count} {item} in one stack.")
                }
            }
        },
        other => {
            if let Some(clip) = other.strip_prefix("play ") {
                crate::game::sexmod::play(clip.trim())
            } else if let Some(action) = other.strip_prefix("scene ") {
                match crate::game::sexmod::prepare_scene(action.trim()) {
                    Err(message) => message,
                    Ok(bill) if bill.cost == 0 || creative => {
                        crate::game::sexmod::play(bill.clip)
                    }
                    Ok(bill) => {
                        if let Some(index) = game.inventory.iter().position(|stack| {
                            stack.as_ref().is_some_and(|stack| {
                                &*stack.item == bill.item && stack.count >= bill.cost
                            })
                        }) {
                            let stack = game.inventory[index].as_mut().expect("stack");
                            stack.count -= bill.cost;
                            if stack.count == 0 {
                                game.inventory[index] = None;
                            }
                            crate::game::sexmod::play(bill.clip)
                        } else {
                            format!("<{0}> you cannot afford that...", bill.who)
                        }
                    }
                }
            } else {
                format!("Unknown sexmod command. {}", crate::game::sexmod::help())
            }
        }
    };
    game.system_message(horizon_client::text::Text::literal(reply));
    core::ops::ControlFlow::Break(true)
}

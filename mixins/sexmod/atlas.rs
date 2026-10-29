fn before_atlas(root: &std::path::Path) {
    let dir = root.join("assets/minecraft/textures/font");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let side = sexmod::ATLAS_SIDE;
    let mut skin = std::fs::read(
        root.join("assets/minecraft/textures/entity/player/wide/steve.png"),
    )
    .ok();
    // The raw client logs in offline, so the Steve rig would show steve.png.
    // The launcher account is the skin the player actually picked.
    if let Some(home) = std::env::var_os("HOME") {
        let accounts = std::path::PathBuf::from(home).join(".local/share/horizon-launcher/accounts.json");
        if let Ok(text) = std::fs::read_to_string(&accounts) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                let selected = json.get("selected").and_then(|value| value.as_str()).unwrap_or("");
                let list = json.get("accounts").and_then(|value| value.as_array());
                if let Some(list) = list {
                    for account in list {
                        let id = account.get("uuid").and_then(|value| value.as_str()).unwrap_or("");
                        if id != selected {
                            continue;
                        }
                        let url = account
                            .get("skin")
                            .and_then(|value| value.as_array())
                            .and_then(|pair| pair.first())
                            .and_then(|value| value.as_str());
                        let Some(url) = url else {
                            continue;
                        };
                        let Some(hash) = horizon_client::skins::texture_hash(url) else {
                            continue;
                        };
                        let location = horizon_client::skins::skin_location(&hash);
                        let Some(digest) = location.strip_prefix("skins/") else {
                            continue;
                        };
                        let mut cache = std::env::current_dir().unwrap_or_default();
                        cache.push("assets");
                        cache.push("skins");
                        if digest.len() >= 2 {
                            cache.push(&digest[..2]);
                            cache.push(digest);
                        }
                        if let Ok(bytes) = std::fs::read(&cache) {
                            skin = Some(bytes);
                        } else if let Ok(response) = ureq::AgentBuilder::new()
                            .timeout(std::time::Duration::from_secs(8))
                            .build()
                            .get(url)
                            .call()
                        {
                            let mut reader = response.into_reader();
                            let mut body = Vec::new();
                            let mut chunk = [0u8; 8192];
                            loop {
                                match std::io::Read::read(&mut reader, &mut chunk) {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        body.extend_from_slice(&chunk[..n]);
                                        if body.len() > (1 << 20) {
                                            break;
                                        }
                                    }
                                    Err(_) => {
                                        body.clear();
                                        break;
                                    }
                                }
                            }
                            if body.len() > 32 {
                                if let Some(parent) = cache.parent() {
                                    let _ = std::fs::create_dir_all(parent);
                                    let _ = std::fs::write(&cache, &body);
                                }
                                skin = Some(body);
                            }
                        }
                    }
                }
            }
        }
    }
    for (name, pixels) in sexmod::character_textures_in(sexmod::asset_root().as_path(), skin.as_deref())
    {
        let Some(id) = name.strip_prefix("font/sexmod_") else {
            continue;
        };
        let Ok(file) = std::fs::File::create(dir.join(format!("sexmod_{id}.png"))) else {
            continue;
        };
        let mut encoder = png::Encoder::new(file, side, side);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        if let Ok(mut writer) = encoder.write_header() {
            let _ = writer.write_image_data(&pixels);
        }
    }
}

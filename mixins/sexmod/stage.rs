// World-space actors. The geometry modules above sample GeckoLib; this file
// keeps who is standing in the world and turns a pose into draw quads.

use std::sync::Mutex;
use std::path::PathBuf;
use std::sync::Arc;

use figure::{Figure, MeshQuad, Motions};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Jenny,
    Ellie,
    Slime,
}

impl Kind {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "jenny" => Some(Self::Jenny),
            "ellie" => Some(Self::Ellie),
            "slime" => Some(Self::Slime),
            _ => None,
        }
    }

    fn id(self) -> &'static str {
        match self {
            Self::Jenny => "jenny",
            Self::Ellie => "ellie",
            Self::Slime => "slime",
        }
    }

    fn sprite(self) -> String {
        format!("font/sexmod_{}", self.id())
    }

    fn files(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Jenny => (
                "geo/jenny/jennydressed.geo.json",
                "geo/jenny/jennynude.geo.json",
                "animations/jenny/jenny.animation.json",
            ),
            Self::Ellie => (
                "geo/ellie/dressed.geo.json",
                "geo/ellie/nude.geo.json",
                "animations/ellie/ellie.animation.json",
            ),
            Self::Slime => (
                "geo/slime/dressed.geo.json",
                "geo/slime/nude.geo.json",
                "animations/slime/slime.animation.json",
            ),
        }
    }
}

struct Pair {
    dressed: Figure,
    nude: Figure,
}

struct Actor {
    kind: Kind,
    pos: [f64; 3],
    yaw: f32,
    model: i32,
    clip: String,
    seconds: f64,
    sneak: bool,
    jump: bool,
}

struct Library {
    jenny: Option<Pair>,
    ellie: Option<Pair>,
    slime: Option<Pair>,
    error: Option<String>,
}

impl Library {
    fn pair(&mut self, kind: Kind) -> Result<&Pair, String> {
        let slot = match kind {
            Kind::Jenny => &mut self.jenny,
            Kind::Ellie => &mut self.ellie,
            Kind::Slime => &mut self.slime,
        };
        if slot.is_none() {
            *slot = Some(load_pair(kind)?);
        }
        Ok(slot.as_ref().expect("pair"))
    }
}

fn load_pair(kind: Kind) -> Result<Pair, String> {
    let root = asset_root();
    let (dressed, nude, anim) = kind.files();
    let animation = std::fs::read_to_string(root.join(anim))
        .map_err(|err| format!("{}: {err}", root.join(anim).display()))?;
    let motions = Arc::new(Motions::parse(&animation)?);
    let dressed = Figure::parse(
        &std::fs::read_to_string(root.join(dressed))
            .map_err(|err| format!("{}: {err}", root.join(dressed).display()))?,
        Arc::clone(&motions),
        &[],
    )?;
    let nude = Figure::parse(
        &std::fs::read_to_string(root.join(nude))
            .map_err(|err| format!("{}: {err}", root.join(nude).display()))?,
        motions,
        &[],
    )?;
    Ok(Pair { dressed, nude })
}

pub fn asset_root() -> PathBuf {
    if let Ok(path) = std::env::var("SEXMOD_ASSETS") {
        return PathBuf::from(path);
    }
    PathBuf::from("assets/sexmod")
}

struct World {
    actors: Vec<Actor>,
    library: Library,
}

impl World {
    const fn new() -> Self {
        Self {
            actors: Vec::new(),
            library: Library {
                jenny: None,
                ellie: None,
                slime: None,
                error: None,
            },
        }
    }
}

// The integrated server runs commands on one thread and draws on another.
// A thread-local list made the spawn succeed in chat and render nothing.
static WORLD: Mutex<World> = Mutex::new(World::new());

fn with_world<T>(body: impl FnOnce(&mut World) -> T) -> T {
    let mut world = WORLD.lock().expect("sexmod world");
    body(&mut world)
}

pub fn help() -> &'static str {
    "sexmod jenny|ellie|slime spawns her in the world. play <clip>, nude, dress, pay, stop."
}

pub fn clear() {
    with_world(|world| world.actors.clear());
}

pub fn spawn(kind_name: &str, feet: [f64; 3], player_yaw: f32) -> String {
    let Some(kind) = Kind::parse(kind_name) else {
        return "Choose jenny, ellie or slime.".to_owned();
    };
    let (sine, cosine) = player_yaw.to_radians().sin_cos();
    let pos = [
        feet[0] + f64::from(-sine) * 2.0,
        feet[1],
        feet[2] + f64::from(cosine) * 2.0,
    ];
    let actor = Actor {
        kind,
        pos,
        yaw: player_yaw + 180.0,
        model: 1,
        clip: format!("animation.{}.idle", kind.id()),
        seconds: 0.0,
        sneak: false,
        jump: false,
    };
    let loaded = with_world(|world| {
        let loaded = world.library.pair(kind).map(|_| ());
        if loaded.is_ok() {
            world.actors.retain(|actor| actor.kind != kind);
            world.actors.push(actor);
        }
        loaded
    });
    match loaded {
        Ok(()) => format!(
            "{kind_name} is at {:.1}, {:.1}, {:.1}.",
            pos[0], pos[1], pos[2]
        ),
        Err(err) => format!("{kind_name} did not load: {err}"),
    }
}

pub fn set_model(model: i32) -> String {
    with_world(|world| {
        let Some(actor) = world.actors.last_mut() else {
            return "Nobody is here.".to_owned();
        };
        actor.model = model;
        if model == 0 {
            "Nude model.".to_owned()
        } else {
            "Dressed model.".to_owned()
        }
    })
}

pub fn play(suffix: &str) -> String {
    with_world(|world| {
        let Some(actor) = world.actors.last_mut() else {
            return "Nobody is here.".to_owned();
        };
        actor.clip = format!("animation.{}.{suffix}", actor.kind.id());
        actor.seconds = 0.0;
        format!("Playing {}.", actor.clip)
    })
}

pub fn payment() -> Option<(&'static str, u32)> {
    with_world(|world| {
        let actor = world.actors.last()?;
        Some(match actor.kind {
            Kind::Jenny => ("minecraft:emerald", 3),
            Kind::Ellie => ("minecraft:diamond", 0),
            Kind::Slime => ("minecraft:air", 0),
        })
    })
}

pub fn begin_paid() -> String {
    let suffix = with_world(|world| {
        world.actors.last().map(|actor| match actor.kind {
            Kind::Jenny => "blowjobintro",
            Kind::Ellie => "cowgirlstart",
            Kind::Slime => "doggystart",
        })
    });
    match suffix {
        Some(suffix) => play(suffix),
        None => "Nobody is here.".to_owned(),
    }
}

pub fn tick(feet: Option<[f64; 3]>, sneak: bool, jump: bool) {
    with_world(|world| {
        for actor in &mut world.actors {
            actor.seconds += 0.05;
            actor.sneak = sneak;
            if jump
                && !actor.jump
                && (actor.clip.ends_with("slow") || actor.clip.ends_with("slow2"))
            {
                let id = actor.kind.id();
                actor.clip = format!("animation.{id}.{}fast", fast_prefix(&actor.clip));
                actor.seconds = 0.0;
            }
            actor.jump = jump;
            if sneak && actor.clip.contains("suck") {
                actor.clip = format!("animation.{}.blowjobthrust", actor.kind.id());
                actor.seconds = 0.0;
                actor.sneak = false;
            }
            if let Some(feet) = feet {
                let dx = feet[0] - actor.pos[0];
                let dz = feet[2] - actor.pos[2];
                if dx * dx + dz * dz > 0.04 {
                    actor.yaw = (dz.atan2(dx).to_degrees() as f32) + 90.0;
                }
            }
        }
    });
}

fn fast_prefix(clip: &str) -> &'static str {
    if clip.contains("cowgirl") {
        "cowgirl"
    } else if clip.contains("doggy") {
        "doggy"
    } else {
        ""
    }
}

pub struct DrawQuad {
    pub sprite: String,
    pub corners: [[f64; 3]; 4],
    pub uv: [[f32; 2]; 4],
    pub normal: [f32; 3],
    pub anchor: [f64; 3],
}

struct Snapshot {
    kind: Kind,
    pos: [f64; 3],
    yaw: f32,
    model: i32,
    clip: String,
    seconds: f64,
}

pub fn visible_quads(camera: [f64; 3], alpha: f64) -> Vec<DrawQuad> {
    with_world(|world| {
        let snapshots: Vec<Snapshot> = world
            .actors
            .iter()
            .map(|actor| Snapshot {
                kind: actor.kind,
                pos: actor.pos,
                yaw: actor.yaw,
                model: actor.model,
                clip: actor.clip.clone(),
                seconds: actor.seconds,
            })
            .collect();
        let mut out = Vec::new();
        let mut error = None;
        for actor in snapshots {
            let dx = actor.pos[0] - camera[0];
            let dy = actor.pos[1] - camera[1];
            let dz = actor.pos[2] - camera[2];
            if dx * dx + dy * dy + dz * dz > 96.0 * 96.0 {
                continue;
            }
            let pair = match world.library.pair(actor.kind) {
                Ok(pair) => pair,
                Err(err) => {
                    error = Some(err);
                    continue;
                }
            };
            let figure = if actor.model == 0 {
                &pair.nude
            } else {
                &pair.dressed
            };
            let time = actor.seconds + alpha * 0.05;
            let posed = figure.pose(
                &actor.clip,
                time as f32,
                Vec3::new(actor.pos[0], actor.pos[1], actor.pos[2]),
                actor.yaw,
            );
            push_quads(&mut out, &actor, posed);
        }
        if let Some(err) = error {
            world.library.error = Some(err);
        }
        out
    })
}

fn push_quads(out: &mut Vec<DrawQuad>, actor: &Snapshot, posed: Vec<MeshQuad>) {
        for quad in posed {
            let _ = quad.color;
            out.push(DrawQuad {
            sprite: actor.kind.sprite(),
            corners: [
                [quad.corners[0].x, quad.corners[0].y, quad.corners[0].z],
                [quad.corners[1].x, quad.corners[1].y, quad.corners[1].z],
                [quad.corners[2].x, quad.corners[2].y, quad.corners[2].z],
                [quad.corners[3].x, quad.corners[3].y, quad.corners[3].z],
            ],
            uv: quad.uv,
            normal: quad.normal,
            anchor: actor.pos,
        });
    }
}

pub struct LookHit {
    pub name: &'static str,
    pub distance: f64,
}

/// The actor box under a view ray, if it is within `reach`.
pub fn look(eye: [f64; 3], dir: [f64; 3], reach: f64) -> Option<LookHit> {
    with_world(|world| {
        let mut best: Option<LookHit> = None;
        for actor in &world.actors {
            let height = match actor.kind {
                Kind::Slime => 1.85,
                Kind::Jenny | Kind::Ellie => 1.95,
            };
            let half = 0.45;
            let min = [actor.pos[0] - half, actor.pos[1], actor.pos[2] - half];
            let max = [
                actor.pos[0] + half,
                actor.pos[1] + height,
                actor.pos[2] + half,
            ];
            let Some(distance) = ray_aabb(eye, dir, reach, min, max) else {
                continue;
            };
            if best.as_ref().is_none_or(|hit| distance < hit.distance) {
                best = Some(LookHit {
                    name: actor.kind.id(),
                    distance,
                });
            }
        }
        best
    })
}

/// Make the looked-at actor the current one and describe the click.
pub fn activate(name: &str) -> String {
    with_world(|world| {
        let Some(index) = world.actors.iter().position(|actor| actor.kind.id() == name) else {
            return "Nobody is here.".to_owned();
        };
        let actor = world.actors.remove(index);
        let label = match actor.kind {
            Kind::Jenny => "Jenny",
            Kind::Ellie => "Ellie",
            Kind::Slime => "Slime",
        };
        world.actors.push(actor);
        format!("{label} looks at you. Try /sexmod pay, nude, or play <clip>.")
    })
}

fn ray_aabb(eye: [f64; 3], dir: [f64; 3], reach: f64, min: [f64; 3], max: [f64; 3]) -> Option<f64> {
    let (mut t0, mut t1) = (0.0_f64, reach);
    for axis in 0..3 {
        if dir[axis].abs() < 1e-9 {
            if eye[axis] < min[axis] || eye[axis] > max[axis] {
                return None;
            }
            continue;
        }
        let mut ta = (min[axis] - eye[axis]) / dir[axis];
        let mut tb = (max[axis] - eye[axis]) / dir[axis];
        if ta > tb {
            std::mem::swap(&mut ta, &mut tb);
        }
        t0 = t0.max(ta);
        t1 = t1.min(tb);
    }
    (t0 <= t1 && t1 >= 0.0).then_some(t0.max(0.0))
}

pub fn camera_relative(world: [f64; 3], camera: [f64; 3]) -> [f32; 3] {
    [
        (world[0] - camera[0]) as f32,
        (world[1] - camera[1]) as f32,
        (world[2] - camera[2]) as f32,
    ]
}

pub fn atlas_uv(texture_uv: [f32; 2], rect: [f32; 4]) -> [f32; 2] {
    [
        rect[0] + texture_uv[0] * (rect[2] - rect[0]),
        rect[1] + texture_uv[1] * (rect[3] - rect[1]),
    ]
}

pub const ATLAS_SIDE: u32 = 1024;

pub fn downsample_rgba(src: &[u8], sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u8> {
    let mut out = vec![0u8; (dw * dh * 4) as usize];
    if sw == 0 || sh == 0 || dw == 0 || dh == 0 {
        return out;
    }
    for y in 0..dh {
        for x in 0..dw {
            let x0 = x * sw / dw;
            let x1 = ((x + 1) * sw / dw).max(x0 + 1).min(sw);
            let y0 = y * sh / dh;
            let y1 = ((y + 1) * sh / dh).max(y0 + 1).min(sh);
            let mut acc = [0u64; 4];
            let mut count = 0u64;
            for sy in y0..y1 {
                for sx in x0..x1 {
                    let i = ((sy * sw + sx) * 4) as usize;
                    for channel in 0..4 {
                        acc[channel] += src[i + channel] as u64;
                    }
                    count += 1;
                }
            }
            let o = ((y * dw + x) * 4) as usize;
            for channel in 0..4 {
                out[o + channel] = (acc[channel] / count.max(1)) as u8;
            }
        }
    }
    out
}

pub fn character_textures() -> Vec<(String, Vec<u8>)> {
    character_textures_in(&asset_root())
}

pub fn character_textures_in(root: &std::path::Path) -> Vec<(String, Vec<u8>)> {
    let mut images = Vec::new();
    for (name, file) in [
        ("font/sexmod_jenny", "textures/entity/jenny/jenny.png"),
        ("font/sexmod_ellie", "textures/entity/ellie/ellie.png"),
        ("font/sexmod_slime", "textures/entity/slime/slime.png"),
    ] {
        let Ok(bytes) = std::fs::read(root.join(file)) else {
            continue;
        };
        let Ok((width, height, pixels)) = decode_rgba(&bytes) else {
            continue;
        };
        images.push((
            name.to_owned(),
            downsample_rgba(&pixels, width, height, ATLAS_SIDE, ATLAS_SIDE),
        ));
    }
    images
}

pub fn push_atlas_images(images: &mut Vec<(String, u32, u32, Vec<u8>)>) {
    for (name, pixels) in character_textures() {
        images.push((name, ATLAS_SIDE, ATLAS_SIDE, pixels));
    }
}

fn decode_rgba(bytes: &[u8]) -> Result<(u32, u32, Vec<u8>), String> {
    let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
    let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
    let mut buffer = vec![0; reader.output_buffer_size().unwrap_or(0)];
    let info = reader.next_frame(&mut buffer).map_err(|err| err.to_string())?;
    let pixels = match info.color_type {
        png::ColorType::Rgba => buffer[..info.buffer_size()].to_vec(),
        png::ColorType::Rgb => buffer[..info.buffer_size()]
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        _ => return Err("unsupported png".into()),
    };
    Ok((info.width, info.height, pixels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_offset_is_world_minus_camera() {
        assert_eq!(
            camera_relative([10.0, 70.0, -2.0], [8.0, 65.0, -2.0]),
            [2.0, 5.0, 0.0]
        );
    }

    #[test]
    fn atlas_uv_maps_into_the_sprite_rect() {
        let uv = atlas_uv([0.0, 1.0], [0.25, 0.5, 0.5, 1.0]);
        assert!((uv[0] - 0.25).abs() < 1e-6);
        assert!((uv[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn downsample_averages_a_two_by_two_block() {
        let src = [
            0, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255,
        ];
        let out = downsample_rgba(&src, 2, 2, 1, 1);
        assert_eq!(out, vec![63, 63, 63, 255]);
    }

    #[test]
    fn missing_texture_is_skipped() {
        assert!(
            character_textures_in(std::path::Path::new("/missing/sexmod/assets")).is_empty()
        );
    }

    #[test]
    fn ray_enters_the_front_of_an_actor_box() {
        let min = [-0.45, 64.0, 1.55];
        let max = [0.45, 65.95, 2.45];
        let hit = ray_aabb([0.0, 65.6, 0.0], [0.0, 0.0, 1.0], 5.0, min, max).expect("front");
        assert!((hit - 1.55).abs() < 1e-6);
        assert!(ray_aabb([0.0, 65.6, 0.0], [0.0, 0.0, -1.0], 5.0, min, max).is_none());
        assert!(ray_aabb([0.0, 65.6, 0.0], [0.0, 0.0, 1.0], 1.0, min, max).is_none());
    }

    #[test]
    fn two_spawns_are_independent_and_stop_clears_them() {
        clear();
        spawn("jenny", [0.0, 64.0, 0.0], 0.0);
        spawn("ellie", [5.0, 64.0, 0.0], 90.0);
        let count = WORLD.lock().expect("sexmod world").actors.len();
        assert_eq!(count, 2);
        clear();
        let count = WORLD.lock().expect("sexmod world").actors.len();
        assert_eq!(count, 0);
    }

    #[test]
    fn unknown_clip_falls_back_to_idle_geometry() {
        let motions = Arc::new(
            Motions::parse(
                r#"{"animations":{"animation.test.idle":{"loop":true,"animation_length":1,"bones":{}}}}"#,
            )
            .unwrap(),
        );
        let figure = Figure::parse(
            r#"{"minecraft:geometry":[{"description":{"texture_width":16,"texture_height":16},"bones":[{"name":"body","pivot":[0,0,0],"cubes":[{"origin":[-8,0,-8],"size":[16,16,16],"uv":{"north":{"uv":[0,0],"uv_size":[16,16]}}}]}]}]}"#,
            motions,
            &[],
        )
        .unwrap();
        let quads = figure.pose("animation.test.missing", 0.0, Vec3::ZERO, 0.0);
        assert_eq!(quads.len(), 1);
        assert!(quads[0].uv.iter().any(|uv| uv[0] == 0.0));
        assert!(quads[0].uv.iter().any(|uv| (uv[0] - 1.0).abs() < 1e-4));
    }

    #[test]
    fn linear_rotation_is_halfway_at_the_midpoint() {
        let motions = Arc::new(Motions::parse(r#"{"animations":{"animation.test.idle":{"loop":false,"animation_length":1,"bones":{"body":{"rotation":{"0.0":{"vector":[0,0,0]},"1.0":{"vector":[0,0,90],"easing":"linear"}}}}}}}"#).unwrap());
        let figure = Figure::parse(
            r#"{"minecraft:geometry":[{"description":{"texture_width":16,"texture_height":16},"bones":[{"name":"body","pivot":[0,0,0],"cubes":[{"origin":[-8,0,-8],"size":[16,16,16],"uv":{"up":{"uv":[0,0],"uv_size":[16,16]}}}]}]}]}"#,
            motions,
            &[],
        )
        .unwrap();
        let mid = figure.pose("animation.test.idle", 0.5, Vec3::ZERO, 0.0);
        let halfway = mid[0].corners.iter().any(|corner| {
            (corner.x + 0.354).abs() < 0.03
                && (corner.y - 1.060).abs() < 0.03
                && (corner.z - 0.5).abs() < 0.03
        });
        assert!(halfway, "{:?}", mid[0].corners);
    }
}

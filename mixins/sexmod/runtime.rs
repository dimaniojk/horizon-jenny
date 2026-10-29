// Native GeckoLib sampler injected into Horizon's client.
// The same file is the standalone test crate.

mod easing {
    // `software.bernie.geckolib3.core.easing.EasingManager`.
    // The argument is the first `easingArgs` entry. `None` uses GeckoLib's default.

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Ease {
        Linear,
        Step(Option<f32>),
        InSine,
        OutSine,
        InOutSine,
        InQuad,
        OutQuad,
        InOutQuad,
        InCubic,
        OutCubic,
        InOutCubic,
        InQuart,
        OutQuart,
        InOutQuart,
        InQuint,
        OutQuint,
        InOutQuint,
        InExpo,
        OutExpo,
        InOutExpo,
        InCirc,
        OutCirc,
        InOutCirc,
        InBack(Option<f32>),
        OutBack(Option<f32>),
        InOutBack(Option<f32>),
        InElastic(Option<f32>),
        OutElastic(Option<f32>),
        InOutElastic(Option<f32>),
        InBounce(Option<f32>),
        OutBounce(Option<f32>),
        InOutBounce(Option<f32>),
    }

    impl Ease {
        pub fn parse(name: &str, arg: Option<f32>) -> Self {
            match name {
                "linear" => Self::Linear,
                "step" => Self::Step(arg),
                "easeInSine" => Self::InSine,
                "easeOutSine" => Self::OutSine,
                "easeInOutSine" => Self::InOutSine,
                "easeInQuad" => Self::InQuad,
                "easeOutQuad" => Self::OutQuad,
                "easeInOutQuad" => Self::InOutQuad,
                "easeInCubic" => Self::InCubic,
                "easeOutCubic" => Self::OutCubic,
                "easeInOutCubic" => Self::InOutCubic,
                "easeInQuart" => Self::InQuart,
                "easeOutQuart" => Self::OutQuart,
                "easeInOutQuart" => Self::InOutQuart,
                "easeInQuint" => Self::InQuint,
                "easeOutQuint" => Self::OutQuint,
                "easeInOutQuint" => Self::InOutQuint,
                "easeInExpo" => Self::InExpo,
                "easeOutExpo" => Self::OutExpo,
                "easeInOutExpo" => Self::InOutExpo,
                "easeInCirc" => Self::InCirc,
                "easeOutCirc" => Self::OutCirc,
                "easeInOutCirc" => Self::InOutCirc,
                "easeInBack" => Self::InBack(arg),
                "easeOutBack" => Self::OutBack(arg),
                "easeInOutBack" => Self::InOutBack(arg),
                "easeInElastic" => Self::InElastic(arg),
                "easeOutElastic" => Self::OutElastic(arg),
                "easeInOutElastic" => Self::InOutElastic(arg),
                "easeInBounce" => Self::InBounce(arg),
                "easeOutBounce" => Self::OutBounce(arg),
                "easeInOutBounce" => Self::InOutBounce(arg),
                _ => Self::Linear,
            }
        }

        pub fn apply(self, t: f32) -> f32 {
            let t = t.clamp(0.0, 1.0) as f64;
            let eased = match self {
                Self::Linear => t,
                Self::Step(arg) => step(t, arg.map(|v| v as i32)),
                Self::InSine => sin(t),
                Self::OutSine => out(sin, t),
                Self::InOutSine => in_out(sin, t),
                Self::InQuad => quad(t),
                Self::OutQuad => out(quad, t),
                Self::InOutQuad => in_out(quad, t),
                Self::InCubic => cubic(t),
                Self::OutCubic => out(cubic, t),
                Self::InOutCubic => in_out(cubic, t),
                Self::InQuart => quart(t),
                Self::OutQuart => out(quart, t),
                Self::InOutQuart => in_out(quart, t),
                Self::InQuint => quint(t),
                Self::OutQuint => out(quint, t),
                Self::InOutQuint => in_out(quint, t),
                Self::InExpo => exp(t),
                Self::OutExpo => out(exp, t),
                Self::InOutExpo => in_out(exp, t),
                Self::InCirc => circle(t),
                Self::OutCirc => out(circle, t),
                Self::InOutCirc => in_out(circle, t),
                Self::InBack(arg) => back(arg, t),
                Self::OutBack(arg) => out(|u| back(arg, u), t),
                Self::InOutBack(arg) => in_out(|u| back(arg, u), t),
                Self::InElastic(arg) => elastic(arg, t),
                Self::OutElastic(arg) => out(|u| elastic(arg, u), t),
                Self::InOutElastic(arg) => in_out(|u| elastic(arg, u), t),
                Self::InBounce(arg) => bounce(arg, t),
                Self::OutBounce(arg) => out(|u| bounce(arg, u), t),
                Self::InOutBounce(arg) => in_out(|u| bounce(arg, u), t),
            };
            eased as f32
        }
    }

    fn out(ease: impl Fn(f64) -> f64, t: f64) -> f64 {
        1.0 - ease(1.0 - t)
    }

    fn in_out(ease: impl Fn(f64) -> f64, t: f64) -> f64 {
        if t < 0.5 {
            ease(t * 2.0) / 2.0
        } else {
            1.0 - ease((1.0 - t) * 2.0) / 2.0
        }
    }

    fn sin(t: f64) -> f64 {
        1.0 - (t * std::f64::consts::FRAC_PI_2).cos()
    }

    fn quad(t: f64) -> f64 {
        t * t
    }

    fn cubic(t: f64) -> f64 {
        t * t * t
    }

    fn quart(t: f64) -> f64 {
        t.powi(4)
    }

    fn quint(t: f64) -> f64 {
        t.powi(5)
    }

    fn exp(t: f64) -> f64 {
        2.0_f64.powf(10.0 * (t - 1.0))
    }

    fn circle(t: f64) -> f64 {
        1.0 - (1.0 - t * t).sqrt()
    }

    fn back(s: Option<f32>, t: f64) -> f64 {
        let p = s.map(|v| f64::from(v) * 1.70158).unwrap_or(1.70158);
        t * t * ((p + 1.0) * t - p)
    }

    fn elastic(bounciness: Option<f32>, t: f64) -> f64 {
        let p = f64::from(bounciness.unwrap_or(1.0)) * std::f64::consts::PI;
        1.0 - (t * std::f64::consts::FRAC_PI_2).cos().powi(3) * (t * p).cos()
    }

    fn bounce(s: Option<f32>, x: f64) -> f64 {
        let k = f64::from(s.unwrap_or(0.5));
        let q = 7.5625 * x * x;
        let w = 30.25 * k * (x - 0.5454545454545454).powi(2) + 1.0 - k;
        let r = 121.0 * k * k * (x - 0.8181818181818182).powi(2) + 1.0 - k * k;
        let t = 484.0 * k * k * k * (x - 0.9545454545454546).powi(2) + 1.0 - k * k * k;
        q.min(w).min(r).min(t)
    }

    fn step(point: f64, steps: Option<i32>) -> f64 {
        let steps = steps.unwrap_or(2).max(2) as usize;
        let step_length = 1.0 / steps as f64;
        let intervals: Vec<f64> = (0..steps).map(|i| i as f64 * step_length).collect();
        intervals[find_interval(point, &intervals)]
    }

    fn find_interval(point: f64, intervals: &[f64]) -> usize {
        if point < intervals[0] {
            return 0;
        }
        if point > intervals[intervals.len() - 1] {
            return intervals.len() - 1;
        }
        let mut left = 0;
        let mut right = intervals.len() - 1;
        while right - left != 1 {
            let mid = left + (right - left) / 2;
            if point >= intervals[mid] {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sine_in_out_midpoint_is_half() {
            let eased = Ease::InOutSine.apply(0.5);
            assert!((eased - 0.5).abs() < 1e-5, "{eased}");
        }

        #[test]
        fn linear_is_identity() {
            assert!((Ease::Linear.apply(0.25) - 0.25).abs() < 1e-6);
        }
    }
}

mod expr {
    // The Molang subset in these animation files: numbers and `+ - * /`.

    pub fn eval(input: &str) -> f32 {
        let mut parser = Parser {
            bytes: input.as_bytes(),
            index: 0,
        };
        parser.expression() as f32
    }

    struct Parser<'a> {
        bytes: &'a [u8],
        index: usize,
    }

    impl Parser<'_> {
        fn expression(&mut self) -> f64 {
            let mut value = self.term();
            loop {
                self.skip();
                if self.eat(b'+') {
                    value += self.term();
                } else if self.eat(b'-') {
                    value -= self.term();
                } else {
                    break;
                }
            }
            value
        }

        fn term(&mut self) -> f64 {
            let mut value = self.unary();
            loop {
                self.skip();
                if self.eat(b'*') {
                    value *= self.unary();
                } else if self.eat(b'/') {
                    let divisor = self.unary();
                    value = if divisor == 0.0 { 0.0 } else { value / divisor };
                } else {
                    break;
                }
            }
            value
        }

        fn unary(&mut self) -> f64 {
            self.skip();
            if self.eat(b'-') {
                return -self.unary();
            }
            if self.eat(b'+') {
                return self.unary();
            }
            if self.eat(b'(') {
                let value = self.expression();
                self.eat(b')');
                return value;
            }
            self.number()
        }

        fn number(&mut self) -> f64 {
            self.skip();
            let start = self.index;
            if self.peek() == Some(b'.') {
                self.index += 1;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.index += 1;
            }
            if self.peek() == Some(b'.') {
                self.index += 1;
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.index += 1;
                }
            }
            if start == self.index {
                return 0.0;
            }
            std::str::from_utf8(&self.bytes[start..self.index])
                .ok()
                .and_then(|text| text.parse().ok())
                .unwrap_or(0.0)
        }

        fn skip(&mut self) {
            while matches!(self.peek(), Some(b' ' | b'\t')) {
                self.index += 1;
            }
        }

        fn eat(&mut self, byte: u8) -> bool {
            if self.peek() == Some(byte) {
                self.index += 1;
                true
            } else {
                false
            }
        }

        fn peek(&self) -> Option<u8> {
            self.bytes.get(self.index).copied()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn molang_add_and_subtract() {
            assert!((eval("12.5-9") - 3.5).abs() < 1e-5);
            assert!((eval("0.595+200") - 200.595).abs() < 1e-4);
        }
    }
}

mod mat {
    // Column vectors, matching `javax.vecmath.Matrix4f` post-multiply.

    #[derive(Clone, Copy, Debug)]
    pub struct Mat4 {
        m: [f32; 16],
    }

    impl Mat4 {
        pub const IDENTITY: Self = Self {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        };

        pub fn translate(x: f32, y: f32, z: f32) -> Self {
            let mut matrix = Self::IDENTITY;
            matrix.m[12] = x;
            matrix.m[13] = y;
            matrix.m[14] = z;
            matrix
        }

        pub fn scale(x: f32, y: f32, z: f32) -> Self {
            let mut matrix = Self::IDENTITY;
            matrix.m[0] = x;
            matrix.m[5] = y;
            matrix.m[10] = z;
            matrix
        }

        pub fn rotate_x(radians: f32) -> Self {
            let (s, c) = radians.sin_cos();
            let mut matrix = Self::IDENTITY;
            matrix.m[5] = c;
            matrix.m[6] = s;
            matrix.m[9] = -s;
            matrix.m[10] = c;
            matrix
        }

        pub fn rotate_y(radians: f32) -> Self {
            let (s, c) = radians.sin_cos();
            let mut matrix = Self::IDENTITY;
            matrix.m[0] = c;
            matrix.m[2] = -s;
            matrix.m[8] = s;
            matrix.m[10] = c;
            matrix
        }

        pub fn rotate_z(radians: f32) -> Self {
            let (s, c) = radians.sin_cos();
            let mut matrix = Self::IDENTITY;
            matrix.m[0] = c;
            matrix.m[1] = s;
            matrix.m[4] = -s;
            matrix.m[5] = c;
            matrix
        }
    }

    impl std::ops::Mul for Mat4 {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            let mut out = [0.0; 16];
            for column in 0..4 {
                for row in 0..4 {
                    let mut value = 0.0;
                    for k in 0..4 {
                        value += self.m[k * 4 + row] * other.m[column * 4 + k];
                    }
                    out[column * 4 + row] = value;
                }
            }
            Self { m: out }
        }
    }

    impl Mat4 {
        pub fn transform(self, point: [f32; 3]) -> [f32; 3] {
            [
                self.m[0] * point[0] + self.m[4] * point[1] + self.m[8] * point[2] + self.m[12],
                self.m[1] * point[0] + self.m[5] * point[1] + self.m[9] * point[2] + self.m[13],
                self.m[2] * point[0] + self.m[6] * point[1] + self.m[10] * point[2] + self.m[14],
            ]
        }

        pub fn transform_dir(self, dir: [f32; 3]) -> [f32; 3] {
            [
                self.m[0] * dir[0] + self.m[4] * dir[1] + self.m[8] * dir[2],
                self.m[1] * dir[0] + self.m[5] * dir[1] + self.m[9] * dir[2],
                self.m[2] * dir[0] + self.m[6] * dir[1] + self.m[10] * dir[2],
            ]
        }
    }
}

mod figure {
    // Bedrock geometry and animation, sampled the way GeckoLib 3 does.
    //
    // Bone matrix: `T(position) * T(pivot) * Rz * Ry * Rx * S * T(-pivot)`.
    // Position X and pivot X are negated, matching the flipped cube corners.
    // Rotation keyframes are degrees; X and Y are negated, then converted to
    // radians, then added to the bind rotation. Cube origin X is
    // `-(origin.x + size.x)`.

    use std::collections::HashMap;
    use std::sync::Arc;

    use serde_json::Value;

    use super::Vec3;
    use super::easing::Ease;
    use super::expr;
    use super::mat::Mat4;

    /// Cubes smaller than this (model pixels, width × height × depth) are dropped.
    /// Horizon draws at most 1024 HUD shapes, and the full mesh is several thousand faces.
    pub const MIN_CUBE_VOLUME: f32 = 0.0;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Play {
        Once,
        Loop,
        Hold,
    }

    #[derive(Clone, Debug)]
    struct Key {
        time: f32,
        value: [f32; 3],
        ease: Ease,
    }

    #[derive(Clone, Debug, Default)]
    struct Channel {
        keys: Vec<Key>,
    }

    #[derive(Clone, Debug, Default)]
    struct BoneAnim {
        position: Channel,
        rotation: Channel,
        scale: Channel,
    }

    #[derive(Clone, Debug)]
    struct Clip {
        length: f32,
        play: Play,
        bones: HashMap<String, BoneAnim>,
        sounds: Vec<(f32, String)>,
    }

    /// Animation clips shared by the dressed and nude geometries.
    #[derive(Clone, Debug)]
    pub struct Motions {
        clips: HashMap<String, Clip>,
        idle: String,
    }

    #[derive(Clone, Debug)]
    struct Face {
        index: [u8; 4],
        color: u32,
        uv: [[f32; 2]; 4],
        /// `EnumFacing` axis, rotated by the cube. GeckoLib does not use the
        /// triangle cross product.
        normal: [f32; 3],
    }

    #[derive(Clone, Debug)]
    struct Cube {
        corners: [[f32; 3]; 8],
        faces: Vec<Face>,
        /// Authored size was ~0 on this axis. GeckoLib flips a back-facing
        /// normal component so a paper-thin card is not lit as a black edge.
        flat: [bool; 3],
    }

    #[derive(Clone, Debug)]
    struct Bone {
        name: String,
        parent: i32,
        pivot: [f32; 3],
        bind_rot: [f32; 3],
        cubes: Vec<Cube>,
        /// Payment props stay hidden. The Steve rig, including the penis, is
        /// drawn only while the clip includes the player.
        hidden: bool,
        player_part: bool,
        head: bool,
        /// Wide-model arms (`steveLeftArm` / `steveRightArm`).
        wide_arm: bool,
        /// Slim-model arms (`alexLeftArm` / `alexRightArm`).
        alex_arm: bool,
    }

    /// One geometry, posed by a shared [`Motions`] clip.
    #[derive(Clone, Debug)]
    pub struct Figure {
        bones: Vec<Bone>,
        motions: Arc<Motions>,
    }

    /// One textured quad in world blocks.
    #[derive(Clone, Copy, Debug)]
    pub struct MeshQuad {
        pub corners: [Vec3; 4],
        pub color: u32,
        pub uv: [[f32; 2]; 4],
        pub normal: [f32; 3],
        /// Paper cards are lit from both sides. Flipping one normal axis, the
        /// way GeckoLib's old GL hack does, turns neighbours into a checkerboard.
        pub two_sided: bool,
        /// UV sits in the 3× skin pasted at pixel (192, 0) of the character sheet.
        pub player_skin: bool,
    }

    struct Rgba {
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    }

    impl Motions {
        pub fn parse(animation_json: &str) -> Result<Self, String> {
            let root: Value =
                serde_json::from_str(animation_json).map_err(|err| err.to_string())?;
            let animations = root
                .get("animations")
                .and_then(Value::as_object)
                .ok_or_else(|| "animation file has no animations object".to_string())?;
            let mut clips = HashMap::new();
            let mut idle = String::new();
            for (name, body) in animations {
                if name.ends_with(".idle") {
                    idle = name.clone();
                }
                clips.insert(name.clone(), parse_clip(body));
            }
            if idle.is_empty() {
                idle = clips.keys().next().cloned().unwrap_or_default();
            }
            Ok(Self { clips, idle })
        }

        /// Sound cues whose time was crossed between the previous tick and this one.
        pub fn cues_crossed(&self, name: &str, prev: f32, now: f32) -> Vec<String> {
            let Some(clip) = self.clips.get(name) else {
                return Vec::new();
            };
            let mut out = Vec::new();
            let mut fire = |from: f32, to: f32| {
                for (time, effect) in &clip.sounds {
                    if *time > from && *time <= to + 0.0001 {
                        out.push(effect.clone());
                    }
                }
            };
            match clip.play {
                Play::Loop if clip.length > 0.0 => {
                    let length = clip.length;
                    let start = prev.rem_euclid(length);
                    let end = now.rem_euclid(length);
                    if end >= start {
                        fire(start, end);
                    } else {
                        fire(start, length);
                        fire(-0.001, end);
                    }
                }
                _ => fire(prev, now),
            }
            out
        }
    }

    impl Figure {
        pub fn cues_crossed(&self, name: &str, prev: f32, now: f32) -> Vec<String> {
            self.motions.cues_crossed(name, prev, now)
        }

        pub fn parse(geo_json: &str, motions: Arc<Motions>, png: &[u8]) -> Result<Self, String> {
            let root: Value = serde_json::from_str(geo_json).map_err(|err| err.to_string())?;
            let geometry = root
                .get("minecraft:geometry")
                .and_then(Value::as_array)
                .and_then(|list| list.first())
                .ok_or_else(|| "geometry file has no minecraft:geometry".to_string())?;
            let description = geometry.get("description").unwrap_or(&Value::Null);
            let texture_width = description
                .get("texture_width")
                .and_then(Value::as_f64)
                .unwrap_or(64.0) as f32;
            let texture_height = description
                .get("texture_height")
                .and_then(Value::as_f64)
                .unwrap_or(64.0) as f32;
            let image = decode_png(png);
            let raw = geometry
                .get("bones")
                .and_then(Value::as_array)
                .ok_or_else(|| "geometry has no bones".to_string())?;
            let mut bones = Vec::with_capacity(raw.len());
            let mut names = HashMap::new();
            for (index, bone) in raw.iter().enumerate() {
                let name = bone
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                names.insert(name.clone(), index);
                let pivot = array3(bone.get("pivot"));
                let bind_rot = rotation_radians(array3(bone.get("rotation")));
                let render = !bone
                    .get("neverRender")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let cubes = if render {
                    parse_cubes(bone.get("cubes"), &image, texture_width, texture_height)
                } else {
                    Vec::new()
                };
                bones.push(Bone {
                    name,
                    parent: -1,
                    pivot,
                    bind_rot,
                    cubes,
                    hidden: false,
                    player_part: false,
                    head: false,
                    wide_arm: false,
                    alex_arm: false,
                });
            }
            for (index, bone) in raw.iter().enumerate() {
                if let Some(parent) = bone.get("parent").and_then(Value::as_str) {
                    if let Some(&parent_index) = names.get(parent) {
                        bones[index].parent = parent_index as i32;
                    }
                }
            }
            hide_reference_rig(&mut bones);
            Ok(Self { bones, motions })
        }

        /// World-space quads. `yaw_deg` is the entity yaw: 0 faces +Z.
        pub fn pose(
            &self,
            clip_name: &str,
            seconds: f32,
            origin: Vec3,
            yaw_deg: f32,
            show_player: bool,
        ) -> Vec<MeshQuad> {
            let clip = self
                .motions
                .clips
                .get(clip_name)
                .or_else(|| self.motions.clips.get(&self.motions.idle));
            let time = clip.map(|clip| local_time(clip, seconds)).unwrap_or(0.0);
            let mut locals = vec![[0.0_f32; 9]; self.bones.len()];
            if let Some(clip) = clip {
                for (index, bone) in self.bones.iter().enumerate() {
                    if let Some(anim) = clip.bones.get(&bone.name) {
                        let position = anim.position.sample(time, [0.0, 0.0, 0.0]);
                        let rotation = anim.rotation.sample(time, [0.0, 0.0, 0.0]);
                        let scale = anim.scale.sample(time, [1.0, 1.0, 1.0]);
                        locals[index] = [
                            position[0],
                            position[1],
                            position[2],
                            rotation[0] + bone.bind_rot[0],
                            rotation[1] + bone.bind_rot[1],
                            rotation[2] + bone.bind_rot[2],
                            scale[0],
                            scale[1],
                            scale[2],
                        ];
                    } else {
                        locals[index] = [
                            0.0,
                            0.0,
                            0.0,
                            bone.bind_rot[0],
                            bone.bind_rot[1],
                            bone.bind_rot[2],
                            1.0,
                            1.0,
                            1.0,
                        ];
                    }
                }
            } else {
                for (index, bone) in self.bones.iter().enumerate() {
                    locals[index] = [
                        0.0,
                        0.0,
                        0.0,
                        bone.bind_rot[0],
                        bone.bind_rot[1],
                        bone.bind_rot[2],
                        1.0,
                        1.0,
                        1.0,
                    ];
                }
            }
            for (index, bone) in self.bones.iter().enumerate() {
                // The sleeve cube is authored about a pixel above the shirt.
                if bone.name == "armR" || bone.name == "armL" {
                    locals[index][1] -= 1.0;
                }
            }
            let matrices = bone_matrices(&self.bones, &locals);
            let (sine, cosine) = yaw_deg.to_radians().sin_cos();
            let mut quads = Vec::new();
            let mut anchor: Option<([f64; 3], f32, f32)> = None;
            for (index, bone) in self.bones.iter().enumerate() {
                if show_player && bone.name == "Head2" {
                    let center = if let Some(cube) = bone.cubes.first() {
                        let mut sum = [0.0_f32; 3];
                        for corner in &cube.corners {
                            sum[0] += corner[0];
                            sum[1] += corner[1];
                            sum[2] += corner[2];
                        }
                        let n = cube.corners.len().max(1) as f32;
                        [sum[0] / n, sum[1] / n, sum[2] / n]
                    } else {
                        [
                            -bone.pivot[0] / 16.0,
                            bone.pivot[1] / 16.0,
                            bone.pivot[2] / 16.0,
                        ]
                    };
                    let eye = world_point(matrices[index].transform(center), sine, cosine, origin);
                    let toward = |z: f32| {
                        world_point(
                            matrices[index].transform([center[0], center[1], center[2] + z]),
                            sine,
                            cosine,
                            origin,
                        )
                    };
                    let aim = |look: [f64; 3]| {
                        let dx = look[0] - eye[0];
                        let dy = look[1] - eye[1];
                        let dz = look[2] - eye[2];
                        let horiz = (dx * dx + dz * dz).sqrt();
                        let yaw = (-dx).atan2(dz).to_degrees() as f32;
                        let pitch = (-dy).atan2(horiz.max(1.0e-4)).to_degrees() as f32;
                        (horiz, yaw, pitch)
                    };
                    // The face is one of the local Z directions. Keep the one
                    // that stays level, so the first look is out of the head
                    // and not into the back of the skull.
                    let front = aim(toward(-1.0));
                    let back = aim(toward(1.0));
                    // Both sides are level on an upright head, so a raw
                    // comparison flips the lens to the other side of the skull.
                    let sign = super::head_z_sign();
                    let use_front = if sign == 0 || (front.0 - back.0).abs() < 0.08 {
                        let choose = if sign == 0 {
                            front.0 + 0.02 >= back.0
                        } else {
                            sign > 0
                        };
                        if sign == 0 {
                            super::set_head_z_sign(if choose { 1 } else { -1 });
                        }
                        choose
                    } else {
                        let choose = front.0 >= back.0;
                        super::set_head_z_sign(if choose { 1 } else { -1 });
                        choose
                    };
                    let (look, yaw, pitch) = if use_front {
                        (toward(-1.0), front.1, front.2)
                    } else {
                        (toward(1.0), back.1, back.2)
                    };
                    // A small step forward along the look, out of the black
                    // geometry in front of the lens.
                    let mut dx = look[0] - eye[0];
                    let mut dy = look[1] - eye[1];
                    let mut dz = look[2] - eye[2];
                    let len = (dx * dx + dy * dy + dz * dz).sqrt();
                    let nudge = 0.3;
                    if len > 1.0e-4 {
                        dx /= len;
                        dy /= len;
                        dz /= len;
                    }
                    let eye = [
                        eye[0] + dx * nudge,
                        eye[1] + dy * nudge,
                        eye[2] + dz * nudge,
                    ];
                    anchor = Some((eye, yaw, pitch));
                }
                // First person hides the Steve head so the lens is not inside
                // the skull. Third person draws it. Torso, arms, legs stay.
                if bone.hidden
                    || (bone.head && !super::show_steve_head())
                    || (bone.player_part && !show_player)
                {
                    continue;
                }
                // GirlModel shows one arm pair: wide Steve or slim Alex.
                let wide = super::wide_arms();
                if (bone.wide_arm && !wide) || (bone.alex_arm && wide) {
                    continue;
                }
                for cube in &bone.cubes {
                    let world: Vec<[f32; 3]> = cube
                        .corners
                        .iter()
                        .map(|corner| matrices[index].transform(*corner))
                        .collect();
                    for face in &cube.faces {
                        let mut corners = [Vec3::ZERO; 4];
                        for (slot, vertex) in face.index.iter().enumerate() {
                            let point = world[*vertex as usize];
                            let x = point[0] * cosine - point[2] * sine;
                            let z = point[0] * sine + point[2] * cosine;
                            corners[slot] = Vec3::new(
                                origin.x + f64::from(x),
                                origin.y + f64::from(point[1]),
                                origin.z + f64::from(z),
                            );
                        }
                        let normal = yaw_dir(
                            flat_cube_normal(
                                cube.flat,
                                unit_dir(matrices[index].transform_dir(face.normal)),
                            ),
                            sine,
                            cosine,
                        );
                        quads.push(MeshQuad {
                            corners,
                            color: face.color,
                            uv: face.uv,
                            normal,
                            two_sided: false,
                            player_skin: pasted_skin(face.uv),
                        });
                    }
                }
            }
            if show_player {
                if let Some(anchor) = anchor {
                    super::set_scene_head(Some(anchor));
                }
            }
            quads
        }
    }

    /// `IGeoRenderer.renderCube`: after the normal matrix, a zero-thickness
    /// face flips any axis that points backwards so the card stays lit.
    fn flat_cube_normal(flat: [bool; 3], mut normal: [f32; 3]) -> [f32; 3] {
        if (flat[1] || flat[2]) && normal[0] < 0.0 {
            normal[0] = -normal[0];
        }
        if (flat[0] || flat[2]) && normal[1] < 0.0 {
            normal[1] = -normal[1];
        }
        if (flat[0] || flat[1]) && normal[2] < 0.0 {
            normal[2] = -normal[2];
        }
        normal
    }

    fn unit_dir(normal: [f32; 3]) -> [f32; 3] {
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length < 1e-8 {
            return [0.0, 1.0, 0.0];
        }
        [normal[0] / length, normal[1] / length, normal[2] / length]
    }

    fn yaw_dir(normal: [f32; 3], sine: f32, cosine: f32) -> [f32; 3] {
        [
            normal[0] * cosine - normal[2] * sine,
            normal[1],
            normal[0] * sine + normal[2] * cosine,
        ]
    }

    fn face_axis(name: &str) -> [f32; 3] {
        match name {
            "west" => [-1.0, 0.0, 0.0],
            "east" => [1.0, 0.0, 0.0],
            "north" => [0.0, 0.0, -1.0],
            "south" => [0.0, 0.0, 1.0],
            "up" => [0.0, 1.0, 0.0],
            _ => [0.0, -1.0, 0.0],
        }
    }

    fn world_point(point: [f32; 3], sine: f32, cosine: f32, origin: Vec3) -> [f64; 3] {
        let x = point[0] * cosine - point[2] * sine;
        let z = point[0] * sine + point[2] * cosine;
        [
            origin.x + f64::from(x),
            origin.y + f64::from(point[1]),
            origin.z + f64::from(z),
        ]
    }

    fn local_time(clip: &Clip, seconds: f32) -> f32 {
        if clip.length <= 0.0 {
            return 0.0;
        }
        match clip.play {
            Play::Loop => seconds.rem_euclid(clip.length),
            Play::Once | Play::Hold => seconds.clamp(0.0, clip.length),
        }
    }

    fn prop_bone(name: &str) -> bool {
        matches!(
            name,
            "bedSlime"
                | "bedSlimeLayer"
                | "gold1"
                | "gold2"
                | "gold3"
                | "diamond1"
                | "diamond2"
                | "diamond3"
                | "emerald1"
                | "emerald2"
                | "emerald3"
        )
    }

    fn hide_reference_rig(bones: &mut [Bone]) {
        for bone in bones.iter_mut() {
            if bone.name == "steve" {
                bone.player_part = true;
            } else if prop_bone(&bone.name) {
                bone.hidden = true;
            }
            if bone.name == "Head2" {
                bone.head = true;
            }
            if bone.name.starts_with("steve") && bone.name.contains("Arm") {
                bone.wide_arm = true;
            }
            if bone.name.starts_with("alex") && bone.name.contains("Arm") {
                bone.alex_arm = true;
            }
        }
        let mut changed = true;
        while changed {
            changed = false;
            for index in 0..bones.len() {
                let parent = bones[index].parent;
                if parent < 0 {
                    continue;
                }
                let hide = bones[parent as usize].hidden;
                let player = bones[parent as usize].player_part;
                let head = bones[parent as usize].head;
                let wide_arm = bones[parent as usize].wide_arm;
                let alex_arm = bones[parent as usize].alex_arm;
                if hide && !bones[index].hidden {
                    bones[index].hidden = true;
                    changed = true;
                }
                if player && !bones[index].player_part {
                    bones[index].player_part = true;
                    changed = true;
                }
                if head && !bones[index].head {
                    bones[index].head = true;
                    changed = true;
                }
                if wide_arm && !bones[index].wide_arm {
                    bones[index].wide_arm = true;
                    changed = true;
                }
                if alex_arm && !bones[index].alex_arm {
                    bones[index].alex_arm = true;
                    changed = true;
                }
            }
        }
    }

    fn bone_matrices(bones: &[Bone], locals: &[[f32; 9]]) -> Vec<Mat4> {
        let mut done = vec![false; bones.len()];
        let mut matrices = vec![Mat4::IDENTITY; bones.len()];
        let mut remaining = bones.len();
        while remaining > 0 {
            let mut progressed = false;
            for index in 0..bones.len() {
                if done[index] {
                    continue;
                }
                let parent = bones[index].parent;
                if parent >= 0 && !done[parent as usize] {
                    continue;
                }
                let parent_matrix = if parent >= 0 {
                    matrices[parent as usize]
                } else {
                    Mat4::IDENTITY
                };
                matrices[index] = parent_matrix * bone_local(&bones[index], locals[index]);
                done[index] = true;
                remaining -= 1;
                progressed = true;
            }
            if !progressed {
                break;
            }
        }
        matrices
    }

    fn bone_local(bone: &Bone, local: [f32; 9]) -> Mat4 {
        // Cube corners already live in the X-flipped model space, so the
        // pivot has to flip with them or a bone spins around the wrong side.
        let pivot = Mat4::translate(
            -bone.pivot[0] / 16.0,
            bone.pivot[1] / 16.0,
            bone.pivot[2] / 16.0,
        );
        let unpivot = Mat4::translate(
            bone.pivot[0] / 16.0,
            -bone.pivot[1] / 16.0,
            -bone.pivot[2] / 16.0,
        );
        Mat4::translate(-local[0] / 16.0, local[1] / 16.0, local[2] / 16.0)
            * pivot
            * Mat4::rotate_z(local[5])
            * Mat4::rotate_y(local[4])
            * Mat4::rotate_x(local[3])
            * Mat4::scale(local[6], local[7], local[8])
            * unpivot
    }

    impl Channel {
        fn sample(&self, time: f32, default_value: [f32; 3]) -> [f32; 3] {
            if self.keys.is_empty() {
                return default_value;
            }
            if time <= self.keys[0].time {
                return self.keys[0].value;
            }
            let last = self.keys.len() - 1;
            if time >= self.keys[last].time {
                return self.keys[last].value;
            }
            let mut index = 1;
            while index < self.keys.len() && self.keys[index].time < time {
                index += 1;
            }
            let previous = &self.keys[index - 1];
            let next = &self.keys[index];
            let span = next.time - previous.time;
            let mut amount = if span > 0.0 {
                (time - previous.time) / span
            } else {
                1.0
            };
            amount = next.ease.apply(amount);
            [
                previous.value[0] + (next.value[0] - previous.value[0]) * amount,
                previous.value[1] + (next.value[1] - previous.value[1]) * amount,
                previous.value[2] + (next.value[2] - previous.value[2]) * amount,
            ]
        }
    }

    fn parse_clip(body: &Value) -> Clip {
        let length = body
            .get("animation_length")
            .and_then(Value::as_f64)
            .unwrap_or(0.0) as f32;
        let play = match body.get("loop") {
            Some(Value::Bool(true)) => Play::Loop,
            Some(Value::String(mode)) if mode == "hold_on_last_frame" => Play::Hold,
            _ => Play::Once,
        };
        let mut bones = HashMap::new();
        if let Some(list) = body.get("bones").and_then(Value::as_object) {
            for (name, channels) in list {
                bones.insert(
                    name.clone(),
                    BoneAnim {
                        position: parse_channel(channels.get("position"), false),
                        rotation: parse_channel(channels.get("rotation"), true),
                        scale: parse_channel(channels.get("scale"), false),
                    },
                );
            }
        }
        let mut sounds = Vec::new();
        if let Some(list) = body.get("sound_effects").and_then(Value::as_object) {
            for (time, frame) in list {
                let Ok(time) = time.parse::<f32>() else {
                    continue;
                };
                let Some(effect) = frame.get("effect").and_then(Value::as_str) else {
                    continue;
                };
                sounds.push((time, effect.to_owned()));
            }
            sounds.sort_by(|left, right| left.0.total_cmp(&right.0));
        }
        Clip {
            length,
            play,
            bones,
            sounds,
        }
    }

    fn parse_channel(value: Option<&Value>, rotation: bool) -> Channel {
        let Some(value) = value else {
            return Channel::default();
        };
        let Some(object) = value.as_object() else {
            return Channel::default();
        };
        let mut keys: Vec<Key> = object
            .iter()
            .filter_map(|(time, frame)| {
                let time = time.parse::<f32>().ok()?;
                let vector = frame.get("vector")?;
                Some(Key {
                    time,
                    value: vector3(vector, rotation),
                    ease: frame
                        .get("easing")
                        .and_then(Value::as_str)
                        .map(|name| Ease::parse(name, easing_arg(frame)))
                        .unwrap_or(Ease::Linear),
                })
            })
            .collect();
        if keys.is_empty() {
            if let Some(vector) = object.get("vector") {
                keys.push(Key {
                    time: 0.0,
                    value: vector3(vector, rotation),
                    ease: Ease::Linear,
                });
            }
        }
        keys.sort_by(|left, right| {
            left.time
                .partial_cmp(&right.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Channel { keys }
    }

    fn easing_arg(frame: &Value) -> Option<f32> {
        frame
            .get("easingArgs")
            .and_then(Value::as_array)
            .and_then(|args| args.first())
            .and_then(Value::as_f64)
            .map(|value| value as f32)
    }

    fn vector3(value: &Value, rotation: bool) -> [f32; 3] {
        let Some(array) = value.as_array() else {
            return [0.0, 0.0, 0.0];
        };
        let mut out = [0.0, 0.0, 0.0];
        for (index, component) in array.iter().take(3).enumerate() {
            let number = match component {
                Value::Number(number) => number.as_f64().unwrap_or(0.0) as f32,
                Value::String(text) => expr::eval(text),
                _ => 0.0,
            };
            out[index] = if rotation {
                signed_radians(index, number)
            } else {
                number
            };
        }
        out
    }

    fn signed_radians(axis: usize, degrees: f32) -> f32 {
        let mut radians = degrees.to_radians();
        if axis == 0 || axis == 1 {
            radians = -radians;
        }
        radians
    }

    fn rotation_radians(degrees: [f32; 3]) -> [f32; 3] {
        [
            signed_radians(0, degrees[0]),
            signed_radians(1, degrees[1]),
            signed_radians(2, degrees[2]),
        ]
    }

    fn array3(value: Option<&Value>) -> [f32; 3] {
        let Some(Value::Array(array)) = value else {
            return [0.0, 0.0, 0.0];
        };
        let mut out = [0.0, 0.0, 0.0];
        for (index, component) in array.iter().take(3).enumerate() {
            out[index] = component.as_f64().unwrap_or(0.0) as f32;
        }
        out
    }

    fn parse_cubes(
        value: Option<&Value>,
        image: &Option<Rgba>,
        texture_width: f32,
        texture_height: f32,
    ) -> Vec<Cube> {
        let Some(list) = value.and_then(Value::as_array) else {
            return Vec::new();
        };
        let mut cubes = Vec::new();
        for cube in list {
            let size = array3(cube.get("size"));
            let volume = size[0].abs() * size[1].abs() * size[2].abs();
            if volume < MIN_CUBE_VOLUME {
                continue;
            }
            let origin = array3(cube.get("origin"));
            let inflate = cube.get("inflate").and_then(Value::as_f64).unwrap_or(0.0) as f32 / 16.0;
            let origin_blocks = [
                -(origin[0] + size[0]) / 16.0,
                origin[1] / 16.0,
                origin[2] / 16.0,
            ];
            let size_blocks = [size[0] / 16.0, size[1] / 16.0, size[2] / 16.0];
            let mut corners = box_corners(origin_blocks, size_blocks, inflate);
            let pivot = array3(cube.get("pivot"));
            let rotation = array3(cube.get("rotation"));
            if rotation != [0.0, 0.0, 0.0] {
                let matrix = cube_matrix(pivot, rotation);
                for corner in &mut corners {
                    *corner = matrix.transform(*corner);
                }
            }
            let mirror = cube.get("mirror").and_then(Value::as_bool).unwrap_or(false);
            let cube_rot = cube_matrix(array3(cube.get("pivot")), array3(cube.get("rotation")));
            let mut faces = parse_faces(
                cube.get("uv"),
                size,
                mirror,
                image,
                texture_width,
                texture_height,
            );
            if faces.is_empty() {
                continue;
            }
            for face in &mut faces {
                face.normal = unit_dir(cube_rot.transform_dir(face.normal));
            }
            let flat = [
                size[0].abs() < 0.05,
                size[1].abs() < 0.05,
                size[2].abs() < 0.05,
            ];
            cubes.push(Cube {
                corners,
                faces,
                flat,
            });
        }
        cubes
    }

    fn box_corners(origin: [f32; 3], size: [f32; 3], inflate: f32) -> [[f32; 3]; 8] {
        let x0 = origin[0] - inflate;
        let y0 = origin[1] - inflate;
        let z0 = origin[2] - inflate;
        let x1 = origin[0] + size[0] + inflate;
        let y1 = origin[1] + size[1] + inflate;
        let z1 = origin[2] + size[2] + inflate;
        [
            [x0, y0, z0],
            [x0, y0, z1],
            [x0, y1, z0],
            [x0, y1, z1],
            [x1, y0, z0],
            [x1, y0, z1],
            [x1, y1, z0],
            [x1, y1, z1],
        ]
    }

    fn cube_matrix(pivot: [f32; 3], rotation: [f32; 3]) -> Mat4 {
        let pivot = [-pivot[0] / 16.0, pivot[1] / 16.0, pivot[2] / 16.0];
        let rotation = rotation_radians(rotation);
        Mat4::translate(pivot[0], pivot[1], pivot[2])
            * Mat4::rotate_z(rotation[2])
            * Mat4::rotate_y(rotation[1])
            * Mat4::rotate_x(rotation[0])
            * Mat4::translate(-pivot[0], -pivot[1], -pivot[2])
    }

    /// `GeoQuad` vertex order. Unmirrored faces put `u2` on vertex 0.
    fn rect_uvs(
        origin: [f32; 2],
        span: [f32; 2],
        width: f32,
        height: f32,
        mirror: bool,
    ) -> [[f32; 2]; 4] {
        let width = width.max(1.0);
        let height = height.max(1.0);
        let u0 = origin[0] / width;
        let v0 = origin[1] / height;
        let u1 = (origin[0] + span[0]) / width;
        let v1 = (origin[1] + span[1]) / height;
        if mirror {
            [[u0, v0], [u1, v0], [u1, v1], [u0, v1]]
        } else {
            [[u1, v0], [u0, v0], [u0, v1], [u1, v1]]
        }
    }

    /// GirlModel pastes the 64×64 player skin at image pixel (192, 0), ×3.
    fn pasted_skin(uv: [[f32; 2]; 4]) -> bool {
        let mut u = 0.0;
        let mut v = 0.0;
        for corner in uv {
            u += corner[0];
            v += corner[1];
        }
        u /= 4.0;
        v /= 4.0;
        (0.04..0.10).contains(&u) && (-0.01..0.055).contains(&v)
    }

    fn parse_faces(
        uv: Option<&Value>,
        size: [f32; 3],
        mirror: bool,
        image: &Option<Rgba>,
        texture_width: f32,
        texture_height: f32,
    ) -> Vec<Face> {
        let mut west = [3, 2, 0, 1];
        let mut east = [6, 7, 5, 4];
        if mirror {
            std::mem::swap(&mut west, &mut east);
        }
        let north = [2, 6, 4, 0];
        let south = [7, 3, 1, 5];
        let up = [3, 7, 6, 2];
        let down = [0, 4, 5, 1];
        let named = [
            ("north", north, 0xFFE7B89A),
            ("east", east, 0xFFD7A888),
            ("south", south, 0xFFC48B6A),
            ("west", west, 0xFFB88870),
            ("up", up, 0xFFF0C8A8),
            ("down", down, 0xFF8A5A48),
        ];
        let mut faces = Vec::new();
        match uv {
            Some(Value::Object(faces_uv)) => {
                for (name, index, fallback) in named {
                    let Some(face) = faces_uv.get(name) else {
                        continue;
                    };
                    let origin = array3_n(face.get("uv"), 2);
                    let span = array3_n(face.get("uv_size"), 2);
                    faces.push(Face {
                        index,
                        color: sample(image, origin, span, texture_width, texture_height, fallback),
                        uv: rect_uvs(origin, span, texture_width, texture_height, mirror),
                        normal: face_axis(name),
                    });
                }
            }
            Some(Value::Array(pair)) if pair.len() >= 2 => {
                let u = pair[0].as_f64().unwrap_or(0.0) as f32;
                let v = pair[1].as_f64().unwrap_or(0.0) as f32;
                let (sx, sy, sz) = (size[0].abs(), size[1].abs(), size[2].abs());
                let boxes = [
                    ("north", north, [u + sz, v + sz], [sx, sy]),
                    ("east", east, [u, v + sz], [sz, sy]),
                    ("south", south, [u + sz + sx + sz, v + sz], [sx, sy]),
                    ("west", west, [u + sz + sx, v + sz], [sz, sy]),
                    ("up", up, [u + sz, v], [sx, sz]),
                    ("down", down, [u + sz + sx, v], [sx, sz]),
                ];
                for (name, index, origin, span) in boxes {
                    faces.push(Face {
                        index,
                        color: sample(
                            image,
                            origin,
                            span,
                            texture_width,
                            texture_height,
                            0xFFE7B89A,
                        ),
                        uv: rect_uvs(origin, span, texture_width, texture_height, mirror),
                        normal: face_axis(name),
                    });
                }
            }
            _ => {}
        }
        faces
    }

    fn array3_n(value: Option<&Value>, count: usize) -> [f32; 2] {
        let Some(Value::Array(array)) = value else {
            return [0.0, 0.0];
        };
        let mut out = [0.0, 0.0];
        for (index, component) in array.iter().take(count).enumerate() {
            out[index] = component.as_f64().unwrap_or(0.0) as f32;
        }
        out
    }

    fn sample(
        image: &Option<Rgba>,
        origin: [f32; 2],
        span: [f32; 2],
        texture_width: f32,
        texture_height: f32,
        fallback: u32,
    ) -> u32 {
        let Some(image) = image else {
            return fallback;
        };
        if texture_width <= 0.0 || texture_height <= 0.0 || image.width == 0 || image.height == 0 {
            return fallback;
        }
        let scale_u = image.width as f32 / texture_width;
        let scale_v = image.height as f32 / texture_height;
        let u0 = origin[0] * scale_u;
        let v0 = origin[1] * scale_v;
        let u1 = (origin[0] + span[0]) * scale_u;
        let v1 = (origin[1] + span[1]) * scale_v;
        let left = u0.min(u1).floor() as i32;
        let right = u0.max(u1).ceil() as i32;
        let top = v0.min(v1).floor() as i32;
        let bottom = v0.max(v1).ceil() as i32;
        let mut red = 0u64;
        let mut green = 0u64;
        let mut blue = 0u64;
        let mut count = 0u64;
        let step_x = ((right - left) / 4).max(1);
        let step_y = ((bottom - top) / 4).max(1);
        let mut y = top;
        while y < bottom.max(top + 1) {
            let mut x = left;
            while x < right.max(left + 1) {
                let px = x.clamp(0, image.width as i32 - 1) as u32;
                let py = y.clamp(0, image.height as i32 - 1) as u32;
                let offset = ((py * image.width + px) * 4) as usize;
                red += image.pixels[offset] as u64;
                green += image.pixels[offset + 1] as u64;
                blue += image.pixels[offset + 2] as u64;
                count += 1;
                x += step_x;
            }
            y += step_y;
        }
        if count == 0 {
            return fallback;
        }
        0xFF00_0000
            | (((red / count) as u32) << 16)
            | (((green / count) as u32) << 8)
            | ((blue / count) as u32)
    }

    fn decode_png(bytes: &[u8]) -> Option<Rgba> {
        if bytes.is_empty() {
            return None;
        }
        let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
        let mut reader = decoder.read_info().ok()?;
        let mut buffer = vec![0; reader.output_buffer_size().unwrap_or(0)];
        let info = reader.next_frame(&mut buffer).ok()?;
        let pixels = match info.color_type {
            png::ColorType::Rgba => buffer[..info.buffer_size()].to_vec(),
            png::ColorType::Rgb => {
                let rgb = &buffer[..info.buffer_size()];
                let mut rgba = Vec::with_capacity(rgb.len() / 3 * 4);
                for chunk in rgb.chunks_exact(3) {
                    rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
                }
                rgba
            }
            _ => return None,
        };
        Some(Rgba {
            width: info.width,
            height: info.height,
            pixels,
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const GEO: &str = r#"{
            "format_version": "1.12.0",
            "minecraft:geometry": [{
                "description": {"identifier": "geometry.test", "texture_width": 16, "texture_height": 16},
                "bones": [{
                    "name": "body",
                    "pivot": [0, 0, 0],
                    "cubes": [{
                        "origin": [-8, 0, -8],
                        "size": [16, 16, 16],
                        "uv": {"north": {"uv": [0, 0], "uv_size": [16, 16]}}
                    }]
                }]
            }]
        }"#;

        const ANIM: &str = r#"{
            "format_version": "1.8.0",
            "animations": {
                "animation.test.idle": {
                    "loop": false,
                    "animation_length": 1.0,
                    "bones": {
                        "body": {
                            "rotation": {
                                "0.0": {"vector": [0, 0, 0]},
                                "1.0": {"vector": [0, 0, 90], "easing": "linear"}
                            }
                        }
                    }
                }
            }
        }"#;

        fn figure() -> Figure {
            let motions = Arc::new(Motions::parse(ANIM).unwrap());
            Figure::parse(GEO, motions, &[]).unwrap()
        }

        #[test]
        fn bind_pose_uses_the_flipped_cube_origin() {
            let quads = figure().pose("animation.test.idle", 0.0, Vec3::ZERO, 0.0, false);
            assert_eq!(quads.len(), 1);
            let corners = quads[0].corners;
            assert!(corners.iter().any(|corner| {
                (corner.x + 0.5).abs() < 1e-4
                    && corner.y.abs() < 1e-4
                    && (corner.z + 0.5).abs() < 1e-4
            }));
        }

        #[test]
        fn rotation_z_is_radians_and_not_negated() {
            let quads = figure().pose("animation.test.idle", 1.0, Vec3::ZERO, 0.0, false);
            let moved = quads[0].corners.iter().any(|corner| {
                corner.x.abs() < 0.05
                    && (corner.y + 0.5).abs() < 0.05
                    && (corner.z + 0.5).abs() < 0.05
            });
            assert!(moved, "{:?}", quads[0].corners);
        }

        #[test]
        fn real_jenny_idle_has_a_body() {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../decomp/extracted/assets/sexmod");
            let geo_path = root.join("geo/jenny/jennydressed.geo.json");
            if !geo_path.is_file() {
                return;
            }
            let geo = std::fs::read_to_string(geo_path).unwrap();
            let anim = std::fs::read_to_string(root.join("animations/jenny/jenny.animation.json"))
                .unwrap();
            let png = std::fs::read(root.join("textures/entity/jenny/jenny.png")).unwrap();
            let motions = Arc::new(Motions::parse(&anim).unwrap());
            let figure = Figure::parse(&geo, motions, &png).unwrap();
            let quads = figure.pose("animation.jenny.idle", 0.2, Vec3::ZERO, 180.0, false);
            assert!(quads.len() > 200, "{}", quads.len());
            let on_body = quads.iter().all(|quad| {
                quad.corners.iter().all(|corner| {
                    corner.y > -0.2
                        && corner.y < 2.2
                        && corner.x.abs() < 0.8
                        && corner.z.abs() < 0.8
                })
            });
            assert!(on_body, "the Steve reference rig is still in the pose");
            assert!(quads.iter().any(|quad| quad.color != 0xFFE7B89A));
            assert!(
                quads
                    .iter()
                    .any(|quad| quad.uv.iter().any(|uv| uv[0] != 0.0 || uv[1] != 0.0))
            );
        }

        #[test]
        fn ellie_and_slime_pose() {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../decomp/extracted/assets/sexmod");
            if !root.join("geo/ellie/dressed.geo.json").is_file() {
                return;
            }
            for (geo, anim, png, clip) in [
                (
                    "geo/ellie/dressed.geo.json",
                    "animations/ellie/ellie.animation.json",
                    "textures/entity/ellie/ellie.png",
                    "animation.ellie.idle",
                ),
                (
                    "geo/slime/nude.geo.json",
                    "animations/slime/slime.animation.json",
                    "textures/entity/slime/slime.png",
                    "animation.slime.idle",
                ),
            ] {
                let motions = Arc::new(
                    Motions::parse(&std::fs::read_to_string(root.join(anim)).unwrap()).unwrap(),
                );
                let figure = Figure::parse(
                    &std::fs::read_to_string(root.join(geo)).unwrap(),
                    motions,
                    &std::fs::read(root.join(png)).unwrap(),
                )
                .unwrap();
                let quads = figure.pose(clip, 0.4, Vec3::ZERO, 0.0, false);
                assert!(quads.len() > 30, "{clip} {}", quads.len());
            }
        }
    }
}

// World-space actors. The geometry modules above sample GeckoLib; this file
// keeps who is standing in the world and turns a pose into draw quads.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI8;
use std::sync::atomic::Ordering;

use figure::{Figure, MeshQuad, Motions};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Follow {
    None,
    Doggy,
    Cowgirl,
}

struct World {
    actors: Vec<Actor>,
    library: Library,
    menu: Option<Kind>,
    follow: Follow,
    ui: bool,
    cum: f64,
    held_sneak: bool,
    held_jump: bool,
    view: View,
    rng: u32,
    wander_left: f32,
    wander_on: bool,
    wander_x: f64,
    wander_z: f64,
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
            menu: None,
            follow: Follow::None,
            ui: false,
            cum: 0.0,
            held_sneak: false,
            held_jump: false,
            view: View::new(),
            rng: 1,
            wander_left: 2.0,
            wander_on: false,
            wander_x: 0.0,
            wander_z: 1.0,
        }
    }
}

struct View {
    origin: Option<[f64; 3]>,
    ret: Option<[f64; 3]>,
    ret_yaw: f32,
    ret_pitch: f32,
    yaw: f32,
    on: bool,
    release: bool,
    pos: [f64; 3],
    look_yaw: f32,
    pitch: f32,
    /// Feet where the player was when a sex clip started. Held still so the
    /// posed Steve head cannot drag the camera.
    held: Option<[f64; 3]>,
    /// The head's facing was applied once. After that the mouse owns the look.
    aimed: bool,
    /// Eye position held still. Later samples inside this distance are ignored.
    steady: Option<[f64; 3]>,
}

impl View {
    const fn new() -> Self {
        Self {
            origin: None,
            ret: None,
            ret_yaw: 0.0,
            ret_pitch: 0.0,
            yaw: 0.0,
            on: false,
            release: false,
            pos: [0.0, 0.0, 0.0],
            look_yaw: 0.0,
            pitch: 0.0,
            held: None,
            aimed: false,
            steady: None,
        }
    }

    fn look(&mut self, x: f64, y: f64, z: f64, yaw_add: f32, pitch: f32) {
        let Some(origin) = self.origin else {
            return;
        };
        let yaw = f64::from(self.yaw.to_radians());
        let side = f64::from((self.yaw + 90.0).to_radians());
        self.pos = [
            origin[0] - side.sin() * x - yaw.sin() * z,
            origin[1] + y,
            origin[2] + side.cos() * x + yaw.cos() * z,
        ];
        self.look_yaw = self.yaw + yaw_add;
        self.pitch = pitch;
        self.on = true;
        self.release = false;
    }

    fn release_player(&mut self) {
        let Some(pos) = self.ret else {
            self.on = false;
            return;
        };
        self.pos = pos;
        self.look_yaw = self.ret_yaw;
        self.pitch = self.ret_pitch;
        self.on = true;
        self.release = true;
    }
}

// The integrated server runs commands on one thread and draws on another.
// A thread-local list made the spawn succeed in chat and render nothing.
static WORLD: Mutex<World> = Mutex::new(World::new());
static SCENE_HEAD: Mutex<Option<([f64; 3], f32, f32)>> = Mutex::new(None);
/// Last skin we classified, whether it is the wide Steve model, and whether
/// the file was already on disk. A missing download is retried.
static SKIN_ARMS: Mutex<(String, bool, bool)> = Mutex::new((String::new(), true, false));
static WIDE_ARMS: AtomicBool = AtomicBool::new(true);
/// First person hides the Steve skull. Third person draws it.
static SHOW_HEAD: AtomicBool = AtomicBool::new(true);
/// 0 = not chosen, 1 = local -Z, -1 = local +Z.
static HEAD_Z: AtomicI8 = AtomicI8::new(0);

pub fn set_show_steve_head(show: bool) {
    SHOW_HEAD.store(show, Ordering::Relaxed);
}

fn show_steve_head() -> bool {
    SHOW_HEAD.load(Ordering::Relaxed)
}

fn head_z_sign() -> i8 {
    HEAD_Z.load(Ordering::Relaxed)
}

fn set_head_z_sign(sign: i8) {
    HEAD_Z.store(sign, Ordering::Relaxed);
}

pub fn in_player_body() -> bool {
    with_world(|world| {
        world
            .actors
            .iter()
            .any(|actor| clip_has_player(&actor.clip))
    })
}

pub fn skin_arms() -> (String, bool, bool) {
    SKIN_ARMS.lock().expect("sexmod skin").clone()
}

pub fn store_skin_arms(name: &str, wide: bool, ready: bool) {
    WIDE_ARMS.store(wide, Ordering::Relaxed);
    *SKIN_ARMS.lock().expect("sexmod skin") = (name.to_owned(), wide, ready);
}

pub fn wide_arms() -> bool {
    WIDE_ARMS.load(Ordering::Relaxed)
}

fn set_scene_head(value: Option<([f64; 3], f32, f32)>) {
    *SCENE_HEAD.lock().expect("sexmod head") = value;
}

pub fn scene_head() -> Option<([f64; 3], f32, f32)> {
    *SCENE_HEAD.lock().expect("sexmod head")
}

fn with_world<T>(body: impl FnOnce(&mut World) -> T) -> T {
    let mut world = WORLD.lock().expect("sexmod world");
    body(&mut world)
}

pub fn help() -> &'static str {
    "sexmod jenny|ellie|slime spawns her in the world. She follows a held emerald, diamond, gold ingot, ender pearl or dye, and wanders. play <clip>, nude, dress, pay, stop."
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
        let dressed = actor.model != 0;
        let start = if dressed && (suffix == "doggystart" || suffix == "cowgirlstart") {
            world.follow = if suffix == "doggystart" {
                Follow::Doggy
            } else {
                Follow::Cowgirl
            };
            "strip"
        } else {
            world.follow = Follow::None;
            suffix
        };
        world.ui = false;
        world.cum = 0.0;
        world.view = View::new();
        set_head_z_sign(0);
        actor.clip = format!("animation.{}.{start}", actor.kind.id());
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

pub fn tick(
    feet: Option<[f64; 3]>,
    sneak: bool,
    jump: bool,
    look: Option<(f32, f32)>,
    tempted: bool,
) {
    with_world(|world| {
        if world.view.origin.is_none()
            && let Some(feet) = feet
            && world.actors.iter().any(|actor| scene_clip(&actor.clip))
        {
            world.view.origin = Some(feet);
            world.view.ret = Some(feet);
            if let Some((yaw, pitch)) = look {
                world.view.ret_yaw = yaw;
                world.view.ret_pitch = pitch;
            }
            if let Some(actor) = world.actors.last() {
                world.view.yaw = actor.yaw + 180.0;
            }
        }
        if world.view.held.is_none()
            && let Some(feet) = feet
            && world
                .actors
                .iter()
                .any(|actor| clip_has_player(&actor.clip))
        {
            world.view.held = Some(feet);
        }
        let kinds: Vec<Kind> = world.actors.iter().map(|actor| actor.kind).collect();
        for kind in kinds {
            let _ = world.library.pair(kind);
        }
        let mut rng = world.rng;
        let mut wander_left = world.wander_left;
        let mut wander_on = world.wander_on;
        let mut wander_x = world.wander_x;
        let mut wander_z = world.wander_z;
        let library = &mut world.library;
        let actors = &mut world.actors;
        let follow = &mut world.follow;
        let ui = &mut world.ui;
        let cum = &mut world.cum;
        let view = &mut world.view;
        for actor in actors.iter_mut() {
            let previous = actor.seconds as f32;
            actor.seconds += 0.05;
            let cues = library
                .pair(actor.kind)
                .map(|pair| {
                    pair.dressed
                        .cues_crossed(&actor.clip, previous, actor.seconds as f32)
                })
                .unwrap_or_default();
            for cue in cues {
                apply_cue(actor, &cue, follow, ui, cum, view);
            }
            let started_sneak = sneak && !actor.sneak;
            actor.sneak = sneak;
            actor.jump = jump;
            if jump && *ui && *cum >= 1.0 && !actor.clip.contains("cum") {
                let next = if actor.clip.contains("cowgirl") {
                    "cowgirlcum"
                } else if actor.clip.contains("doggy") {
                    "doggycum"
                } else if actor.clip.contains("blowjob") {
                    "blowjobcum"
                } else {
                    ""
                };
                if !next.is_empty() {
                    let id = actor.kind.id();
                    actor.clip = format!("animation.{id}.{next}");
                    actor.seconds = 0.0;
                    if next == "blowjobcum" {
                        view.look(0.0, 0.0, 0.0, 0.0, 70.0);
                    }
                }
            } else if started_sneak && actor.clip.contains("suck") {
                let id = actor.kind.id();
                actor.clip = format!("animation.{id}.blowjobthrust");
                actor.seconds = 0.0;
            } else if started_sneak
                && (actor.clip.contains("slow") || actor.clip.ends_with("slow2"))
            {
                let id = actor.kind.id();
                let next = if actor.clip.contains("cowgirl") {
                    "cowgirlfast"
                } else {
                    "doggyfast"
                };
                actor.clip = format!("animation.{id}.{next}");
                actor.seconds = 0.0;
            }
            if let Some(feet) = feet {
                if scene_clip(&actor.clip) {
                    continue;
                }
                let dx = feet[0] - actor.pos[0];
                let dz = feet[2] - actor.pos[2];
                let dist2 = dx * dx + dz * dz;
                let mut stepping = false;
                // EntityAITempt: speed 0.4 times the 0.5 movement attribute,
                // while the held item is within 10 blocks.
                if tempted && dist2 > 1.8 * 1.8 && dist2 < 100.0 {
                    let dist = dist2.sqrt();
                    let step = 0.2_f64.min(dist - 1.6);
                    actor.pos[0] += dx / dist * step;
                    actor.pos[2] += dz / dist * step;
                    actor.yaw = (dz.atan2(dx).to_degrees() as f32) + 90.0;
                    stepping = true;
                } else if !tempted {
                    if wander_left <= 0.0 {
                        wander_on = unit(&mut rng) < 0.3;
                        let angle = f64::from(unit(&mut rng)) * std::f64::consts::TAU;
                        wander_x = angle.sin();
                        wander_z = angle.cos();
                        wander_left = if wander_on {
                            1.5
                        } else {
                            3.0 + unit(&mut rng) * 4.0
                        };
                    }
                    wander_left -= 0.05;
                    if wander_on {
                        actor.pos[0] += wander_x * 0.175;
                        actor.pos[2] += wander_z * 0.175;
                        actor.yaw = (wander_z.atan2(wander_x).to_degrees() as f32) + 90.0;
                        stepping = true;
                    }
                }
                if !stepping && dist2 > 0.04 {
                    actor.yaw = (dz.atan2(dx).to_degrees() as f32) + 90.0;
                }
                if actor.kind != Kind::Slime {
                    let id = actor.kind.id();
                    if stepping && !actor.clip.ends_with(".walk") {
                        actor.clip = format!("animation.{id}.walk");
                        actor.seconds = 0.0;
                    } else if !stepping && actor.clip.ends_with(".walk") {
                        actor.clip = format!("animation.{id}.idle");
                        actor.seconds = 0.0;
                    }
                }
            }
        }
        world.rng = rng;
        world.wander_left = wander_left;
        world.wander_on = wander_on;
        world.wander_x = wander_x;
        world.wander_z = wander_z;
    });
}

fn unit(rng: &mut u32) -> f32 {
    let mut x = *rng;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    if x == 0 {
        x = 1;
    }
    *rng = x;
    (x as f32) / (u32::MAX as f32)
}

pub fn temptation(item: &str) -> bool {
    matches!(
        item,
        "minecraft:emerald"
            | "minecraft:diamond"
            | "minecraft:gold_ingot"
            | "minecraft:ender_pearl"
    ) || item.ends_with("_dye")
        || matches!(
            item,
            "minecraft:ink_sac"
                | "minecraft:cocoa_beans"
                | "minecraft:lapis_lazuli"
                | "minecraft:bone_meal"
        )
}

fn scene_clip(clip: &str) -> bool {
    !clip.ends_with(".idle") && !clip.ends_with(".null") && !clip.ends_with(".walk")
}

fn clip_has_player(clip: &str) -> bool {
    clip.contains("blowjob")
        || clip.contains("doggystart")
        || clip.contains("doggyslow")
        || clip.contains("doggyfast")
        || clip.contains("doggycum")
        || clip.contains("cowgirl")
}

fn apply_cue(
    actor: &mut Actor,
    cue: &str,
    follow: &mut Follow,
    ui: &mut bool,
    cum: &mut f64,
    view: &mut View,
) {
    let id = actor.kind.id();
    match cue {
        "becomeNude" => actor.model = 0,
        "stripDone" => match *follow {
            Follow::Doggy => {
                actor.model = 0;
                actor.clip = format!("animation.{id}.doggystart");
                actor.seconds = 0.0;
                *follow = Follow::None;
                view.look(0.0, 0.0, 0.4, 0.0, 60.0);
            }
            Follow::Cowgirl => {
                actor.model = 0;
                actor.clip = format!("animation.{id}.cowgirlstart");
                actor.seconds = 0.0;
                *follow = Follow::None;
            }
            Follow::None => finish_scene(actor, ui, cum, view),
        },
        "sexUiOn" | "bjiDone" => {
            *ui = true;
            if cue == "bjiDone" {
                actor.clip = format!("animation.{id}.blowjobsuck");
                actor.seconds = 0.0;
            }
        }
        "doggystartDone" => {
            *ui = true;
            actor.clip = format!("animation.{id}.doggyslow");
            actor.seconds = 0.0;
        }
        "cowgirlStartDone" => {
            *ui = true;
            actor.clip = format!("animation.{id}.cowgirlslow2");
            actor.seconds = 0.0;
        }
        "bjiMSG1" | "doggystartMSG4" | "cowgirlStartMSG1" => {
            *cum = 0.0;
            if cue == "bjiMSG1" {
                view.yaw = actor.yaw + 180.0;
            }
        }
        "bjiMSG10" => view.look(-0.4, -0.8, -0.2, 60.0, -3.0),
        "bjiMSG11" | "bjiMSG12" | "doggyfastMSG1" | "cowgirlStartMSG2" => {
            *cum = (*cum + 0.02).min(1.0);
        }
        "bjtMSG1" | "cowgirlfastMSG1" => *cum = (*cum + 0.04).min(1.0),
        "doggyslowMSG1" => *cum = (*cum + 0.00666).min(1.0),
        "bjcMSG2" | "cowgirlcumMSG4" => *ui = false,
        "bjcDone" | "doggyCumDone" | "cowgirlcumDone" => finish_scene(actor, ui, cum, view),
        _ => {}
    }
}

fn finish_scene(actor: &mut Actor, ui: &mut bool, cum: &mut f64, view: &mut View) {
    let id = actor.kind.id();
    actor.clip = format!("animation.{id}.idle");
    actor.seconds = 0.0;
    *ui = false;
    *cum = 0.0;
    view.release_player();
}

pub fn controls_locked() -> bool {
    with_world(|world| {
        world.menu.is_some() || world.actors.iter().any(|actor| scene_clip(&actor.clip))
    })
}

pub fn note_controls(sneak: bool, jump: bool) {
    with_world(|world| {
        world.held_sneak = sneak;
        world.held_jump = jump;
    });
}

pub fn held_controls() -> (bool, bool) {
    with_world(|world| (world.held_sneak, world.held_jump))
}

pub struct Place {
    pub pos: [f64; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub release: bool,
    pub lock_look: bool,
    pub lock_pitch: bool,
    /// `pos` is the eye, not the feet. The camera sits an eye-height above feet.
    pub at_eye: bool,
}

pub fn player_place() -> Option<Place> {
    let anchor = scene_head();
    with_world(|world| {
        if world.view.release && world.view.on {
            world.view.aimed = false;
            return Some(Place {
                pos: world.view.pos,
                yaw: world.view.look_yaw,
                pitch: world.view.pitch,
                release: true,
                lock_look: true,
                lock_pitch: true,
                at_eye: false,
            });
        }
        let has_player = world
            .actors
            .iter()
            .any(|actor| clip_has_player(&actor.clip));
        let Some((eye, yaw, pitch)) = anchor else {
            if !has_player {
                world.view.aimed = false;
            }
            return None;
        };
        if !has_player {
            world.view.aimed = false;
            world.view.steady = None;
            set_head_z_sign(0);
            return None;
        }
        let eye = hold_eye(&mut world.view, eye);
        let first = !world.view.aimed;
        world.view.aimed = true;
        Some(Place {
            pos: eye,
            yaw,
            pitch,
            release: false,
            lock_look: first,
            lock_pitch: first,
            at_eye: true,
        })
    })
}

/// Keep the eye that was already good. Centimetre bone noise and a side flip
/// must not move the lens; a real pose change eases in.
fn hold_eye(view: &mut View, eye: [f64; 3]) -> [f64; 3] {
    let Some(prev) = view.steady else {
        view.steady = Some(eye);
        return eye;
    };
    let dx = eye[0] - prev[0];
    let dy = eye[1] - prev[1];
    let dz = eye[2] - prev[2];
    if dx * dx + dy * dy + dz * dz < 0.12 * 0.12 {
        return prev;
    }
    let next = [
        prev[0] + dx * 0.15,
        prev[1] + dy * 0.15,
        prev[2] + dz * 0.15,
    ];
    view.steady = Some(next);
    next
}

pub fn clear_place() {
    with_world(|world| world.view = View::new());
}

pub struct Meter {
    pub shown: bool,
    pub fill: f32,
    pub thrusting: bool,
}

pub fn meter() -> Meter {
    with_world(|world| Meter {
        shown: world.ui,
        fill: world.cum as f32,
        thrusting: world.held_sneak,
    })
}

pub struct DrawQuad {
    pub sprite: String,
    pub corners: [[f64; 3]; 4],
    pub uv: [[f32; 2]; 4],
    pub normal: [f32; 3],
    pub anchor: [f64; 3],
    pub two_sided: bool,
    pub player_skin: bool,
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
    let (quads, playing) = with_world(|world| {
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
        let playing = snapshots.iter().any(|actor| clip_has_player(&actor.clip));
        let mut out = Vec::new();
        let mut error = None;
        for actor in snapshots {
            let dx = actor.pos[0] - camera[0];
            let dy = actor.pos[1] - camera[1];
            let dz = actor.pos[2] - camera[2];
            // A scene still has to be posed when she is far, or the camera
            // never learns where her head is and the player is left behind.
            if !scene_clip(&actor.clip) && dx * dx + dy * dy + dz * dz > 96.0 * 96.0 {
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
                clip_has_player(&actor.clip),
            );
            push_quads(&mut out, &actor, posed);
        }
        if let Some(err) = error {
            world.library.error = Some(err);
        }
        (out, playing)
    });
    if !playing {
        set_scene_head(None);
        set_head_z_sign(0);
    }
    quads
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
            two_sided: quad.two_sided,
            player_skin: quad.player_skin,
        });
    }
}

pub struct LookHit {
    pub name: &'static str,
    pub distance: f64,
}

/// Width and height of the standing box, in blocks.
fn body_size(kind: Kind) -> (f64, f64) {
    match kind {
        Kind::Slime => (0.9, 1.85),
        Kind::Jenny | Kind::Ellie => (1.0, 1.95),
    }
}

pub struct ActorBox {
    pub feet: [f64; 3],
    pub width: f64,
    pub height: f64,
}

/// Standing boxes, for the debug hitbox renderer.
pub fn actor_boxes() -> Vec<ActorBox> {
    with_world(|world| {
        world
            .actors
            .iter()
            .map(|actor| {
                let (width, height) = body_size(actor.kind);
                ActorBox {
                    feet: actor.pos,
                    width,
                    height,
                }
            })
            .collect()
    })
}

/// The actor box under a view ray, if it is within `reach`.
pub fn look(eye: [f64; 3], dir: [f64; 3], reach: f64) -> Option<LookHit> {
    with_world(|world| {
        let mut best: Option<LookHit> = None;
        for actor in &world.actors {
            let (width, height) = body_size(actor.kind);
            let half = width * 0.5;
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

struct Scene {
    label: &'static str,
    price: &'static str,
    action: &'static str,
    item: &'static str,
    cost: u32,
    clip: &'static str,
}

fn scenes(kind: Kind) -> &'static [Scene] {
    match kind {
        Kind::Jenny => &[
            Scene {
                label: "Blowjob",
                price: "3x emerald",
                action: "blowjob",
                item: "minecraft:emerald",
                cost: 3,
                clip: "blowjobintro",
            },
            Scene {
                label: "Doggy",
                price: "2x diamond",
                action: "doggy",
                item: "minecraft:diamond",
                cost: 2,
                clip: "doggystart",
            },
            Scene {
                label: "Strip",
                price: "1x gold",
                action: "strip",
                item: "minecraft:gold_ingot",
                cost: 1,
                clip: "strip",
            },
        ],
        Kind::Ellie => &[
            Scene {
                label: "Strip",
                price: "",
                action: "strip",
                item: "",
                cost: 0,
                clip: "strip",
            },
            Scene {
                label: "Cowgirl",
                price: "",
                action: "cowgirl",
                item: "",
                cost: 0,
                clip: "cowgirlstart",
            },
        ],
        Kind::Slime => &[],
    }
}

pub struct MenuButton {
    pub label: &'static str,
    pub price: &'static str,
    pub command: &'static str,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub fn menu_open() -> bool {
    with_world(|world| world.menu.is_some())
}

static RELEASING_CURSOR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The next inventory-click reset belongs to us letting the cursor go,
/// not to the player pressing a button.
pub fn begin_release() {
    RELEASING_CURSOR.store(true, std::sync::atomic::Ordering::Relaxed);
}

pub fn take_release() -> bool {
    RELEASING_CURSOR.swap(false, std::sync::atomic::Ordering::Relaxed)
}

pub fn close_menu() {
    with_world(|world| world.menu = None);
}

/// Select her and open the original button list. Slime has no menu.
pub fn open_menu(name: &str) -> bool {
    with_world(|world| {
        let Some(kind) = Kind::parse(name) else {
            return false;
        };
        let Some(index) = world.actors.iter().position(|actor| actor.kind == kind) else {
            return false;
        };
        let actor = world.actors.remove(index);
        world.actors.push(actor);
        if scenes(kind).is_empty() {
            world.menu = None;
            return false;
        }
        world.menu = Some(kind);
        true
    })
}

pub fn menu_buttons(width: f32, height: f32) -> Vec<MenuButton> {
    with_world(|world| {
        let Some(kind) = world.menu else {
            return Vec::new();
        };
        let list = scenes(kind);
        let mut buttons = Vec::with_capacity(list.len());
        for (index, scene) in list.iter().enumerate() {
            let step = index as f32;
            buttons.push(MenuButton {
                label: scene.label,
                price: scene.price,
                command: scene.action,
                x: width - 120.0,
                y: height - 150.0 - step * 30.0,
                w: 100.0,
                h: 20.0,
            });
        }
        buttons
    })
}

pub fn menu_command_at(x: f32, y: f32, width: f32, height: f32) -> Option<&'static str> {
    menu_buttons(width, height).into_iter().find_map(|button| {
        if x >= button.x && x <= button.x + button.w && y >= button.y && y <= button.y + button.h {
            Some(button.command)
        } else {
            None
        }
    })
}

pub struct SceneBill {
    pub item: &'static str,
    pub cost: u32,
    pub clip: &'static str,
    pub who: &'static str,
}

/// Price and clip for a menu action on the actor who was just clicked.
pub fn prepare_scene(action: &str) -> Result<SceneBill, String> {
    with_world(|world| {
        let Some(actor) = world.actors.last() else {
            return Err("Nobody is here.".to_owned());
        };
        let who = match actor.kind {
            Kind::Jenny => "Jenny",
            Kind::Ellie => "Ellie",
            Kind::Slime => "Slime",
        };
        let Some(scene) = scenes(actor.kind)
            .iter()
            .find(|scene| scene.action == action)
        else {
            return Err(format!("{who} does not do that."));
        };
        Ok(SceneBill {
            item: scene.item,
            cost: scene.cost,
            clip: scene.clip,
            who,
        })
    })
}

/// Make the looked-at actor the current one and describe the click.
pub fn activate(name: &str) -> String {
    with_world(|world| {
        let Some(index) = world
            .actors
            .iter()
            .position(|actor| actor.kind.id() == name)
        else {
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

/// Character-sheet UV in the pasted skin window, as a 0–1 coordinate on the
/// 64×64 player skin. `GirlModel` stamps that skin at pixel 192 with a ×3 repeat.
pub fn player_skin_uv(texture_uv: [f32; 2]) -> [f32; 2] {
    let scale = 64.0 / 3.0;
    [
        (texture_uv[0] * scale - 1.0).clamp(0.0, 1.0),
        (texture_uv[1] * scale).clamp(0.0, 1.0),
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
    character_textures_in(&asset_root(), None)
}

pub fn character_textures_in(
    root: &std::path::Path,
    skin_png: Option<&[u8]>,
) -> Vec<(String, Vec<u8>)> {
    let skin = skin_png.and_then(|bytes| decode_rgba(bytes).ok());
    let mut images = Vec::new();
    for (name, file) in [
        ("font/sexmod_jenny", "textures/entity/jenny/jenny.png"),
        ("font/sexmod_ellie", "textures/entity/ellie/ellie.png"),
        ("font/sexmod_slime", "textures/entity/slime/slime.png"),
    ] {
        let Ok(bytes) = std::fs::read(root.join(file)) else {
            continue;
        };
        let Ok((width, height, mut pixels)) = decode_rgba(&bytes) else {
            continue;
        };
        if let Some((skin_w, skin_h, skin_px)) = &skin {
            paste_player_skin(&mut pixels, width, height, skin_px, *skin_w, *skin_h);
        }
        images.push((
            name.to_owned(),
            downsample_rgba(&pixels, width, height, ATLAS_SIDE, ATLAS_SIDE),
        ));
    }
    images
}

/// `GirlModel` stamps the 64×64 player skin at pixel (192, 0), three image
/// pixels per skin pixel, on the 4096² sheet. Those UVs are the Steve rig.
fn paste_player_skin(pixels: &mut [u8], width: u32, height: u32, skin: &[u8], sw: u32, sh: u32) {
    if sw == 0 || sh == 0 || width == 0 || height == 0 {
        return;
    }
    let scale = width as f32 / 4096.0;
    let origin = (192.0 * scale).round() as i32;
    let step = (3.0 * scale).round().max(1.0) as i32;
    for y in 0..sh {
        for x in 0..sw {
            let si = ((y * sw + x) * 4) as usize;
            if si + 3 >= skin.len() {
                continue;
            }
            for ey in 0..step {
                for ex in 0..step {
                    let dx = origin + x as i32 * step + ex;
                    let dy = y as i32 * step + ey;
                    if dx < 0 || dy < 0 || dx >= width as i32 || dy >= height as i32 {
                        continue;
                    }
                    let di = ((dy as u32 * width + dx as u32) * 4) as usize;
                    pixels[di..di + 4].copy_from_slice(&skin[si..si + 4]);
                }
            }
        }
    }
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
    let info = reader
        .next_frame(&mut buffer)
        .map_err(|err| err.to_string())?;
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
    fn pasted_skin_uv_lands_on_the_player_texture() {
        let uv = player_skin_uv([3.0 / 64.0, 1.5 / 64.0]);
        assert!(uv[0].abs() < 1e-5);
        assert!((uv[1] - 0.5).abs() < 1e-5);
    }

    #[test]
    fn downsample_averages_a_two_by_two_block() {
        let src = [0, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255];
        let out = downsample_rgba(&src, 2, 2, 1, 1);
        assert_eq!(out, vec![63, 63, 63, 255]);
    }

    #[test]
    fn missing_texture_is_skipped() {
        assert!(
            character_textures_in(std::path::Path::new("/missing/sexmod/assets"), None).is_empty()
        );
    }

    #[test]
    fn jenny_menu_matches_the_original() {
        let jenny = scenes(Kind::Jenny);
        assert_eq!(
            jenny.iter().map(|scene| scene.label).collect::<Vec<_>>(),
            ["Blowjob", "Doggy", "Strip"]
        );
        assert_eq!(jenny[0].cost, 3);
        assert_eq!(jenny[1].item, "minecraft:diamond");
        assert_eq!(jenny[2].clip, "strip");
        assert_eq!(scenes(Kind::Ellie).len(), 2);
        assert!(scenes(Kind::Slime).is_empty());
        assert_eq!(scenes(Kind::Ellie)[1].clip, "cowgirlstart");
    }

    #[test]
    fn strip_sound_cue_crosses_become_nude() {
        let motions = super::figure::Motions::parse(
            r#"{"animations":{"animation.jenny.strip":{"animation_length":5,"sound_effects":{"3.0":{"effect":"becomeNude"},"5.0":{"effect":"stripDone"}}}}}"#,
        )
        .unwrap();
        assert!(
            motions
                .cues_crossed("animation.jenny.strip", 2.9, 3.05)
                .iter()
                .any(|cue| cue == "becomeNude")
        );
        assert!(
            motions
                .cues_crossed("animation.jenny.strip", 0.0, 2.9)
                .is_empty()
        );
        let done = motions.cues_crossed("animation.jenny.strip", 4.9, 5.05);
        assert_eq!(done, ["stripDone"]);
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
        let quads = figure.pose("animation.test.missing", 0.0, Vec3::ZERO, 0.0, false);
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
        let mid = figure.pose("animation.test.idle", 0.5, Vec3::ZERO, 0.0, false);
        let halfway = mid[0].corners.iter().any(|corner| {
            (corner.x + 0.354).abs() < 0.03
                && (corner.y - 1.060).abs() < 0.03
                && (corner.z - 0.5).abs() < 0.03
        });
        assert!(halfway, "{:?}", mid[0].corners);
    }
}

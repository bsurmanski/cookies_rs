use crate::entity::*;
use crate::collision::*;

use nalgebra::{Vector3, Vector4, Matrix4};
use nalgebra::zero;
use rockwork::mesh::*;
use rockwork::texture::*;

use sdl2::mixer::*;

use std::cell::RefCell;
use std::sync::Mutex;
use lazy_static::*;

macro_rules! include_wav {
    ($x:literal) => {
        sdl2::rwops::RWops::from_bytes(include_bytes!($x).as_ref()).unwrap().load_wav().unwrap()
    };
}

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_png_texture!("../res/pillduck.png");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/pillduck.mdl");
}

thread_local!(static HOP: RefCell<Option<Chunk>> = RefCell::new(None));

#[derive(Clone)]
pub struct DuckMan {
    position: Vector3<f32>,
    rotation: f32,
    scale: f32,
    nummy_timer: f32,
    tick: f32,
    dead: bool,
    moved: bool,
}

impl DuckMan {
    pub fn new() -> DuckMan {
        HOP.with(|h| {
            *h.borrow_mut() = Some(include_wav!("../res/hop.wav"));
        });

        DuckMan {
            position: zero(),
            rotation: 0.0,
            scale: 0.1,
            nummy_timer: 0.0,
            tick: 0.0,
            dead: false,
            moved: false,
        }
    }

    pub fn eat(&mut self, e: &dyn Entity) {
        self.scale += e.nummies();
        self.nummy_timer += e.yummy_nummies();
    }

    pub fn kill(&mut self) {
        self.dead = true;
    }

    pub fn nummy_timer(&self) -> f32 {
        self.nummy_timer
    }

    pub fn set_scale(&mut self, s: f32) {
        self.scale = s;
    }

    pub fn rotate(&mut self, r: f32) {
        self.rotation += r;
    }

    pub fn step(&mut self) {
        let rot = Matrix4::from_scaled_axis(Vector3::new(0.0, self.rotation, 0.0));
        let mut dv = rot * Vector4::new(0.0, 0.0, -0.2 * f32::sqrt(self.scale), 0.0);

        if self.nummy_timer > 0.0 {
            dv *= 1.5;
        }

        self.position += dv.xyz();
        self.moved = true;
    }

    pub fn reset(&mut self) {
        self.dead = false;
        self.nummy_timer = 0.0;
        self.scale = 0.1;
        self.position = zero();
    }
}

impl Entity for DuckMan {
    fn is_dead(&self) -> bool { self.dead }

    fn nummies(&self) -> f32 { 0.15 }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, dt: f32) {
        let inflection = f32::cos(self.tick * 20.0) < 0.0 && f32::cos((self.tick + dt) * 20.0)  > 0.0;
        self.tick += dt;

        let mut targety = 0.0;
        if self.moved {
            targety = f32::abs(f32::sin(self.tick * 10.0)) / 4.0;
        }
        self.position.y += (targety - self.position.y) * 0.6;

        if inflection && self.moved {
            HOP.with(|h| {
                sdl2::mixer::Channel(-1).play_timed(h.borrow().as_ref().unwrap(), 0, -1);
            });
            // TODO play sound
        }

        if self.nummy_timer > 0.0 {
            self.nummy_timer -= dt;
        }
        self.moved = false;
        //TODO
    }

    fn get_mesh(&self) -> &Mesh {
        return &MESH;
    }

    fn get_texture(&self) -> &Texture {
        return &TEXTURE;
    }

    fn get_hitbox(&self) -> Box3 {
        Box3::new(self.position.xyz(), Vector3::new(0.9, 1.3, 0.9) * self.scale)
    }
}

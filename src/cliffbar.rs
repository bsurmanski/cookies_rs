use crate::collision::*;
use crate::entity::*;
use crate::man::*;

use nalgebra::Vector3;
use rand::Rng;
use rand::random;
use rockwork::mesh::*;
use rockwork::texture::*;

use lazy_static::*;

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_png_texture!("../res/cliffbar.png");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/cliffbar.mdl");
}

pub struct Cliffbar {
    position: Vector3<f32>,
    rotation: f32,
    dead: bool
}

impl Cliffbar {
    pub fn new(position: Vector3<f32>, rotation: f32) -> Cliffbar {
        Cliffbar {
            position,
            rotation,
            dead: false
        }
    }
}

impl Entity for Cliffbar {
    fn is_dead(&self) -> bool { self.dead }

    fn nummies(&self) -> f32 { 0.1 }

    fn yummy_nummies(&self) -> f32 { 10.0 }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, dt: f32) {
        self.rotation += 3.2 * dt;
        self.position.y = (f32::sin(self.rotation) / 2.0 + 0.5) / 10.0
    }

    fn on_collision(&mut self, o: &mut DuckMan) {
        if !self.dead && o.get_scale() > 0.4 {
            self.dead = true;
            o.eat(self);
        }
    }

    fn get_mesh(&self) -> &Mesh {
        return &MESH;
    }

    fn get_texture(&self) -> &Texture {
        return &TEXTURE;
    }

    fn get_hitbox(&self) -> Box3 {
        Box3::new(self.position.xyz(), Vector3::new(0.5, 0.5, 0.5))
    }
}

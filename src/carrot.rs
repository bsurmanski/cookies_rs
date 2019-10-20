use crate::entity::*;
use crate::man::*;
use crate::collision::*;

use nalgebra::Vector3;
use rockwork::mesh::*;
use rockwork::texture::*;

use lazy_static::*;

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_png_texture!("../res/carrot.png");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/carrot.mdl");
}

pub struct Carrot {
    position: Vector3<f32>,
    rotation: f32,
    dead: bool
}

impl Carrot {
    pub fn new(position: Vector3<f32>, rotation: f32) -> Carrot {
        Carrot {
            position,
            rotation,
            dead: false
        }
    }
}

impl Entity for Carrot {
    fn is_dead(&self) -> bool { self.dead }

    fn nummies(&self) -> f32 { 0.15 }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, _dt: f32) {
        //TODO
    }

    fn on_collision(&mut self, o: &mut DuckMan) {
        if o.get_scale() * 2.2 > 1.5 {
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
        Box3::new(self.position.xyz(), Vector3::new(1.0, 0.98, 0.86))
    }
}

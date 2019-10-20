use crate::collision::*;
use crate::entity::*;
use crate::draw_context::*;
use crate::man::*;

use nalgebra::{Vector3, Matrix4, zero};
use rockwork::mesh::*;
use rockwork::texture::*;

use lazy_static::*;

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_png_texture!("../res/cookie.png");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/cookie.mdl");
    static ref MONKEY: Mesh = rockwork::include_mdl!("../res/monkey.mdl");
}

pub struct Cookie {
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    rotation: f32,
    tick: f32,
    falls: bool,
    dead: bool,
}

impl Cookie {
    pub fn new(position: Vector3<f32>, falls: bool) -> Cookie {
        Cookie {
            position,
            velocity: zero(),
            rotation: 0.0,
            tick: 0.0,
            falls,
            dead: false,
        }
    }

    pub fn draw(&self, ctx: &DrawContext, view: &Matrix4<f32>) {
        let model: Matrix4<f32> = 
            Matrix4::new_scaling(2.0) * 
            Matrix4::new_translation(
            &self.position) *
            Matrix4::from_scaled_axis(Vector3::new(0.0, self.tick * 2.0, 0.0)) *
            Matrix4::from_scaled_axis(Vector3::new(0.71, 0.0, 0.0));
        let mv = view * model;
        ctx.run_mesh_program(self.get_mesh(), self.get_texture(), &mv);
    }
}

impl Entity for Cookie {
    fn is_dead(&self) -> bool { self.dead }

    fn are_you_cookie(&self) -> bool { true }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, dt: f32) {
        self.tick += dt;
        if self.falls {
            if self.position.y > 1.0 {
                self.velocity -= Vector3::new(0.0, 0.64 * dt, 0.0);
                self.position += self.velocity;
            } else {
                self.velocity *= -0.7;
                self.position.y = 1.01;
            }
        }
        //TODO
    }

    fn on_collision(&mut self, o: &mut DuckMan) {
        self.dead = true;
        o.eat(self);
    }

    fn get_mesh(&self) -> &Mesh {
        return &MESH;
    }

    fn get_texture(&self) -> &Texture {
        return &TEXTURE;
    }

    fn get_hitbox(&self) -> Box3 {
        Box3::new(self.position.xyz(), Vector3::new(0.25, 0.25, 0.25))
    }
}

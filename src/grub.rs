use crate::entity::*;
use crate::man::*;
use crate::collision::*;

use nalgebra::{Matrix4, Vector3, Vector4};
use rand::random;
use rockwork::mesh::*;
use rockwork::texture::*;

use lazy_static::*;

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_tga_texture!("../res/grub.tga");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/grub.mdl");
}

pub struct Grub {
    position: Vector3<f32>,
    rotation: f32,
    tick: f32,
    timer: f32,
    dead: bool,
    scale: f32,
    state: i8,
    spin: i8,
}

impl Grub {
    pub fn new(position: Vector3<f32>, rotation: f32) -> Grub {
        Grub {
            position,
            rotation,
            tick: 0.0,
            timer: 0.0,
            scale: 0.5 + random::<f32>(),
            state: 0,
            spin: 1,
            dead: false
        }
    }
}

impl Entity for Grub {
    fn is_dead(&self) -> bool { self.dead }

    fn nummies(&self) -> f32 { 0.06 }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_scale(&self) -> f32 { self.scale }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, dt: f32) {
        self.tick += dt;
        self.timer -= dt;

        if self.timer <= 0.0 {
            self.state = !self.state;
            if random::<f32>() > 0.4 {
                self.spin = -self.spin;
            }
            self.timer = random::<f32>() * 3.0;
        }

        if self.state == 0 {
            self.rotation += 10.0 * dt * self.spin as f32;
        } else {
            let rot = Matrix4::from_scaled_axis(Vector3::new(0.0, self.rotation, 0.0));
            let mut dv = rot * Vector4::new(0.0, 0.0, -6.4 * dt * f32::sqrt(self.scale), 0.0);
            self.position += dv.xyz();
            if self.position.x < -9.0 || self.position.x > 9.0 ||
               self.position.z < -9.0 || self.position.z > 9.0 {
                   if dv.xyz().dot(&self.position) > 0.0 {
                       self.state = 0;
                       self.timer /= 2.0;
                       return
                   }
            }
        }
    }

    fn on_collision(&mut self, o: &mut DuckMan) {
        if o.get_scale() * 2.2 > self.scale * 0.4 {
            self.dead = true;
            o.eat(self);
        } else {
            o.kill();
        }
    }

    fn get_mesh(&self) -> &Mesh {
        return &MESH;
    }

    fn get_texture(&self) -> &Texture {
        return &TEXTURE;
    }

    fn get_hitbox(&self) -> Box3 {
        Box3::new(self.position.xyz(), Vector3::new(0.30, 0.2, 0.30))
    }
}

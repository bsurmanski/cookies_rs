use crate::entity::*;
use crate::man::*;
use crate::collision::*;

use nalgebra::{Matrix4, Vector3, Vector4};
use rand::random;
use rockwork::mesh::*;
use rockwork::texture::*;

use lazy_static::*;

lazy_static!{
    static ref TEXTURE: Texture = rockwork::include_tga_texture!("../res/girlduck.tga");
    static ref MESH: Mesh = rockwork::include_mdl!("../res/pillduck.mdl");
}

pub struct Girl {
    position: Vector3<f32>,
    rotation: f32,
    tick: f32,
    timer: f32,
    dead: bool,
    scale: f32,
    state: i8,
    spin: i8,
}

impl Girl {
    pub fn new(position: Vector3<f32>, rotation: f32) -> Girl {
        Girl {
            position,
            rotation,
            tick: 0.0,
            timer: 0.0,
            scale: 1.5,
            state: 0,
            spin: 1,
            dead: false
        }
    }
}

impl Entity for Girl {
    fn is_dead(&self) -> bool { self.dead }

    fn nummies(&self) -> f32 { 0.07 }

    fn get_position(&self) -> Vector3<f32> {
        self.position
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn update(&mut self, dt: f32) {
        self.tick += dt;
        self.timer -= dt;
        let inflection = f32::cos(self.tick * 20.0) < 0.0 && f32::cos((self.tick + dt) * 20.0) > 0.0;
        let targety = f32::abs(f32::sin(self.tick * 10.0)) / 4.0;

        if self.timer <= 0.0 {
            self.state = !self.state;
            if random::<f32>() > 0.4 {
                self.spin = -self.spin;
            }

            if self.state == 0 {
                self.timer = random::<f32>() * 1.5;
            } else {
                self.timer = random::<f32>() * 3.0;
            }
        }

        if self.state == 0 {
            self.rotation += 8.0 * dt * self.spin as f32;
            if random::<f32>() > 0.95 {
                self.spin = -self.spin;
            }
        } else {
            self.rotation += 3.2 * dt * self.spin as f32;
            if random::<f32>() > 0.96 {
                self.spin = -self.spin;
            }

            let rot = Matrix4::from_scaled_axis(Vector3::new(0.0, self.rotation, 0.0));
            let mut dv = rot * Vector4::new(0.0, 0.0, -1.6 * dt * f32::sqrt(self.scale), 0.0);
            self.position += dv.xyz();
            self.position.y += (targety - self.position.y) * 0.6;

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
        if o.get_scale() * 2.2 > 1.5 {
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
        Box3::new(self.position.xyz(), Vector3::new(1.2, 2.5, 1.2))
    }
}

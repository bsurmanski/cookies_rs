use nalgebra::Matrix4;
use nalgebra::Vector3;
use crate::collision::*;
use crate::draw_context::*;
use crate::man::*;
use rockwork::mesh::*;
use rockwork::texture::*;

pub trait Entity {
    fn nummies(&self) -> f32 { 0.01 }
    fn yummy_nummies(&self) -> f32 { 0.0 }
    fn are_you_cookie(&self) -> bool { false }
    fn is_dead(&self) -> bool { false }
    fn is_baddie(&self) -> bool { false }
    fn is_girl(&self) -> bool { false }

    fn get_position(&self) -> Vector3<f32>;
    fn get_rotation(&self) -> f32;

    fn update(&mut self, dt: f32);
    fn on_collision(&mut self, _o: &mut DuckMan) {}
    fn get_mesh(&self) -> &Mesh;
    fn get_texture(&self) -> &Texture;
    fn get_scale(&self) -> f32 { 1.0 }
    fn get_hitbox(&self) -> Box3;
    fn draw(&self, ctx: &DrawContext, view: Matrix4<f32>) {
        let m = 
            Matrix4::new_translation(
                &self.get_position()) *
            Matrix4::new_scaling(self.get_scale()) *
            Matrix4::from_scaled_axis(Vector3::new(0.0, self.get_rotation(), 0.0));

        ctx.run_mesh_program(self.get_mesh(), self.get_texture(), &(view * m));

        //TODO draw hitbox?
    }
}

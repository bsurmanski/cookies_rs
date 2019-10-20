use crate::cookie::*;
use crate::entity::*;
use crate::draw_context::*;

use nalgebra::{Vector3, Matrix4};
use nalgebra::zero;

use rockwork::texture::*;




pub struct Title {
    title_name: Texture,
    cookie: Cookie,
}

impl Title {
    pub fn new() -> Title {
        Title {
            title_name: rockwork::include_tga_texture!("../res/title.tga"),
            cookie: Cookie::new(zero(), false),
        }
    }

    pub fn update(&mut self, tick: f32) {
        self.cookie.update(tick);
    }

    pub fn draw(&self, ctx: &DrawContext, tick: f32) { 
        ctx.run_title_program(&self.title_name, &Matrix4::identity(), tick);
        ctx.clear_depth();
        let view: Matrix4<f32> = Matrix4::new_translation(
            &Vector3::new(0.0, 0.0, -5.0));
        self.cookie.draw(ctx, &view);
    }
}

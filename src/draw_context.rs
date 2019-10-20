use std::f32::consts::PI;
use nalgebra::{Matrix4, Perspective3};
use rockwork::{
    framebuffer::*,
    mesh::*,
    texture::*,
    texture::TextureFormat::*,
    program::*
};


pub struct DrawContext {
    width: usize,
    height: usize,
    quad: Mesh,
    framebuffer: Framebuffer,
    simple_program: Program,
    mesh_program: Program,
    title_program: Program,
}

impl DrawContext {
    pub fn new(w: usize, h: usize) -> DrawContext {
        unsafe {
            gl::Disable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
        }
        DrawContext{
            width: w, height: h,
            quad: rockwork::include_mdl!("../res/unit_quad.mdl"),
            framebuffer: Framebuffer::new(w, h, &[Rgba, Rgba, Rgba, Depth]),
            simple_program: rockwork::include_simple_program!("Simple".to_string(), 
                                                             "../res/glsl/simple.vs",
                                                             "../res/glsl/simple.fs"),
            mesh_program: rockwork::include_simple_program!("Mesh".to_string(), 
                                                             "../res/glsl/mesh.vs",
                                                             "../res/glsl/mesh.fs"),
            title_program: rockwork::include_simple_program!("Title".to_string(), 
                                                             "../res/glsl/title.vs",
                                                             "../res/glsl/title.fs"),
        }
    }

    pub fn cull_faces(&self, x: bool) {
        unsafe {
            if x {
                gl::Enable(gl::CULL_FACE);
            } else {
                gl::Disable(gl::CULL_FACE);
            }
        }
    }

    pub fn clear_buffer(&self) {
        unsafe {
            self.framebuffer.bind();
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
            gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_depth(&self) {
        unsafe {
            self.framebuffer.bind();
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_color(&self) {
        unsafe {
            self.framebuffer.bind();
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn run_mesh_program(&self, mesh: &Mesh, tex: &Texture, mv: &Matrix4<f32>) {
        self.framebuffer.bind();
        self.mesh_program.bind();
        mesh.bind();
        self.mesh_program.bind_texture("t_color", tex, 0);
        let persp = Perspective3::new(1.0, PI / 2.0, 1.0, 10000.0);
        let mvp = persp.as_matrix() * mv;
        self.mesh_program.set_uniform_mat4("matrix", &mvp);
        mesh.draw();
    }

    pub fn run_simple_program(&self, mesh: &Mesh, tex: &Texture, mv: &Matrix4<f32>, tick: f32) {
        Framebuffer::unbind();
        self.simple_program.bind();
        mesh.bind();
        self.simple_program.bind_texture("tex", tex, 0);
        self.simple_program.set_uniform_bool("crazy", false);
        self.simple_program.set_uniform_bool("boring", false);
        let persp = Perspective3::new(1.0, PI / 2.0, 1.0, 10000.0);
        let mvp = persp.as_matrix() * mv;
        self.simple_program.set_uniform_mat4("matrix", &mvp);
        self.simple_program.set_uniform_float("tick", tick);
        mesh.draw();
    }

    pub fn run_deferred_program(&self, w: usize, h: usize, tick: f32) {
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
        }
        self.run_simple_program(&self.quad, self.framebuffer.color_target(0), &Matrix4::identity(), tick);
    }

    pub fn run_title_program(&self, tex: &Texture, view: &Matrix4<f32>, tick: f32) {
        let persp = Perspective3::new(1.0, PI / 2.0, 1.0, 10000.0);
        let mvp = persp.as_matrix() * view;
        self.framebuffer.bind();
        self.title_program.bind();
        self.quad.bind();
        self.title_program.bind_texture("tex", tex, 0);
        self.title_program.set_uniform_float("tick", tick);
        self.title_program.set_uniform_bool("crazy", false);
        self.title_program.set_uniform_mat4("matrix", &mvp);
        self.quad.draw();
    }
}

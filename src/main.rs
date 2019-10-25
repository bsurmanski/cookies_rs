mod carrot;
mod cliffbar;
mod collision;
mod cookie;
mod crumb;
mod draw_context;
mod entity;
mod girl;
mod grub;
mod man;
mod mouse;
mod music;
mod title;

use carrot::*;
use cliffbar::*;
use cookie::*;
use crumb::*;
use draw_context::*;
use entity::*;
use girl::*;
use grub::*;
use man::*;
use mouse::*;
use music::*;
use title::*;

use nalgebra::{Vector3, Matrix4};
use rand::random;
use rockwork::context::Context;
use rockwork::mesh::*;
use rockwork::texture::*;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::time::Duration;

enum Screen {
    Title,
    Instructions,
    Game,
    Win,
    Lose
}

pub struct GameData {
    man: DuckMan,
    music: MusicContext,
    cookie: Option<Cookie>,
    where_are_we: Screen,
    space_down: bool,
    house_inside_mesh: Mesh,
    house_inside_tex: Texture,
    title: Title,
    instructions: Texture,
    win: Texture,
    lose: Texture,
    draw: DrawContext,
    entities: Vec<Box<dyn Entity>>,
    view: Matrix4<f32>,
    tick: f32,
}

fn input(ctx: &mut Context<GameData>) {
    ctx.sdl_event_pump.pump_events();

    for event in ctx.sdl_event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                std::process::exit(0);
            }
            _ => {}
        }
    }

    let keystate = ctx.sdl_event_pump.keyboard_state();

    let mut gd = ctx.game_data_mut();
    match gd.where_are_we {
        Screen::Title => {
            if keystate.is_scancode_pressed(Scancode::Space) && !gd.space_down {
                gd.where_are_we = Screen::Instructions;
            }
        }
        Screen::Instructions => {
            if keystate.is_scancode_pressed(Scancode::Space) && !gd.space_down {
                gd.entities.clear();
                init_entities(&mut gd.entities);
                gd.man.reset();
                gd.where_are_we = Screen::Game;
            }
        }
        Screen::Game => {
            if keystate.is_scancode_pressed(Scancode::Left) {
                gd.man.rotate(0.25);
            }

            if keystate.is_scancode_pressed(Scancode::Right) {
                gd.man.rotate(-0.25);
            }

            if keystate.is_scancode_pressed(Scancode::Up) {
                gd.man.step();
            }
            
        }
        Screen::Win => {
            if keystate.is_scancode_pressed(Scancode::Space) && !gd.space_down {
                gd.where_are_we = Screen::Title;
            }
        }
        Screen::Lose => {
            if keystate.is_scancode_pressed(Scancode::Space) && !gd.space_down {
                gd.where_are_we = Screen::Instructions;
            }
        }
    }
    if keystate.is_scancode_pressed(Scancode::A) {
        gd.man.set_scale(1.5);
    }
    gd.space_down = keystate.is_scancode_pressed(Scancode::Space);
}

fn update(ctx: &Context<GameData>, dt: f32) {
    let mut gd = ctx.game_data_mut();
    gd.tick += dt;
    match gd.where_are_we {
        Screen::Title => {
            gd.title.update(dt);
        }
        Screen::Game => {
            gd.man.update(dt);
            let mut man = gd.man.clone();
            for e in gd.entities.iter_mut() {
                e.as_mut().update(dt);
                if man.get_hitbox().collides(&e.get_hitbox()) {
                    e.on_collision(&mut man);
                }
            }

            if gd.man.is_dead() {
                gd.where_are_we = Screen::Lose;
            }

            gd.entities.retain(|e| !e.is_dead());
            gd.man = man.clone();

            if gd.entities.len() == 0 {
                if gd.cookie.is_none() {
                    gd.cookie = Some(Cookie::new(Vector3::new(0.0, 15.0, 0.0), true));
                }
                gd.cookie.as_mut().unwrap().update(dt);
                if man.get_hitbox().collides(&gd.cookie.as_mut().unwrap().get_hitbox()) {
                    gd.cookie.as_mut().unwrap().on_collision(&mut man);
                }

                if gd.cookie.as_ref().unwrap().is_dead() {
                    gd.where_are_we = Screen::Win;
                    gd.cookie = None;
                }
            }

            let pos = gd.man.get_position();
            let scale = gd.man.get_scale();
            gd.view = 
                Matrix4::from_scaled_axis(Vector3::new(0.5, 0.0, 0.0)) *
                Matrix4::new_translation(
                &Vector3::new(-pos.x,
                              -6.0 * scale - 1.0,
                              -8.0 * scale - pos.z - 1.0));
        }
        _ => {}
    }

    if !gd.man.is_dead() {
        if gd.man.nummy_timer() > 0.0 {
            gd.music.update(dt * 2.0);
        } else {
            gd.music.update(dt);
        }
    }
}

fn draw_house(ctx: &DrawContext, mesh: &Mesh, tex: &Texture, view: Matrix4<f32>) {
    ctx.cull_faces(true);
    ctx.run_mesh_program(mesh, tex, &view);
    ctx.cull_faces(false);
}

fn draw(ctx: &mut Context<GameData>) {
    ctx.window().clear_with_color(0.0, 0.0, 0.0);
    let gd = ctx.game_data();
    gd.draw.clear_buffer();

    match gd.where_are_we {
        Screen::Title => {
            gd.title.draw(&gd.draw, gd.tick);
        }
        Screen::Instructions => {
            gd.draw.run_title_program(&gd.instructions, &Matrix4::identity(), gd.tick);
        }
        Screen::Game => {
            draw_house(&gd.draw, &gd.house_inside_mesh, &gd.house_inside_tex, gd.view);
            gd.man.draw(&gd.draw, gd.view);
            for e in gd.entities.iter() {
                e.draw(&gd.draw, gd.view);
            }

            if gd.cookie.is_some() {
                gd.cookie.as_ref().unwrap().draw(&gd.draw, &gd.view);
            }
        }
        Screen::Win => {
            gd.draw.run_title_program(&gd.win, &Matrix4::identity(), gd.tick);
        }
        Screen::Lose => {
            gd.draw.run_title_program(&gd.lose, &Matrix4::identity(), gd.tick);
        }
    }
    gd.draw.run_deferred_program(920, 720, gd.tick);
    ctx.window().swap_buffers();
}

fn tick(ctx: &mut Context<GameData>, dt: Duration) {
    input(ctx);
    update(ctx, dt.as_micros() as f32 / 1000000.0);
    draw(ctx);
}

fn random_position() -> Vector3<f32> {
    Vector3::new(random::<f32>() * 18.0 - 9.0, 
                 0.0,
                 random::<f32>() * 18.0 - 9.0)
}

fn random_rotation() -> f32 {
    random::<f32>() * 6.0 // 6 = 2PI (close enough)
}

fn init_entities(entities: &mut Vec<Box<dyn Entity>>) {
    for _ in 0..40 {
        entities.push(Box::new(Crumb::new(
                    random_position(),
                    random_rotation())));
    }

    for _ in 0..3 {
        entities.push(Box::new(Carrot::new(
                    random_position(),
                    random_rotation())));
    }

    for _ in 0..2 {
    entities.push(Box::new(Mouse::new(
                random_position(),
                random_rotation())));
    }

    for _ in 0..5 {
    entities.push(Box::new(Grub::new(
                random_position(),
                random_rotation())));
    }

    entities.push(Box::new(Girl::new(
                random_position(),
                random_rotation())));

    for _ in 0..2 {
        entities.push(Box::new(Cliffbar::new(
                    random_position(),
                    random_rotation())));
    }
}

fn main() {
    let w = 920;
    let h = 720;

    let mut ctx = Context::new();
    ctx.open_window("Who Ate Cookies?".to_string(), 920, 720);

    let entities: Vec<Box<dyn Entity>> = vec![];

    ctx.attach_game_data(GameData {
        man: DuckMan::new(),
        music: MusicContext::new(),
        cookie: None,
        where_are_we: Screen::Title,
        space_down: false,
        title: Title::new(),
        draw: DrawContext::new(w / 4, h / 4),
        instructions: rockwork::include_tga_texture!("../res/instructions.tga"),
        house_inside_mesh: rockwork::include_mdl!("../res/house_inside.mdl"),
        house_inside_tex: rockwork::include_tga_texture!("../res/house_inside.tga"),
        win: rockwork::include_tga_texture!("../res/win.tga"),
        lose: rockwork::include_tga_texture!("../res/lose.tga"),
        entities,
        view: Matrix4::identity(),
        tick: 0.0,
    });

    ctx.run(&mut tick);
}

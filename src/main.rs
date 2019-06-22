pub mod bird;
pub mod game_state;
pub mod pipe;
pub mod tex_loader;

// agnostic graphics backend imports
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_graphics;

// crate for handling scene and sprite
// transitions in a sane way.
extern crate sprite;

// piston imports
extern crate piston;

extern crate shader_version;

// window management
use piston::window::*;
use piston_window::*;

// user input
use piston::event_loop::{EventLoop, EventSettings, Events};

//graphics APIS

use gfx_graphics::TextureContext;
use graphics::rectangle::*;
use std::rc::Rc;

static WINSIZE: Size = Size {
    height: 700.0,
    width: 350.0,
};

fn main() -> Result<(), u32> {
    let mut window: PistonWindow =
        WindowSettings::new("Not FlappyBird", (WINSIZE.height, WINSIZE.width))
            .graphics_api(OpenGL::V3_2)
            .resizable(false)
            .build()
            .expect("Window didn't build");

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    //set events at synced updates and FPS
    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
    let ds = graphics::DrawState::default();

    let mut state = game_state::GameState::new(1.8, 0.0);
    let am: tex_loader::AssetMap = tex_loader::AssetMap::load_assets(&mut texture_context);

    // the bird flapping up texture, for closure reasons
    let unflap_tex = Rc::new(am.bird_tex.clone());
    let flap_tex = Rc::new(am.bird_up_tex.clone());
    let pipe_tex = Rc::new(am.pipe_tex.clone());

    let mut pipe = sprite::Sprite::from_texture(pipe_tex.clone());
    let mut reverse = sprite::Sprite::from_texture(pipe_tex.clone());
    let mut bird = sprite::Sprite::from_texture(unflap_tex.clone());

    bird.set_scale(0.08, 0.08);
    pipe.set_scale(0.5, 0.5);
    reverse.set_scale(0.5, 0.5);
    reverse.set_rotation(180.0);

    while let Some(ev) = events.next(&mut window) {
        if let Some(p) = ev.press_args() {
            state.update(p);
            if !state.paused {
                state.bird.key_event(p);
            }
            // if escape or something pressed, call reset.
        }

        if let Some(u) = ev.update_args() {
            // increment challenge as it runs
            //state.xvel = (state.ticks as f64 / 3000.0) + 1.0;
            //  check and set pipe state
            if !state.paused && state.bird.is_pressed {
                state.ticks += 1;
                state.bird.update(&ev, u);
                pipe::update_pipe_state(
                    &mut state.pipe_deque,
                    state.xvel,
                    state.ticks,
                    &mut state.bird,
                    &mut state.score,
                );

                if state.bird.ypos > 283.0 || state.bird.collide {
                    state.lose();
                }
                state.update_score();
            }
        }

        if let Some(_) = ev.render_args() {
            // increment stage movement
            state.stage_offset -= state.xvel;
            state.stage_offset %= WINSIZE.width;
            window.draw_2d(&ev, |c, g, _| {
                // clear bg
                clear([1.0; 4], g);
                // BACKGROUND PARALLAX CODE BEGIN
                for image_idx in 0..3 {
                    // prevents image seams
                    let jitter_offset = f64::from(image_idx);
                    let x_coord = (jitter_offset * 350.0) + state.stage_offset - jitter_offset;
                    let j = Image::new().rect(square(x_coord, 0.0, WINSIZE.width));
                    // call makeup : image itself, context mutations, graphics
                    j.draw(&am.bg_tex, &ds, c.transform, g);

                    for pipe_idx in 0..state.pipe_deque.len() {
                        let p = &mut state.pipe_deque[pipe_idx];
                        pipe.set_position(p.x, p.height);
                        reverse.set_position(p.x, p.height - p.gap - 400.0);
                        pipe.draw(c.transform, g);
                        reverse.draw(c.transform, g);
                    }

                    // BACKGROUND PARALLAX CODE END

                    // BIRD DRAWING CODE
                    bird.set_position(state.bird_pos, state.bird.ypos);
                    bird.set_rotation(state.bird.rotation);
                    // incorrect state upward
                    if !state.bird.flapping && state.bird.rotation < 0.0 {
                        bird.set_texture(unflap_tex.clone());
                        state.bird.flapping = true;
                    }
                    //incorrect state downward
                    if state.bird.flapping && state.bird.rotation > 0.0 {
                        bird.set_texture(flap_tex.clone());
                        state.bird.flapping = false;
                    }
                    bird.draw(c.transform, g);

                    // BIRD DRAWING CODE END
                }

                //render ground separately
                //ground has to render after bird bc bird falls behind ground
                for image_idx in 0..3 {
                    let jitter_offset = f64::from(image_idx);
                    let x_coord = (jitter_offset * 350.0) + state.stage_offset - jitter_offset;
                    let j = Image::new().rect(square(x_coord, 1500.0, WINSIZE.width));
                    j.draw(&am.ground_tex, &ds, c.transform.scale(1.3, 0.2), g);
                }

                //render start screen
                if state.ticks < 125 && !state.bird.is_pressed {
                    let start = Image::new().rect(square(
                        (WINSIZE.width - 75.0 * 2.6) / 2.0,
                        (WINSIZE.height - 75.0 * 8.5) / 2.0,
                        75.0,
                    ));
                    start.draw(&am.start_tex, &ds, c.transform.scale(3.0, 1.0), g);
                }

                //render game over
                if state.lose {
                    let loss = Image::new().rect(square(
                        (WINSIZE.width - 75.0 * 2.5) / 2.0,
                        (WINSIZE.height - 75.0 * 6.0) / 2.0,
                        75.0,
                    ));
                    loss.draw(&am.game_over_tex, &ds, c.transform.scale(3.0, 1.0), g);
                }
            });
        }
    }
    Ok(())
}

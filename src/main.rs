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
    let mut unflap_tex = Rc::new(am.bird_tex.clone());
    let mut flap_tex = Rc::new(am.bird_up_tex.clone());

    let mut bird = sprite::Sprite::from_texture(unflap_tex);
    bird.set_scale(0.08, 0.08); // so this is a bad hack, but in the future use a standard sprite size

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
            //  xvel = ((score / 10) + 1) as f64;
            //  check and set pipe state
            if !state.paused {
                state.ticks += 1;
                state.bird.update(&ev, &u);
                pipe::update_pipe_state(&mut state.pipe_deque, state.ticks);
                if state.bird.ypos > 275.0 || state.bird.collide {
                    state.lose();
                }
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
                // Background Section
                // due to math, there are always three images
                for image_idx in 0..3 {
                    // prevents image seams
                    let jitter_offset = image_idx as f64;
                    let x_coord = (jitter_offset * 350.0) + state.stage_offset - jitter_offset;
                    let j = Image::new().rect(square(x_coord, 0.0, WINSIZE.width));
                    // call makeup : image itself, context mutations, graphics
                    j.draw(&am.bg_tex, &ds, c.transform, g);
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
            });
        }
    }
    Ok(())
}


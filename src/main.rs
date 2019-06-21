pub mod bird;
pub mod game_state;
pub mod pipe;
pub mod tex_loader;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;
// piston reqs
use piston_window::*;
// user input
use graphics::rectangle::square;
use opengl_graphics::Texture;
use piston::event_loop::{EventLoop, EventSettings, Events};
use sprite::*;
use std::rc::Rc;

use vecmath::Vector2;

// width height pair
static WINDIMS: [f64; 2] = [700.0, 350.0];

fn main() {
    // build window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("flappy bird", (WINDIMS[0], WINDIMS[1]))
        .title("Not Flappybird".to_string())
        .graphics_api(opengl)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Window didn't build: {}", e));

    // intialize entity textures
    let am: tex_loader::AssetMap = tex_loader::AssetMap::load_assets(&mut window);

    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
    let ds = graphics::DrawState::default();
    let mut scene: sprite::Scene<opengl_graphics::Texture> = Scene::new();
    let mut bird_sprite = Sprite::from_texture(Rc::new(am.bird_tex));

    let mut state = game_state::GameState::new(1.8, 0.0);
    while let Some(ev) = events.next(&mut window) {
        if let Some(p) = ev.press_args() {
            state.bird.key_event(p);
        }

        if let Some(update_arg) = ev.update_args() {
            // increment challenge as it runs
            //  xvel = ((score / 10) + 1) as f64;
            //  check and set pipe state
            state.bird.update(&ev, &update_arg);
        }

        if let Some(r) = ev.render_args() {
            // increment stage movement
            state.stage_offset -= state.xvel;
            state.stage_offset %= WINDIMS[1];
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
                    let j = Image::new().rect(square(x_coord, 0.0, WINDIMS[1]));
                    // call makeup : image itself, context mutations, graphics
                    j.draw(&am.bg_tex, &ds, c.transform, g);
                }
                // BACKGROUND PARALLAX CODE END
                // BIRD DRAWING CODE
                let bimage = Image::new().rect(square(state.bird_pos, state.bird.ypos, 35.0));
                bimage.draw(&am.bird_tex, &ds, c.transform, g);
                println!("{}", state.bird.ypos);
                println!("{}", state.bird.window_pos);
                // BIRD DRAWING CODE END
            });
        }
    }
}

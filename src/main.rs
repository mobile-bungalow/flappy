mod bird;
mod pipe;
mod tex_loader;

extern crate graphics;
extern crate piston;
// piston reqs
use piston_window::*;
// user input
use graphics::rectangle::square;
use piston::event_loop::{EventLoop, EventSettings, Events};


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

    //TODO: move all these variables into GAMESTATE struct
    let mut stage_offset: f64 = 0.0;
    let mut xvel: f64 = 1.0; // the starting x velocity
    let mut score: u32 = 0;

    while let Some(ev) = events.next(&mut window) {

        if let Some(_) = ev.update_args() {
            xvel = ((score / 10) + 1) as f64;
        }

        if let Some(_) = ev.render_args() {

            // increment stage movement
            stage_offset -= xvel;
            stage_offset %= WINDIMS[1];
            // BACKGROUND PARALLAX CODE BEGIN
            window.draw_2d(&ev, |c, g, _| {
                clear([1.0; 4], g);
                // Background Section
                // due to math, there are always three images
                for image_idx in 0..3 {
                    // prevents image seams
                    let jitter_offset = image_idx as f64;
                    let x_coord = (jitter_offset * 350.0) + stage_offset - jitter_offset;
                    let j = Image::new().rect(square(x_coord, 0.0, WINDIMS[1]));
                    // call makeup : image itself, context mutations, graphics
                    j.draw(&am.bg_tex, &ds, c.transform, g);
                }
                // BACKGROUND PARALLAX CODE END
            });
        }

    }
}

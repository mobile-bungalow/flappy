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
    let mut pipe_deque: Vec<pipe::Pipe> = Vec::new(); // maintains list of pipe items
    let mut stage_offset: f64 = 0.0;
    let mut xvel: f64 = 1.8; // the starting x velocity
    let mut score: u32 = 0;
    let mut bird: bird::Bird = bird::Bird::new();

    while let Some(ev) = events.next(&mut window) {

        if let Some(p) = ev.press_args() {
            bird.key_event(p);
        }

        if let Some(_) = ev.update_args() {
            // increment challenge as it runs
            //  xvel = ((score / 10) + 1) as f64;
            //  check and set pipe state
            bird.update(&ev);
        }


        if let Some(_) = ev.render_args() {

            // increment stage movement
            stage_offset -= xvel;
            stage_offset %= WINDIMS[1];
            window.draw_2d(&ev, |c, g, _| {
                // clear bg
                clear([1.0; 4], g);
                // BACKGROUND PARALLAX CODE BEGIN
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
                // BIRD DRAWING CODE
                let bimage = Image::new().rect(square(bird.xpos, bird.ypos, 35.0));
                bimage.draw(&am.bird_tex, &ds, c.transform, g);
                // BIRD DRAWING CODE END
            });

        }

    }
}

mod bird;
mod pipe;
mod render_pl;
extern crate piston;

// piston reqs
use piston_window::*;
// user input
use input::*;
use piston::event_loop::{EventLoop, EventSettings, Events};


use vecmath::Vector2;

fn main() {
    // build window
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("flappy bird", (950, 800))
        .title("Not Flappybird".to_string())
        .graphics_api(opengl)
        .build()
        .unwrap_or_else(|e| panic!("Window didn't build: {}", e));


    // intialize entities

    // event loop

    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));


    while let Some(ev) = events.next(&mut window) {
        if let Some(_) = ev.update_args() {}
    }
}

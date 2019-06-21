use piston_window::*;

extern crate graphics;
extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};

static G: f64 = 0.15;

///bird struct
pub struct Bird {
    //bird's x position within the current window, in order to track collisions. not used in render
    pub xpos: f64,
    //bird's y position
    pub ypos: f64,
    //bird's up velocity
    pub up_vel: f64,
    //whether or not the bird has collided with another object
    pub collide: bool,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            xpos: 40.0,
            ypos: 150.0,
            up_vel: 0.0,
            collide: false,
        }
    }

    pub fn update(&mut self, ev: &Event) {
        // accelerated fall over time determined by gravity
        self.up_vel -= G;
        // increment x position to track position relative to other objects
        self.xpos += 1.0;
        // decrement y position to signify falling
        self.ypos -= self.up_vel;
    }

    ///behaviour determined by key presses
    pub fn key_event(&mut self, button: piston::input::Button) {
        if let Button::Keyboard(key) = button {
            match key {
                //if the key pressed is a space, bird jumps
                Key::Space => self.up_vel = 4.0,
                _ => {}
            }
        }
    }
}

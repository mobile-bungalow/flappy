use piston_window::*;

extern crate graphics;
extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};

static G: f64 = 0.15;

pub struct Bird {
    pub xpos: f64,
    pub ypos: f64,
    pub up_vel: f64,
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
        self.up_vel -= G;
        //self.xpos += 1.0; // should be a variable once xvel is in game_state
        self.ypos -= self.up_vel;
    }

    pub fn key_event(&mut self, button: piston::input::Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::Space => self.up_vel = 4.0,
                _ => {}
            }
        }
    }
}

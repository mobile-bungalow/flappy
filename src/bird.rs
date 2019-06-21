use piston_window::*;

extern crate graphics;
use graphics::rectangle::square;
extern crate piston;
use piston::event_loop::{EventLoop, EventSettings, Events};


static G: f64 = 0.002;

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
        self.key_event(ev);
    }


    fn key_event(&mut self, e: &Event) {

        if let Some(button) = e.press_args() {
            if let Button::Keyboard(key) = button {
                match key {
                    Key::Space => self.up_vel = 6.0,
                    _ => {}
                }
            }
        }
    }
}

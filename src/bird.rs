
use ncollide2d::math::Vector;
use ncollide2d::shape::Cuboid;
use piston::input::Event;

static G: f64 = 0.15;

///bird struct
pub struct Bird {
    //bird's x position within the current window, in order to track collisions. not used in render
    pub window_pos: f64,
    //bird's y position
    pub ypos: f64,
    //bird's up velocity
    pub up_vel: f64,
    //whether or not the bird has collided with another object
    pub collide: bool,
    pub is_pressed: bool,
    pub rotation: f64,
    // which image the bird is using, set by user
    pub flapping: bool,
    pub hitbox: Cuboid<f64>,
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            window_pos: 40.0,
            ypos: 150.0,
            up_vel: 0.0,
            collide: false,
            is_pressed: false,
            rotation: 0.0,
            flapping: false,
            hitbox: Cuboid::new(Vector::new(10.0, 10.0)),
        }
    }

    pub fn update(&mut self, _ev: &Event, _args: input::UpdateArgs) {
        // increment x position to track position relative to other objects
        self.window_pos += 1.0;
        // decrement y position to signify falling
        if self.is_pressed && !self.collide {
            self.up_vel -= G;
            self.ypos -= self.up_vel;
            if self.rotation < 90.0 {
                self.rotation = self.up_vel * -11.0
            };
        }
    }

    ///behaviour determined by key presses
    pub fn key_event(&mut self, button: input::Button) {
        if let input::Button::Keyboard(key) = button {
            if let input::Key::Space = key {
                if self.collide {
                    return;
                }
                self.up_vel = 4.0;
                self.is_pressed = true;
            }
        }
    }
}

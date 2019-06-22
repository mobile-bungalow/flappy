extern crate rand;
use ncollide2d::math::Vector;
use ncollide2d::shape::Cuboid;
use rand::Rng;
use std::collections::VecDeque;
// Max challenge for pipes
static MINDIFF: f64 = 70.0;
static MAXDIFF: f64 = 150.0;

// range for the gap
static MAXHEIGHT: f64 = 450.0;
static MINHEIGHT: f64 = 250.0;

static LATENT: u64 = 60;

pub struct Pipe {
    pub spawn_time: u64,
    pub gap: f64,    // the gap height in screen units
    pub height: f64, // the height of the bottom pipe
    pub x: f64,      // the x offset of the pipe
    pub hit_boxes: [Cuboid<f64>; 2],
}

impl Pipe {
    pub fn new(start: f64, spawn_time: u64) -> Self {
        let mut rng = rand::thread_rng();
        let hit_boxes = [
            Cuboid::new(Vector::new(50.0, 350.0)),
            Cuboid::new(Vector::new(50.0, 350.0)),
        ];
        Pipe {
            spawn_time,
            gap: rng.gen_range(MINDIFF, MAXDIFF),
            height: rng.gen_range(MINHEIGHT, MAXHEIGHT),
            x: start,
            hit_boxes,
        }
    }

}

/// Takes a list of pipes, and delta time since the
/// game began, every N seconds after a certain time
/// it pushes a pipe off the stack and generates a new one
/// Yeah , I know this is ugly at the moment, there are better
/// solutions
pub fn update_pipe_state(pipe_deque: &mut VecDeque<Pipe>, xvel: f64, dt: u64) {
    // latent stage

    if dt < LATENT {
        return;
    };

    for i in 0..pipe_deque.len() {
        pipe_deque[i].x -= xvel * 0.9;
    }

    if let Some(p) = pipe_deque.get(0) {
        if p.x < -50.0 {
            pipe_deque.pop_front();
            pipe_deque.push_back(Pipe::new(850.0, 0));
        }
    }

}


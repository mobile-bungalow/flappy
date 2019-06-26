use crate::bird;
extern crate rand;
use rand::Rng;
use std::collections::VecDeque;

// Max challenge for pipes
const MINDIFF: f64 = 75.0;
const MAXDIFF: f64 = 90.0;

// range for the gap
const MAXHEIGHT: f64 = 450.0;
const MINHEIGHT: f64 = 350.0;

const LATENT: u64 = 60;

const PIPE_WIDTH: f64 = 40.0; //width form center of pipe sprite
const PIPE_HEIGHT: f64 = 200.0; //height from the center of the pipe sprite

pub struct Pipe {
    pub spawn_time: u64,
    pub gap: f64,    // the gap height in screen units
    pub height: f64, // the height of the bottom pipe
    pub x: f64,      // the x offset of the pipe
    pub counted: bool,
}

impl Pipe {
    pub fn new(start: f64, spawn_time: u64) -> Self {
        let mut rng = rand::thread_rng();
        Pipe {
            spawn_time,
            gap: rng.gen_range(MINDIFF, MAXDIFF),
            height: rng.gen_range(MINHEIGHT, MAXHEIGHT),
            x: start,
            counted: false,
        }
    }
}

pub fn update_pipe_state(
    pipe_deque: &mut VecDeque<Pipe>,
    xvel: f64,
    dt: u64,
    bird: &mut bird::Bird,
) {
    if dt < LATENT {
        return;
    };

    for mut pipe in pipe_deque.iter_mut() {
        pipe.x -= xvel;
        //if we are near a pipe, 340.0 is bird x pos
        if (340.0 - pipe.x).abs() < PIPE_WIDTH {
            let p = &mut pipe;
            // lower than pipe lip
            if bird.ypos + PIPE_HEIGHT >= p.height || bird.ypos <= p.height - p.gap - PIPE_HEIGHT {
                bird.collide = true;
            }
        }
    }

    if let Some(p) = pipe_deque.get(0) {
        if p.x < -50.0 {
            pipe_deque.pop_front();
            pipe_deque.push_back(Pipe::new(850.0, 0));
        }
    }
}

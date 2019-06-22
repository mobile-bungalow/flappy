extern crate rand;

use rand::Rng;
use std::collections::VecDeque;
// Max challenge for pipes
static MINDIFF: f64 = 30.0;
static MAXDIFF: f64 = 60.0;

// range for the gap
static MAXHEIGHT: f64 = 340.0;
static MINHEIGHT: f64 = 10.0;

static LATENT: u64 = 60;
static SPAWNING: u64 = 560;
static MAINTENANCE: u64 = 980;

pub struct Pipe {
    spawn_time: u64,
    gap: f64,    // the gap height in screen units
    height: f64, // the height of the bottom pipe
    x: f64,      // the x offset of the pipe
}

impl Pipe {
    /// generate a new pipe with difficulty
    /// based off of ther current speed
    /// xvel is the current challenge, start is the spawn position,
    /// spawn time is the time the pipe was born.
    pub fn new(start: f64, spawn_time: u64) -> Self {
        let mut rng = rand::thread_rng();
        Pipe {
            spawn_time,
            gap: rng.gen_range(MINDIFF, MAXDIFF),
            height: rng.gen_range(MINHEIGHT, MAXHEIGHT),
            x: start,
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

    if dt < SPAWNING {
        if !pipe_deque.is_empty() {
            if (dt % 101 / 100) > 0 {
                // enough time has elapsed to spawn another pipe
                // current tick, and where it should spawn off screen
                pipe_deque.push_back(Pipe::new(850.0, dt));
            }
        } else {
            // current tick, and where it should spawn off screen
            pipe_deque.push_back(Pipe::new(850.0, dt));
        }
        for i in 0..pipe_deque.len() {
            pipe_deque[i].x -= xvel * 0.85;
        }
        return;
    };

    if dt < MAINTENANCE {
        if let Some(p) = pipe_deque.pop_front() {
            if p.x < -50.0 {
                pipe_deque.pop_front();
                pipe_deque.push_back(Pipe::new(850.0, dt));
            }
        }
        for i in 0..pipe_deque.len() {
            pipe_deque[i].x -= xvel * 0.85;
        }
        print!("{}\n", pipe_deque.len());
        return;
    };
}


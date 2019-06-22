
extern crate rand;

use rand::Rng;

// Max challenge for pipes
static MINDIFF: f64 = 60.0;
static MAXDIFF: f64 = 30.0;

// range for the gap
static MAXHEIGHT: f64 = 340.0;
static MINHEIGHT: f64 = 10.0;

static LATENT: f64 = 340.0;
static SPAWNING: f64 = 500.0;
static MAINTENANCE: f64 = 1000.0;

pub struct Pipe {
    spawn_time: f64,
    gap: f64,    // the gap height in screen units
    height: f64, // the height of the bottom pipe
    x: f64,      // the x offset of the pipe
}

impl Pipe {
    /// generate a new pipe with difficulty
    /// based off of ther current speed
    /// xvel is the current challenge, start is the spawn position,
    /// spawn time is the time the pipe was born.
    pub fn new(xvel: f64, start: f64, spawn_time: f64) -> Self {
        let mut rng = rand::thread_rng();
        Pipe {
            spawn_time: spawn_time,
            gap: rng.gen_range(MAXDIFF, MINDIFF),
            height: rng.gen_range(MAXHEIGHT, MINHEIGHT),
            x: start,
        }
    }

    pub fn render_call(&self) -> fn() {
        let l = || {};
        l
    }

}

/// Takes a list of pipes, and delta time since the
/// game began, every N seconds after a certain time
/// it pushes a pipe off the stack and generates a new one
/// at the end of the chain. dt should be TIME SINCE THE GAME
/// STARTED
pub fn update_pipe_state(mut pipe_deque: Vec<Pipe>, dt: f64) {
    // latent stage
    if (dt < LATENT) {
        return;
    };

    if (dt < SPAWNING) {
        print!("SPAWN LOGIC");
        return;
    };

    if (dt < MAINTENANCE) {
        print!("MAINTAINENCE LOGIC");
    }
}


extern crate rand;

use rand::Rng;

// Max challenge for pipes
static MINDIFF: f64 = 60.0;
static MAXDIFF: f64 = 30.0;

// range for the gap
static MAXHEIGHT: f64 = 340.0;
static MINHEIGHT: f64 = 10.0;

static LATENT: u64 = 60;
static SPAWNING: u64 = 360;
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
    pub fn new(_xvel: f64, start: f64, spawn_time: u64) -> Self {
        let mut rng = rand::thread_rng();
        Pipe {
            spawn_time,
            gap: rng.gen_range(MAXDIFF, MINDIFF),
            height: rng.gen_range(MAXHEIGHT, MINHEIGHT),
            x: start,
        }
    }

}

/// Takes a list of pipes, and delta time since the
/// game began, every N seconds after a certain time
/// it pushes a pipe off the stack and generates a new one
pub fn update_pipe_state(_pipe_deque: &mut Vec<Pipe>, dt: u64) {
    // latent stage
    if dt < LATENT {
        return;
    };

    if dt < SPAWNING {
        return;
    };

    if dt < MAINTENANCE {};
}


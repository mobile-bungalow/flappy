use crate::bird;
use crate::pipe;
pub struct GameState {
    pub xvel: f64,
    pub stage_offset: f64,
    pub score: u32,
    pub bird: bird::Bird,
    pub pipe_deque: Vec<pipe::Pipe>,
}

impl GameState {
    pub fn new(xvel: f64, stage_offset: f64) -> GameState {
        GameState {
            xvel: xvel,
            stage_offset: stage_offset,
            score: 0,
            bird: bird::Bird::new(),
            pipe_deque: Vec::new(),
        }
    }
}

use crate::bird;
use crate::pipe::Pipe;
use std::collections::VecDeque;
///struct of important values in the state of the game
pub struct GameState {
    // frames since the game started
    pub ticks: u64,
    //velocity of the screen
    pub xvel: f64,
    //offset for tiling images
    pub stage_offset: f64,
    //bird position in image
    pub bird_pos: f64,
    //bird score
    pub score: u32,
    //bird object
    pub bird: bird::Bird,
    //deque of pipe objects
    pub pipe_deque: VecDeque<Pipe>,
    //is the game lost?
    pub lose: bool,
    pub paused: bool,
}

impl GameState {
    pub fn new(_xvel: f64, stage_offset: f64) -> GameState {

        let pipe_vec: Vec<Pipe> = [0.0, 1.0, 2.0, 3.0, 4.0]
            .iter()
            .map(|x| Pipe::new(850.0 + 170.0 * x, 0))
            .collect();

        GameState {
            ticks: 0,
            xvel: 1.8,
            stage_offset,
            bird_pos: 340.0,
            score: 0,
            bird: bird::Bird::new(),
            pipe_deque: VecDeque::from(pipe_vec),
            lose: false,
            paused: false,
        }
    }

    pub fn update(&mut self, button: input::Button) {

        if let input::Button::Keyboard(key) = button {
            if let input::Key::Escape = key {
                if self.lose {
                    self.reset();
                } else if self.paused == false {
                    self.paused = true;
                    self.pause();
                } else {
                    self.xvel = 1.8;
                    self.paused = false;
                }
            }
        }
    }

    pub fn lose(&mut self) {
        self.lose = true;
        self.xvel = 0.0;
        self.bird.collide = true;
    }

    pub fn reset(&mut self) {
        self.ticks = 0;
        self.lose = false;
        self.score = 0;
        self.xvel = 1.8;
        self.bird = bird::Bird::new();
        let pipe_vec: Vec<Pipe> = [0.0, 1.0, 2.0, 3.0, 4.0]
            .iter()
            .map(|x| Pipe::new(850.0 + 170.0 * x, 0))
            .collect();
        self.pipe_deque = VecDeque::from(pipe_vec);
    }

    pub fn pause(&mut self) {
        self.xvel = 0.0;
    }
}

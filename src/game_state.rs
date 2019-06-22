use crate::bird;
use crate::pipe;

///struct of important values in the state of the game
pub struct GameState {
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
    //pub pipe_deque: Vec<pipe::Pipe>,
    //is the game lost?
    pub lose: bool,

}

impl GameState {
    pub fn new(xvel: f64, stage_offset: f64) -> GameState {
        GameState {
            xvel: 1.8,
            stage_offset: stage_offset,
            bird_pos: 350.0,
            score: 0,
            bird: bird::Bird::new(),
            //pipe_deque: Vec::new(),
            lose: false,
        }
    }

    pub fn update(&mut self) {}


    pub fn lose(&mut self) {
        self.lose = true;
        self.xvel = 0.0;
        self.bird.collide = true;
    }

    pub fn reset(&mut self) {
        self.lose = false;
        self.score = 0;
        self.xvel = 1.8;
        self.bird = bird::Bird::new();
    }

}

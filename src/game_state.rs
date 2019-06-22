use crate::bird;
use crate::pipe;

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
    pub pipe_deque: Vec<pipe::Pipe>,
    //is the game lost?
    pub lose: bool,
    pub paused: bool,

}

impl GameState {
    pub fn new(xvel: f64, stage_offset: f64) -> GameState {
        GameState {
            ticks: 0,
            xvel: 1.8,
            stage_offset: stage_offset,
            bird_pos: 350.0,
            score: 0,
            bird: bird::Bird::new(),
            pipe_deque: Vec::new(),
            lose: false,
            paused: false,
        }
    }


    pub fn update(&mut self, button: input::Button) {
        if let input::Button::Keyboard(key) = button {
            match key {
                input::Key::Escape => {
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
                _ => {}
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
    }


    pub fn pause(&mut self) {
        self.xvel = 0.0;
    }
}

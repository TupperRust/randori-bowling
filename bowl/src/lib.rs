use std::fmt::{Display, Error, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub enum NextAction {
    NextDraw,
    NextFrame,
    Finish,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Pins {
    FinalPins(u32),
    InprogressPins(u32),
}

pub enum Draw {
    Open(u32),
    Split(u32),
    Spare,
    Strike,
    Fool,
}

// Frames

pub trait Frame {
    // fn new() -> Self;
    fn score(&self, Option<(usize, &Game)>) -> Pins;
    fn set_draw(&mut self, draw: Draw) -> NextAction;
    fn sum_n_draws(&self, n: usize) -> Pins;
}

/// Represent the firts ninths frames
pub struct RegularFrame {
    draws: Vec<Draw>,
}

pub struct TenthFrame {
    draws: Vec<Draw>,
}

impl RegularFrame {
    fn new() -> RegularFrame {
        RegularFrame {
            draws: Vec::with_capacity(2),
        }
    }
}

impl Frame for RegularFrame {
    fn set_draw(&mut self, draw: Draw) -> NextAction {
        let next = if self.draws.len() == 0 {
            match draw {
                Draw::Open(_)|Draw::Fool => NextAction::NextDraw,
                Draw::Strike => NextAction::NextFrame,
                _ => panic!("Shouldn't append!"),
            }
        } else {
            match draw {
                Draw::Strike => panic!("Shouldn't append!"),
                _ => NextAction::NextFrame,
            }
        };
        self.draws.push(draw);
        next
    }

    fn sum_n_draws(&self, n: usize) -> Pins {
        let mut score = 0;
        if let Some(draw) = self.draws.as_slice().first() {
            score += match draw {
                &Draw::Strike => 10,
                &Draw::Open(x)|&Draw::Split(x) => x,
                &Draw::Fool => 0,
                &Draw::Spare => panic!("Shouldn't append"),
            }
        }
        if n == 1 {
            Pins::FinalPins(score)
        } else {
            if let Some(draw) = self.draws.as_slice().get(1) {
                match draw {
                    &Draw::Strike|&Draw::Split(_) => panic!("Shouldn't append"),
                    &Draw::Spare => {
                        score = 10;
                    },
                    &Draw::Open(x) => {
                        score += x;
                    },
                    &Draw::Fool => (),
                }
                Pins::FinalPins(score)
            } else {
                Pins::InprogressPins(score)
            }
        }
    }

    fn score(&self, pos: Option<(usize, &Game)>) -> Pins {
        let mut score = 0;
        let mut in_progress = false;
        if let &Some(draw) = &self.draws.as_slice().get(0) {
            match draw {
                &Draw::Open(x)|&Draw::Split(x) => {
                    score += x;
                    in_progress = true;
                },
                &Draw::Fool => {
                    in_progress = true;
                },
                &Draw::Strike => {
                    score = 10;
                    in_progress = true;
                    if let Some((pos, game)) = pos {
                        if let Some(next_frame) = game.frames.as_slice().get(pos + 1) {
                            score += match next_frame.sum_n_draws(2) {
                                Pins::InprogressPins(x) => {
                                    in_progress = true;
                                    x
                                },
                                Pins::FinalPins(x) => {
                                    in_progress = false;
                                    x
                                },
                            }
                        }
                        if in_progress {
                            if let Some(next_frame) = game.frames.as_slice().get(pos + 2) {
                                score += match next_frame.sum_n_draws(1) {
                                    Pins::InprogressPins(x) => {
                                        in_progress = true;
                                        x
                                    },
                                    Pins::FinalPins(x) => {
                                        in_progress = false;
                                        x
                                    },
                                }
                            }
                        }
                    }
                },
                &Draw::Spare => panic!("Shouldn't append"),
            }
        }
        if let &Some(draw) = &self.draws.as_slice().get(1) {
            match draw {
                &Draw::Open(x) => {
                    score += x;
                    in_progress = false;
                },
                &Draw::Fool => {
                    in_progress = false;
                },
                &Draw::Spare => {
                    score = 10;
                    if let Some((pos, game)) = pos {
                        if let Some(next_frame) = game.frames.as_slice().get(pos + 1) {
                            score += match next_frame.sum_n_draws(1) {
                                Pins::InprogressPins(x) => {
                                    in_progress = true;
                                    x
                                },
                                Pins::FinalPins(x) => {
                                    in_progress = false;
                                    x
                                },
                            }
                        }
                    }
                },
                &Draw::Strike|&Draw::Split(_) => panic!("Shouldn't append"),
            }
        }
        if in_progress {
            Pins::InprogressPins(score)
        } else {
            Pins::FinalPins(score)
        }
    }
}

impl TenthFrame {
    fn new() -> TenthFrame {
        TenthFrame {
            draws: Vec::with_capacity(3),
        }
    }
}

impl Frame for TenthFrame {
    fn score(&self, _: Option<(usize, &Game)>) -> Pins {
        let mut in_progress = true;
        let mut score = 0;
        if let Some(draw) = self.draws.as_slice().first() {
            score += match draw {
                &Draw::Open(x)|&Draw::Split(x) => x,
                &Draw::Strike => 10,
                &Draw::Fool => 0,
                &Draw::Spare => panic!("Shouldn't append"),
            }
        }
        if let Some(draw) = self.draws.as_slice().get(1) {
            in_progress = false;
            match draw {
                &Draw::Open(x) => {
                    score += x;
                }
                &Draw::Strike => {
                    in_progress = true;
                    score += 10
                },
                &Draw::Spare => {
                    in_progress = true;
                    score = 10
                },
                &Draw::Fool => (),
                &Draw::Split(_) => panic!("Shouldn't append"),
            }
        }
        if let Some(draw) = self.draws.as_slice().get(2) {
            in_progress = false;
            score += match draw {
                &Draw::Open(x)|&Draw::Split(x) => x,
                &Draw::Strike => 10,
                &Draw::Fool => 0,
                &Draw::Spare => panic!("Shouldn't append"),
            }
        }
        if in_progress {
            Pins::InprogressPins(score)
        } else {
            Pins::FinalPins(score)
        }
    }

    fn set_draw(&mut self, draw: Draw) -> NextAction {
        let next = if self.draws.len() == 0 {
            match draw {
                Draw::Open(_)|Draw::Fool|Draw::Split(_) => NextAction::NextDraw,
                Draw::Strike => NextAction::NextDraw,
                Draw::Spare => panic!("Shouldn't append!"),
            }
        } else if self.draws.len() == 1 {
            match draw {
                Draw::Open(_)|Draw::Fool => NextAction::Finish,
                Draw::Strike|Draw::Spare => NextAction::NextDraw,
                Draw::Split(_) => panic!("Shouldn't append!"),
            }
        } else {
            NextAction::Finish
        };
        self.draws.push(draw);
        next
    }

    fn sum_n_draws(&self, n: usize) -> Pins {
        let mut score = 0;
        if let Some(draw) = self.draws.as_slice().first() {
            score += match draw {
                &Draw::Strike => 10,
                &Draw::Open(x)|&Draw::Split(x) => x,
                &Draw::Fool => 0,
                &Draw::Spare => panic!("Shouldn't append"),
            }
        }
        if n == 1 {
            Pins::FinalPins(score)
        } else {
            if let Some(draw) = self.draws.as_slice().get(1) {
                match draw {
                    &Draw::Strike => {
                        score += 10;
                    },
                    &Draw::Spare => {
                        score = 10;
                    },
                    &Draw::Open(x) => {
                        score += x;
                    },
                    &Draw::Fool => (),
                    &Draw::Split(_) => panic!("Shouldn't append"),
                }
                Pins::FinalPins(score)
            } else {
                Pins::InprogressPins(score)
            }
        }
    }
}

// Game

pub struct Game {
    frames: Vec<Box<Frame>>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            frames: Vec::with_capacity(10), 
        }
    }

    pub fn next_frame(&mut self) -> &mut Box<Frame> {
        match self.frames.len() {
            0 ... 8 => self.frames.push(Box::new(RegularFrame::new())),
            9 => self.frames.push(Box::new(TenthFrame::new())),
            _ => panic!("Shouldn't append"),
        }

        self.frames.as_mut_slice().last_mut().unwrap()
    }

    pub fn score(&self) -> Pins {
        let mut score = 0;
        let mut in_progress = self.frames.len() < 10;
        for (pos, frame) in self.frames.iter().enumerate() {
            score += match frame.score(Some((pos, &self))) {
                Pins::InprogressPins(x) => {
                    in_progress = true;
                    x
                },
                Pins::FinalPins(x) => x,
            }
        }
        if in_progress {
            Pins::InprogressPins(score)
        } else {
            Pins::FinalPins(score)
        }
    }
}

// Tools
impl From<Draw> for u32 {
    fn from(draw: Draw) -> u32 {
        match draw {
            Draw::Strike|Draw::Spare => 10,
            Draw::Open(x)|Draw::Split(x) => x,
            Draw::Fool => 0,
        }
    }
}
impl Display for Pins {
    fn fmt(&self, formatter: &mut Formatter) -> std::result::Result<(), Error> {
        let score = match self {
            &Pins::InprogressPins(x)|&Pins::FinalPins(x) => x,
        };
        write!(formatter, "{}", score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mono frame tests
    #[test]
    fn reg_open_score() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
        assert_eq!(frame.score(None), Pins::InprogressPins(4));
        assert_eq!(frame.set_draw(Draw::Open(2)), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::FinalPins(6));
    }

    #[test]
    fn first_draw_fooled_score() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Fool), NextAction::NextDraw);
        assert_eq!(frame.score(None), Pins::InprogressPins(0));
        assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::FinalPins(4));
    }

    #[test]
    fn second_draw_fooled_score() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
        assert_eq!(frame.score(None), Pins::InprogressPins(4));
        assert_eq!(frame.set_draw(Draw::Fool), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::FinalPins(4));
    }

    #[test]
    fn both_draw_fooled_score() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Fool), NextAction::NextDraw);
        assert_eq!(frame.score(None), Pins::InprogressPins(0));
        assert_eq!(frame.set_draw(Draw::Fool), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::FinalPins(0));
    }

    #[test]
    fn reg_spare_inprogress() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
        assert_eq!(frame.score(None), Pins::InprogressPins(4));
        assert_eq!(frame.set_draw(Draw::Spare), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::InprogressPins(10));
    }

    #[test]
    fn reg_strike_inprogress() {
        let mut frame = RegularFrame::new();
        assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
        assert_eq!(frame.score(None), Pins::InprogressPins(10));
    }

    // Game tests
    #[test]
    fn two_open_frames() {
        let mut game = Game::new();
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(5)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(5));
            assert_eq!(frame.set_draw(Draw::Open(2)), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::FinalPins(7));
        }
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(4));
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::FinalPins(8));
        }
        assert_eq!(game.score(), Pins::InprogressPins(15));
    }

    // Game tests
    #[test]
    fn two_spares_frames() {
        let mut game = Game::new();
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(5)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(5));
            assert_eq!(frame.set_draw(Draw::Spare), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
        }
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(4));
            assert_eq!(frame.set_draw(Draw::Spare), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
        }
        assert_eq!(game.score(), Pins::InprogressPins(24));
    }

    #[test]
    fn three_strikes_frames() {
        let mut game = Game::new();
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
        }
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
        }
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
        }
        assert_eq!(game.score(), Pins::InprogressPins(60));
    }

    #[test]
    fn max_score() {
        let mut game = Game::new();
        for _ in 0..9 {
            {
            let mut frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            }
        }
        assert_eq!(game.score(), Pins::InprogressPins(240));
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(20));
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::Finish);
            assert_eq!(frame.score(None), Pins::FinalPins(30));
        }
        assert_eq!(game.score(), Pins::FinalPins(300));
    }

    #[test]
    fn spare_at_end() {
        let mut game = Game::new();
        for _ in 0..9 {
            {
            let mut frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            }
        }
        assert_eq!(game.score(), Pins::InprogressPins(240));
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(4));
            assert_eq!(frame.set_draw(Draw::Spare), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::Finish);
            assert_eq!(frame.score(None), Pins::FinalPins(20));
        }
        assert_eq!(game.score(), Pins::FinalPins(274));
    }

    #[test]
    fn open_at_end() {
        let mut game = Game::new();
        for _ in 0..9 {
            {
            let mut frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            }
        }
        assert_eq!(game.score(), Pins::InprogressPins(240));
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(4));
            assert_eq!(frame.set_draw(Draw::Open(3)), NextAction::Finish);
            assert_eq!(frame.score(None), Pins::FinalPins(7));
        }
        assert_eq!(game.score(), Pins::FinalPins(258));
    }

    #[test]
    fn two_open_at_end() {
        let mut game = Game::new();
        for _ in 0..8 {
            {
            let mut frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Strike), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::InprogressPins(10));
            }
        }
        assert_eq!(game.score(), Pins::InprogressPins(210));
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(4)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(4));
            assert_eq!(frame.set_draw(Draw::Open(3)), NextAction::NextFrame);
            assert_eq!(frame.score(None), Pins::FinalPins(7));
        }
        assert_eq!(game.score(), Pins::InprogressPins(228));
        {
            let frame = game.next_frame();
            assert_eq!(frame.set_draw(Draw::Open(6)), NextAction::NextDraw);
            assert_eq!(frame.score(None), Pins::InprogressPins(6));
            assert_eq!(frame.set_draw(Draw::Open(2)), NextAction::Finish);
            assert_eq!(frame.score(None), Pins::FinalPins(8));
        }
        assert_eq!(game.score(), Pins::FinalPins(236));
    }

    #[test]
    fn play_ones() {
        let mut game = Game::new();
        'frames: loop {
            let frame = game.next_frame();
            loop {
                match frame.set_draw(Draw::Open(1)) {
                    NextAction::NextDraw => {},
                    NextAction::NextFrame => { continue 'frames; },
                    NextAction::Finish => { break 'frames; },
                }
            }
        }
        assert_eq!(game.score(), Pins::FinalPins(20));
    }
}

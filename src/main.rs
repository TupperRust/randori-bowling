extern crate bowl;

use std::io;


fn main() {
    let mut game = bowl::Game::new();
    let reader = io::stdin();
    'frames: loop {
        let frame = game.next_frame();
        loop {
            match frame.set_draw(stdin2draw(&reader)) {
                bowl::NextAction::NextDraw => {},
                bowl::NextAction::NextFrame => { continue 'frames; },
                bowl::NextAction::Finish => { break 'frames; },
            }
        }
    }
    println!("Your final score is: {:?}", game.score());
}


fn stdin2draw(reader: &io::Stdin) -> bowl::Draw {
    loop {
        let mut input = String::new();
        println!("Please enter your draw ('0' to '9', or '/' or 'X'");
        match reader.read_line(&mut input) {
            Ok(2) => match input.chars().next() {
                    Some('X') => {
                        return bowl::Draw::Strike;
                    },
                    Some('/') => {
                        return bowl::Draw::Spare;
                    },
                    Some(x @ '0' ... '9') => {
                        if let Some(digit) = x.to_digit(10) {
                            return bowl::Draw::Open(digit);
                        }
                    },
                    _ => {},
            },
            _ => {},
        };
        println!("	Error reading your input!");
    };
}

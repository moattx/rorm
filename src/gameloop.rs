use std::io;

//use rorm::handler::handle_keys;
//use rorm::game::Game;

use crate::game::Game;
use crate::handler::handle_keys;

use crossterm::event::{read, Event};

//pub fn run(stdout: &io::Stdout) -> std::io::Result<(u16)> {
pub fn run(stdout: &io::Stdout) -> std::io::Result<u16> {
    let mut game = Game::new(stdout);

    // display worm and set default cursor position
    game.display();
    loop {
        if !game.running {
            break;
        }

        // handle terminal events
        match read()? {
            // handle key input events
            Event::Key(keys) => handle_keys(keys, &mut game),
            // ignore the rest
            _ => (),
        }

        // clear and update
        game.update();
    }
    //println!("Well, you ran into something and the game is over.\n Your final score was {}", game.score);
    Ok(game.score)
}

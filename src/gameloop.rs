use crate::game::Game;
use crossterm::event::{poll, read, Event, KeyCode};
use std::io;
use std::time::Duration;

pub fn run(stdout: &io::Stdout) -> std::io::Result<u16> {
    let mut game = Game::new(stdout);

    // display worm and set default cursor position
    game.display()?;
    loop {
        if !game.running {
            break;
        }

        if poll(Duration::from_secs(1))? {
            // handle key input events
            if let Event::Key(keys) = read()? {
                match keys.code {
                    KeyCode::Char('q') => Game::quit(&mut game),
                    KeyCode::Char('l') => Game::go_right(&mut game),
                    KeyCode::Char('h') => Game::go_left(&mut game),
                    KeyCode::Char('k') => Game::go_up(&mut game),
                    KeyCode::Char('j') => Game::go_down(&mut game),
                    KeyCode::Char('L') => Game::upper_go_right(&mut game),
                    KeyCode::Char('H') => Game::upper_go_left(&mut game),
                    KeyCode::Char('K') => Game::upper_go_up(&mut game),
                    KeyCode::Char('J') => Game::upper_go_down(&mut game),
                    _ => continue,
                }
            }
        } else {
            Game::go_forward(&mut game);
        }

        // clear and update
        game.update();
    }
    Ok(game.score)
}

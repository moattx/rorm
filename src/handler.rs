use crate::game::Game;
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_keys(key: KeyEvent, game: &mut Game) {
    match key.code {
        KeyCode::Char('q') => Game::quit(game),
        KeyCode::Char('l') => Game::go_right(game),
        KeyCode::Char('h') => Game::go_left(game),
        KeyCode::Char('k') => Game::go_up(game),
        KeyCode::Char('j') => Game::go_down(game),
        KeyCode::Char('L') => Game::upper_go_right(game),
        KeyCode::Char('H') => Game::upper_go_left(game),
        KeyCode::Char('K') => Game::upper_go_up(game),
        KeyCode::Char('J') => Game::upper_go_down(game),
        _ => (),
    }
}

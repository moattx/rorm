use crossterm::event;
use std::io;

use crossterm::event::{
    poll, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    cursor::position,
    event::{
        read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture, Event, KeyCode,
    },
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::game::Game;

pub fn handle_keys(key: event::KeyEvent, game: &mut Game){
    match key.code {
        //Event::Key(KeyCode::Char('q').into()) => game::toggle_running(),
        KeyCode::Char('q') => Game::quit(game),
        KeyCode::Char('l') => Game::go_right(game),
        KeyCode::Char('h') => Game::go_left(game),
        KeyCode::Char('k') => Game::go_up(game),
        KeyCode::Char('j') => Game::go_down(game),
        _ => (),
    }
}

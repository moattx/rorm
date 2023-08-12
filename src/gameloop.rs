use std::io::{self, stdout, Write};


//use rorm::handler::handle_keys;
//use rorm::game::Game;

use crate::handler::handle_keys;
use crate::game::Game;

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
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crossterm::ExecutableCommand;
use std::time::Duration;


use std::{thread, time};


pub fn run(stdout: &io::Stdout) -> std::io::Result<()> {
    let mut game = Game::new(stdout);
    

    // display worm and set default cursor position
    game.display();
    loop {
            if !game.running{
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
    Ok(())
}


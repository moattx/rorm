use std::io;


use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode},
};

use rorm::gameloop;


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    //stdout.EnterAlternateScreen();
    execute!(stdout, EnterAlternateScreen)?;

    gameloop::run(&stdout)?;

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen);
    Ok(())
}


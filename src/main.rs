use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use rorm::gameloop;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let score = gameloop::run(&stdout)?;
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;

    println!(
        "\nWell, you ran into something and the game is over.\nYour final score was {}",
        score
    );
    Ok(())
}

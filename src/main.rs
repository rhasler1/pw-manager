use std::io;
use crossterm::{
    ExecutableCommand,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    event::DisableMouseCapture,
    execute,
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //terminal setup
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;



    terminal.clear()?;
    loop {
        //draw terminal

        //process next event

        break;
    }

    //terminal teardown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    println!("No Error!");
    Ok(())
}

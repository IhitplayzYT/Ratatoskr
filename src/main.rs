mod helper;
mod model;
mod ratatoskr;
mod db;
mod tree;
mod conversion;
mod run;

use helper::Helper;
use run::Run;
use std::{io, time::Instant};
use std::time::Duration;
 
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend, 
    Terminal,
};
 



use conversion::Conversion;

use crate::model::app::App::{App, Settings};
fn main() -> anyhow::Result<()> {
    let mut clargs = Helper::CLI::new();
    clargs.Parse_Args();
    if clargs.debug {
        println!("{clargs:?}");
    }
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(Conversion::update_exchange_rates())?;
    
   
    let mut app = App::new(if let Some(x) = clargs.url {x} else {format!("mysql://{}:{}@localhost:{}/mydb",clargs.user,clargs.password.unwrap_or("".to_string()),clargs.port)});
    if let Some(x) = clargs.settings{
        app.settings.load(x);
    }

//    for i in app.db.load_all_tags()?{
//        app.features.tags.insert(i);
//    }
//    for i in app.db.load_all_journal_task()?{
//        app.features.journals.insert(i);
//    }
//    for i in app.db.load_all_note_task()?{
//        app.features.notes.insert(i);
//    }
//    for i in app.db.load_all_todo_task()?{
//        app.features.todos.insert(i);
//    }
//    for i in app.db.get_events()?{
//        app.features.calendars.insert(i);
//    }
//    app.features.finance = app.db.load_ledger()?;   

    // TODO: Add pomodero qnd Calorie tracker


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    app.pomo_last_second = Instant::now();
    let tick_rate = Duration::from_millis(100);
    let ret = Run::run_app(&mut terminal,&mut app,tick_rate);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),LeaveAlternateScreen,DisableMouseCapture)?;
    terminal.show_cursor()?;
    if let Err(x) = ret{
        eprintln!("Err: {x}");
    } 
    Ok(())
}




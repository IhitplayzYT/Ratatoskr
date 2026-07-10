mod helper;
mod model;
mod ratatoskr;
mod db;
mod tree;
mod conversion;
mod run;
mod render;
mod input;

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
use std::io::stdout;






use conversion::Conversion;

use crate::model::app::App::{App};
fn main() -> anyhow::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal(); 
        original_hook(panic_info);
    }));

    let mut clargs = Helper::CLI::new();
    clargs.Parse_Args();
    if clargs.debug {
        println!("{clargs:?}");
    }
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(Conversion::update_exchange_rates())?;



    
    let db_url = format!("mysql://{}:{}@localhost:{}/{}",clargs.user,clargs.password.unwrap_or("".to_string()),clargs.port,clargs.database);
    let mut app = App::new(if let Some(x) = clargs.url {x} else {db_url});
    if let Some(x) = clargs.settings{
        app.settings.load(x);
    }
    app.db.init_dbs()?;

    for i in app.db.load_all_tags()?{
        app.features.tags.insert(i);
    }
    for i in app.db.load_all_journal_task()?{
        app.features.journals.insert(i);
    }
    for i in app.db.load_all_note_task()?{
        app.features.notes.insert(i);
    }
    for i in app.db.load_all_todo_task()?{
        app.features.todos.insert(i);
    }
    for i in app.db.get_events()?{
        app.features.calendars.insert(i);
    }
    app.features.finance = app.db.load_ledger()?;   


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


fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
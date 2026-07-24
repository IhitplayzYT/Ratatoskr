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
use once_cell::sync::Lazy;
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


use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};


    static running:Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(true)));
    static r: Lazy<Arc<AtomicBool>> = Lazy::new(|| running.clone());

pub fn add_sigint_handler(){

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).unwrap();
}

struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

use conversion::Conversion;

use crate::model::app::App::{App};
fn main() -> anyhow::Result<()> {
    add_sigint_handler();
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
    if clargs.clear{
        app.db.clear()?;
    }
    app.db.init_dbs()?;
    println!("Database Init Completed");

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
    println!("All data collected");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout,EnterAlternateScreen,EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut terminal = TerminalGuard {terminal};
    app.pomo_last_second = Instant::now();
    let tick_rate = Duration::from_millis(100);
    let ret = Run::run_app(&mut terminal.terminal,&mut app,tick_rate);
    disable_raw_mode()?;
    execute!(&mut &mut terminal.terminal.backend_mut(),LeaveAlternateScreen,DisableMouseCapture)?;
    terminal.terminal.show_cursor()?;
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
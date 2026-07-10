#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Run{
    use std::io;
use ratatui::{Terminal, layout::Alignment, style::Stylize, symbols};
use std::time::{Instant,Duration};

use crate::{input::{general_input::Input::{escalate_overdue_todos, handle_key, handle_mouse}, pomo_input::Input::tick_pomodoro}, model::{app::App::{App, Color_channel, EditorMode, EditorState, JournalFocus, JournalMode, Page, Pending, PomoFocus, SettingsFocus, Theme_comp, Vim_mode, parse_autosave_duration, parse_fmt_date}, journal::Journal::Journal_task, meta::Meta::{Mood, MyColor, Tag}}, render::{journal_render::Render::render_journal, pomo_render::Render::render_pomodoro, setting_render::Render::render_settings, todo_render::Render::render_todo}};
 
use crossterm::event::{
        self, Event, KeyEventKind,
    };
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};



pub fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;
 
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));
 
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(app, key),
                Event::Mouse(mouse) => handle_mouse(app, mouse),
                _ => {}
            }
        }
 
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        tick_pomodoro(app);
        escalate_overdue_todos(app);

        if app.is_quit {
            return Ok(());
        }
    }
}
 



// ---------------------------------------------------------------------------
// input handling
// ---------------------------------------------------------------------------



// ---------------------------------------------------------------------------
// rendering
// ---------------------------------------------------------------------------
fn ui(f: &mut Frame, app: &mut App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());
 
    render_tabs(f, root[0], app);
 
    match app.page {
        Page::Home => render_home(f, root[1], app),
        Page::Pomodoro => render_pomodoro(f, root[1], app),
        Page::Settings => render_settings(f, root[1], app),
        Page::Journal => render_journal(f, root[1], app),
        Page::Todo => render_todo(f,root[1],app),
        Page::Note => render_notes(f,root[1],app),
        _ => {
            app.last_editor_section = root[1];
            render_editor_canvas(f, root[1], &app.editor, app.page.title());
        }
    }
}






 
fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = Page::ALL.iter().map(|p| Line::from(p.title())).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Nav"))
        .select(app.page.idx())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(app.settings.theme.secondary.to_color()));
    f.render_widget(tabs, area);
}
 
fn render_home(f: &mut Frame, area: Rect, app: &App) {
    // TODO: pull summary stats via your backend, e.g.:
    // let todo_count = get_all_todos().len();
    let mode_str = match app.editor.mode {
        EditorMode::Normal => "Normal (arrows + mouse)",
        EditorMode::Vim => "Vim",
    };
    let text = vec![
        Line::from("Welcome back."),
        Line::from(""),
        Line::from(format!("Editor mode: {mode_str}  (F2 to toggle)")),
        Line::from("Tab / Shift+Tab to switch pages. q to quit from Home."),
    ];
    let block = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Home"));
    f.render_widget(block, area);
}
 

/// Shared editor canvas — reused on Todo / Note / Journal / Calendar / Finance.
fn render_editor_canvas(f: &mut Frame, area: Rect, editor: &EditorState, page_label: &str) {
    let mode_str = match (editor.mode, editor.vim.submode) {
        (EditorMode::Normal, _) => "NORMAL (plain)".to_string(),
        (EditorMode::Vim, Vim_mode::Normal) => "VIM: NORMAL".to_string(),
        (EditorMode::Vim, Vim_mode::Insert) => "VIM: INSERT".to_string(),
    };
    let dirty_str = if editor.dirty { "[+]" } else { "" };
    let title = format!("{page_label} — {mode_str} {dirty_str}");
    let block = Block::default().borders(Borders::ALL).title(title);
 
    // TODO: this is where real page content goes, e.g.:
    // let todos = get_all_todos();
    let content = editor.buffer.to_string();
    let paragraph = Paragraph::new(content).block(block);
    f.render_widget(paragraph, area);
 
    let show_cursor = editor.mode == EditorMode::Normal
        || (editor.mode == EditorMode::Vim && editor.vim.submode == Vim_mode::Insert)
        || (editor.mode == EditorMode::Vim && editor.vim.submode == Vim_mode::Normal);
    if show_cursor {
        f.set_cursor_position((area.x + 1 + editor.cursor_x as u16, area.y + 1 + editor.cursor_y as u16));
    }
}


}
#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Run{
    use std::io;
use ratatui::{Terminal, layout::Alignment, style::Stylize, symbols};
use std::time::{Instant,Duration};

use crate::model::app::App::{App, Color_channel, EditorMode, EditorState, Page, Pending, PomoFocus, SettingsFocus, Theme_comp, Vim_mode, parse_autosave_duration, parse_fmt_date};
 
use crossterm::event::{
        self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind,
    };
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;
use std::process::Command;



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

        if app.is_quit {
            return Ok(());
        }
    }
}
 
fn tick_pomodoro(app: &mut App) {
    let pomo = &mut app.features.pomodero;
    if !pomo.running || pomo.paused {
        // reset the reference point so we don't "catch up" a bunch of
        // seconds the moment it's resumed
        app.pomo_last_second = Instant::now();
        return;
    }

    if app.pomo_last_second.elapsed() >= Duration::from_secs(1) {
        app.pomo_last_second = Instant::now();
        if pomo.duration_left > 0 {
            pomo.duration_left -= 1;
        }
        if pomo.duration_left == 0 {
            pomo.running = false;
            pomo.paused = false;
            pomo_notif();
        }
    }
}

fn pomo_notif(){
Command::new("notify-send").arg("Pomodoro Timer complete").arg("Time Up!Take a Break!").status().unwrap();
}

fn handle_pomodoro_key(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        return;
    }

    let pomo = &mut app.features.pomodero;

    match key.code {
        KeyCode::Left => {
            pomo.focus = prev_pomo_focus(pomo.focus);
            return;
        }
        KeyCode::Right => {
            pomo.focus = next_pomo_focus(pomo.focus);
            return;
        }
        _ => {}
    }

    match pomo.focus {
        PomoFocus::Hour | PomoFocus::Minute | PomoFocus::Second => {
            if pomo.running {
                return;
            }
            match key.code {
                KeyCode::Up => bump_pomo_component(pomo, pomo.focus, 1),
                KeyCode::Down => bump_pomo_component(pomo, pomo.focus, -1),
                _ => {}
            }
        }
        PomoFocus::StartPause => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                if !pomo.running {
                    pomo.running = true;
                    pomo.paused = false;
                    app.pomo_last_second = Instant::now();
                } else if pomo.paused {
                    pomo.paused = false;
                    app.pomo_last_second = Instant::now();
                } else {
                    pomo.paused = true;
                }
            }
        }
        PomoFocus::Reset => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                pomo.duration_left = pomo.duration;
                app.pomo_last_second = Instant::now();
            }
        }
        PomoFocus::Cancel => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                pomo.running = false;
                pomo.paused = false;
                pomo.duration_left = pomo.duration;
            }
        }
    }
}

fn bump_pomo_component(pomo: &mut crate::model::pomodero::Pomodero::Pomodero, focus: PomoFocus, delta: i64) {
    let mut h = (pomo.duration / 3600) as i64;
    let mut m = ((pomo.duration % 3600) / 60) as i64;
    let mut s = (pomo.duration % 60) as i64;

    match focus {
        PomoFocus::Hour => h = (h + delta).rem_euclid(24),
        PomoFocus::Minute => m = (m + delta).rem_euclid(60),
        PomoFocus::Second => s = (s + delta).rem_euclid(60),
        _ => {}
    }

    pomo.duration = (h * 3600 + m * 60 + s) as usize;
    pomo.duration_left = pomo.duration;
}



fn next_pomo_focus(f: PomoFocus) -> PomoFocus {
    match f {
        PomoFocus::Hour => PomoFocus::Minute,
        PomoFocus::Minute => PomoFocus::Second,
        PomoFocus::Second => PomoFocus::StartPause,
        PomoFocus::StartPause => PomoFocus::Reset,
        PomoFocus::Reset => PomoFocus::Cancel,
        PomoFocus::Cancel => PomoFocus::Hour,
    }
}

fn prev_pomo_focus(f: PomoFocus) -> PomoFocus {
    match f {
        PomoFocus::Hour => PomoFocus::Cancel,
        PomoFocus::Minute => PomoFocus::Hour,
        PomoFocus::Second => PomoFocus::Minute,
        PomoFocus::StartPause => PomoFocus::Second,
        PomoFocus::Reset => PomoFocus::StartPause,
        PomoFocus::Cancel => PomoFocus::Reset,
    }
}


// ---------------------------------------------------------------------------
// input handling
// ---------------------------------------------------------------------------
fn handle_key(app: &mut App, key: KeyEvent) {
    // Global: F2 flips top-level EditorMode (Normal <-> Vim) from anywhere
    // inside an editor page. Safe to intercept unconditionally since it's a
    // function key, never something you'd type into a buffer.
    if key.code == KeyCode::F(2) {
        app.editor.mode = match app.editor.mode {
            EditorMode::Normal => EditorMode::Vim,
            EditorMode::Vim => EditorMode::Normal,
        };
        return;
    }
 
    // Tab / Shift+Tab cycles pages regardless of mode.
    match key.code {
        KeyCode::Tab => {
            app.page = Page::from_idx(app.page.idx() + 1);
            return;
        }
        KeyCode::BackTab => {
            app.page = Page::from_idx(app.page.idx() + 6);
            return;
        }
        _ => {}
    }
 
    match app.page {
        Page::Settings => handle_settings_key(app, key),
  //      Page::Todo => handle_todos_key(app,key),
        Page::Pomodoro => handle_pomodoro_key(app, key),
        Page::Home => {
            if key.code == KeyCode::Char('q') {
                app.is_quit = true;
            app.db.save_all(&app.features).unwrap();
            }
        }
        _ => handle_editor_key(app, key),
    }
}
 
fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    // Mouse is only wired up for the plain (non-vim) editor, per spec.
    if app.editor.mode != EditorMode::Normal {
        return;
    }
    if !matches!(
        app.page,
        Page::Journal | Page::Note | Page::Todo | Page::Calendar | Page::Finance
    ) {
        return;
    }
 
    let area = app.last_editor_section;
    match mouse.kind {
        MouseEventKind::Down(_) => {
            if mouse.row > area.y && mouse.column > area.x {
                let y = (mouse.row - area.y - 1) as usize;
                let x = (mouse.column - area.x - 1) as usize;
                app.editor.cursor_y = y.min(app.editor.buffer.len_lines().saturating_sub(1));
                app.editor.cursor_x = x;
                clamp_cursor(&mut app.editor);
            }
        }
        MouseEventKind::ScrollUp => {
            app.editor.cursor_y = app.editor.cursor_y.saturating_sub(1);
            clamp_cursor(&mut app.editor);
        }
        MouseEventKind::ScrollDown => {
            app.editor.cursor_y += 1;
            clamp_cursor(&mut app.editor);
        }
        _ => {}
    }
}

/// Only called while SettingsFocus::ColorPicker has focus. `[`/`]` cycle which
fn handle_color_picker_field_key(app: &mut App, key: KeyEvent) {
    let picker = &mut app.settings.color_picker;
    match key.code {
        KeyCode::Char('[') => {
            picker.component = picker.component.prev();
            picker.reload_from_theme(&app.settings.theme);
        }
        KeyCode::Char(']') => {
            picker.component = picker.component.next();
            picker.reload_from_theme(&app.settings.theme);
        }
        KeyCode::Left => picker.selected = picker.selected.prev(),
        KeyCode::Right | KeyCode::Tab => picker.selected = picker.selected.next(),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            let buf = picker.buffer_mut(picker.selected);
            if buf.len() < 3 {
                buf.push(c);
            }
            picker.commit_focused_channel(&mut app.settings.theme);
        }
        KeyCode::Backspace => {
            picker.buffer_mut(picker.selected).pop();
            picker.commit_focused_channel(&mut app.settings.theme);
        }
        _ => {}
    }
}

 
fn handle_settings_key(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Char('q') && app.settings.settings_ui.focus != SettingsFocus::Timezone
        && app.settings.settings_ui.focus != SettingsFocus::AutosaveFreq && app.settings.settings_ui.focus != SettingsFocus::Datefmt{
        app.is_quit = true;
        return;
    }
    if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL){
        app.settings.save().unwrap();
    }
    match key.code {
        KeyCode::Up => { cycle_settings_focus(app, false); return; }
        KeyCode::Down => { cycle_settings_focus(app, true); return; }
        _ => {}
    }

    match app.settings.settings_ui.focus {
        SettingsFocus::ColorPicker => handle_color_picker_field_key(app, key), // your existing Left/Right/digit logic, unchanged
        SettingsFocus::AutosaveEnabled => {
            if matches!(key.code, KeyCode::Left | KeyCode::Right | KeyCode::Enter) {
                app.settings.autosave = !app.settings.autosave;
            }
        }
        SettingsFocus::AutosaveFreq => {
            let ui = &mut app.settings.settings_ui;
            match key.code {
                KeyCode::Char(c) => ui.autosave_freq_input.push(c),
                KeyCode::Backspace => { ui.autosave_freq_input.pop(); }
                _ => {}
            }
            match parse_autosave_duration(&ui.autosave_freq_input) {
                Some(secs) => { ui.autosave_freq_valid = true; app.settings.autosave_freq = secs; }
                None => ui.autosave_freq_valid = false,
            }
        },
        SettingsFocus::Datefmt => {
          let ui = &mut app.settings.settings_ui;
          match key.code{
          KeyCode::Char(c) => ui.date_fmt.push(c),
          KeyCode::Backspace => {ui.date_fmt.pop();},
          _ => {}
          }
          match parse_fmt_date(&ui.date_fmt){
            Ok(x) => {ui.date_valid = true;app.settings.date_format = x;},
            _ => ui.date_valid = false,
          }

        }
        SettingsFocus::Currency => match key.code {
            KeyCode::Left => app.settings.currency = app.settings.currency.prev(),
            KeyCode::Right => app.settings.currency = app.settings.currency.next(),
            _ => {}
        },
        SettingsFocus::Timezone => {
            let ui = &mut app.settings.settings_ui;
            match key.code {
                KeyCode::Char(c) => ui.timezone_input.push(c),
                KeyCode::Backspace => { ui.timezone_input.pop(); }
                _ => {}
            }
            app.settings.timezone = ui.timezone_input.clone(); // TODO: validate/map real IANA zones later
        },
        SettingsFocus::AutocompleteEnabled =>{
            if matches!(key.code, KeyCode::Left | KeyCode::Right | KeyCode::Enter) {
                app.settings.autocomplete = !app.settings.autocomplete;
            }
        },
        SettingsFocus::ConfirmdeleteEnabled => {
            if matches!(key.code, KeyCode::Left | KeyCode::Right | KeyCode::Enter) {
                app.settings.confirm_delete = !app.settings.confirm_delete;
            }
        },
        SettingsFocus::VimmodeEnabled => {
            if matches!(key.code, KeyCode::Left | KeyCode::Right | KeyCode::Enter) {
                app.settings.vim_mode = !app.settings.vim_mode;
            }
        }
    }
}

fn cycle_settings_focus(app: &mut App, forward: bool) {
    let order = [SettingsFocus::ColorPicker, SettingsFocus::AutosaveEnabled, SettingsFocus::AutosaveFreq,SettingsFocus::AutocompleteEnabled,SettingsFocus::VimmodeEnabled,SettingsFocus::ConfirmdeleteEnabled,SettingsFocus::Datefmt, SettingsFocus::Currency, SettingsFocus::Timezone];
    let i = order.iter().position(|f| *f == app.settings.settings_ui.focus).unwrap();
    let next = if forward { (i + 1) % order.len() } else { (i + order.len() - 1) % order.len() };
    app.settings.settings_ui.focus = order[next];
}
 
/// EditorMode::Normal -> plain typing/arrow editor.
/// EditorMode::Vim    -> dispatches into the modal vim state machine.
fn handle_editor_key(app: &mut App, key: KeyEvent) {
    match app.editor.mode {
        EditorMode::Normal => handle_plain_editor_key(app, key),
        EditorMode::Vim => handle_vim_key(app, key),
    }
}
 
fn handle_plain_editor_key(app: &mut App, key: KeyEvent) {
    let editor = &mut app.editor;
    match key.code {
        KeyCode::Left => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Right => editor.cursor_x += 1,
        KeyCode::Up => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Down => editor.cursor_y += 1,
        KeyCode::Char(c) => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, c);
            editor.cursor_x += 1;
            editor.dirty = true;
        }
        KeyCode::Backspace => {
            let idx = cursor_char_idx(editor);
            if idx > 0 {
                editor.buffer.remove(idx - 1..idx);
                editor.cursor_x = editor.cursor_x.saturating_sub(1);
                editor.dirty = true;
            }
        }
        KeyCode::Enter => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, '\n');
            editor.cursor_y += 1;
            editor.cursor_x = 0;
            editor.dirty = true;
        },
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => editor.cursor_x = line_len_no_newline(&editor.buffer, editor.cursor_y),
        _ => {}
    }
    clamp_cursor(editor);
}
 
/// True modal vim emulation. Only handles the requested bind set:
/// hjkl, gg, G, yy, yiw, dd, diw, r, ~, O, p, plus i/Esc to flip submode.
fn handle_vim_key(app: &mut App, key: KeyEvent) {
    let editor = &mut app.editor;
    match editor.vim.submode {
        Vim_mode::Insert => handle_vim_insert_key(editor, key),
        Vim_mode::Normal => handle_vim_normal_key(editor, key),
    }
    clamp_cursor(editor);
}
 
fn handle_vim_insert_key(editor: &mut EditorState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            editor.vim.submode = Vim_mode::Normal;
            editor.cursor_x = editor.cursor_x.saturating_sub(1); // vim: cursor steps back on Esc
        }
        KeyCode::Char(c) => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, c);
            editor.cursor_x += 1;
            editor.dirty = true;
        }
        KeyCode::Backspace => {
            let idx = cursor_char_idx(editor);
            if idx > 0 {
                editor.buffer.remove(idx - 1..idx);
                editor.cursor_x = editor.cursor_x.saturating_sub(1);
                editor.dirty = true;
            }
        }
        KeyCode::Enter => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, '\n');
            editor.cursor_y += 1;
            editor.cursor_x = 0;
            editor.dirty = true;
        },
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => editor.cursor_x = line_len_no_newline(&editor.buffer, editor.cursor_y),
        _ => {}
    }
}
 
fn handle_vim_normal_key(editor: &mut EditorState, key: KeyEvent) {
    // Resolve any command still waiting on a second/third key first.
    match editor.vim.pending {
        Pending::G => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('g') {
                editor.cursor_y = 0;
                editor.cursor_x = 0;
            }
            return;
        }
        Pending::Y => {
            editor.vim.pending = Pending::None;
            match key.code {
                KeyCode::Char('y') => yank_line(editor),
                KeyCode::Char('i') => editor.vim.pending = Pending::YI,
                _ => {}
            }
            return;
        }
        Pending::YI => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('w') {
                yank_inner_word(editor);
            }
            return;
        }
        Pending::D => {
            editor.vim.pending = Pending::None;
            match key.code {
                KeyCode::Char('d') => delete_line(editor),
                KeyCode::Char('i') => editor.vim.pending = Pending::DI,
                _ => {}
            }
            return;
        }
        Pending::DI => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('w') {
                delete_inner_word(editor);
            }
            return;
        }
        Pending::R => {
            editor.vim.pending = Pending::None;
            if let KeyCode::Char(c) = key.code {
                replace_char_under_cursor(editor, c);
            }
            return;
        }
        Pending::None => {}
    }
 
    match key.code {
        KeyCode::Char('h') | KeyCode::Left => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Char('l') | KeyCode::Right => editor.cursor_x += 1,
        KeyCode::Char('k') | KeyCode::Up => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Char('j') | KeyCode::Down => editor.cursor_y += 1,
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => {
            let len = line_len_no_newline(&editor.buffer, editor.cursor_y);
            editor.cursor_x = len.saturating_sub(1); // vim End lands ON the last char, not past it
        },
        KeyCode::Char('g') => editor.vim.pending = Pending::G,
        KeyCode::Char('G') => {
            editor.cursor_y = editor.buffer.len_lines().saturating_sub(1);
            editor.cursor_x = 0;
        }
        KeyCode::Char('y') => editor.vim.pending = Pending::Y,
        KeyCode::Char('d') => editor.vim.pending = Pending::D,
        KeyCode::Char('r') => editor.vim.pending = Pending::R,
        KeyCode::Char('~') => toggle_case_under_cursor(editor),
        KeyCode::Char('O') => open_line_above(editor),
        KeyCode::Char('p') => paste_register(editor),
        KeyCode::Char('i') => editor.vim.submode = Vim_mode::Insert,
        _ => {}
    }
}
 
// --- vim command implementations -------------------------------------------
 
fn cursor_char_idx(editor: &EditorState) -> usize {
    let line_start = editor.buffer.line_to_char(editor.cursor_y.min(editor.buffer.len_lines() - 1));
    line_start + editor.cursor_x
}
 
fn line_len_no_newline(rope: &Rope, y: usize) -> usize {
    let line = rope.line(y);
    let len = line.len_chars();
    if y + 1 < rope.len_lines() {
        len.saturating_sub(1) // strip trailing '\n'
    } else {
        len
    }
}
 
fn clamp_cursor(editor: &mut EditorState) {
    let max_line = editor.buffer.len_lines().saturating_sub(1);
    editor.cursor_y = editor.cursor_y.min(max_line);
    let len = line_len_no_newline(&editor.buffer, editor.cursor_y);

    let caret_rests_past_end = editor.mode == EditorMode::Normal
        || (editor.mode == EditorMode::Vim && editor.vim.submode == Vim_mode::Insert);

    let max_x = if caret_rests_past_end { len } else { len.saturating_sub(1) };
    editor.cursor_x = editor.cursor_x.min(max_x);
}
 
fn current_line_string(editor: &EditorState) -> String {
    let len = line_len_no_newline(&editor.buffer, editor.cursor_y);
    let start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.slice(start..start + len).to_string()
}
 
/// Byte/char index of the word under `col` in `line` (ASCII-oriented; see
/// note below for multi-byte caveats).
fn word_bounds_at(line: &str, col: usize) -> (usize, usize) {
    for (start, word) in line.split_word_bound_indices() {
        let end = start + word.chars().count();
        if col >= start && col < end {
            return (start, end);
        }
    }
    (col, col + 1)
    // NOTE: split_word_bound_indices returns *byte* offsets; for pure-ASCII
    // buffers (typical for todo/note text) byte == char index, so this is
    // fine as-is. For full multi-byte correctness, map byte offsets to char
    // offsets via `line.char_indices()` before comparing against `col`.
}
 
fn yank_line(editor: &mut EditorState) {
    let mut line = current_line_string(editor);
    line.push('\n');
    editor.vim.register = line;
    editor.vim.register_line = true;
}
 
fn delete_line(editor: &mut EditorState) {
    yank_line(editor);
    let start = editor.buffer.line_to_char(editor.cursor_y);
    let end = editor.buffer.line_to_char((editor.cursor_y + 1).min(editor.buffer.len_lines()));
    editor.buffer.remove(start..end);
    if editor.buffer.len_lines() == 0 {
        editor.buffer = Rope::from_str("\n");
    }
    editor.dirty = true;
}
 
fn yank_inner_word(editor: &mut EditorState) {
    let line = current_line_string(editor);
    let (s, e) = word_bounds_at(&line, editor.cursor_x);
    editor.vim.register = line.get(s..e).unwrap_or("").to_string();
    editor.vim.register_line = false;
}
 
fn delete_inner_word(editor: &mut EditorState) {
    let line = current_line_string(editor);
    let (s, e) = word_bounds_at(&line, editor.cursor_x);
    editor.vim.register = line.get(s..e).unwrap_or("").to_string();
    editor.vim.register_line = false;
 
    let line_start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.remove(line_start + s..line_start + e);
    editor.cursor_x = s;
    editor.dirty = true;
}
 
fn replace_char_under_cursor(editor: &mut EditorState, c: char) {
    let idx = cursor_char_idx(editor);
    if idx < editor.buffer.len_chars() {
        editor.buffer.remove(idx..idx + 1);
        editor.buffer.insert_char(idx, c);
        editor.dirty = true;
    }
}
 
fn toggle_case_under_cursor(editor: &mut EditorState) {
    let idx = cursor_char_idx(editor);
    if idx < editor.buffer.len_chars() {
        let ch = editor.buffer.char(idx);
        let toggled = if ch.is_uppercase() {
            ch.to_lowercase().next().unwrap_or(ch)
        } else {
            ch.to_uppercase().next().unwrap_or(ch)
        };
        editor.buffer.remove(idx..idx + 1);
        editor.buffer.insert_char(idx, toggled);
        editor.cursor_x += 1;
        editor.dirty = true;
    }
}
 
fn open_line_above(editor: &mut EditorState) {
    let line_start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.insert_char(line_start, '\n');
    editor.cursor_x = 0;
    editor.vim.submode = Vim_mode::Insert;
    editor.dirty = true;
    // cursor_y stays put: the new blank line is now *at* cursor_y, and the
    // old content shifted down to cursor_y + 1, matching vim's `O`.
}
 
fn paste_register(editor: &mut EditorState) {
    if editor.vim.register.is_empty() {
        return;
    }
    if editor.vim.register_line {
        let next_line_start = editor
            .buffer
            .line_to_char((editor.cursor_y + 1).min(editor.buffer.len_lines()));
        editor.buffer.insert(next_line_start, &editor.vim.register);
        editor.cursor_y += 1;
        editor.cursor_x = 0;
    } else {
        let idx = cursor_char_idx(editor) + 1; // paste *after* cursor, like vim `p`
        let idx = idx.min(editor.buffer.len_chars());
        editor.buffer.insert(idx, &editor.vim.register);
        editor.cursor_x += 1;
    }
    editor.dirty = true;
}
 
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
        _ => {
            app.last_editor_section = root[1];
            render_editor_canvas(f, root[1], &app.editor, app.page.title());
        }
    }
}

fn render_pomodoro(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(9), Constraint::Length(3)])
        .split(area);

    if pomo.running {
        render_pomo_running(f, rows[0], app);
    } else {
        render_pomo_wheels(f, rows[0], app);
    }

    render_pomo_buttons(f, rows[1], app);
}


fn render_pomo_wheels(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.primary.to_color();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Span::styled(
            format!(" Pomodoro — {} ", pomo.format_time_left()),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner);

    let h = (pomo.duration_left / 3600) as i64;
    let m = ((pomo.duration_left % 3600) / 60) as i64;
    let s = (pomo.duration_left % 60) as i64;

    render_pomo_wheel(f, cols[0], "HOUR", h, 24, pomo.focus == PomoFocus::Hour, color);
    render_pomo_wheel(f, cols[1], "MIN", m, 60, pomo.focus == PomoFocus::Minute, color);
    render_pomo_wheel(f, cols[2], "SEC", s, 60, pomo.focus == PomoFocus::Second, color);
}

fn render_pomo_running(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.primary.to_color();
    let status = if pomo.paused { "PAUSED" } else { "RUNNING" };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Span::styled(
            format!(" Pomodoro — {status} "),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let text = pomo.format_time_left();

    // figlet glyphs are 5 rows tall, 7 cols wide per char (6 + 1 spacing)
    let big_height = 5u16;
    let big_width = (text.chars().count() as u16) * 7;

    let v_pad = inner.height.saturating_sub(big_height) / 2;
    let h_pad = inner.width.saturating_sub(big_width) / 2;

    let centered = Rect {
        x: inner.x + h_pad,
        y: inner.y + v_pad,
        width: big_width.min(inner.width),
        height: big_height.min(inner.height),
    };

    let dim = pomo.paused; // dim the digits while paused, still centered
    render_big_time(f, centered, &text, color, !dim);
}

fn render_pomo_wheel(f: &mut Frame, area: Rect, label: &str, value: i64, modulus: i64, focused: bool, color: Color) {
    let prev = (value - 1).rem_euclid(modulus);
    let next = (value + 1).rem_euclid(modulus);

    let dim = Style::default().add_modifier(Modifier::DIM);
    let cur_style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    };

    let lines = vec![
        Line::from(Span::styled(format!("{next:02}"), dim)).alignment(Alignment::Center),
        Line::from(Span::styled(format!("{value:02}"), cur_style)).alignment(Alignment::Center),
        Line::from(Span::styled(format!("{prev:02}"), dim)).alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(label, Style::default().fg(color))).alignment(Alignment::Center),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

fn render_pomo_buttons(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.secondary.to_color();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

    let start_pause_label = if !pomo.running {
        "Start"
    } else if pomo.paused {
        "Resume"
    } else {
        "Pause"
    };

    render_pomo_button(f, cols[0], start_pause_label, pomo.focus == PomoFocus::StartPause, color);
    render_pomo_button(f, cols[1], "Reset", pomo.focus == PomoFocus::Reset, color);
    render_pomo_button(f, cols[2], "Cancel", pomo.focus == PomoFocus::Cancel, color);
}

/// 5-row block-digit font for 0-9 and ':'.
fn big_digit_rows(c: char) -> [&'static str; 5] {
    match c {
        '0' => ["██████", "██  ██", "██  ██", "██  ██", "██████"],
        '1' => ["  ██  ", "  ██  ", "  ██  ", "  ██  ", "  ██  "],
        '2' => ["██████", "    ██", "██████", "██    ", "██████"],
        '3' => ["██████", "    ██", "██████", "    ██", "██████"],
        '4' => ["██  ██", "██  ██", "██████", "    ██", "    ██"],
        '5' => ["██████", "██    ", "██████", "    ██", "██████"],
        '6' => ["██████", "██    ", "██████", "██  ██", "██████"],
        '7' => ["██████", "    ██", "    ██", "    ██", "    ██"],
        '8' => ["██████", "██  ██", "██████", "██  ██", "██████"],
        '9' => ["██████", "██  ██", "██████", "    ██", "██████"],
        ':' => ["      ", "  ██  ", "      ", "  ██  ", "      "],
        _   => ["      ", "      ", "      ", "      ", "      "],
    }
}

/// Renders `text` (digits/colons only) as figlet-style block art.
fn render_big_time(f: &mut Frame, area: Rect, text: &str, color: Color, bold: bool) {
    let mut row_strs: [String; 5] = Default::default();
    for c in text.chars() {
        let glyph = big_digit_rows(c);
        for i in 0..5 {
            row_strs[i].push_str(glyph[i]);
            row_strs[i].push(' '); // spacing between characters
        }
    }

    let style = if bold {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    };

    let lines: Vec<Line> = row_strs
        .iter()
        .map(|r| Line::from(Span::styled(r.clone(), style)).alignment(Alignment::Center))
        .collect();

    f.render_widget(Paragraph::new(lines), area);
}

fn render_pomo_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color)
    };
    let block = Block::default().borders(Borders::ALL).border_set(symbols::border::DOUBLE).fg(color);
    let text = Paragraph::new(Line::from(Span::styled(format!(" {label} "), style)))
        .alignment(Alignment::Center)
        .block(block);
    f.render_widget(text, area);
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
 
fn render_settings(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(10)])
        .split(area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(rows[0]);
    render_theme_component_list(f, cols[0], app);
    render_color_picker(f, cols[1], app);

    render_general_settings_fields(f, rows[1], app); // new
}

fn render_general_settings_fields(f: &mut Frame, area: Rect, app: &App) {
    let s = &app.settings;
    let ui = &s.settings_ui;
    let hl = |focused: bool| if focused {
        Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED).fg(app.settings.theme.fontfg.to_color()).bg(app.settings.theme.fontbg.to_color())
    } else { Style::default() };

let lines = vec![
    enabled_line("Autosave", s.autosave, ui.focus == SettingsFocus::AutosaveEnabled,&app),
    Line::from(Span::styled(
        format!("Autosave every: {} ({}s){}", ui.autosave_freq_input, s.autosave_freq,
            if ui.autosave_freq_valid { "" } else { " — unparsed" }),
        hl(ui.focus == SettingsFocus::AutosaveFreq),
    )),
    enabled_line("Autocomplete", s.autocomplete, ui.focus == SettingsFocus::AutocompleteEnabled,&app),
    enabled_line("VimMode", s.vim_mode, ui.focus == SettingsFocus::VimmodeEnabled,&app),
    enabled_line("ConfirmDelete", s.confirm_delete, ui.focus == SettingsFocus::ConfirmdeleteEnabled,&app),
    Line::from(Span::styled(
        format!("Date Format: {} ({}){}", ui.date_fmt, s.date_format,
            if ui.date_valid { "" } else { " — unparsed" }),
        hl(ui.focus == SettingsFocus::Datefmt),
    )),
    Line::from(Span::styled(
        format!("Currency: {}", s.currency.title()),
        hl(ui.focus == SettingsFocus::Currency),
    )),
    Line::from(Span::styled(
        format!("Timezone: {}", ui.timezone_input),
        hl(ui.focus == SettingsFocus::Timezone),
    )),
];

    let block = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("General (↑/↓ field, ←/→ or type, Ctrl+S save)"));
    f.render_widget(block, area);
}

fn enabled_line(label: &str, enabled: bool, focused: bool,app:&App) -> Line<'static> {
    let hl = |focused: bool| if focused {
        Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED).fg(app.settings.theme.fontfg.to_color()).bg(app.settings.theme.fontbg.to_color())
    } else { Style::default() };

    let base = hl(focused);
    let status_style = base
        .fg(if enabled { Color::Green } else { Color::Red })
        .add_modifier(Modifier::BOLD);

    Line::from(vec![
        Span::styled(format!("{label}: "), base),
        Span::styled(if enabled { "Enabled" } else { "Disabled" }, status_style),
    ])
}


fn render_theme_component_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = Theme_comp::ALL
        .iter()
        .map(|c| {
            let color = c.get(&app.settings.theme).to_color();
            let selected = *c == app.settings.color_picker.component;
            let style = if selected {
                Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(color)
            };
            ListItem::new(Line::from(Span::styled(format!(" {} ", c.title()), style)))
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL).border_style(Style::default().fg(app.settings.theme.primary.to_color()))
            .title(Span::styled("Theme components (↑/↓)",Style::default().fg(app.settings.theme.secondary.to_color()))).title_alignment(Alignment::Center)
    );
    f.render_widget(list, area);
}
 
/// Live text-field RGB picker for whichever theme component is selected.
fn render_color_picker(f: &mut Frame, area: Rect, app: &App) {
    let picker = &app.settings.color_picker;
    let color = picker.component.get(&app.settings.theme);
 
    let block = Block::default()
        .borders(Borders::ALL).border_style(Style::default().fg(color.to_color()))
        .title(Span::styled(format!(" {} — Use '[' and ']' to change channels, type for editing", picker.component.title()),Style::default().fg(app.settings.theme.primary.to_color())));
    f.render_widget(block, area);
 
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
 
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(inner);
    render_channel_field(f,&app, rows[0], "R", &picker.buffers[0], picker.selected == Color_channel::R, Color::Red);
    render_channel_field(f,&app, rows[1], "G", &picker.buffers[1], picker.selected == Color_channel::G, Color::Green);
    render_channel_field(f,&app, rows[2], "B", &picker.buffers[2], picker.selected == Color_channel::B, Color::Blue);
 
    let (r, g, b) = color.get_rgb();
    let hex = Paragraph::new(format!("hex: #{r:02X}{g:02X}{b:02X}")).fg(color.to_color());
    f.render_widget(hex, rows[3]);
 
    let swatch = Block::default().style(Style::default().bg(color.to_color()));
    f.render_widget(swatch, rows[4]);
}
 
fn render_channel_field(f: &mut Frame,app:&App, area: Rect, label: &str, buf: &str, focused: bool, color: Color) {
    let style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color)
    };
    let line = Line::from(vec![
        Span::styled(format!("{label}: "), style),
        Span::styled(format!("[{buf:<3}]"), style),
        if focused { Span::styled(" ←type digits",Style::default().fg(app.settings.theme.accent.to_color())) } else { Span::raw("") },
    ]);
    f.render_widget(Paragraph::new(line), area);
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
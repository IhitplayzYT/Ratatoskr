#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Run{
    use std::io;
use ratatui::Terminal;
use std::time::{Instant,Duration};

use crate::model::{app::App::{App, Color_channel, EditorMode, EditorState, Page, Pending, Theme_comp, Vim_mode}};
 
use crossterm::{
    event::{
        self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind,
    },
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
 
        if app.is_quit {
            return Ok(());
        }
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
        Page::Home => {
            if key.code == KeyCode::Char('q') {
                app.is_quit = true;
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
 
fn handle_settings_key(app: &mut App, key: KeyEvent) {
    let picker = &mut app.settings.color_picker;
    match key.code {
        KeyCode::Up => {
            picker.component = picker.component.prev();
            picker.reload_from_theme(&app.settings.theme);
        }
        KeyCode::Down => {
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
        KeyCode::Char('q') => app.is_quit = true,
        _ => {}
    }
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
        }
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
        }
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
        KeyCode::Char('h') => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Char('l') => editor.cursor_x += 1,
        KeyCode::Char('k') => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Char('j') => editor.cursor_y += 1,
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
    editor.cursor_x = editor.cursor_x.min(len.saturating_sub(if len == 0 { 0 } else { 1 }).max(0));
    if len == 0 {
        editor.cursor_x = 0;
    }
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
        Page::Settings => render_settings(f, root[1], app),
        _ => {
            app.last_editor_section = root[1];
            render_editor_canvas(f, root[1], &app.editor, app.page.title());
        }
    }
}
 
fn render_tabs(f: &mut Frame, area: Rect, app: &App) {
    let titles: Vec<Line> = Page::ALL.iter().map(|p| Line::from(p.title())).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("nav"))
        .select(app.page.idx())
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow));
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
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);
 
    render_theme_component_list(f, chunks[0], app);
    render_color_picker(f, chunks[1], app);
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
            .borders(Borders::ALL)
            .title("Theme components (↑/↓)"),
    );
    f.render_widget(list, area);
}
 
/// Live text-field RGB picker for whichever theme component is selected.
fn render_color_picker(f: &mut Frame, area: Rect, app: &App) {
    let picker = &app.settings.color_picker;
    let color = picker.component.get(&app.settings.theme);
 
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("{} — Tab/←/→ field, type digits, Backspace to edit", picker.component.title()));
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
 
    render_channel_field(f, rows[0], "R", &picker.buffers[0], picker.selected == Color_channel::R, Color::Red);
    render_channel_field(f, rows[1], "G", &picker.buffers[1], picker.selected == Color_channel::G, Color::Green);
    render_channel_field(f, rows[2], "B", &picker.buffers[2], picker.selected == Color_channel::B, Color::Blue);
 
    let (r, g, b) = color.get_rgb();
    let hex = Paragraph::new(format!("hex: #{r:02X}{g:02X}{b:02X}"));
    f.render_widget(hex, rows[3]);
 
    let swatch = Block::default().style(Style::default().bg(color.to_color()));
    f.render_widget(swatch, rows[4]);
}
 
fn render_channel_field(f: &mut Frame, area: Rect, label: &str, buf: &str, focused: bool, color: Color) {
    let style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color)
    };
    let line = Line::from(vec![
        Span::styled(format!("{label}: "), style),
        Span::styled(format!("[{buf:<3}]"), style),
        if focused { Span::raw(" ←type digits") } else { Span::raw("") },
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
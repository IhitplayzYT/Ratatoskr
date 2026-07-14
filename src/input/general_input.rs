pub mod Input{
    use chrono::NaiveDateTime;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use ropey::Rope;

use crate::{input::{calender_input::Input::handle_calendar, editor::Input::handle_editor_key, journal_input::Input::handle_journal_key, ledger_input::Input::handle_ledger, note_input::Input::handle_notes_key, pomo_input::Input::handle_pomodoro_key, setting_input::Input::handle_settings_key, todo_input::Input::handle_todo_key}, model::{app::App::{App, EditorMode, EditorState, Page, Vim_mode}, meta::Meta::{MyColor, Priority, Tag}}};



pub fn cursor_char_idx(editor: &EditorState) -> usize {
    let line_start = editor.buffer.line_to_char(editor.cursor_y.min(editor.buffer.len_lines() - 1));
    line_start + editor.cursor_x
}
 
pub fn line_len_no_newline(rope: &Rope, y: usize) -> usize {
    let line = rope.line(y);
    let len = line.len_chars();
    if y + 1 < rope.len_lines() {
        len.saturating_sub(1) // strip trailing '\n'
    } else {
        len
    }
}
 
pub fn clamp_cursor(editor: &mut EditorState) {
    let max_line = editor.buffer.len_lines().saturating_sub(1);
    editor.cursor_y = editor.cursor_y.min(max_line);
    let len = line_len_no_newline(&editor.buffer, editor.cursor_y);

    let caret_rests_past_end = editor.mode == EditorMode::Normal
        || (editor.mode == EditorMode::Vim && editor.vim.submode == Vim_mode::Insert);

    let max_x = if caret_rests_past_end { len } else { len.saturating_sub(1) };
    editor.cursor_x = editor.cursor_x.min(max_x);
}

pub fn add_tag_from_inputs(app: &mut App) {
    let ui = &mut app.journal_ui;
    if ui.tag_name_input.is_empty() || ui.tag_hex_input.len() != 6 { return; }
    if let Some(color) = parse_hex_color(&ui.tag_hex_input) {
        ui.editing.add_tag(Tag::new(ui.tag_name_input.clone(), Some(color)));
        ui.tag_name_input.clear();
        ui.tag_hex_input.clear();
    }
}

pub fn parse_hex_color(hex: &str) -> Option<MyColor> {
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(MyColor::RGB(r, g, b))
}
pub fn handle_key(app: &mut App, key: KeyEvent) {
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
 

    if (key.code == KeyCode::Left && key.modifiers.contains(KeyModifiers::SHIFT)) || key.code == KeyCode::BackTab {
        app.page = Page::from_idx(app.page.idx() + 7);
        return;
    } else if (key.code == KeyCode::Right && key.modifiers.contains(KeyModifiers::SHIFT)) || key.code == KeyCode::Tab {
        app.page = Page::from_idx(app.page.idx() + 1);
        return;
    }else if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::ALT){
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }
 
    match app.page {
        Page::Settings => handle_settings_key(app, key),
        Page::Todo => handle_todo_key(app,key),    
        Page::Note => handle_notes_key(app,key),    
        Page::Journal => handle_journal_key(app, key),
        Page::Pomodoro => handle_pomodoro_key(app, key),
        Page::Finance => handle_ledger(app,key),
        Page::Calendar => handle_calendar(app,key),
        Page::Home => {
            if key.code == KeyCode::Char('q') {
                app.is_quit = true;
            app.db.save_all(&app.features).unwrap();
            }
        }
        _ => handle_editor_key(app, key),
    }
}
 
pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
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
pub fn handle_color_picker_field_key(app: &mut App, key: KeyEvent) {
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

pub fn escalate_overdue_todos(app: &mut App) {
    let now = chrono::Utc::now();
    for t in app.todo_ui.list.iter_mut().chain(std::iter::once(&mut app.todo_ui.editing)) {
        if !t.status {
            if let Some(due) = t.due_date {
                if due <= now && t.priority != Priority::Critical {
                    t.priority = Priority::Critical;
                }
            }
        }
    }
}

pub fn parse_due_date_input(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    if s.is_empty() { return None; }
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").ok()?;
    Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc))
}



}
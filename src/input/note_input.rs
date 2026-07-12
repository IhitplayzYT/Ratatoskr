pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ropey::Rope;

use crate::{input::{editor::Input::handle_editor_key, general_input::Input::{add_tag_from_inputs, parse_hex_color}}, model::{app::App::{App, NoteFilter, NoteFocus, NoteMode}, meta::Meta::Tag, notes::Note::Note_task}};

pub fn handle_notes_key(app: &mut App, key: KeyEvent) {
    if app.note_ui.mode == NoteMode::Load {
        handle_note_list_key(app, key);
        return;
    }
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.note_ui.focus = app.note_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.note_ui.focus = app.note_ui.focus.prev();
        return;
    }
    if app.note_ui.focus != NoteFocus::Content {
        match key.code {
            KeyCode::Up => { app.note_ui.focus = app.note_ui.focus.prev(); return; }
            KeyCode::Down => { app.note_ui.focus = app.note_ui.focus.next(); return; }
            _ => {}
        }
    }
    let text_field = matches!(app.note_ui.focus,
        NoteFocus::Title | NoteFocus::Content | NoteFocus::TagName
        | NoteFocus::TagHex | NoteFocus::Member | NoteFocus::Topic);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }
    if key.code == KeyCode::Esc {
        app.note_ui.mode = NoteMode::Load;
        return;
    }
    match app.note_ui.focus {
        NoteFocus::Title => match key.code {
            KeyCode::Char(c) => { app.note_ui.editing.title.push(c); touch_note(app); }
            KeyCode::Backspace => { app.note_ui.editing.title.pop(); touch_note(app); }
            _ => {}
        },
        NoteFocus::Member => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => {
                app.note_ui.editing.member.get_or_insert_with(String::new).push(c);
                touch_note(app);
            }
            KeyCode::Backspace => {
                if let Some(m) = app.note_ui.editing.member.as_mut() { m.pop(); }
                touch_note(app);
            }
            _ => {}
        },
        NoteFocus::Topic => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => {
                app.note_ui.editing.topic.get_or_insert_with(String::new).push(c);
                touch_note(app);
            }
            KeyCode::Backspace => {
                if let Some(t) = app.note_ui.editing.topic.as_mut() { t.pop(); }
                touch_note(app);
            }
            _ => {}
        },
        NoteFocus::Favourite => match key.code {
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Left | KeyCode::Right => {
                app.note_ui.editing.favorite ^= true;
                touch_note(app);
            }
            _ => {}
        },
        NoteFocus::Pinned => match key.code {
            KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Left | KeyCode::Right => {
                app.note_ui.editing.pinned ^= true;
                touch_note(app);
            }
            _ => {}
        },
        NoteFocus::Content => handle_editor_key(app, key),
        NoteFocus::TagName => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.note_ui.tag_name_input.push(c), // fixed
            KeyCode::Backspace => { app.note_ui.tag_name_input.pop(); }                    // fixed
            _ => {}
        },
        NoteFocus::TagHex => match key.code {
            KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                if app.note_ui.tag_hex_input.len() < 6 { app.note_ui.tag_hex_input.push(c); }
            }
            KeyCode::Backspace => { app.note_ui.tag_hex_input.pop(); }
            _ => {}
        },
        NoteFocus::TagAdd => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                let ui = &mut app.note_ui;
                if !ui.tag_name_input.is_empty() && ui.tag_hex_input.len() == 6 {
                    if let Some(color) = parse_hex_color(&ui.tag_hex_input) {
                        ui.editing.add_tag(Tag::new(ui.tag_name_input.clone(), Some(color)));
                        ui.tag_name_input.clear();
                        ui.tag_hex_input.clear();
                    }
                }
            }
        }
        NoteFocus::Save => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { save_note(app); app.note_ui.mode = NoteMode::Load; app.note_ui.list = app.db.load_all_note_task().unwrap_or_default(); }
        }
        NoteFocus::New => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { new_note(app); }
        }
        NoteFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { open_note_load_list(app); }
        }
    }
}

/// pinned/favourite notes float to the top (original relative order kept
/// within each group), plain notes after — mirrors todo's completed-last split.
pub fn note_display_order(list: &[Note_task], filter: NoteFilter) -> Vec<usize> {
    let visible: Vec<usize> = (0..list.len()).filter(|&i| match filter {
        NoteFilter::All => true,
        NoteFilter::Pinned => list[i].pinned,
        NoteFilter::Favourite => list[i].favorite,
    }).collect();

    let mut special: Vec<usize> = visible.iter().copied().filter(|&i| list[i].pinned || list[i].favorite).collect();
    let mut normal: Vec<usize> = visible.iter().copied().filter(|&i| !list[i].pinned && !list[i].favorite).collect();
    special.append(&mut normal);
    special
}

fn handle_note_list_key(app: &mut App, key: KeyEvent) {
    if app.note_ui.list.is_empty() {
        app.note_ui.list = app.db.load_all_note_task().unwrap_or_default();
    }

    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }
    if key.code == KeyCode::Char('n') {
        new_note(app);
        app.note_ui.mode = NoteMode::Edit;
        return;
    }
    if key.code == KeyCode::Char('f') {
        app.note_ui.filter = app.note_ui.filter.next();
        app.note_ui.list_selected = 0;
        return;
    }

    let order = note_display_order(&app.note_ui.list, app.note_ui.filter);
    if order.is_empty() { return; }

    match key.code {
        KeyCode::Up => app.note_ui.list_selected = app.note_ui.list_selected.saturating_sub(1),
        KeyCode::Down => app.note_ui.list_selected = (app.note_ui.list_selected + 1).min(order.len() - 1),
        KeyCode::Enter => {
            let real_idx = order[app.note_ui.list_selected.min(order.len() - 1)];
            let task = app.note_ui.list[real_idx].clone();
            app.editor.buffer = Rope::from_str(&format!("{}\n", task.content));
            app.editor.cursor_x = 0;
            app.editor.cursor_y = 0;
            app.note_ui.editing = task;
            app.note_ui.mode = NoteMode::Edit;
            app.note_ui.focus = NoteFocus::Title;
        }
        _ => {}
    }
}

fn touch_note(app: &mut App) {
    app.note_ui.editing.updated_at = chrono::Utc::now();
}

fn save_note(app: &mut App) {
    app.note_ui.editing.content = app.editor.buffer.to_string();
    app.note_ui.editing.updated_at = chrono::Utc::now();
    app.db.save_note_task(&app.note_ui.editing).unwrap();
    app.features.notes.insert(app.note_ui.editing.clone());
}

fn new_note(app: &mut App) {
    app.note_ui.editing = Note_task::new(None, None, false, false, None, None, None);
    app.note_ui.tag_name_input.clear();
    app.note_ui.tag_hex_input.clear();
    app.editor.buffer = Rope::from_str("\n");
    app.editor.cursor_x = 0;
    app.editor.cursor_y = 0;
    app.note_ui.focus = NoteFocus::Title;
}

fn open_note_load_list(app: &mut App) {
    app.note_ui.list = app.db.load_all_note_task().unwrap_or_default();
    app.note_ui.list_selected = 0;
    app.note_ui.mode = NoteMode::Load;
}

}
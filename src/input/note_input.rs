pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ropey::Rope;

use crate::{input::{editor::Input::handle_editor_key, general_input::Input::add_tag_from_inputs}, model::{app::App::{App, NoteFocus, NoteMode}, notes::Note::Note_task}};


pub fn handle_notes_key(app: &mut App, key: KeyEvent) {
    if app.note_ui.mode == NoteMode::Load {
        handle_note_list_key(app, key);
        return;
    }

    // field cycling: Ctrl+N/Ctrl+Down = next, Ctrl+P/Ctrl+Up = prev, works
    // from any field (including Content). Plain Up/Down also cycles, but
    // only when NOT on Content, since Content needs Up/Down for the cursor.
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
            KeyCode::Enter | KeyCode::Char(' ')=> {
                app.note_ui.editing.favorite ^= true;
                touch_note(app);
            }
            _ => {}
        },
        NoteFocus::Pinned => match key.code {
            KeyCode::Enter | KeyCode::Char(' ')=> {
                app.note_ui.editing.pinned ^= true;
                touch_note(app);
            }
            _ => {}
        },        
        NoteFocus::Content => handle_editor_key(app, key),
        NoteFocus::TagName => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.journal_ui.tag_name_input.push(c),
            KeyCode::Backspace => { app.journal_ui.tag_name_input.pop(); }
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
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { add_tag_from_inputs(app); }
        }
        NoteFocus::Save => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { save_note(app); }
        }
        NoteFocus::New => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { new_note(app); }
        }
        NoteFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { open_note_load_list(app); }
        }
    }
}

fn handle_note_list_key(app: &mut App, key: KeyEvent) {
    let ui = &mut app.note_ui;
    match key.code {
        KeyCode::Up => ui.list_selected = ui.list_selected.saturating_sub(1),
        KeyCode::Down => {
            if !ui.list.is_empty() { ui.list_selected = (ui.list_selected + 1).min(ui.list.len() - 1); }
        }
        KeyCode::Enter => {
            if let Some(task) = ui.list.get(ui.list_selected).cloned() {
                app.editor.buffer = Rope::from_str(&format!("{}\n", task.content));
                app.editor.cursor_x = 0;
                app.editor.cursor_y = 0;
                app.note_ui.editing = task;
                app.note_ui.mode = NoteMode::Edit;
                app.note_ui.focus = NoteFocus::Title;
            }
        }
        KeyCode::Esc => app.note_ui.mode = NoteMode::Edit,
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
    app.note_ui.editing = Note_task::new(None,None,false,false,None,None,None);
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
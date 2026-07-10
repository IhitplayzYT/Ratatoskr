pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ropey::Rope;

use crate::{input::{editor::Input::handle_editor_key, general_input::Input::add_tag_from_inputs}, model::{app::App::{App, JournalFocus, JournalMode}, journal::Journal::Journal_task, meta::Meta::Mood}};

pub fn handle_journal_key(app: &mut App, key: KeyEvent) {
    if app.journal_ui.mode == JournalMode::Load {
        handle_journal_list_key(app, key);
        return;
    }

    // field cycling: Ctrl+N/Ctrl+Down = next, Ctrl+P/Ctrl+Up = prev, works
    // from any field (including Content). Plain Up/Down also cycles, but
    // only when NOT on Content, since Content needs Up/Down for the cursor.
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.journal_ui.focus = app.journal_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.journal_ui.focus = app.journal_ui.focus.prev();
        return;
    }
    if app.journal_ui.focus != JournalFocus::Content {
        match key.code {
            KeyCode::Up => { app.journal_ui.focus = app.journal_ui.focus.prev(); return; }
            KeyCode::Down => { app.journal_ui.focus = app.journal_ui.focus.next(); return; }
            _ => {}
        }
    }
    let text_field = matches!(app.journal_ui.focus,
        JournalFocus::Title | JournalFocus::Content | JournalFocus::TagName
        | JournalFocus::TagHex | JournalFocus::Member | JournalFocus::Topic);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match app.journal_ui.focus {
        JournalFocus::Title => match key.code {
            KeyCode::Char(c) => { app.journal_ui.editing.title.push(c); touch_journal(app); }
            KeyCode::Backspace => { app.journal_ui.editing.title.pop(); touch_journal(app); }
            _ => {}
        },
        JournalFocus::Member => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => {
                app.journal_ui.editing.member.get_or_insert_with(String::new).push(c);
                touch_journal(app);
            }
            KeyCode::Backspace => {
                if let Some(m) = app.journal_ui.editing.member.as_mut() { m.pop(); }
                touch_journal(app);
            }
            _ => {}
        },
        JournalFocus::Topic => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => {
                app.journal_ui.editing.topic.get_or_insert_with(String::new).push(c);
                touch_journal(app);
            }
            KeyCode::Backspace => {
                if let Some(t) = app.journal_ui.editing.topic.as_mut() { t.pop(); }
                touch_journal(app);
            }
            _ => {}
        },
        JournalFocus::Mood => match key.code {
            KeyCode::Left => {
                app.journal_ui.editing.mood = Some(app.journal_ui.editing.mood.unwrap_or(Mood::Neutral).prev());
                touch_journal(app);
            }
            KeyCode::Right => {
                app.journal_ui.editing.mood = Some(app.journal_ui.editing.mood.unwrap_or(Mood::Neutral).next());
                touch_journal(app);
            }
            KeyCode::Backspace | KeyCode::Delete => {
                app.journal_ui.editing.mood = None;
                touch_journal(app);
            }
            _ => {}
        },
        JournalFocus::Content => handle_editor_key(app, key),
        JournalFocus::TagName => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.journal_ui.tag_name_input.push(c),
            KeyCode::Backspace => { app.journal_ui.tag_name_input.pop(); }
            _ => {}
        },
        JournalFocus::TagHex => match key.code {
            KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                if app.journal_ui.tag_hex_input.len() < 6 { app.journal_ui.tag_hex_input.push(c); }
            }
            KeyCode::Backspace => { app.journal_ui.tag_hex_input.pop(); }
            _ => {}
        },
        JournalFocus::TagAdd => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { add_tag_from_inputs(app); }
        }
        JournalFocus::Save => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { save_journal(app); }
        }
        JournalFocus::New => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { new_journal(app); }
        }
        JournalFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) { open_journal_load_list(app); }
        }
    }
}

fn handle_journal_list_key(app: &mut App, key: KeyEvent) {
    let ui = &mut app.journal_ui;
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
                app.journal_ui.editing = task;
                app.journal_ui.mode = JournalMode::Edit;
                app.journal_ui.focus = JournalFocus::Title;
            }
        }
        KeyCode::Esc => app.journal_ui.mode = JournalMode::Edit,
        _ => {}
    }
}

fn touch_journal(app: &mut App) {
    app.journal_ui.editing.updated_at = chrono::Utc::now();
}

fn save_journal(app: &mut App) {
    app.journal_ui.editing.content = app.editor.buffer.to_string();
    app.journal_ui.editing.updated_at = chrono::Utc::now();
    app.db.save_journal_task(&app.journal_ui.editing).unwrap();
    app.features.journals.insert(app.journal_ui.editing.clone());
}

fn new_journal(app: &mut App) {
    app.journal_ui.editing = Journal_task::new(None, None, None, None, None, None);
    app.journal_ui.tag_name_input.clear();
    app.journal_ui.tag_hex_input.clear();
    app.editor.buffer = Rope::from_str("\n");
    app.editor.cursor_x = 0;
    app.editor.cursor_y = 0;
    app.journal_ui.focus = JournalFocus::Title;
}

fn open_journal_load_list(app: &mut App) {
    app.journal_ui.list = app.db.load_all_journal_task().unwrap_or_default();
    app.journal_ui.list_selected = 0;
    app.journal_ui.mode = JournalMode::Load;
}


}
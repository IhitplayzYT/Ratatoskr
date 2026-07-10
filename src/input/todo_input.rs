pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};


use crate::input::general_input::Input::{parse_due_date_input, parse_hex_color};
use crate::model::app::App::App;
use crate::model::app::App::{TodoFocus, TodoMode};
use crate::model::meta::Meta::{Tag};
use crate::model::todo::Todo::Todo_task;                    


pub fn handle_todo_key(app: &mut App, key: KeyEvent) {

    match app.todo_ui.mode {
        TodoMode::Load => handle_todo_list_key(app, key),
        TodoMode::Edit => handle_todo_edit_key(app, key),
    }
}

pub fn todo_display_order(list: &[Todo_task]) -> Vec<usize> {
    let mut incomplete: Vec<usize> = (0..list.len()).filter(|&i| !list[i].status).collect();
    let mut complete: Vec<usize> = (0..list.len()).filter(|&i| list[i].status).collect();
    incomplete.append(&mut complete);
    incomplete
}

fn handle_todo_list_key(app: &mut App, key: KeyEvent) {
    if app.todo_ui.list.is_empty() {
        app.todo_ui.list = app.db.load_all_todo_task().unwrap_or_default();
    }

    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }
    if key.code == KeyCode::Char('n') {
        app.todo_ui.editing = Todo_task::new("Untitled".to_string(), None, None, None, Vec::new(), None, None);
        app.todo_ui.tag_name_input.clear();
        app.todo_ui.tag_hex_input.clear();
        app.todo_ui.due_date_input.clear();
        app.todo_ui.due_date_valid = true;
        app.todo_ui.mode = TodoMode::Edit;
        app.todo_ui.focus = TodoFocus::Title;
        return;
    }

    let order = todo_display_order(&app.todo_ui.list);
    if order.is_empty() { return; }

    match key.code {
        KeyCode::Up => app.todo_ui.list_selected = app.todo_ui.list_selected.saturating_sub(1),
        KeyCode::Down => app.todo_ui.list_selected = (app.todo_ui.list_selected + 1).min(order.len() - 1),
        KeyCode::Left | KeyCode::Right => app.todo_ui.checkbox_focused = !app.todo_ui.checkbox_focused,
        KeyCode::Enter => {
            let real_idx = order[app.todo_ui.list_selected.min(order.len() - 1)];
            if app.todo_ui.checkbox_focused {
                let task = &mut app.todo_ui.list[real_idx];
                task.status = !task.status;
                task.completed_at = if task.status { Some(chrono::Utc::now()) } else { None };
                app.db.save_todo_task(task).unwrap();
            } else {
                let task = app.todo_ui.list[real_idx].clone();
                app.todo_ui.due_date_input = task.due_date.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_default();
                app.todo_ui.due_date_valid = true;
                app.todo_ui.editing = task;
                app.todo_ui.mode = TodoMode::Edit;
                app.todo_ui.focus = TodoFocus::Title;
            }
        }
        _ => {}
    }
}

fn handle_todo_edit_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.todo_ui.focus = app.todo_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.todo_ui.focus = app.todo_ui.focus.prev();
        return;
    }
    if app.todo_ui.focus != TodoFocus::Description {
        match key.code {
            KeyCode::Up => { app.todo_ui.focus = app.todo_ui.focus.prev(); return; }
            KeyCode::Down => { app.todo_ui.focus = app.todo_ui.focus.next(); return; }
            _ => {}
        }
    }

    if key.code == KeyCode::Esc {
        app.todo_ui.mode = TodoMode::Load;
        return;
    }

    let text_field = matches!(app.todo_ui.focus,
        TodoFocus::Title | TodoFocus::Description | TodoFocus::TagName
        | TodoFocus::TagHex | TodoFocus::Member | TodoFocus::Topic | TodoFocus::DueDate);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match app.todo_ui.focus {
        TodoFocus::Title => match key.code {
            KeyCode::Char(c) => { app.todo_ui.editing.title.push(c); }
            KeyCode::Backspace => { app.todo_ui.editing.title.pop(); }
            _ => {}
        },
        TodoFocus::Description => match key.code {
            KeyCode::Char(c) => { app.todo_ui.editing.description.get_or_insert_with(String::new).push(c); }
            KeyCode::Backspace => { if let Some(d) = app.todo_ui.editing.description.as_mut() { d.pop(); } }
            _ => {}
        },
        TodoFocus::Status => {
            if matches!(key.code, KeyCode::Left | KeyCode::Right | KeyCode::Enter) {
                app.todo_ui.editing.status = !app.todo_ui.editing.status;
                app.todo_ui.editing.completed_at = if app.todo_ui.editing.status { Some(chrono::Utc::now()) } else { None };
            }
        }
        TodoFocus::Priority => match key.code {
            KeyCode::Left => app.todo_ui.editing.priority = app.todo_ui.editing.priority.prev(),
            KeyCode::Right => app.todo_ui.editing.priority = app.todo_ui.editing.priority.next(),
            _ => {}
        },
        TodoFocus::DueDate => {
            match key.code {
                KeyCode::Char(c) => app.todo_ui.due_date_input.push(c),
                KeyCode::Backspace => { app.todo_ui.due_date_input.pop(); }
                _ => {}
            }
            match parse_due_date_input(&app.todo_ui.due_date_input) {
                Some(dt) => { app.todo_ui.due_date_valid = true; app.todo_ui.editing.due_date = Some(dt); }
                None if app.todo_ui.due_date_input.is_empty() => { app.todo_ui.due_date_valid = true; app.todo_ui.editing.due_date = None; }
                None => app.todo_ui.due_date_valid = false,
            }
        }
        TodoFocus::Member => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.todo_ui.editing.member.get_or_insert_with(String::new).push(c),
            KeyCode::Backspace => { if let Some(m) = app.todo_ui.editing.member.as_mut() { m.pop(); } }
            _ => {}
        },
        TodoFocus::Topic => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.todo_ui.editing.topic.get_or_insert_with(String::new).push(c),
            KeyCode::Backspace => { if let Some(t) = app.todo_ui.editing.topic.as_mut() { t.pop(); } }
            _ => {}
        },
        TodoFocus::TagName => match key.code {
            KeyCode::Char(c) if !c.is_whitespace() => app.todo_ui.tag_name_input.push(c),
            KeyCode::Backspace => { app.todo_ui.tag_name_input.pop(); }
            _ => {}
        },
        TodoFocus::TagHex => match key.code {
            KeyCode::Char(c) if c.is_ascii_hexdigit() => { if app.todo_ui.tag_hex_input.len() < 6 { app.todo_ui.tag_hex_input.push(c); } }
            KeyCode::Backspace => { app.todo_ui.tag_hex_input.pop(); }
            _ => {}
        },
        TodoFocus::TagAdd => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                let ui = &mut app.todo_ui;
                if !ui.tag_name_input.is_empty() && ui.tag_hex_input.len() == 6 {
                    if let Some(color) = parse_hex_color(&ui.tag_hex_input) {
                        ui.editing.tags.push(Tag::new(ui.tag_name_input.clone(), Some(color)));
                        ui.tag_name_input.clear();
                        ui.tag_hex_input.clear();
                    }
                }
            }
        }
        TodoFocus::Save => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.db.save_todo_task(&app.todo_ui.editing).unwrap();
                app.todo_ui.list = app.db.load_all_todo_task().unwrap_or_default();
                app.todo_ui.mode = TodoMode::Load;
            }
        }
        TodoFocus::New => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.todo_ui.editing = Todo_task::new("Untitled".to_string(), None, None, None, Vec::new(), None, None);
                app.todo_ui.tag_name_input.clear();
                app.todo_ui.tag_hex_input.clear();
                app.todo_ui.due_date_input.clear();
                app.todo_ui.due_date_valid = true;
                app.todo_ui.focus = TodoFocus::Title;
            }
        }
        TodoFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.todo_ui.list = app.db.load_all_todo_task().unwrap_or_default();
                app.todo_ui.list_selected = 0;
                app.todo_ui.mode = TodoMode::Load;
            }
        }
    }
}


}
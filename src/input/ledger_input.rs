pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::model::app::App::{App, LedgerFocus, LedgerMode};



    pub fn handle_ledger(app:&mut App,key: KeyEvent){
    match app.ledger_ui.mode {
            LedgerMode::Load => handle_ledger_list_key(app, key),
            LedgerMode::Edit => handle_ledger_edit_key(app, key),
        }

    }

    fn handle_ledger_edit_key(app: &mut App,key: KeyEvent){
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.ledger_ui.focus = app.ledger_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.ledger_ui.focus = app.ledger_ui.focus.prev();
        return;
    }
    if app.ledger_ui.focus != LedgerFocus::Description {
        match key.code {
            KeyCode::Up => { app.ledger_ui.focus = app.ledger_ui.focus.prev(); return; }
            KeyCode::Down => { app.ledger_ui.focus = app.ledger_ui.focus.next(); return; }
            _ => {}
        }
    }

    if key.code == KeyCode::Esc {
        app.ledger_ui.mode = LedgerMode::Load;
        return;
    }

let text_field = matches!(app.ledger_ui.focus,LedgerFocus::Description | LedgerFocus::Item | LedgerFocus::Txn_time);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match app.ledger_ui.focus {
        LedgerFocus::Item => match key.code {
            KeyCode::Char(c) => { app.ledger_ui.editing.item.push(c); }
            KeyCode::Backspace => { app.ledger_ui.editing.item.pop(); }
            _ => {}
        },
        LedgerFocus::Description => match key.code {
            KeyCode::Char(c) => { app.ledger_ui.editing.desc.get_or_insert_with(String::new).push(c); }
            KeyCode::Backspace => { if let Some(d) = app.ledger_ui.editing.desc.as_mut() { d.pop(); } }
            _ => {}
        },
        LedgerFocus::Frequency => match key.code {
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

    fn handle_ledger_list_key(app: &mut App,key: KeyEvent){}

fn handle_todo_edit_key(app: &mut App, key: KeyEvent) {

    


}


}
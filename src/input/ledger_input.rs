pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rust_decimal::Decimal;

use crate::model::{app::App::{App, LedgerFocus, LedgerMode}, finance::Finance::{Finance_task, Ledger}, meta::Meta::Txn_Type};

pub fn tick_ledger_recurrence(app: &mut App) {
    let now = chrono::Utc::now();
    let due: Vec<Finance_task> = app.ledger_ui.list.retrive_txn().iter()
        .filter_map(|t| {
            let period = t.freq.period_seconds()?;
            let elapsed = (now - t.txn_time).num_seconds();
            if elapsed >= period {
                let mut clone = t.clone(); // requires Finance_task: Clone
                clone.id = uuid::Uuid::new_v4();
                clone.txn_time = now;
                Some(clone)
            } else { None }
        })
        .collect();

    for new_txn in due {
        app.db.save_ledger_txn(&new_txn).unwrap(); // see note below re: this fn
        app.ledger_ui.list.from_txn(&new_txn);
    }
}

// ---------------------------------------------------------------------------
// ledger: input handling
// ---------------------------------------------------------------------------
pub fn handle_ledger(app: &mut App, key: KeyEvent) {
    match app.ledger_ui.mode {
        LedgerMode::Load => handle_ledger_list_key(app, key),
        LedgerMode::Edit => handle_ledger_edit_key(app, key),
    }
}

fn handle_ledger_list_key(app: &mut App, key: KeyEvent) {
    if app.ledger_ui.list.retrive_txn().is_empty() {
        app.ledger_ui.list = app.db.load_ledger().unwrap_or_else(|_| Ledger::new());
    }

    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }
    if key.code == KeyCode::Char('n') {
        app.ledger_ui.editing = Finance_task::new("".to_string(), None, Txn_Type::DEBIT, Decimal::ZERO, None);
        app.ledger_ui.amnt_input.clear();
        app.ledger_ui.amnt_valid = true;
        app.ledger_ui.txn_time_input = chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string();
        app.ledger_ui.txn_time_valid = true;
        app.ledger_ui.mode = LedgerMode::Edit;
        app.ledger_ui.focus = LedgerFocus::Item;
        return;
    }

    let len = app.ledger_ui.list.retrive_txn().len();
    if len == 0 { return; }

    match key.code {
        KeyCode::Up => app.ledger_ui.list_selected = app.ledger_ui.list_selected.saturating_sub(1),
        KeyCode::Down => app.ledger_ui.list_selected = (app.ledger_ui.list_selected + 1).min(len - 1),
        KeyCode::Enter => {
            let task = app.ledger_ui.list.retrive_txn()[app.ledger_ui.list_selected].clone();
            app.ledger_ui.amnt_input = task.amnt.to_string();
            app.ledger_ui.amnt_valid = true;
            app.ledger_ui.txn_time_input = task.txn_time.format("%Y-%m-%d %H:%M").to_string();
            app.ledger_ui.txn_time_valid = true;
            app.ledger_ui.editing = task;
            app.ledger_ui.mode = LedgerMode::Edit;
            app.ledger_ui.focus = LedgerFocus::Item;
        }
        _ => {}
    }
}

fn handle_ledger_edit_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.ledger_ui.focus = app.ledger_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.ledger_ui.focus = app.ledger_ui.focus.prev();
        return;
    }
    match key.code {
        KeyCode::Up => { app.ledger_ui.focus = app.ledger_ui.focus.prev(); return; }
        KeyCode::Down => { app.ledger_ui.focus = app.ledger_ui.focus.next(); return; }
        _ => {}
    }

    if key.code == KeyCode::Esc {
        app.ledger_ui.mode = LedgerMode::Load;
        return;
    }

    let text_field = matches!(app.ledger_ui.focus, LedgerFocus::Item | LedgerFocus::Description | LedgerFocus::Amount | LedgerFocus::TxnTime);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match app.ledger_ui.focus {
        LedgerFocus::Item => match key.code {
            KeyCode::Char(c) => app.ledger_ui.editing.item.push(c),
            KeyCode::Backspace => { app.ledger_ui.editing.item.pop(); }
            _ => {}
        },
        LedgerFocus::Description => match key.code {
            KeyCode::Char(c) => app.ledger_ui.editing.desc.get_or_insert_with(String::new).push(c),
            KeyCode::Backspace => { if let Some(d) = app.ledger_ui.editing.desc.as_mut() { d.pop(); } }
            _ => {}
        },
        LedgerFocus::Amount => {
            match key.code {
                KeyCode::Char(c) if c.is_ascii_digit() || c == '.' || c == '-' => app.ledger_ui.amnt_input.push(c),
                KeyCode::Backspace => { app.ledger_ui.amnt_input.pop(); }
                _ => {}
            }
            match Decimal::from_str_exact(&app.ledger_ui.amnt_input) {
                Ok(d) => { app.ledger_ui.amnt_valid = true; app.ledger_ui.editing.amnt = d; }
                Err(_) => app.ledger_ui.amnt_valid = false,
            }
        }
        LedgerFocus::TxnType => match key.code {
            KeyCode::Left => app.ledger_ui.editing.txn_type = app.ledger_ui.editing.txn_type.prev(),
            KeyCode::Right => app.ledger_ui.editing.txn_type = app.ledger_ui.editing.txn_type.next(),
            _ => {}
        },
        LedgerFocus::Frequency => match key.code {
            KeyCode::Left => app.ledger_ui.editing.freq = app.ledger_ui.editing.freq.prev_kind(),
            KeyCode::Right => app.ledger_ui.editing.freq = app.ledger_ui.editing.freq.next_kind(),
            KeyCode::Up => app.ledger_ui.editing.freq = app.ledger_ui.editing.freq.bump_value(1),
            KeyCode::Down => app.ledger_ui.editing.freq = app.ledger_ui.editing.freq.bump_value(-1),
            _ => {}
        },
        LedgerFocus::TxnTime => {
            match key.code {
                KeyCode::Char(c) => app.ledger_ui.txn_time_input.push(c),
                KeyCode::Backspace => { app.ledger_ui.txn_time_input.pop(); }
                _ => {}
            }
            match chrono::NaiveDateTime::parse_from_str(&app.ledger_ui.txn_time_input, "%Y-%m-%d %H:%M") {
                Ok(naive) => {
                    app.ledger_ui.txn_time_valid = true;
                    app.ledger_ui.editing.txn_time = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc);
                }
                Err(_) => app.ledger_ui.txn_time_valid = false,
            }
        }
        LedgerFocus::Add => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.db.save_ledger_txn(&app.ledger_ui.editing).unwrap(); // see note below
                app.ledger_ui.list.from_txn(&app.ledger_ui.editing);
                app.ledger_ui.mode = LedgerMode::Load;
            }
        }
        LedgerFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.ledger_ui.list = app.db.load_ledger().unwrap_or_else(|_| Ledger::new());
                app.ledger_ui.list_selected = 0;
                app.ledger_ui.mode = LedgerMode::Load;
            }
        }
    }
}


}
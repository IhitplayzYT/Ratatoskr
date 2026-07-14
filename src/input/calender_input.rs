pub mod Input{
    use crossterm::event::KeyEvent;

use crate::model::app::App::App;


    use chrono::{NaiveDate, Datelike, Duration as ChronoDuration};
use crossterm::event::{KeyCode,KeyModifiers};

use crate::{input::general_input::Input::parse_hex_color, model::{app::App::{CalendarFocus, CalendarMode}, calendar::Calendar::Calendar_task}};

pub fn shift_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let total = year * 12 + (month as i32 - 1) + delta;
    let y = total.div_euclid(12);
    let m = total.rem_euclid(12) + 1;
    (y, m as u32)
}

/// 42-cell (6x7) grid starting from the Sunday on/before the 1st of the
/// given month — the standard "half prev / full current / half next" view.
pub fn month_grid(year: i32, month: u32) -> Vec<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let offset = first.weekday().num_days_from_sunday();
    let grid_start = first - ChronoDuration::days(offset as i64);
    (0..42).map(|i| grid_start + ChronoDuration::days(i)).collect()
}

/// Events whose [date, date + duration) span includes `day`.
pub fn events_on_day<'a>(events: &'a [Calendar_task], day: NaiveDate) -> Vec<&'a Calendar_task> {
    events.iter().filter(|e| {
        let start = e.date.date_naive();
        let end = start + ChronoDuration::days(e.duration.approx_days().max(1));
        day >= start && day < end
    }).collect()
}

pub fn handle_calendar(app: &mut App, key: KeyEvent) {
    if app.calendar_ui.events.is_empty() {
        app.calendar_ui.events = app.db.get_events().unwrap_or_default();
    }
    match app.calendar_ui.mode {
        CalendarMode::Load => handle_calendar_grid_key(app, key),
        CalendarMode::Edit => handle_calendar_edit_key(app, key),
    }
}

fn sync_view_to_cursor(ui: &mut crate::model::app::App::CalendarUiState) {
    ui.view_year = ui.cursor_date.year();
    ui.view_month = ui.cursor_date.month();
}

fn handle_calendar_grid_key(app: &mut App, key: KeyEvent) {
    let ui = &mut app.calendar_ui;

    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match key.code {
        KeyCode::Left => { ui.cursor_date -= ChronoDuration::days(1); sync_view_to_cursor(ui); }
        KeyCode::Right => { ui.cursor_date += ChronoDuration::days(1); sync_view_to_cursor(ui); }
        KeyCode::Up => { ui.cursor_date -= ChronoDuration::days(7); sync_view_to_cursor(ui); }
        KeyCode::Down => { ui.cursor_date += ChronoDuration::days(7); sync_view_to_cursor(ui); }
        KeyCode::Char('[') | KeyCode::PageUp => {
            let (y, m) = shift_month(ui.view_year, ui.view_month, -1);
            ui.view_year = y; ui.view_month = m;
        }
        KeyCode::Char(']') | KeyCode::PageDown => {
            let (y, m) = shift_month(ui.view_year, ui.view_month, 1);
            ui.view_year = y; ui.view_month = m;
        }
        KeyCode::Char('n') => {
            let cursor = ui.cursor_date;
            open_calendar_new(app, cursor);
        }
        KeyCode::Enter => {
            let cursor = ui.cursor_date;
            let hit = events_on_day(&ui.events, cursor).first().map(|e| (*e).clone());
            match hit {
                Some(task) => open_calendar_edit(app, task),
                None => open_calendar_new(app, cursor),
            }
        }
        _ => {}
    }
}

fn open_calendar_new(app: &mut App, on_date: NaiveDate) {
    let dt = on_date.and_hms_opt(9, 0, 0).unwrap().and_utc();
    app.calendar_ui.editing = Calendar_task::new("".to_string(), None, None, None, dt, None);
    app.calendar_ui.date_input = dt.format("%Y-%m-%d %H:%M").to_string();
    app.calendar_ui.date_valid = true;
    app.calendar_ui.color_hex_input.clear();
    app.calendar_ui.mode = CalendarMode::Edit;
    app.calendar_ui.focus = CalendarFocus::Event;
}

fn open_calendar_edit(app: &mut App, task: Calendar_task) {
    app.calendar_ui.date_input = task.date.format("%Y-%m-%d %H:%M").to_string();
    app.calendar_ui.date_valid = true;
    let (r, g, b) = task.color.get_rgb();
    app.calendar_ui.color_hex_input = format!("{r:02X}{g:02X}{b:02X}");
    app.calendar_ui.editing = task;
    app.calendar_ui.mode = CalendarMode::Edit;
    app.calendar_ui.focus = CalendarFocus::Event;
}

fn handle_calendar_edit_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('n') | KeyCode::Down) {
        app.calendar_ui.focus = app.calendar_ui.focus.next();
        return;
    }
    if key.modifiers.contains(KeyModifiers::ALT) && matches!(key.code, KeyCode::Char('p') | KeyCode::Up) {
        app.calendar_ui.focus = app.calendar_ui.focus.prev();
        return;
    }
    // Duration/Frequency consume Up/Down for value-bump; everything else cycles focus with them.
    if !matches!(app.calendar_ui.focus, CalendarFocus::Duration | CalendarFocus::Frequency) {
        match key.code {
            KeyCode::Up => { app.calendar_ui.focus = app.calendar_ui.focus.prev(); return; }
            KeyCode::Down => { app.calendar_ui.focus = app.calendar_ui.focus.next(); return; }
            _ => {}
        }
    }

    if key.code == KeyCode::Esc {
        app.calendar_ui.mode = CalendarMode::Load;
        return;
    }

    let text_field = matches!(app.calendar_ui.focus, CalendarFocus::Event | CalendarFocus::Description | CalendarFocus::Date | CalendarFocus::ColorHex);
    if key.code == KeyCode::Char('q') && !text_field {
        app.is_quit = true;
        app.db.save_all(&app.features).unwrap();
        return;
    }

    match app.calendar_ui.focus {
        CalendarFocus::Event => match key.code {
            KeyCode::Char(c) => app.calendar_ui.editing.event.push(c),
            KeyCode::Backspace => { app.calendar_ui.editing.event.pop(); }
            _ => {}
        },
        CalendarFocus::Description => match key.code {
            KeyCode::Char(c) => app.calendar_ui.editing.desc.get_or_insert_with(String::new).push(c),
            KeyCode::Backspace => { if let Some(d) = app.calendar_ui.editing.desc.as_mut() { d.pop(); } }
            _ => {}
        },
        CalendarFocus::Duration => match key.code {
            KeyCode::Left => app.calendar_ui.editing.duration = app.calendar_ui.editing.duration.prev_kind(),
            KeyCode::Right => app.calendar_ui.editing.duration = app.calendar_ui.editing.duration.next_kind(),
            KeyCode::Up => app.calendar_ui.editing.duration = app.calendar_ui.editing.duration.bump_value(1),
            KeyCode::Down => app.calendar_ui.editing.duration = app.calendar_ui.editing.duration.bump_value(-1),
            _ => {}
        },
        CalendarFocus::Frequency => match key.code {
            KeyCode::Left => app.calendar_ui.editing.frequency = app.calendar_ui.editing.frequency.prev_kind(),
            KeyCode::Right => app.calendar_ui.editing.frequency = app.calendar_ui.editing.frequency.next_kind(),
            KeyCode::Up => app.calendar_ui.editing.frequency = app.calendar_ui.editing.frequency.bump_value(1),
            KeyCode::Down => app.calendar_ui.editing.frequency = app.calendar_ui.editing.frequency.bump_value(-1),
            _ => {}
        },
        CalendarFocus::Date => {
            match key.code {
                KeyCode::Char(c) => app.calendar_ui.date_input.push(c),
                KeyCode::Backspace => { app.calendar_ui.date_input.pop(); }
                _ => {}
            }
            match chrono::NaiveDateTime::parse_from_str(&app.calendar_ui.date_input, "%Y-%m-%d %H:%M") {
                Ok(naive) => {
                    app.calendar_ui.date_valid = true;
                    app.calendar_ui.editing.date = chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc);
                }
                Err(_) => app.calendar_ui.date_valid = false,
            }
        }
        CalendarFocus::ColorHex => match key.code {
            KeyCode::Char(c) if c.is_ascii_hexdigit() => {
                if app.calendar_ui.color_hex_input.len() < 6 { app.calendar_ui.color_hex_input.push(c); }
                if let Some(col) = parse_hex_color(&app.calendar_ui.color_hex_input) { app.calendar_ui.editing.color = col; }
            }
            KeyCode::Backspace => {
                app.calendar_ui.color_hex_input.pop();
                if let Some(col) = parse_hex_color(&app.calendar_ui.color_hex_input) { app.calendar_ui.editing.color = col; }
            }
            _ => {}
        },
        CalendarFocus::Save => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                let existing = app.calendar_ui.events.iter().any(|e| e.id == app.calendar_ui.editing.id);
                if existing {
                    app.db.update_event(app.calendar_ui.editing.id, &app.calendar_ui.editing).unwrap();
                } else {
                    app.db.add_event(&app.calendar_ui.editing).unwrap();
                }
                app.calendar_ui.events = app.db.get_events().unwrap_or_default();
                app.calendar_ui.mode = CalendarMode::Load;
            }
        }
        CalendarFocus::New => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                let cursor = app.calendar_ui.cursor_date;
                open_calendar_new(app, cursor);
            }
        }
        CalendarFocus::Load => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                app.calendar_ui.events = app.db.get_events().unwrap_or_default();
                app.calendar_ui.mode = CalendarMode::Load;
            }
        }
    }
}



}
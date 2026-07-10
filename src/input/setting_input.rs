pub mod Input{
use crossterm::event::{KeyEvent, KeyModifiers,KeyCode};

use crate::{input::general_input::Input::handle_color_picker_field_key, model::app::App::{App, SettingsFocus, parse_autosave_duration, parse_fmt_date}};

   

 
pub fn handle_settings_key(app: &mut App, key: KeyEvent) {
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




}
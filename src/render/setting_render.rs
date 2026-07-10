pub mod Render{
    use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};

    use ratatui::{Frame, symbols};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::{Color, Modifier, Style, Stylize};

use crate::model::app::App::{App, SettingsFocus};
use crate::render::general_render::Render::{enabled_line, render_color_picker, render_theme_component_list}; 

pub fn render_settings(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(10)])
        .split(area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(rows[0]);
    render_theme_component_list(f, cols[0], app);
    render_color_picker(f, cols[1], app);

    render_general_settings_fields(f, rows[1], app); // new
}

fn render_general_settings_fields(f: &mut Frame, area: Rect, app: &App) {
    let s = &app.settings;
    let ui = &s.settings_ui;
    let hl = |focused: bool| if focused {
        Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED).fg(app.settings.theme.fontfg.to_color()).bg(app.settings.theme.fontbg.to_color())
    } else { Style::default() };

let lines = vec![
    enabled_line("Autosave", s.autosave, ui.focus == SettingsFocus::AutosaveEnabled,&app),
    Line::from(Span::styled(
        format!("Autosave every: {} ({}s){}", ui.autosave_freq_input, s.autosave_freq,
            if ui.autosave_freq_valid { "" } else { " — unparsed" }),
        hl(ui.focus == SettingsFocus::AutosaveFreq),
    )),
    enabled_line("Autocomplete", s.autocomplete, ui.focus == SettingsFocus::AutocompleteEnabled,&app),
    enabled_line("VimMode", s.vim_mode, ui.focus == SettingsFocus::VimmodeEnabled,&app),
    enabled_line("ConfirmDelete", s.confirm_delete, ui.focus == SettingsFocus::ConfirmdeleteEnabled,&app),
    Line::from(Span::styled(
        format!("Date Format: {} ({}){}", ui.date_fmt, s.date_format,
            if ui.date_valid { "" } else { " — unparsed" }),
        hl(ui.focus == SettingsFocus::Datefmt),
    )),
    Line::from(Span::styled(
        format!("Currency: {}", s.currency.title()),
        hl(ui.focus == SettingsFocus::Currency),
    )),
    Line::from(Span::styled(
        format!("Timezone: {}", ui.timezone_input),
        hl(ui.focus == SettingsFocus::Timezone),
    )),
];

    let block = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("General (↑/↓ field, ←/→ or type, Ctrl+S save)"));
    f.render_widget(block, area);
}




}
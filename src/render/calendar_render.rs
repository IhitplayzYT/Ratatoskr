pub mod Render{
    use chrono::Datelike;
use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph}};

use crate::{input::calender_input::Input::{events_on_day, month_grid}, model::app::App::{App, CalendarFocus, CalendarMode}, render::general_render::Render::render_color_swatch};


pub fn render_calendar(f: &mut Frame, area: Rect, app: &App) {
    match app.calendar_ui.mode {
        CalendarMode::Load => render_calendar_grid(f, area, app),
        CalendarMode::Edit => render_calendar_edit(f, area, app),
    }
}

fn render_calendar_grid(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let ui = &app.calendar_ui;

    let cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);

    let month_name = chrono::NaiveDate::from_ymd_opt(ui.view_year, ui.view_month, 1).unwrap().format("%B %Y").to_string();
    let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
        .title(format!(" {month_name}  (←/→/↑/↓ day, [ ] month, Enter, n = new, q = quit) "));
    let inner = block.inner(cols[0]);
    f.render_widget(block, cols[0]);

    let grid_rows = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1),
            Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    let headers = ["Su","Mo","Tu","We","Th","Fr","Sa"];
    let header_spans: Vec<Span> = headers.iter().map(|h| Span::styled(format!("{h:^6}"), Style::default().fg(color).add_modifier(Modifier::BOLD))).collect();
    f.render_widget(Paragraph::new(Line::from(header_spans)), grid_rows[0]);

    let days = month_grid(ui.view_year, ui.view_month);
    for week in 0..6 {
        let mut spans = Vec::with_capacity(7);
        for d in 0..7 {
            let day = days[week * 7 + d];
            let in_month = day.month() == ui.view_month;
            let is_cursor = day == ui.cursor_date;
            let hits = events_on_day(&ui.events, day);

            let mut style = if in_month { Style::default().fg(color) } else { Style::default().fg(Color::DarkGray) };
            if let Some(e) = hits.first() { style = style.fg(e.color.to_color()); }
            if is_cursor { style = style.add_modifier(Modifier::BOLD | Modifier::REVERSED); }

            let marker = if hits.len() > 1 { "+" } else if hits.len() == 1 { "*" } else { " " };
            spans.push(Span::styled(format!("{:>2}{marker:<1} ", day.day()), style));
        }
        f.render_widget(Paragraph::new(Line::from(spans)), grid_rows[week + 1]);
    }

    render_calendar_day_panel(f, cols[1], app);
}

fn render_calendar_day_panel(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.calendar_ui;
    let color = app.settings.theme.primary.to_color();
    let hits = events_on_day(&ui.events, ui.cursor_date);

    let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
        .title(format!(" {} ", ui.cursor_date.format("%Y-%m-%d")));
    let inner = block.inner(area);
    f.render_widget(block, area);

    if hits.is_empty() {
        f.render_widget(Paragraph::new("(no events)"), inner);
        return;
    }

    let lines: Vec<Line> = hits.iter().flat_map(|e| {
        vec![
            Line::from(Span::styled(format!("● {}", e.event), Style::default().fg(e.color.to_color()).add_modifier(Modifier::BOLD))),
            Line::from(Span::styled(format!("  {}", e.duration.title()), Style::default().fg(e.color.to_color()))),
            Line::from(""),
        ]
    }).collect();
    f.render_widget(Paragraph::new(lines), inner);
}

fn render_calendar_edit(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let ui = &app.calendar_ui;

    let rows = Layout::default().direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3)])
        .split(area);

    render_calendar_field(f, rows[0], "Event", &ui.editing.event, ui.focus == CalendarFocus::Event, color);
    render_calendar_field(f, rows[1], "Description", ui.editing.desc.as_deref().unwrap_or(""), ui.focus == CalendarFocus::Description, color);
    render_calendar_picker(f, rows[2], "Duration (←/→ kind, ↑/↓ value)", &ui.editing.duration.title(), ui.focus == CalendarFocus::Duration, color);
    render_calendar_picker(f, rows[3], "Frequency (←/→ kind, ↑/↓ value)", &ui.editing.frequency.title(), ui.focus == CalendarFocus::Frequency, color);
    render_calendar_date_field(f, rows[4], app);

    let color_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)]).split(rows[5]);
    render_calendar_field(f, color_cols[0], "Color (hex RRGGBB)", &ui.color_hex_input, ui.focus == CalendarFocus::ColorHex, color);
    render_color_swatch(f, color_cols[1], Some(ui.editing.color.clone())); // adjust if MyColor isn't Clone

    let btn_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).split(rows[6]);
    let btn_color = app.settings.theme.secondary.to_color();
    render_calendar_button(f, btn_cols[0], "Save", ui.focus == CalendarFocus::Save, btn_color);
    render_calendar_button(f, btn_cols[1], "New", ui.focus == CalendarFocus::New, btn_color);
    render_calendar_button(f, btn_cols[2], "Load", ui.focus == CalendarFocus::Load, btn_color);
}

fn render_calendar_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(value).block(Block::default().borders(Borders::ALL).border_style(style).title(label)), area);
}

fn render_calendar_picker(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(value).block(Block::default().borders(Borders::ALL).border_style(style).title(label)), area);
}

fn render_calendar_date_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.calendar_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == CalendarFocus::Date;
    let base = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let suffix = if !ui.date_valid { " — unparsed (fmt: YYYY-MM-DD HH:MM)" } else { "" };
    let style = if !ui.date_valid { Style::default().fg(Color::Red) } else { base };
    f.render_widget(Paragraph::new(format!("{}{}", ui.date_input, suffix)).style(style)
        .block(Block::default().borders(Borders::ALL).border_style(base).title("Date (YYYY-MM-DD HH:MM)")), area);
}

fn render_calendar_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style);
    f.render_widget(Paragraph::new(Line::from(Span::styled(format!(" {label} "), style))).alignment(Alignment::Center).block(block), area);
}

}
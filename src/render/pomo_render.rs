pub mod Render{
    use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
    use ratatui::{Frame, symbols};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
    use ratatui::style::{Color, Modifier, Style, Stylize}; 
    use crate::model::app::App::PomoFocus;
use crate::model::app::App::App;
use crate::render::general_render::Render::{render_big_time};
    


pub fn render_pomodoro(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(9), Constraint::Length(3)])
        .split(area);

    if pomo.running {
        render_pomo_running(f, rows[0], app);
    } else {
        render_pomo_wheels(f, rows[0], app);
    }

    render_pomo_buttons(f, rows[1], app);
}


fn render_pomo_wheels(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.primary.to_color();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Span::styled(
            format!(" Pomodoro — {} ", pomo.format_time_left()),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(inner);

    let h = (pomo.duration_left / 3600) as i64;
    let m = ((pomo.duration_left % 3600) / 60) as i64;
    let s = (pomo.duration_left % 60) as i64;

    render_pomo_wheel(f, cols[0], "HOUR", h, 24, pomo.focus == PomoFocus::Hour, color);
    render_pomo_wheel(f, cols[1], "MIN", m, 60, pomo.focus == PomoFocus::Minute, color);
    render_pomo_wheel(f, cols[2], "SEC", s, 60, pomo.focus == PomoFocus::Second, color);
}

fn render_pomo_running(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.primary.to_color();
    let status = if pomo.paused { "PAUSED" } else { "RUNNING" };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .title(Span::styled(
            format!(" Pomodoro — {status} "),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);
    f.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let text = pomo.format_time_left();

    // figlet glyphs are 5 rows tall, 7 cols wide per char (6 + 1 spacing)
    let big_height = 5u16;
    let big_width = (text.chars().count() as u16) * 7;

    let v_pad = inner.height.saturating_sub(big_height) / 2;
    let h_pad = inner.width.saturating_sub(big_width) / 2;

    let centered = Rect {
        x: inner.x + h_pad,
        y: inner.y + v_pad,
        width: big_width.min(inner.width),
        height: big_height.min(inner.height),
    };

    let dim = pomo.paused; // dim the digits while paused, still centered
    render_big_time(f, centered, &text, color, !dim);
}

fn render_pomo_wheel(f: &mut Frame, area: Rect, label: &str, value: i64, modulus: i64, focused: bool, color: Color) {
    let prev = (value - 1).rem_euclid(modulus);
    let next = (value + 1).rem_euclid(modulus);

    let dim = Style::default().add_modifier(Modifier::DIM);
    let cur_style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    };

    let lines = vec![
        Line::from(Span::styled(format!("{next:02}"), dim)).alignment(Alignment::Center),
        Line::from(Span::styled(format!("{value:02}"), cur_style)).alignment(Alignment::Center),
        Line::from(Span::styled(format!("{prev:02}"), dim)).alignment(Alignment::Center),
        Line::from(""),
        Line::from(Span::styled(label, Style::default().fg(color))).alignment(Alignment::Center),
    ];

    f.render_widget(Paragraph::new(lines), area);
}

fn render_pomo_buttons(f: &mut Frame, area: Rect, app: &App) {
    let pomo = &app.features.pomodero;
    let color = app.settings.theme.secondary.to_color();

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(area);

    let start_pause_label = if !pomo.running {
        "Start"
    } else if pomo.paused {
        "Resume"
    } else {
        "Pause"
    };

    render_pomo_button(f, cols[0], start_pause_label, pomo.focus == PomoFocus::StartPause, color);
    render_pomo_button(f, cols[1], "Reset", pomo.focus == PomoFocus::Reset, color);
    render_pomo_button(f, cols[2], "Cancel", pomo.focus == PomoFocus::Cancel, color);
}

fn render_pomo_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color)
    };
    let block = Block::default().borders(Borders::ALL).border_set(symbols::border::DOUBLE).fg(color);
    let text = Paragraph::new(Line::from(Span::styled(format!(" {label} "), style)))
        .alignment(Alignment::Center)
        .block(block);
    f.render_widget(text, area);
}


}
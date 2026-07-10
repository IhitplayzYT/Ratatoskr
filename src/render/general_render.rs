pub mod Render{
    use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
    use ratatui::{Frame};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
    use ratatui::style::{Color, Modifier, Style, Stylize};

use crate::model::app::App::{App, Color_channel, Theme_comp};
use crate::model::meta::Meta::MyColor; 


/// 5-row block-digit font for 0-9 and ':'.
pub fn big_digit_rows(c: char) -> [&'static str; 5] {
    match c {
        '0' => ["██████", "██  ██", "██  ██", "██  ██", "██████"],
        '1' => ["  ██  ", "  ██  ", "  ██  ", "  ██  ", "  ██  "],
        '2' => ["██████", "    ██", "██████", "██    ", "██████"],
        '3' => ["██████", "    ██", "██████", "    ██", "██████"],
        '4' => ["██  ██", "██  ██", "██████", "    ██", "    ██"],
        '5' => ["██████", "██    ", "██████", "    ██", "██████"],
        '6' => ["██████", "██    ", "██████", "██  ██", "██████"],
        '7' => ["██████", "    ██", "    ██", "    ██", "    ██"],
        '8' => ["██████", "██  ██", "██████", "██  ██", "██████"],
        '9' => ["██████", "██  ██", "██████", "    ██", "██████"],
        ':' => ["      ", "  ██  ", "      ", "  ██  ", "      "],
        _   => ["      ", "      ", "      ", "      ", "      "],
    }
}

pub fn render_color_swatch(f: &mut Frame, area: Rect, color: Option<MyColor>) {
    let bg = color.map(|c| c.to_color()).unwrap_or(Color::DarkGray);
    let block = Block::default().borders(Borders::ALL).style(Style::default().bg(bg));
    f.render_widget(block, area);
}

/// Renders `text` (digits/colons only) as figlet-style block art.
pub fn render_big_time(f: &mut Frame, area: Rect, text: &str, color: Color, bold: bool) {
    let mut row_strs: [String; 5] = Default::default();
    for c in text.chars() {
        let glyph = big_digit_rows(c);
        for i in 0..5 {
            row_strs[i].push_str(glyph[i]);
            row_strs[i].push(' '); // spacing between characters
        }
    }

    let style = if bold {
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color)
    };

    let lines: Vec<Line> = row_strs
        .iter()
        .map(|r| Line::from(Span::styled(r.clone(), style)).alignment(Alignment::Center))
        .collect();

    f.render_widget(Paragraph::new(lines), area);
}


pub fn enabled_line(label: &str, enabled: bool, focused: bool,app:&App) -> Line<'static> {
    let hl = |focused: bool| if focused {
        Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED).fg(app.settings.theme.fontfg.to_color()).bg(app.settings.theme.fontbg.to_color())
    } else { Style::default() };

    let base = hl(focused);
    let status_style = base
        .fg(if enabled { Color::Green } else { Color::Red })
        .add_modifier(Modifier::BOLD);

    Line::from(vec![
        Span::styled(format!("{label}: "), base),
        Span::styled(if enabled { "Enabled" } else { "Disabled" }, status_style),
    ])
}


pub fn render_theme_component_list(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = Theme_comp::ALL
        .iter()
        .map(|c| {
            let color = c.get(&app.settings.theme).to_color();
            let selected = *c == app.settings.color_picker.component;
            let style = if selected {
                Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(color)
            };
            ListItem::new(Line::from(Span::styled(format!(" {} ", c.title()), style)))
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL).border_style(Style::default().fg(app.settings.theme.primary.to_color()))
            .title(Span::styled("Theme components (↑/↓)",Style::default().fg(app.settings.theme.secondary.to_color()))).title_alignment(Alignment::Center)
    );
    f.render_widget(list, area);
}
 
/// Live text-field RGB picker for whichever theme component is selected.
pub fn render_color_picker(f: &mut Frame, area: Rect, app: &App) {
    let picker = &app.settings.color_picker;
    let color = picker.component.get(&app.settings.theme);
 
    let block = Block::default()
        .borders(Borders::ALL).border_style(Style::default().fg(color.to_color()))
        .title(Span::styled(format!(" {} — Use '[' and ']' to change channels, type for editing", picker.component.title()),Style::default().fg(app.settings.theme.primary.to_color())));
    f.render_widget(block, area);
 
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };
 
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .split(inner);
    render_channel_field(f,&app, rows[0], "R", &picker.buffers[0], picker.selected == Color_channel::R, Color::Red);
    render_channel_field(f,&app, rows[1], "G", &picker.buffers[1], picker.selected == Color_channel::G, Color::Green);
    render_channel_field(f,&app, rows[2], "B", &picker.buffers[2], picker.selected == Color_channel::B, Color::Blue);
 
    let (r, g, b) = color.get_rgb();
    let hex = Paragraph::new(format!("hex: #{r:02X}{g:02X}{b:02X}")).fg(color.to_color());
    f.render_widget(hex, rows[3]);
 
    let swatch = Block::default().style(Style::default().bg(color.to_color()));
    f.render_widget(swatch, rows[4]);
}
 
pub fn render_channel_field(f: &mut Frame,app:&App, area: Rect, label: &str, buf: &str, focused: bool, color: Color) {
    let style = if focused {
        Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
    } else {
        Style::default().fg(color)
    };
    let line = Line::from(vec![
        Span::styled(format!("{label}: "), style),
        Span::styled(format!("[{buf:<3}]"), style),
        if focused { Span::styled(" ←type digits",Style::default().fg(app.settings.theme.accent.to_color())) } else { Span::raw("") },
    ]);
    f.render_widget(Paragraph::new(line), area);
}
 


}
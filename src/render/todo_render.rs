pub mod Render{
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use chrono::NaiveDateTime;

use crate::input::general_input::Input::parse_hex_color;
use crate::input::todo_input::Input::todo_display_order;
use crate::model::app::App::{App, TodoFocus, TodoMode};
use crate::render::general_render::Render::render_color_swatch;


pub fn render_todo(f: &mut Frame, area: Rect, app: &App) {
    match app.todo_ui.mode {
        TodoMode::Load => render_todo_list(f, area, app),
        TodoMode::Edit => render_todo_edit(f, area, app),
    }
}

fn render_todo_list(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let order = todo_display_order(&app.todo_ui.list);

    let items: Vec<ListItem> = order.iter().enumerate().map(|(row_i, &real_i)| {
        let t = &app.todo_ui.list[real_i];
        let desc_preview: String = t.description.as_deref().unwrap_or("").chars().take(15).collect();
        let due_str = t.due_date.map(|d| d.format("%Y-%m-%d %H:%M").to_string()).unwrap_or_else(|| "-".to_string());
        let checkbox = if t.status { "[x]" } else { "[ ]" };
        let is_row_selected = row_i == app.todo_ui.list_selected;

        let base_style = if t.status {
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM | Modifier::CROSSED_OUT)
        } else {
            Style::default().fg(color)
        };
        let row_hl = if is_row_selected && !app.todo_ui.checkbox_focused {
            base_style.add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else { base_style };
        let box_hl = if is_row_selected && app.todo_ui.checkbox_focused {
            Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else { base_style };

        let status_label = if t.status { " COMPLETED " } else { "" };
        let priority_style = if t.status { base_style } else { Style::default().fg(t.priority.color()) };

        let line = Line::from(vec![
            Span::styled(format!(" {checkbox} "), box_hl),
            Span::styled(format!("{:<20} ", t.title), row_hl),
            Span::styled(format!("{:<17} ", desc_preview), row_hl),
            Span::styled(format!("{:<9} ", t.priority.title()), priority_style),
            Span::styled(format!("{:<17} ", due_str), row_hl),
            Span::styled(status_label, base_style.add_modifier(Modifier::BOLD)),
        ]);
        ListItem::new(line)
    }).collect();

    let list = List::new(items).block(
        Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
            .title("Todos (↑/↓ select, ←/→ toggle checkbox focus, Enter, n = new, q = quit)")
    );
    f.render_widget(list, area);
}

fn render_todo_edit(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let ui = &app.todo_ui;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
        ])
        .split(area);

    render_todo_field(f, rows[0], "Title", &ui.editing.title, ui.focus == TodoFocus::Title, color);
    render_todo_field(f, rows[1], "Description", ui.editing.description.as_deref().unwrap_or(""), ui.focus == TodoFocus::Description, color);
    render_todo_status_field(f, rows[2], app);
    render_todo_priority_field(f, rows[3], app);
    render_todo_due_field(f, rows[4], app);

    let tag_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(25), Constraint::Percentage(15), Constraint::Percentage(30)])
        .split(rows[5]);
    render_todo_field(f, tag_cols[0], "Tag name", &ui.tag_name_input, ui.focus == TodoFocus::TagName, color);
    render_todo_field(f, tag_cols[1], "Hex (RRGGBB)", &ui.tag_hex_input, ui.focus == TodoFocus::TagHex, color);
    render_color_swatch(f, tag_cols[2], parse_hex_color(&ui.tag_hex_input));
    render_todo_button(f, tag_cols[3], "Add Tag", ui.focus == TodoFocus::TagAdd, color);

    let spans: Vec<Span> = ui.editing.tags.iter()
        .map(|t| Span::styled(format!(" {} ", t.name), Style::default().fg(t.color.to_color())))
        .collect();
    let tag_line = if spans.is_empty() { Line::from("(no tags yet)") } else { Line::from(spans) };
    f.render_widget(Paragraph::new(tag_line).block(Block::default().borders(Borders::ALL).title("Tags")), rows[6]);

    let mt_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[7]);
    render_todo_field(f, mt_cols[0], "Member", ui.editing.member.as_deref().unwrap_or(""), ui.focus == TodoFocus::Member, color);
    render_todo_field(f, mt_cols[1], "Topic", ui.editing.topic.as_deref().unwrap_or(""), ui.focus == TodoFocus::Topic, color);

    let btn_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).split(rows[8]);
    let btn_color = app.settings.theme.secondary.to_color();
    render_todo_button(f, btn_cols[0], "Save", ui.focus == TodoFocus::Save, btn_color);
    render_todo_button(f, btn_cols[1], "New", ui.focus == TodoFocus::New, btn_color);
    render_todo_button(f, btn_cols[2], "Load", ui.focus == TodoFocus::Load, btn_color);
}

fn render_todo_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(value).block(Block::default().borders(Borders::ALL).border_style(style).title(label)), area);
}

fn render_todo_status_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.todo_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == TodoFocus::Status;
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let box_str = if ui.editing.status { "[x] Completed" } else { "[ ] Not completed" };
    f.render_widget(Paragraph::new(box_str).block(Block::default().borders(Borders::ALL).border_style(style).title("Status (←/→/Enter)")), area);
}

fn render_todo_priority_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.todo_ui;
    let focused = ui.focus == TodoFocus::Priority;
    let pcolor = ui.editing.priority.color();
    let style = if focused { Style::default().fg(pcolor).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(pcolor) };
    f.render_widget(Paragraph::new(ui.editing.priority.title()).block(Block::default().borders(Borders::ALL).border_style(style).title("Priority (←/→)")), area);
}

fn render_todo_due_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.todo_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == TodoFocus::DueDate;
    let base = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let error_style = if !ui.due_date_valid { Style::default().fg(Color::Red) } else { base };
    let suffix = if !ui.due_date_valid { " — unparsed (fmt: YYYY-MM-DD HH:MM)" } else { "" };
    let text = format!("{}{}", ui.due_date_input, suffix);
    f.render_widget(Paragraph::new(text).style(error_style).block(Block::default().borders(Borders::ALL).border_style(base).title("Due Date (YYYY-MM-DD HH:MM)")), area);
}

fn render_todo_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style);
    f.render_widget(Paragraph::new(Line::from(Span::styled(format!(" {label} "), style))).alignment(Alignment::Center).block(block), area);
}



}
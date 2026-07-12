pub mod Render{


use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Modifier, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}};
use ratatui::style::Style;
use ratatui::style::Color;

use crate::{input::{general_input::Input::parse_hex_color, note_input::Input::note_display_order}, model::{app::App::{App, EditorMode, JournalFocus, JournalMode, NoteFocus, NoteMode, Vim_mode}, meta::Meta::MyColor}, render::general_render::Render::render_color_swatch};

pub fn render_notes(f: &mut Frame, area: Rect, app: &App) {
    match app.note_ui.mode {
        NoteMode::Load => render_note_list(f, area, app),
        NoteMode::Edit => render_note_edit(f, area, app),
    }
}

fn render_note_list(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let order = note_display_order(&app.note_ui.list, app.note_ui.filter);

    let items: Vec<ListItem> = order.iter().enumerate().map(|(row_i, &real_i)| {
        let n = &app.note_ui.list[real_i];
        let is_selected = row_i == app.note_ui.list_selected;
        let base = if is_selected {
            Style::default().fg(color).add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        let pin_icon = if n.pinned { "📌" } else { "  " };
        let fav_icon = if n.favorite { "♥" } else { " " };
        let date_str = n.created_at.format("%Y-%m-%d").to_string();

        let line = Line::from(vec![
            Span::styled(format!(" {pin_icon} {fav_icon} "), base),
            Span::styled(format!("{:<30} ", n.title), base),
            Span::styled(date_str, base),
        ]);
        ListItem::new(line)
    }).collect();

    let list = List::new(items).block(
        Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
            .title(format!("Notes — {} (↑/↓ select, Enter open, f = filter, n = new, q = quit)", app.note_ui.filter.title()))
    );
    f.render_widget(list, area);
}

fn render_note_edit(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let ui = &app.note_ui;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), Constraint::Length(3), Constraint::Min(6),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
        ])
        .split(area);

    render_note_field(f, rows[0], "Title", &ui.editing.title, ui.focus == NoteFocus::Title, color);
    render_note_pin_fav_row(f, rows[1], app);
    render_note_content(f, rows[2], app);

    let tag_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(25), Constraint::Percentage(15), Constraint::Percentage(30)])
        .split(rows[3]);
    render_note_field(f, tag_cols[0], "Tag name", &ui.tag_name_input, ui.focus == NoteFocus::TagName, color);
    render_note_field(f, tag_cols[1], "Hex (RRGGBB)", &ui.tag_hex_input, ui.focus == NoteFocus::TagHex, color);
    render_color_swatch(f, tag_cols[2], parse_hex_color(&ui.tag_hex_input));
    render_note_button(f, tag_cols[3], "Add Tag", ui.focus == NoteFocus::TagAdd, color);

    let spans: Vec<Span> = ui.editing.tags.iter()
        .map(|t| Span::styled(format!(" {} ", t.name), Style::default().fg(t.color.to_color())))
        .collect();
    let tag_line = if spans.is_empty() { Line::from("(no tags yet)") } else { Line::from(spans) };
    f.render_widget(Paragraph::new(tag_line).block(Block::default().borders(Borders::ALL).title("Tags")), rows[4]);

    let mt_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[5]);
    render_note_field(f, mt_cols[0], "Member", ui.editing.member.as_deref().unwrap_or(""), ui.focus == NoteFocus::Member, color);
    render_note_field(f, mt_cols[1], "Topic", ui.editing.topic.as_deref().unwrap_or(""), ui.focus == NoteFocus::Topic, color);

    let btn_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).split(rows[6]);
    let btn_color = app.settings.theme.secondary.to_color();
    render_note_button(f, btn_cols[0], "Save", ui.focus == NoteFocus::Save, btn_color);
    render_note_button(f, btn_cols[1], "New", ui.focus == NoteFocus::New, btn_color);
    render_note_button(f, btn_cols[2], "Load", ui.focus == NoteFocus::Load, btn_color);
}

fn render_note_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(value).block(Block::default().borders(Borders::ALL).border_style(style).title(label)), area);
}

fn render_note_pin_fav_row(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.note_ui;
    let color = app.settings.theme.primary.to_color();
    let cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

    let pin_focused = ui.focus == NoteFocus::Pinned;
    let pin_style = if pin_focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let pin_str = if ui.editing.pinned { "📌 [x] Pinned" } else { "📌 [ ] Not pinned" };
    f.render_widget(Paragraph::new(pin_str).block(Block::default().borders(Borders::ALL).border_style(pin_style).title("Pinned (Enter/←/→)")), cols[0]);

    let fav_focused = ui.focus == NoteFocus::Favourite;
    let fav_style = if fav_focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let fav_str = if ui.editing.favorite { "♥ [x] Favourite" } else { "♥ [ ] Not favourite" };
    f.render_widget(Paragraph::new(fav_str).block(Block::default().borders(Borders::ALL).border_style(fav_style).title("Favourite (Enter/←/→)")), cols[1]);
}

fn render_note_content(f: &mut Frame, area: Rect, app: &App) {
    let editor = &app.editor;
    let focused = app.note_ui.focus == NoteFocus::Content;
    let color = app.settings.theme.primary.to_color();
    let mode_str = match (editor.mode, editor.vim.submode) {
        (EditorMode::Normal, _) => "NORMAL (plain)".to_string(),
        (EditorMode::Vim, Vim_mode::Normal) => "VIM: NORMAL".to_string(),
        (EditorMode::Vim, Vim_mode::Insert) => "VIM: INSERT".to_string(),
    };
    let border_style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(border_style)
        .title(format!("Content — {mode_str} (F2 vim toggle)"));
    f.render_widget(Paragraph::new(editor.buffer.to_string()).block(block), area);

    if focused {
        f.set_cursor_position((area.x + 1 + editor.cursor_x as u16, area.y + 1 + editor.cursor_y as u16));
    }
}

fn render_note_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style);
    f.render_widget(Paragraph::new(Line::from(Span::styled(format!(" {label} "), style))).alignment(Alignment::Center).block(block), area);
}


}
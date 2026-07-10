pub mod Render{


use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::Modifier, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, Paragraph}};
use ratatui::style::Style;
use ratatui::style::Color;

use crate::{input::general_input::Input::parse_hex_color, model::{app::App::{App, EditorMode, JournalFocus, JournalMode, NoteMode, Vim_mode}, meta::Meta::MyColor}, render::general_render::Render::render_color_swatch};


pub fn render_notes(f: &mut Frame, area: Rect, app: &App) {
    if app.note_ui.mode == NoteMode::Load {
        render_note_list(f, area, app);
        return;
    }
    let color = app.settings.theme.primary.to_color();
    let ui = &app.note_ui;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), Constraint::Length(3), Constraint::Min(6),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
        ])
        .split(area);

    render_journal_text_field(f, rows[0], "Title", &ui.editing.title, ui.focus == JournalFocus::Title, color);
    render_journal_mood_field(f, rows[1], app);
    render_journal_content(f, rows[2], app);
    render_journal_tag_inputs(f, rows[3], app);
    render_journal_tag_list(f, rows[4], app);
    render_journal_member_topic(f, rows[5], app);
    render_journal_buttons(f, rows[6], app);
}

fn render_journal_text_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style).title(label).title_alignment(Alignment::Center);
    f.render_widget(Paragraph::new(value).block(block), area);
}

fn render_journal_mood_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == JournalFocus::Mood;
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let label = ui.editing.mood.map(|m| m.title()).unwrap_or("(none — ←/→ to set, Backspace to clear)");
    let block = Block::default().borders(Borders::ALL).border_style(style).title("Mood (←/→)");
    f.render_widget(Paragraph::new(label).block(block), area);
}

fn render_journal_content(f: &mut Frame, area: Rect, app: &App) {
    let editor = &app.editor;
    let focused = app.journal_ui.focus == JournalFocus::Content;
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

fn render_journal_tag_inputs(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let color = app.settings.theme.primary.to_color();
    let cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(10), Constraint::Percentage(30)])
        .split(area);

    render_journal_text_field(f, cols[0], "Tag name", &ui.tag_name_input, ui.focus == JournalFocus::TagName, color);
    render_journal_text_field(f, cols[1], "Hex (RRGGBB)", &ui.tag_hex_input, ui.focus == JournalFocus::TagHex, color);
    render_color_swatch(f, cols[2], parse_hex_color(&ui.tag_hex_input));
    render_journal_button(f, cols[3], "Add Tag", ui.focus == JournalFocus::TagAdd, color);
}

/// Factored out from the settings color picker's swatch so both can use it.


fn render_journal_tag_list(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let spans: Vec<Span> = ui.editing.tags.iter()
        .map(|t| Span::styled(format!(" 󰓹 {} ", t.name), Style::default().fg(t.color.to_color())))
        .collect();
    let line = if spans.is_empty() { Line::from("(no tags yet)") } else { Line::from(spans) };
    let block = Block::default().borders(Borders::ALL).title("Tags");
    f.render_widget(Paragraph::new(line).block(block), area);
}

fn render_journal_member_topic(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let color = app.settings.theme.primary.to_color();
    let cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    render_journal_text_field(f, cols[0], "Member", ui.editing.member.as_deref().unwrap_or(""), ui.focus == JournalFocus::Member, color);
    render_journal_text_field(f, cols[1], "Topic", ui.editing.topic.as_deref().unwrap_or(""), ui.focus == JournalFocus::Topic, color);
}

fn render_journal_buttons(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let color = app.settings.theme.secondary.to_color();
    let cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33)]).split(area);
    render_journal_button(f, cols[0], "Save", ui.focus == JournalFocus::Save, color);
    render_journal_button(f, cols[1], "New", ui.focus == JournalFocus::New, color);
    render_journal_button(f, cols[2], "Load", ui.focus == JournalFocus::Load, color);
}

fn render_journal_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style);
    let text = Paragraph::new(Line::from(Span::styled(format!(" {label} "), style))).alignment(Alignment::Center).block(block);
    f.render_widget(text, area);
}

fn render_journal_list(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.journal_ui;
    let color = app.settings.theme.primary.to_color();
    let items: Vec<ListItem> = ui.list.iter().enumerate().map(|(i, j)| {
        let date_str = j.created_at.format("%Y-%m-%d").to_string();
        let style = if i == ui.list_selected { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
        ListItem::new(Line::from(Span::styled(format!(" {}  —  {} ", j.title, date_str), style)))
    }).collect();
    let list = List::new(items).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
        .title("Load Journal Entry (↑/↓, Enter to load, Esc to cancel)"));
    f.render_widget(list, area);
}





}
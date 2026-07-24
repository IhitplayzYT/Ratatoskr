pub mod Render{
    use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, Paragraph}};

use crate::conversion::Conversion::CONVERSION_RATES;
use crate::model::app::App::{App, LedgerFocus, LedgerMode};
use crate::model::meta::Meta::Txn_Type;

pub fn render_ledger(f: &mut Frame, area: Rect, app: &App) {
    match app.ledger_ui.mode {
        LedgerMode::Load => render_ledger_list(f, area, app),
        LedgerMode::Edit => render_ledger_edit(f, area, app),
    }
}

/// Vertical "linked-list" style: each txn is a bordered block stacked
/// top-to-bottom, connected visually by a thin connector line.
fn render_ledger_list(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let txns = app.ledger_ui.list.retrive_txn();

    let outer = Block::default().borders(Borders::ALL).border_style(Style::default().fg(color))
        .title(format!("Ledger — balance: {} (↑/↓ select, Enter edit, n = new,r = reload, q = quit)", app.ledger_ui.list.balance()));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    if txns.is_empty() {
        f.render_widget(Paragraph::new("(no transactions yet — press n)"), inner);
        return;
    }

    let node_height = 4u16; // 3-line bordered block + 1 connector row
    let constraints: Vec<Constraint> = (0..txns.len()).map(|_| Constraint::Length(node_height)).collect();
    let rows = Layout::default().direction(Direction::Vertical).constraints(constraints).split(inner);

    for (i, t) in txns.iter().enumerate() {
        if i >= rows.len() { break; } // more txns than visible rows; a real impl would scroll-offset here
        let node_area = Rect { height: node_height.saturating_sub(1), ..rows[i] };
        let selected = i == app.ledger_ui.list_selected;

        let base = if selected {
            Style::default().fg(t.txn_type.color()).add_modifier(Modifier::BOLD | Modifier::REVERSED)
        } else {
            Style::default().fg(t.txn_type.color())
        };

        let title_preview: String = t.item.chars().take(15).collect();
        let sign = match t.txn_type { Txn_Type::CREDIT => "+", Txn_Type::DEBIT => "-", Txn_Type::BLOCKED => "~" };
        let line = Line::from(vec![
            Span::styled(format!(" {:<15} ", title_preview), base),
            Span::styled(format!("{sign}{} ", t.amnt * CONVERSION_RATES.try_read().unwrap().get(&app.settings.currency).unwrap()), base),
            Span::styled(format!("[{}]", t.txn_type.title()), base),
        ]);

        let block = Block::default().borders(Borders::ALL).border_style(base);
        f.render_widget(Paragraph::new(line).block(block), node_area);

        if i + 1 < rows.len() {
            let connector = Rect { y: node_area.y + node_area.height, height: 1, ..node_area };
            f.render_widget(Paragraph::new(Line::from(Span::styled("      │", Style::default().fg(color)))), connector);
        }
    }
}

fn render_ledger_edit(f: &mut Frame, area: Rect, app: &App) {
    let color = app.settings.theme.primary.to_color();
    let ui = &app.ledger_ui;

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
            Constraint::Length(3), Constraint::Length(3), Constraint::Length(3), Constraint::Length(3),
        ])
        .split(area);

    render_ledger_field(f, rows[0], "Item", &ui.editing.item, ui.focus == LedgerFocus::Item, color);
    render_ledger_field(f, rows[1], "Description", ui.editing.desc.as_deref().unwrap_or(""), ui.focus == LedgerFocus::Description, color);
    render_ledger_amount_field(f, rows[2], app);
    render_ledger_txn_type_field(f, rows[3], app);
    render_ledger_frequency_field(f, rows[4], app);
    render_ledger_txn_time_field(f, rows[5], app);

    let btn_cols = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[6]);
    let btn_color = app.settings.theme.secondary.to_color();
    render_ledger_button(f, btn_cols[0], "Add", ui.focus == LedgerFocus::Add, btn_color);
    render_ledger_button(f, btn_cols[1], "Load", ui.focus == LedgerFocus::Load, btn_color);
}

fn render_ledger_field(f: &mut Frame, area: Rect, label: &str, value: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(value).block(Block::default().borders(Borders::ALL).border_style(style).title(label)), area);
}

fn render_ledger_amount_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.ledger_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == LedgerFocus::Amount;
    let base = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let style = if !ui.amnt_valid { Style::default().fg(Color::Red) } else { base };
    let text = format!("{}{}", ui.amnt_input, if ui.amnt_valid { "" } else { " — invalid number" });
    f.render_widget(Paragraph::new(text).style(style).block(Block::default().borders(Borders::ALL).border_style(base).title(format!("Amount in [{}]",app.settings.currency.title()))), area);
}

fn render_ledger_txn_type_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.ledger_ui;
    let focused = ui.focus == LedgerFocus::TxnType;
    let tcolor = ui.editing.txn_type.color();
    let style = if focused { Style::default().fg(tcolor).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(tcolor) };
    f.render_widget(Paragraph::new(ui.editing.txn_type.title()).block(Block::default().borders(Borders::ALL).border_style(style).title("Type (←/→)")), area);
}

fn render_ledger_frequency_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.ledger_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == LedgerFocus::Frequency;
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    f.render_widget(Paragraph::new(ui.editing.freq.title()).block(Block::default().borders(Borders::ALL).border_style(style).title("Frequency (←/→ kind, ↑/↓ value)")), area);
}

fn render_ledger_txn_time_field(f: &mut Frame, area: Rect, app: &App) {
    let ui = &app.ledger_ui;
    let color = app.settings.theme.primary.to_color();
    let focused = ui.focus == LedgerFocus::TxnTime;
    let base = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let suffix = if !ui.txn_time_valid { " — unparsed (fmt: YYYY-MM-DD HH:MM)" } else { "" };
    let text = format!("{}{}", ui.txn_time_input, suffix);
    let style = if !ui.txn_time_valid { Style::default().fg(Color::Red) } else { base };
    f.render_widget(Paragraph::new(text).style(style).block(Block::default().borders(Borders::ALL).border_style(base).title("Txn Time (YYYY-MM-DD HH:MM)")), area);
}

fn render_ledger_button(f: &mut Frame, area: Rect, label: &str, focused: bool, color: Color) {
    let style = if focused { Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED) } else { Style::default().fg(color) };
    let block = Block::default().borders(Borders::ALL).border_style(style);
    f.render_widget(Paragraph::new(Line::from(Span::styled(format!(" {label} "), style))).alignment(Alignment::Center).block(block), area);
}

}
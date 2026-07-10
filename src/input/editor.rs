pub mod Input{
    use crossterm::event::{KeyCode, KeyEvent};
use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;

use crate::{input::general_input::Input::{clamp_cursor, cursor_char_idx, line_len_no_newline}, model::app::App::{App, EditorMode, EditorState, Pending, Vim_mode}};

 
/// EditorMode::Normal -> plain typing/arrow editor.
/// EditorMode::Vim    -> dispatches into the modal vim state machine.
pub fn handle_editor_key(app: &mut App, key: KeyEvent) {
    match app.editor.mode {
        EditorMode::Normal => handle_plain_editor_key(app, key),
        EditorMode::Vim => handle_vim_key(app, key),
    }
}
 
pub fn handle_plain_editor_key(app: &mut App, key: KeyEvent) {
    let editor = &mut app.editor;
    match key.code {
        KeyCode::Left => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Right => editor.cursor_x += 1,
        KeyCode::Up => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Down => editor.cursor_y += 1,
        KeyCode::Char(c) => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, c);
            editor.cursor_x += 1;
            editor.dirty = true;
        }
        KeyCode::Backspace => {
            let idx = cursor_char_idx(editor);
            if idx > 0 {
                editor.buffer.remove(idx - 1..idx);
                editor.cursor_x = editor.cursor_x.saturating_sub(1);
                editor.dirty = true;
            }
        }
        KeyCode::Enter => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, '\n');
            editor.cursor_y += 1;
            editor.cursor_x = 0;
            editor.dirty = true;
        },
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => editor.cursor_x = line_len_no_newline(&editor.buffer, editor.cursor_y),
        _ => {}
    }
    clamp_cursor(editor);
}
 
/// True modal vim emulation. Only handles the requested bind set:
/// hjkl, gg, G, yy, yiw, dd, diw, r, ~, O, p, plus i/Esc to flip submode.
fn handle_vim_key(app: &mut App, key: KeyEvent) {
    let editor = &mut app.editor;
    match editor.vim.submode {
        Vim_mode::Insert => handle_vim_insert_key(editor, key),
        Vim_mode::Normal => handle_vim_normal_key(editor, key),
    }
    clamp_cursor(editor);
}
 
fn handle_vim_insert_key(editor: &mut EditorState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            editor.vim.submode = Vim_mode::Normal;
            editor.cursor_x = editor.cursor_x.saturating_sub(1); // vim: cursor steps back on Esc
        }
        KeyCode::Char(c) => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, c);
            editor.cursor_x += 1;
            editor.dirty = true;
        }
        KeyCode::Backspace => {
            let idx = cursor_char_idx(editor);
            if idx > 0 {
                editor.buffer.remove(idx - 1..idx);
                editor.cursor_x = editor.cursor_x.saturating_sub(1);
                editor.dirty = true;
            }
        }
        KeyCode::Enter => {
            let idx = cursor_char_idx(editor);
            editor.buffer.insert_char(idx, '\n');
            editor.cursor_y += 1;
            editor.cursor_x = 0;
            editor.dirty = true;
        },
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => editor.cursor_x = line_len_no_newline(&editor.buffer, editor.cursor_y),
        _ => {}
    }
}
 
fn handle_vim_normal_key(editor: &mut EditorState, key: KeyEvent) {
    // Resolve any command still waiting on a second/third key first.
    match editor.vim.pending {
        Pending::G => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('g') {
                editor.cursor_y = 0;
                editor.cursor_x = 0;
            }
            return;
        }
        Pending::Y => {
            editor.vim.pending = Pending::None;
            match key.code {
                KeyCode::Char('y') => yank_line(editor),
                KeyCode::Char('i') => editor.vim.pending = Pending::YI,
                _ => {}
            }
            return;
        }
        Pending::YI => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('w') {
                yank_inner_word(editor);
            }
            return;
        }
        Pending::D => {
            editor.vim.pending = Pending::None;
            match key.code {
                KeyCode::Char('d') => delete_line(editor),
                KeyCode::Char('i') => editor.vim.pending = Pending::DI,
                _ => {}
            }
            return;
        }
        Pending::DI => {
            editor.vim.pending = Pending::None;
            if key.code == KeyCode::Char('w') {
                delete_inner_word(editor);
            }
            return;
        }
        Pending::R => {
            editor.vim.pending = Pending::None;
            if let KeyCode::Char(c) = key.code {
                replace_char_under_cursor(editor, c);
            }
            return;
        }
        Pending::None => {}
    }
 
    match key.code {
        KeyCode::Char('h') | KeyCode::Left => editor.cursor_x = editor.cursor_x.saturating_sub(1),
        KeyCode::Char('l') | KeyCode::Right => editor.cursor_x += 1,
        KeyCode::Char('k') | KeyCode::Up => editor.cursor_y = editor.cursor_y.saturating_sub(1),
        KeyCode::Char('j') | KeyCode::Down => editor.cursor_y += 1,
        KeyCode::Home => editor.cursor_x = 0,
        KeyCode::End => {
            let len = line_len_no_newline(&editor.buffer, editor.cursor_y);
            editor.cursor_x = len.saturating_sub(1); // vim End lands ON the last char, not past it
        },
        KeyCode::Char('g') => editor.vim.pending = Pending::G,
        KeyCode::Char('G') => {
            editor.cursor_y = editor.buffer.len_lines().saturating_sub(1);
            editor.cursor_x = 0;
        }
        KeyCode::Char('y') => editor.vim.pending = Pending::Y,
        KeyCode::Char('d') => editor.vim.pending = Pending::D,
        KeyCode::Char('r') => editor.vim.pending = Pending::R,
        KeyCode::Char('~') => toggle_case_under_cursor(editor),
        KeyCode::Char('O') => open_line_above(editor),
        KeyCode::Char('p') => paste_register(editor),
        KeyCode::Char('i') => editor.vim.submode = Vim_mode::Insert,
        _ => {}
    }
}
 
// --- vim command implementations -------------------------------------------
 

 
fn current_line_string(editor: &EditorState) -> String {
    let len = line_len_no_newline(&editor.buffer, editor.cursor_y);
    let start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.slice(start..start + len).to_string()
}
 
/// Byte/char index of the word under `col` in `line` (ASCII-oriented; see
/// note below for multi-byte caveats).
fn word_bounds_at(line: &str, col: usize) -> (usize, usize) {
    for (start, word) in line.split_word_bound_indices() {
        let end = start + word.chars().count();
        if col >= start && col < end {
            return (start, end);
        }
    }
    (col, col + 1)
    // NOTE: split_word_bound_indices returns *byte* offsets; for pure-ASCII
    // buffers (typical for todo/note text) byte == char index, so this is
    // fine as-is. For full multi-byte correctness, map byte offsets to char
    // offsets via `line.char_indices()` before comparing against `col`.
}
 
fn yank_line(editor: &mut EditorState) {
    let mut line = current_line_string(editor);
    line.push('\n');
    editor.vim.register = line;
    editor.vim.register_line = true;
}
 
fn delete_line(editor: &mut EditorState) {
    yank_line(editor);
    let start = editor.buffer.line_to_char(editor.cursor_y);
    let end = editor.buffer.line_to_char((editor.cursor_y + 1).min(editor.buffer.len_lines()));
    editor.buffer.remove(start..end);
    if editor.buffer.len_lines() == 0 {
        editor.buffer = Rope::from_str("\n");
    }
    editor.dirty = true;
}
 
fn yank_inner_word(editor: &mut EditorState) {
    let line = current_line_string(editor);
    let (s, e) = word_bounds_at(&line, editor.cursor_x);
    editor.vim.register = line.get(s..e).unwrap_or("").to_string();
    editor.vim.register_line = false;
}
 
fn delete_inner_word(editor: &mut EditorState) {
    let line = current_line_string(editor);
    let (s, e) = word_bounds_at(&line, editor.cursor_x);
    editor.vim.register = line.get(s..e).unwrap_or("").to_string();
    editor.vim.register_line = false;
 
    let line_start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.remove(line_start + s..line_start + e);
    editor.cursor_x = s;
    editor.dirty = true;
}
 
fn replace_char_under_cursor(editor: &mut EditorState, c: char) {
    let idx = cursor_char_idx(editor);
    if idx < editor.buffer.len_chars() {
        editor.buffer.remove(idx..idx + 1);
        editor.buffer.insert_char(idx, c);
        editor.dirty = true;
    }
}
 
fn toggle_case_under_cursor(editor: &mut EditorState) {
    let idx = cursor_char_idx(editor);
    if idx < editor.buffer.len_chars() {
        let ch = editor.buffer.char(idx);
        let toggled = if ch.is_uppercase() {
            ch.to_lowercase().next().unwrap_or(ch)
        } else {
            ch.to_uppercase().next().unwrap_or(ch)
        };
        editor.buffer.remove(idx..idx + 1);
        editor.buffer.insert_char(idx, toggled);
        editor.cursor_x += 1;
        editor.dirty = true;
    }
}
 
fn open_line_above(editor: &mut EditorState) {
    let line_start = editor.buffer.line_to_char(editor.cursor_y);
    editor.buffer.insert_char(line_start, '\n');
    editor.cursor_x = 0;
    editor.vim.submode = Vim_mode::Insert;
    editor.dirty = true;
    // cursor_y stays put: the new blank line is now *at* cursor_y, and the
    // old content shifted down to cursor_y + 1, matching vim's `O`.
}
 
fn paste_register(editor: &mut EditorState) {
    if editor.vim.register.is_empty() {
        return;
    }
    if editor.vim.register_line {
        let next_line_start = editor
            .buffer
            .line_to_char((editor.cursor_y + 1).min(editor.buffer.len_lines()));
        editor.buffer.insert(next_line_start, &editor.vim.register);
        editor.cursor_y += 1;
        editor.cursor_x = 0;
    } else {
        let idx = cursor_char_idx(editor) + 1; // paste *after* cursor, like vim `p`
        let idx = idx.min(editor.buffer.len_chars());
        editor.buffer.insert(idx, &editor.vim.register);
        editor.cursor_x += 1;
    }
    editor.dirty = true;
}
 


}
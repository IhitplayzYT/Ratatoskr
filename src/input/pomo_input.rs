pub mod Input{
    use std::{process::Command, time::{Duration, Instant}};

use crossterm::event::{KeyCode, KeyEvent};

use crate::model::app::App::{App, PomoFocus};




pub fn tick_pomodoro(app: &mut App) {
    let pomo = &mut app.features.pomodero;
    if !pomo.running || pomo.paused {
        // reset the reference point so we don't "catch up" a bunch of
        // seconds the moment it's resumed
        app.pomo_last_second = Instant::now();
        return;
    }

    if app.pomo_last_second.elapsed() >= Duration::from_secs(1) {
        app.pomo_last_second = Instant::now();
        if pomo.duration_left > 0 {
            pomo.duration_left -= 1;
        }
        if pomo.duration_left == 0 {
            pomo.running = false;
            pomo.paused = false;
            pomo_notif();
        }
    }
}

pub fn pomo_notif(){
Command::new("notify-send").arg("Pomodoro Timer complete").arg("Time Up!Take a Break!").status().unwrap();
}

pub fn handle_pomodoro_key(app: &mut App, key: KeyEvent) {
    if key.code == KeyCode::Char('q') {
        app.is_quit = true;
        return;
    }

    let pomo = &mut app.features.pomodero;

    match key.code {
        KeyCode::Left => {
            pomo.focus = prev_pomo_focus(pomo.focus);
            return;
        }
        KeyCode::Right => {
            pomo.focus = next_pomo_focus(pomo.focus);
            return;
        }
        _ => {}
    }

    match pomo.focus {
        PomoFocus::Hour | PomoFocus::Minute | PomoFocus::Second => {
            if pomo.running {
                return;
            }
            match key.code {
                KeyCode::Up => bump_pomo_component(pomo, pomo.focus, 1),
                KeyCode::Down => bump_pomo_component(pomo, pomo.focus, -1),
                _ => {}
            }
        }
        PomoFocus::StartPause => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                if !pomo.running {
                    pomo.running = true;
                    pomo.paused = false;
                    app.pomo_last_second = Instant::now();
                } else if pomo.paused {
                    pomo.paused = false;
                    app.pomo_last_second = Instant::now();
                } else {
                    pomo.paused = true;
                }
            }
        }
        PomoFocus::Reset => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                pomo.duration_left = pomo.duration;
                app.pomo_last_second = Instant::now();
            }
        }
        PomoFocus::Cancel => {
            if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                pomo.running = false;
                pomo.paused = false;
                pomo.duration_left = pomo.duration;
            }
        }
    }
}

pub fn bump_pomo_component(pomo: &mut crate::model::pomodero::Pomodero::Pomodero, focus: PomoFocus, delta: i64) {
    let mut h = (pomo.duration / 3600) as i64;
    let mut m = ((pomo.duration % 3600) / 60) as i64;
    let mut s = (pomo.duration % 60) as i64;

    match focus {
        PomoFocus::Hour => h = (h + delta).rem_euclid(24),
        PomoFocus::Minute => m = (m + delta).rem_euclid(60),
        PomoFocus::Second => s = (s + delta).rem_euclid(60),
        _ => {}
    }

    pomo.duration = (h * 3600 + m * 60 + s) as usize;
    pomo.duration_left = pomo.duration;
}



pub fn next_pomo_focus(f: PomoFocus) -> PomoFocus {
    match f {
        PomoFocus::Hour => PomoFocus::Minute,
        PomoFocus::Minute => PomoFocus::Second,
        PomoFocus::Second => PomoFocus::StartPause,
        PomoFocus::StartPause => PomoFocus::Reset,
        PomoFocus::Reset => PomoFocus::Cancel,
        PomoFocus::Cancel => PomoFocus::Hour,
    }
}

pub fn prev_pomo_focus(f: PomoFocus) -> PomoFocus {
    match f {
        PomoFocus::Hour => PomoFocus::Cancel,
        PomoFocus::Minute => PomoFocus::Hour,
        PomoFocus::Second => PomoFocus::Minute,
        PomoFocus::StartPause => PomoFocus::Second,
        PomoFocus::Reset => PomoFocus::StartPause,
        PomoFocus::Cancel => PomoFocus::Reset,
    }
}

}
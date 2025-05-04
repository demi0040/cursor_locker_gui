use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::time::Duration;

/// Enables raw mode for instant key reading.
pub fn enable_raw() {
    enable_raw_mode().unwrap();
}

/// Disables raw mode.
pub fn disable_raw() {
    disable_raw_mode().unwrap();
}

/// Polls for key events and returns Some(KeyCode) if a relevant key is pressed.
pub fn poll_key_event(timeout_ms: u64) -> Option<KeyCode> {
    if event::poll(Duration::from_millis(timeout_ms)).unwrap() {
        if let CEvent::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read().unwrap() {
            return Some(code);
        }
    }
    None
}
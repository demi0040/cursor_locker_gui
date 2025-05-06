mod cursor;
mod display;
mod input;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use winit::event_loop::EventLoop;
use crossterm::event::KeyCode;

fn main() {
    // Create an event loop (required by winit)
    let event_loop = EventLoop::new().unwrap();
    let monitors = display::list_monitors(&event_loop);

    if monitors.is_empty() {
        println!("No displays found.");
        return;
    }

    let monitor = match display::select_monitor(&monitors) {
        Some(m) => m,
        None => return,
    };

    let (lock_x, lock_y, lock_w, lock_h) = display::select_region(&monitor);

    println!(
        "Locking cursor to region: ({}, {}) size {}x{}",
        lock_x, lock_y, lock_w, lock_h
    );

    // Atomic flags for running and paused state
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let paused = Arc::new(AtomicBool::new(false));
    let paused_clone = paused.clone();

    // Spawn a background thread to keep the cursor locked
    thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            if !paused_clone.load(Ordering::Relaxed) {
                cursor::lock_cursor_to_rect(lock_x, lock_y, lock_w, lock_h);
                cursor::force_cursor_inside(lock_x, lock_y, lock_w, lock_h);
            } else {
                cursor::unlock_cursor();
            }
            thread::sleep(Duration::from_millis(10)); // 20 times per second
        }
        // Unlock cursor when exiting
        cursor::unlock_cursor();
    });

    println!("Press 'z' to pause/resume cursor lock, 'q' to quit.");

    input::enable_raw();

    // Main loop: listen for key events
    while running.load(Ordering::Relaxed) {
        if let Some(code) = input::poll_key_event(100) {
            match code {
                KeyCode::Char('q') => {
                    running.store(false, Ordering::Relaxed);
                    break;
                }
                KeyCode::Char('z') => {
                    let was_paused = paused.load(Ordering::Relaxed);
                    paused.store(!was_paused, Ordering::Relaxed);
                    if was_paused {
                        println!("[z] Lock re-activated.");
                    } else {
                        println!("[z] Lock paused.");
                    }
                }
                _ => {}
            }
        }
    }

    // Clean up
    input::disable_raw();
    running.store(false, Ordering::Relaxed);
    thread::sleep(Duration::from_millis(100));
    println!("Exiting. Cursor lock released.");
}
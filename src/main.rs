
use std::io::{self, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use winit::{
    event_loop::EventLoop,
    monitor::MonitorHandle,
};

// Add crossterm for non-blocking key input
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent, KeyEventKind},
    terminal::{enable_raw_mode, disable_raw_mode},
};

#[cfg(target_os = "windows")]
use winapi::um::winuser::ClipCursor;
#[cfg(target_os = "windows")]
use winapi::shared::windef::RECT;

#[cfg(target_os = "windows")]
fn lock_cursor_to_rect(x: i32, y: i32, width: i32, height: i32) {
    let rect = RECT {
        left: x,
        top: y,
        right: x + width,
        bottom: y + height,
    };
    unsafe {
        ClipCursor(&rect);
    }
}

#[cfg(target_os = "windows")]
fn unlock_cursor() {
    unsafe {
        ClipCursor(std::ptr::null());
    }
}

#[cfg(not(target_os = "windows"))]
fn lock_cursor_to_rect(_x: i32, _y: i32, _width: i32, _height: i32) {
    println!("Cursor locking is only implemented for Windows in this example.");
}

#[cfg(not(target_os = "windows"))]
fn unlock_cursor() {}

fn main() {
    // Create an event loop (required by winit)
    let event_loop = EventLoop::new().unwrap();
    let monitors: Vec<MonitorHandle> = event_loop.available_monitors().collect();

    if monitors.is_empty() {
        println!("No displays found.");
        return;
    }

    println!("Connected displays:");
    for (i, monitor) in monitors.iter().enumerate() {
        let name = monitor.name().unwrap_or_else(|| "Unknown".to_string());
        let size = monitor.size();
        println!(
            "{}: {} ({}x{} at {},{})",
            i,
            name,
            size.width,
            size.height,
            monitor.position().x,
            monitor.position().y
        );
    }

    print!("Select a display to lock the cursor to (0-{}): ", monitors.len() - 1);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let selection: usize = match input.trim().parse() {
        Ok(num) if num < monitors.len() => num,
        _ => {
            println!("Invalid selection.");
            return;
        }
    };

    let monitor = &monitors[selection];
    let pos = monitor.position();
    let size = monitor.size();

    println!(
        "Selected display: {} at ({}, {}) size {}x{}",
        monitor.name().unwrap_or_else(|| "Unknown".to_string()),
        pos.x,
        pos.y,
        size.width,
        size.height
    );

    println!("Lock cursor to:");
    println!("1. Full display");
    println!("2. Left half");
    println!("3. Right half");
    println!("4. Custom rectangle (enter coordinates manually)");
    print!("Choose an option (1-4): ");
    io::stdout().flush().unwrap();

    let mut region_input = String::new();
    io::stdin().read_line(&mut region_input).unwrap();
    let region_choice = region_input.trim();

    let (lock_x, lock_y, lock_w, lock_h) = match region_choice {
        "1" => (pos.x, pos.y, size.width as i32, size.height as i32),
        "2" => (pos.x, pos.y, (size.width as i32) / 2, size.height as i32),
        "3" => (
            pos.x + (size.width as i32) / 2,
            pos.y,
            (size.width as i32) / 2,
            size.height as i32,
        ),
        "4" => {
            println!("Enter left (x): ");
            io::stdout().flush().unwrap();
            let mut x_input = String::new();
            io::stdin().read_line(&mut x_input).unwrap();
            let x = x_input.trim().parse::<i32>().unwrap_or(pos.x);

            println!("Enter top (y): ");
            io::stdout().flush().unwrap();
            let mut y_input = String::new();
            io::stdin().read_line(&mut y_input).unwrap();
            let y = y_input.trim().parse::<i32>().unwrap_or(pos.y);

            println!("Enter width: ");
            io::stdout().flush().unwrap();
            let mut w_input = String::new();
            io::stdin().read_line(&mut w_input).unwrap();
            let w = w_input.trim().parse::<i32>().unwrap_or(size.width as i32);

            println!("Enter height: ");
            io::stdout().flush().unwrap();
            let mut h_input = String::new();
            io::stdin().read_line(&mut h_input).unwrap();
            let h = h_input.trim().parse::<i32>().unwrap_or(size.height as i32);

            (x, y, w, h)
        }
        _ => {
            println!("Invalid option, defaulting to full display.");
            (pos.x, pos.y, size.width as i32, size.height as i32)
        }
    };

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
                lock_cursor_to_rect(lock_x, lock_y, lock_w, lock_h);
            } else {
                unlock_cursor();
            }
            thread::sleep(Duration::from_millis(50)); // 20 times per second
        }
        // Unlock cursor when exiting
        unlock_cursor();
    });

    println!("Press 'z' to pause/resume cursor lock, 'q' to quit.");

    // Enable raw mode for instant key reading
    enable_raw_mode().unwrap();

    // Main loop: listen for key events
    while running.load(Ordering::Relaxed) {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let CEvent::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) = event::read().unwrap() {
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
    }

    // Clean up
    disable_raw_mode().unwrap();
    running.store(false, Ordering::Relaxed);
    thread::sleep(Duration::from_millis(100));
    println!("Exiting. Cursor lock released.");
}

use std::io::{self, Write};
use winit::{
    event_loop::EventLoop,
    monitor::MonitorHandle,
};

/// Lists available monitors and returns a vector of MonitorHandle.
pub fn list_monitors(event_loop: &EventLoop<()>) -> Vec<MonitorHandle> {
    event_loop.available_monitors().collect()
}

/// Prompts the user to select a monitor and returns the selected MonitorHandle.
pub fn select_monitor(monitors: &[MonitorHandle]) -> Option<MonitorHandle> {
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
            return None;
        }
    };

    Some(monitors[selection].clone())
}

/// Prompts the user to select a region and returns (x, y, width, height).
pub fn select_region(monitor: &MonitorHandle) -> (i32, i32, i32, i32) {
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

    match region_choice {
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
    }
}
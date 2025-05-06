
/// Platform-specific cursor locking/unlocking logic.

#[cfg(target_os = "windows")]
use winapi::um::winuser::{ClipCursor, SetCursorPos, GetCursorPos};
#[cfg(target_os = "windows")]
use winapi::shared::windef::{RECT, POINT};

/// Locks the cursor to the specified rectangle (Windows only).
#[cfg(target_os = "windows")]
pub fn lock_cursor_to_rect(x: i32, y: i32, width: i32, height: i32) {
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

/// Unlocks the cursor (Windows only).
#[cfg(target_os = "windows")]
pub fn unlock_cursor() {
    unsafe {
        ClipCursor(std::ptr::null());
    }
}

/// Forces the cursor to stay inside the specified rectangle (Windows only).
/// If the cursor escapes, it is repositioned to the center of the region.
#[cfg(target_os = "windows")]
pub fn force_cursor_inside(x: i32, y: i32, width: i32, height: i32) {
    let mut pt = POINT { x: 0, y: 0 };
    unsafe {
        if GetCursorPos(&mut pt) != 0 {
            if pt.x < x || pt.x > x + width || pt.y < y || pt.y > y + height {
                // Move cursor to the center of the region
                SetCursorPos(x + width / 2, y + height / 2);
            }
        }
    }
}

/// Stub for non-Windows platforms.
#[cfg(not(target_os = "windows"))]
pub fn lock_cursor_to_rect(_x: i32, _y: i32, _width: i32, _height: i32) {
    println!("Cursor locking is only implemented for Windows in this example.");
}

#[cfg(not(target_os = "windows"))]
pub fn unlock_cursor() {}

#[cfg(not(target_os = "windows"))]
pub fn force_cursor_inside(_x: i32, _y: i32, _width: i32, _height: i32) {}

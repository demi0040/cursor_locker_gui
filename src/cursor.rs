/// Platform-specific cursor locking/unlocking logic.

#[cfg(target_os = "windows")]
use winapi::um::winuser::ClipCursor;
#[cfg(target_os = "windows")]
use winapi::shared::windef::RECT;

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

/// Stub for non-Windows platforms.
#[cfg(not(target_os = "windows"))]
pub fn lock_cursor_to_rect(_x: i32, _y: i32, _width: i32, _height: i32) {
    println!("Cursor locking is only implemented for Windows in this example.");
}

#[cfg(not(target_os = "windows"))]
pub fn unlock_cursor() {}
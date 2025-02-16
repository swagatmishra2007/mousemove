// src/platform/platform_macos.rs
#[cfg(target_os = "macos")]

use crate::platform::MouseController;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWarpMouseCursorPosition(newPosition: CGPoint);
    fn CGGetCurrentMouseLocation() -> CGPoint;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct CGPoint {
    x: f64,
    y: f64,
}

/// macOS-specific implementation of the MouseController trait.
pub struct MacMouseController {}

impl MacMouseController {
    /// Creates a new MacMouseController instance.
    pub fn new() -> Self {
        MacMouseController {}
    }
}

impl MouseController for MacMouseController {
    /// Gets the current mouse cursor position on macOS using CoreGraphics framework.
    fn get_mouse_position(&self) -> (i32, i32) {
        unsafe {
            let point: CGPoint = CGGetCurrentMouseLocation();
            (point.x as i32, point.y as i32)
        }
    }

    /// Sets the mouse cursor position on macOS using CoreGraphics framework.
    fn set_mouse_position(&self, x: i32, y: i32) {
        unsafe {
            CGWarpMouseCursorPosition(CGPoint { x: x as f64, y: y as f64 });
        }
    }
}
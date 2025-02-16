// src/platform/mod.rs
pub mod platform_windows;
pub mod platform_macos;

/// Defines the interface for platform-specific mouse controller implementations.
pub trait MouseController {
    /// Gets the current mouse cursor position as (x, y) coordinates.
    fn get_mouse_position(&self) -> (i32, i32);

    /// Sets the mouse cursor position to the specified (x, y) coordinates.
    fn set_mouse_position(&self, x: i32, y: i32);
}

// Function to get the appropriate MouseController implementation based on the target OS.
pub fn create_mouse_controller() -> Box<dyn MouseController> {
    #[cfg(target_os = "windows")]
    {
        // Corrected: Function name to snake_case, Box::new() used correctly
        Box::new(crate::platform::platform_windows::WindowsMouseController::new())
    }
    #[cfg(target_os = "macos")]
    {
        // Corrected: Using MacMouseController (not MacOSMouseController), Box::new() used correctly
        Box::new(crate::platform::platform_macos::MacMouseController::new())
    }
    // #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    // {
    //     Box::new(crate::platform::platform_unsupported::UnsupportedMouseController::new())
    // }
}
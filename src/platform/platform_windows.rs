#[cfg(target_os = "windows")]
use crate::platform::MouseController; // Keep the use statement OUTSIDE the #[cfg]

#[cfg(target_os = "windows")]
// This is windows specific implementation
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[cfg(target_os = "windows")]
//import functions from user32.dll
#[link(name = "user32")]
extern "system" {
    fn GetCursorPos(lpPoint: *mut POINT) -> i32;
    fn SetCursorPos(X: i32, Y: i32) -> i32;
}

#[cfg(target_os = "windows")]
pub struct WindowsMouseController;

#[cfg(target_os = "windows")]
impl WindowsMouseController {
    pub fn new() -> Self {
        WindowsMouseController
    }
}

#[cfg(target_os = "windows")]
// implement the MouseController trait for WindowsMouseController
impl MouseController for WindowsMouseController {
    fn get_mouse_position(&self) -> (i32, i32) {
        let mut point: POINT = POINT { x: 0, y: 0 };
        unsafe {
            // call the windows api function GetCursorPos
            GetCursorPos(&mut point);
        }
        // return the value
        (point.x, point.y)
    }

    fn set_mouse_position(&self, x: i32, y: i32) {
        unsafe {
            // call the windows api function SetCursorPos
            SetCursorPos(x, y);
        }
    }
}
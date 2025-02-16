mod platform;
use std::{thread, time};

fn main() {
    println!("Mouse over application started. press ctrl +c to exit.");

    let mouse_controller = platform::create_mouse_controller();
    const PIXELS_TO_MOVE:i32 = 200;
    let mut move_right = true;

    loop {
        let (current_x, current_y) = mouse_controller.get_mouse_position();
        let new_x;
        if move_right {
            new_x = current_x + PIXELS_TO_MOVE;
        }
        else{
            new_x = current_x - PIXELS_TO_MOVE;
        }
        mouse_controller.set_mouse_position(new_x, current_y);
        let (final_x, final_y) = mouse_controller.get_mouse_position();
        println!("Mouse position: ({}, {})",final_x, final_y);
        move_right = !move_right;
        thread::sleep(time::Duration::from_secs(5));
    }


}

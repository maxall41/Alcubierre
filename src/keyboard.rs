use raylib::ffi::{IsKeyDown, IsKeyPressed, IsKeyReleased, IsKeyUp};

pub fn is_key_pressed(key: i64) -> bool {
    unsafe {
        return IsKeyPressed(key as i32);
    }
}

pub fn is_key_released(key: i64) -> bool {
    unsafe {
        return IsKeyReleased(key as i32);
    }
}

pub fn is_key_up(key: i64) -> bool {
    unsafe {
        return IsKeyUp(key as i32);
    }
}

pub fn is_key_down(key: i64) -> bool {
    unsafe {
        return IsKeyDown(key as i32);
    }
}
static mut ENABLED: bool = false;

pub fn enable() -> bool {
    unsafe {
        ENABLED = true;
    }
    true
}

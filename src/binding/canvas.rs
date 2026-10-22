use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    pub fn poll_width(id: i32) -> i32;
    pub fn poll_height(id: i32) -> i32;
    pub fn draw(id: i32, color: u64, x: i32, y: i32) -> bool;
    pub fn flush(id: i32, width: i32, height: i32) -> bool;
    pub fn reset(id: i32) -> bool;
}

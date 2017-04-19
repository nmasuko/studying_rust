use std::ptr::read;
fn func(ptr: *const i64) {
    unsafe {
        println!("{}", read::<i64>(ptr));
    }
}
fn main() {
    let a = [1i64, 2i64, 3i64];
    let ptr = &a[1] as *const i64;
    func(ptr);
}
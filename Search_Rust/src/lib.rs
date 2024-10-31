
#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn my_test(x: i32, y: i32) -> i32 {
    x * y
}
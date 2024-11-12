//use std::slice;
//use ndarray::Array2;
use winapi::um::winuser::{MessageBoxW, MB_OK, MB_ICONINFORMATION};

use widestring::U16CString;
use std::ptr;

const COLUMNS: usize = 10;
const ROWS: usize = 20;

#[repr(C)]
pub struct SreachField {
    flat_data : Vec<i32>,
    array_ptr :*mut i32,
}

impl SreachField{
    pub fn new() -> SreachField {
        SreachField {
            flat_data: Vec::new(),
            array_ptr: std::ptr::null_mut(),
        }
    }
    fn field_read(&mut self, ptr: *const *const i32) -> *mut i32 {
        self.flat_data.clear();
        self.flat_data.resize(ROWS * COLUMNS, 0);
        for num in 0..ROWS * COLUMNS {
            unsafe {self.flat_data[num] = **ptr.add(num)}
        }
        self.array_ptr = self.flat_data.as_mut_ptr();

        self.array_ptr
    }
}


#[no_mangle]
pub unsafe extern "C" fn show_message_box() -> () {
        unsafe {
            // 固定のタイトルとメッセージ
            let title = U16CString::from_str("Rust Message").unwrap();
            let message = U16CString::from_str("This is a message box created in Rust.").unwrap();

            // メッセージボックスを表示
            MessageBoxW(
                ptr::null_mut(), // NULLの代わり
                message.as_ptr(),
                title.as_ptr(),
                MB_OK | MB_ICONINFORMATION,
            );
        }
    }

#[no_mangle]
pub unsafe extern "C" fn SearchMove() -> SreachField{
    let player = SreachField::new();
    player
}
// マクロの定義
#[macro_export]
macro_rules! DllexportFn {
    (
        $name:ident (&self, $($arg:ident : $type:ty),*) -> $ret:ty $body:block
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(instance: &SreachField, $($arg: $type),*) -> $ret {
            instance.$name($($arg),*)
        }
    };

    (
        $name:ident (&mut self, $($arg:ident : $type:ty),*) -> $ret:ty $body:block
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(instance: &mut SreachField, $($arg: $type),*) -> $ret {
            instance.$name($($arg),*)
        }
    };

    (
        $name:ident ($($arg:ident : $type:ty),*) -> $ret:ty $body:block
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $type),*) -> $ret $body
    };
}

// 構造体の定義
struct SreachField {
    flat_data: Vec<i32>,
    array_ptr: *mut i32,
}

// 構造体の実装
impl SreachField {
    // この関数は内部でマクロを使って生成される関数から呼び出される
    fn field_rust(&mut self, ptr: *const *const i32) -> *mut i32 {
        self.flat_data.clear();
        self.flat_data.resize(10 * 10, 0); // サイズは例
        for num in 0..(10 * 10) {
            unsafe {
                self.flat_data[num] = *ptr.add(num) as i32;
            }
        }

        self.array_ptr = self.flat_data.as_mut_ptr();
        self.array_ptr
    }
}

// マクロを使用してエクスポート関数を定義

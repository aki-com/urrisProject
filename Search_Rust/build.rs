fn main() {
    //C++のコードをC#で使えるようにする
    cc::Build::new()
        .cpp(true)
        .file("src/lib.rs")
        .compile("Search");




    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("Search")
        .generate_csharp_file("src/NativeMethods.g.cs")
        .unwrap();
}
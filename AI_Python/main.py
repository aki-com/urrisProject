import ctypes

Dll=ctypes.CDLL("../Search_Rust/target/debug/search_rust.dll")
Dll.test_add.argtypes = (ctypes.c_int, ctypes.c_int)
Dll.test_add.restype = ctypes.c_int
print(Dll.test_add(3,2))



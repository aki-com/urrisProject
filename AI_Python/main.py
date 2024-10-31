from ctypes import *
import os


# DLLを読み込む


dll_path=r"C:\Users\aki31\OneDrive\ドキュメント\urris_AI\python\TetrisLogic.dll"

if os.path.exists(dll_path):
    try:
        dll = pydll.LoadLibrary(dll_path)
        print("DLL loaded successfully.")
    except OSError as e:
        print("Error loading DLL:", e)
else:
    print("DLL path does not exist:", dll_path)
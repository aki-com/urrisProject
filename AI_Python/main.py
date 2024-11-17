import ctypes
import tkinter as tk
from tkinter import Canvas
#nuitka --standalone --enable-plugin=tk-inter --windows-console-mode=disable --onefile main.py

DLL = ctypes.CDLL("SearchRust.dll")


# フィールドのサイズ
FIELD_WIDTH = 10
FIELD_HEIGHT = 20
CELL_SIZE = 20  # セルのサイズ（ピクセル）

# GUIの設定
root = tk.Tk()
root.title("Tetris Field")
canvas = Canvas(root, width=FIELD_WIDTH * CELL_SIZE, height=FIELD_HEIGHT * CELL_SIZE)
canvas.pack()
Dll = ctypes.CDLL("Tetrislogic.Dll")


# 配列を取得してGUI上に描画
def display_field():
    # Rustから配列を取得
    array_ptr = Dll.get_array()
    field_data = [array_ptr[i] for i in range(FIELD_WIDTH * FIELD_HEIGHT)]
    field_data = [1] * FIELD_WIDTH * FIELD_HEIGHT
    # 配列の内容をGUIで表示
    for y in range(FIELD_HEIGHT):
        for x in range(FIELD_WIDTH):
            value = field_data[y * FIELD_WIDTH + x]
            color = "black" if value == 0 else "blue"

    root.after(500, display_field)

# フィールドの表示を初期化
display_field()

# Tkinterのメインループ開始
root.mainloop()



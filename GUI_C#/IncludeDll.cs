using System;
using System.Runtime.InteropServices;


static public class IncludeDll{
    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr CreateTetrisGame();
    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void DestroyTetrisGame(IntPtr ptr);

    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int TetrisGameRun(IntPtr ptr);
    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int MoveMino(IntPtr ptr,char key);
    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr Field_Read(IntPtr ptr);
    [DllImport("tetrisLogic.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr Next_Read(IntPtr ptr);
    [DllImport("search_rust.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr field_rust(IntPtr ptr);
    }
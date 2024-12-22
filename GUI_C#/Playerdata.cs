    using System;

    using System.Runtime.InteropServices;
    using RGiesecke.DllExport;
    using System.Security.Cryptography.X509Certificates;
    using System.Windows.Forms;

    public class TetrisPlayer
    {
        static IntPtr Instance;
        IntPtr fieldPtr;
        IntPtr NextPtr;
        const int Columns = 10; // カラム数
        const int Rows = 20;    // 行数 

        public string Name;
        public int[,] Field { get; private set; } 
        public int[] NextMino { get; private set; }
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
        [DllExport("GetInstancePointer", CallingConvention = CallingConvention.Cdecl)]
        public static IntPtr GetInstancePointer()
        {
            // C++のCreateTetrisGameから取得したポインタを返す
            if (Instance == IntPtr.Zero)
            {
                Instance = CreateTetrisGame();
            }
            return Instance;
        }

        public TetrisPlayer(int i)
        {
            Instance = CreateTetrisGame();
            fieldPtr = Field_Read(Instance);
            NextPtr = Next_Read(Instance);
            Name = "Player" + i;
            Field = new int[Rows, Columns];
            NextMino = new int[5];
        }
        public void UpdateField()
        {

            NextPtr = Next_Read(Instance);
            Marshal.Copy(NextPtr, NextMino, 0, 5);

            int[] rawData = new int[Columns*Rows];

            for (int row = 0; row < Rows; row++)
            {
                IntPtr rowPtr = Marshal.ReadIntPtr(fieldPtr, (row+9) *Columns* IntPtr.Size);
                Marshal.Copy(rowPtr, rawData, 0, Columns);
                for (int col = 0; col < Columns; col++)
                {
                    Field[row, col] = rawData[col];
                }
            }
        }
        /*public void UpdateField_Rust(){
            NextPtr = Next_Read(Instance);
            Marshal.Copy(NextPtr, NextMino, 0, 5);
            int[] tempFIeld = new int[Rows * Columns];
            Marshal.Copy(field_rust(fieldPtr), tempFIeld, 0, Rows * Columns);
            Buffer.BlockCopy(tempFIeld, 0, Field, 0,  Rows * Columns*sizeof(int));
        }*/

        public int GameStart(){
            return TetrisGameRun(Instance);
        }
        public void MinoMove(char key){
            MoveMino(Instance,key);
           
        }
    }

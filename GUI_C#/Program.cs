using System;
using System.Windows.Forms;
using System.IO;
using System;
using System.Diagnostics;
using System.Runtime.InteropServices;

/*
dotnet publish -r win-x64 -c Release --self-contained true   --output release 
*/

namespace Tetris_UI
{
    internal static class Program
    {
        
        [STAThread]
        private static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new TetrisForm());
        }
        
    }
}
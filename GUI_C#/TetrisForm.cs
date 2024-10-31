using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO.MemoryMappedFiles;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace Tetris_UI
{
    public partial class TetrisForm : Form
    {
        private List<TetrisPlayer> players;
        private int numPlayers = 1; // プレイヤー数
        private const int Columns = 10;
        private const int Rows = 20;
        private const int CellSize = 30;
        private const int FieldSpacing = 10;
        private readonly Color[] cellColors = {
            Color.White,
            Color.Green,
            Color.Red,
            Color.Blue,
            Color.Orange,
            Color.Purple,
            Color.Yellow,
            Color.SkyBlue
        };



        public TetrisForm()
        {
            InitializeComponent();
            InitializePlayers();
            StartGame();
            System.Windows.Forms.Timer gameTimer = new System.Windows.Forms.Timer();
            gameTimer.Interval = 1000 / 60;
            gameTimer.Tick += GameTimer_Tick;
            gameTimer.Start();

            this.KeyDown += keyboard_IN;
            this.KeyPreview = true;
        }

        private void InitializePlayers()
        {
            players = new List<TetrisPlayer>();

            for (int i = 0; i < numPlayers; i++)
            {
                players.Add(new TetrisPlayer(i));
            }
        }

        private void StartGame()
        {
            foreach (var player in players)
            {
                Task.Run(() => player.GameStart());
            }
        }

        private void GameTimer_Tick(object sender, EventArgs e)
        {
            foreach (var player in players)
                {
                    player.UpdateField();      
                } 
            this.Invalidate();
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            base.OnPaint(e);
            Graphics g = e.Graphics;

            int totalWidth = numPlayers * Columns * CellSize;
            int startX = (this.ClientSize.Width - totalWidth) / 2;

            for (int i = 0; i < numPlayers; i++)
            {
                int offsetX = startX + i * (Columns * CellSize + FieldSpacing);
                DrawField(g, players[i].Field, offsetX);
                DrawPlayerName(g, players[i].Name, offsetX);

                // NEXTミノの表示をプレイヤーのフィールドの右側に描画
                int nextOffsetX = offsetX + Columns * CellSize + FieldSpacing; // ここでオフセットを計算
                DrawNextMinos(g, players[i].NextMino ,nextOffsetX); 
            }
        }

        private void DrawField(Graphics g, int[,] field, int offsetX)
        {
            for (int y = 0; y < Rows; y++) // 行を先に
            {
                for (int x = 0; x < Columns; x++) // 列を後に
                {
                    Brush cellBrush = new SolidBrush(cellColors[field[y, x]]); // ここも行列の順序に注意
                    g.FillRectangle(cellBrush, offsetX + x * CellSize, y * CellSize, CellSize, CellSize);
                    g.DrawRectangle(Pens.Black, offsetX + x * CellSize, y * CellSize, CellSize, CellSize);
                }
            }
        }
        private void DrawNextMinos(Graphics g, int[] NextMinos, int offsetX)
        {
            int nextCellHeight = CellSize; // 各ミノの高さ（普通のセルと同じサイズ）

            for (int i = 0; i < NextMinos.Length; i++) // 5つのNEXTミノを描画
            {
                
                Brush cellBrush = new SolidBrush(cellColors[NextMinos[i]+1]);
                int offsetY = i * nextCellHeight; // 縦に並べるためY座標をオフセット


                g.FillRectangle(cellBrush, offsetX, offsetY, CellSize, nextCellHeight);
                g.DrawRectangle(Pens.Black, offsetX, offsetY, CellSize, nextCellHeight);
            }
        }



        private void DrawPlayerName(Graphics g, string playerName, int offsetX)
        {
            SizeF textSize = g.MeasureString(playerName, this.Font);
            float textX = offsetX + (Columns * CellSize - textSize.Width) / 2;
            float textY = Rows * CellSize + FieldSpacing; // フィールドの下に隙間を追加して表示
            g.DrawString(playerName, this.Font, Brushes.Black, textX, textY);
        }

        private void keyboard_IN(object sender, KeyEventArgs e)
        {
            char direction = (char)e.KeyValue;
                foreach (var player in players)
                {
                    player.MinoMove(direction);       
                } 
        }

        private Keys GetKeyForPlayer(int playerId)
        {
            // プレイヤーごとのキー設定を決定
            switch (playerId)
            {
                case 0: return Keys.Left;    // プレイヤー1のキー設定
                case 1: return Keys.A;       // プレイヤー2のキー設定
                case 2: return Keys.NumPad4; // プレイヤー3のキー設定
                case 3: return Keys.J;       // プレイヤー4のキー設定
                default: return Keys.None;
            }
        }
    }
}

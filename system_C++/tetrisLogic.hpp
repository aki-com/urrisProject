#define EIGEN_DEFAULT_TO_ROW_MAJOR
#ifndef TETRISLGIC_H
#define TETRISLGIC_H
#define DLL __declspec(dllexport) 
#include <windows.h>
#include <iostream>
#include <stdlib.h>
#include <time.h>
#include <random> 
#include <conio.h>
#include "Eigen/Dense"
#include <set>
#include <queue>
#include <tuple>
#include <vector>
#include <windows.h>
#include <algorithm>
// 定数
static const int FIELD_WALL =  3;

static const int FIELD_W_out = 10;
static const int FIELD_H_out = 29;
static const int FIELD_W_in = FIELD_W_out + (FIELD_WALL * 2);
static const int FIELD_H_in = FIELD_H_out + FIELD_WALL;
// 列挙型
enum { Z, S, J, L, T, O, I, MINO_TYPE };

// テトリミノの形状
const Eigen::MatrixXi minoShapes[MINO_TYPE] = {
    (Eigen::MatrixXi(3, 3) << 1, 1, 0, 0, 1, 1, 0, 0, 0                     ).finished(), // Z
    (Eigen::MatrixXi(3, 3) << 0, 2, 2, 2, 2, 0, 0, 0, 0                     ).finished(), // S
    (Eigen::MatrixXi(3, 3) << 3, 0, 0, 3, 3, 3, 0, 0, 0                     ).finished(), // J
    (Eigen::MatrixXi(3, 3) << 0, 0, 4, 4, 4, 4, 0, 0, 0                     ).finished(), // L
    (Eigen::MatrixXi(3, 3) << 0, 5, 0, 5, 5, 5, 0, 0, 0                     ).finished(), // T
    (Eigen::MatrixXi(2, 2) << 6, 6, 6, 6                                    ).finished(), // O
    (Eigen::MatrixXi(4, 4) << 0, 0, 0, 0, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0).finished()  // I
};

// 壁キックオフセット
const std::vector<std::vector<std::pair<int, int>>> wallKickOffsetsI = {
    {{0, 0}, {-1, 0}, {2, 0}, {-1, -2}, {2, 1}}, // 0 -> 3
    {{0, 0}, {-2, 0}, {1, 0}, {-2, 1}, {1, -2}}, // 0 -> 1
    {{0, 0}, {2, 0}, {-1, 0}, {2, -1}, {-1, 2}}, // 1 -> 0
    {{0, 0}, {-1, 0}, {2, 0}, {-1, -2}, {2, 1}}, // 1 -> 2
    {{0, 0}, {1, 0}, {-2, 0}, {1, 2}, {-2, -1}}, // 2 -> 1
    {{0, 0}, {2, 0}, {-1, 0}, {2, -1}, {-1, 2}}, // 2 -> 3
    {{0, 0}, {1, 0}, {-2, 0}, {-2, 1}, {1, -2}}, // 3 -> 2
    {{0, 0}, {-2, 0}, {1, 0}, {1, 2}, {-2, -1}}  // 3 -> 0
};

const std::vector<std::vector<std::pair<int, int>>> wallKickOffsets = {
    {{0, 0}, {1, 0}, {1, -1}, {0, 2}, {1, 2}},    // 0 -> 3
    {{0, 0}, {-1, 0}, {-1, -1}, {0, 2}, {-1, 2}}, // 0 -> 1
    {{0, 0}, {1, 0}, {1, 1}, {0, -2}, {1, -2}},   // 1 -> 0
    {{0, 0}, {1, 0}, {1, 1}, {0, -2}, {1, -2}},    // 1 -> 2
    {{0, 0}, {-1, 0}, {-1, 1}, {0, -2}, {-1, -2}},// 2 -> 1
    {{0, 0}, {1, 0}, {1, 1}, {0, -2}, {1, -2}},   // 2 -> 3
    {{0, 0}, {-1, 0}, {-1, 1}, {0, -2}, {-1, -2}}, // 3 -> 2
    {{0, 0}, {-2, 0}, {-2, 1}, {0, -2}, {-1, -2}} // 3 -> 0
};
// 関数
class Tetris_Game{
    public:
        Tetris_Game();
        int tetris_run();
        void moveMino(int direction);
        void field_read();
        //変数

        Eigen::MatrixXi field_in;
        Eigen::MatrixXi field_out;
        int* field_ptr[FIELD_H_out*FIELD_W_out];
        time_t t;
        std::vector<int> NextMino;
        int minoType, minoX, minoY, minoA;
        Eigen::MatrixXi currentMino;
        std::vector<int> minoBag;
        int bagIndex;
        int holdMinoType;
        bool holdUsed;
        bool isPaused;




    private:
        bool initializeField();
        bool isHit(int x, int y, const Eigen::MatrixXi& mino);
        void resetMino();
        void generateMinoBag();
        bool rotateAndKick(int& x, int& y, Eigen::MatrixXi& mino, bool clockwise);
        void WriteField(Eigen::MatrixXi& field);
        bool isGameOver();
        void holdMino();
        void hardDrop();
        void display();
        void field_update();
        
};

extern "C"{
    DLL  Tetris_Game* CreateTetrisGame() {
    Tetris_Game* game = new Tetris_Game();
    return game;
    }
    DLL void DestroyTetrisGame(Tetris_Game* tetris) {
        delete tetris;
    }
    DLL int TetrisGameRun(Tetris_Game* tetris) {
        return tetris->tetris_run();
    }
    DLL void MoveMino(Tetris_Game* tetris, int direction) {
        tetris->moveMino(direction);
    }
    DLL int** Field_Read(Tetris_Game* tetris) {
        return tetris->field_ptr;
    }
    DLL int* Next_Read(Tetris_Game* tetris) {
        return tetris->NextMino.data();
    }
}
#endif // TETRISLGIC_H

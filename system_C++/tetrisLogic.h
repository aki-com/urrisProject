
#ifndef TETRISLGIC_H
#define TETRISLGIC_H
#include <windows.h>
#include <iostream>
#include <stdlib.h>
#include <time.h>
#include <conio.h>
#include <Eigen/Dense>
#include <set>
#include <queue>
#include <tuple>
#include <vector>
#include <windows.h>
#include <algorithm>

// 定数
static const int FIELD_WALL =  4;
static const int FIELD_W_in = 10;
static const int FIELD_H_in = 20;
static const int FIELD_W_out = FIELD_W_in + (FIELD_WALL * 2);
static const int FIELD_H_out = FIELD_H_in + FIELD_WALL;

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
extern "C" __declspec(dllexport) int tetris_run(wchar_t *sharedMemName);
extern "C" __declspec(dllexport) void moveMino(int direction);
bool initializeField();
void updateSharedMemory(int* sharedData);
bool isHit(int x, int y, const Eigen::MatrixXi& mino);
void resetMino();
void generateMinoBag();
bool rotateAndKick(int& x, int& y, Eigen::MatrixXi& mino, bool clockwise);
void putMino();
bool isGameOver();
void holdMino();
void hardDrop();
void recordAllPlacementPatterns();
void findPlacementsForMino(int minoType, std::vector<std::tuple<int, int, int, std::string>>& placements);

#endif // TETRISLGIC_H

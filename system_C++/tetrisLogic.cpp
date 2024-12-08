#pragma GCC optimize("O3")
#pragma GCC optimize("unroll-loops")
#include "tetrisLogic.hpp"
/*
cd system_C++
g++ -shared -o TetrisLogic.dll -O3 TetrisLogic.cpp -I"C:/CppLib"
*/
    // �֐��錾
    Tetris_Game::Tetris_Game(){

    initializeField(); 
    minoBag = {0,1,2,3,4,5,6};
    t = time(NULL);
    srand(t);
    generateMinoBag();
    resetMino();
    holdMinoType = -1;

}
int Tetris_Game::tetris_run() {
    std::ios::sync_with_stdio(false);

    while (true) {

        if (_kbhit()) {
            moveMino(_getch());
        }
        if (isPaused) continue; 
        if (time(NULL) != t) {
            t = time(NULL);
            field_update();
            //display();
            if (isHit(minoX, minoY + 1, currentMino)) {
                WriteField(field_in); resetMino();
                if (isHit(minoX, minoY, currentMino)) {
                    // �Q�[���I�[�o�[����
                    std::cout << "Game Over!" << std::endl;
                    break;
                }
            } else {
                minoY++;
            }
        }
    }

    return 0;
}

void Tetris_Game::moveMino(int direction) {
        if (direction == 0x50) { // 'P'キーで一時停止をトグル
        isPaused = !isPaused;
        return;
    }
    if (isPaused) return;
    switch (direction) {
        case VK_UP:    hardDrop(); WriteField(field_in); resetMino();      break;
        case VK_DOWN:  if (!isHit(minoX, minoY + 1, currentMino)) minoY++; break;
        case 0x5A:     if (!isHit(minoX - 1, minoY, currentMino)) minoX--; break;
        case 0x58:     if (!isHit(minoX + 1, minoY, currentMino)) minoX++; break;
        case 0x43:     holdMino();                                         break;
        case VK_LEFT : rotateAndKick(minoX, minoY, currentMino, true);     break;
        case VK_RIGHT: rotateAndKick(minoX, minoY, currentMino, false);    break;
    }
    field_update();
}

bool Tetris_Game::initializeField() {
    SetConsoleTitle(TEXT("tetris"));
    field_in = field_out = Eigen::MatrixXi::Zero(FIELD_H_in, FIELD_W_in);
    field_in.block(0, 0, FIELD_H_in, FIELD_WALL).setOnes(); // �����̕�
    field_in.block(0, FIELD_W_in - FIELD_WALL, FIELD_H_in, FIELD_WALL).setOnes(); // �E���̕�
    field_in.bottomRows(FIELD_WALL).setOnes(); // ��̕�
    field_read();

    return true;
}

void Tetris_Game::resetMino() {
    if (NextMino.size() >= MINO_TYPE) generateMinoBag();
    minoType = NextMino.front();
    NextMino.erase(NextMino.begin());;
    minoX = 3 + FIELD_WALL, minoY = 0, minoA = 0;
    currentMino = minoShapes[minoType];
    holdUsed = false; 
}

void Tetris_Game::generateMinoBag() {
    std::mt19937 g(t);
    std::shuffle(minoBag.begin(), minoBag.end(),g);
    NextMino.insert(NextMino.end(),minoBag.begin(), minoBag.end());
}

bool Tetris_Game::rotateAndKick(int& x, int& y, Eigen::MatrixXi& mino, bool clockwise) {
    const auto& offsets = (minoType == I) ? wallKickOffsetsI : wallKickOffsets;
    Eigen::MatrixXi rotated;
    if (clockwise)  rotated = mino.transpose().colwise().reverse();
    else            rotated = mino.transpose().rowwise().reverse();
   
    for (const auto& offset : offsets[(minoA * 2) + (clockwise ? 0 : 1)]) {
        int newX = x + offset.first;
        int newY = y + offset.second;

        if (!isHit(newX, newY, rotated)) {
            x = newX, y = newY, mino = rotated;
            minoA = clockwise ? (minoA + 3) % 4 : (minoA + 1) % 4;
            return true;
        }
    }
    return false;
}

void Tetris_Game::hardDrop() {
    while (!isHit(minoX, minoY + 1, currentMino)) {
        minoY++;
    }
}

void Tetris_Game::holdMino() {
    if (holdUsed) return;

    if (holdMinoType == -1) {
        holdMinoType = minoType;
        resetMino();
    } else {
        std::swap(holdMinoType, minoType);
        minoX = 5, minoY = 0, minoA = 0;
        currentMino = minoShapes[minoType];
    }

    holdUsed = true;
}

void Tetris_Game::WriteField(Eigen::MatrixXi& field) {
    field.block(minoY, minoX, currentMino.rows(), currentMino.cols()) += currentMino;
    for (int i = 0; i < FIELD_H_out; ++i) {
        if ((field.block(i, FIELD_WALL, 1, FIELD_W_out).array() > 0).all()) {
            for (int k = i; k > 0; --k) {
                field.block(k, FIELD_WALL, 1, FIELD_W_out) = field.block(k - 1, FIELD_WALL, 1, FIELD_W_out);
            }
            field.block(0, FIELD_WALL, 1, FIELD_W_out).setZero();
            --i;
        }
    }
}
void Tetris_Game::field_update(){
    field_out = field_in;
    field_out.block(minoY, minoX, currentMino.rows(), currentMino.cols()) += currentMino;  
}

bool Tetris_Game::isHit(int x, int y, const Eigen::MatrixXi& mino) {
    return (field_in.block(y, x, mino.rows(), mino.cols()).array() * mino.array()).any();
}
void Tetris_Game::field_read(){
    int index = 0;
    for (int row = 0; row < FIELD_H_in-FIELD_WALL; ++row) {
        for (int col = FIELD_WALL; col < FIELD_W_in-FIELD_WALL; ++col) {
            field_ptr[index++] = &field_out(row, col);
        }
    }

}


void Tetris_Game::display(){
    int index = 0;
    system("cls");
    //NEXT
    std::cout<<"NEXT"<<std::endl;
    for (int i = 0; i < 8; i++)
    {    
        std::cout<<*(&NextMino[0]+i)<<std::endl;
    }
    for (int i = 0; i < FIELD_H_out; i++)
    {
        for (int j = 0; j < FIELD_W_out; j++)
        {
            std::cout<<*field_ptr[index++];
        }
        std::cout<<std::endl;   
    }

}
/*
int main(){
    Tetris_Game tetris= Tetris_Game();
    tetris.tetris_run();
    return 0;
}*/
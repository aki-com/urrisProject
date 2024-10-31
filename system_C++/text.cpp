
#include <iostream>
#include <vector>


int main() {

    std::vector<int> num = {1,2,3,4,5};
    int* p = &num[0];
        for (int i = 0; i < 5; i++)
        {
                num.erase(num.begin());
    printf("先頭：%d\t",*p);
    printf("残りサイズ%d\n",num.size());
            
        }
        



    return 0;
}

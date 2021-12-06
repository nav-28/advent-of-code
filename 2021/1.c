#include <stdio.h>
#include <stdlib.h>


int main(int argc, char * argv[]) {

    int x;
    int data[10000];
    int count = 0;

    while(scanf("%d", &data[count]) > 0 ) {
        count++;
    }

    int big = 0;
    int newList[10000];
    for (int i = 0; i < count - 2; i++){
        newList[i] = data[i] + data[i+1] + data[i+2];
        if (i > 0){
            if (newList[i-1] < newList[i]){
                big++;
            }
        }
    }


    printf("%d", big);
    return 0;
}

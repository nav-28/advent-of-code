// this is not a good solution

#include <stdlib.h>
#include <string.h>
#include <stdio.h>


int toInt(const char *s){
    return (int) strtol(s, NULL, 2);
}

int main(int argc, char **argv) {

    if (argc == 0){
        printf("incorrect usage!! Please provide input file");
    }
    int size = 0;
    char data[10000][13];
    char input[13];
    // read data
    while(scanf("%s", input) > 0){
        for (int j = 0; j < 12; j++)
            data[size][j] = input[j];
        size++;
    }


    // part 1
    int counterZero = 0;
    int counterOne = 0;
    char gamma[13];
    char epi[13];
    for (int i = 0; i < 12; i++){
        counterZero = 0;
        counterOne = 0;

        for (int j = 0; j < size; j++){
            if (data[j][i] == '0'){
                counterZero++;
            } else {
                counterOne++;
            }

        }
        if (counterZero > counterOne){
            gamma[i] = '0';
            epi[i] = '1';
        }
        else {
            gamma[i] = '1';
            epi[i] = '0';
        }

    }
    int intGamma = toInt(gamma);
    int intEpi = toInt(epi);

    printf("part 1: %d", intEpi*intGamma);

    // part 2
    int valid[size];
    char ch1;
    for (int i = 0; i < size; i++){
        valid[i] = 0;
    }


    for (int i = 0; i < 12; i++){
        counterZero = 0; counterOne = 0;
        for (int j = 0; j < size; j++){
            if (data[j][i] == '0' && valid[j] == 0){
                counterZero++;
            }
            else if (data[j][i] == '1' && valid[j] == 0 ){
                counterOne++;
            }
        }
        if (counterZero == counterOne){
            ch1 = '1';
        } else if (counterZero < counterOne){
            ch1 = '1';
        } else if (counterZero > counterOne){
            ch1 = '0';
        }
        for (int k = 0; k < size; k++){
            if (data[k][i] != ch1)
                valid[k] = 1;
        }


    }

    int o2;
    for (int i = 0; i < size; i++)
        if (valid[i] == 0 ){
            o2 = toInt(data[i]);
        }

    printf("%d\n", o2);
     for (int i = 0; i < size; i++){
        valid[i] = 0;
    }


    for (int i = 0; i < 12; i++){
        counterZero = 0;
        counterOne = 0;
        for (int j = 0; j < size; j++){
            if (data[j][i] == '0' && valid[j] == 0){
                counterZero++;
            }
            else if (data[j][i] == '1' && valid[j] == 0 ){
                counterOne++;
            }
        }
        if (counterZero == counterOne){
            ch1 = '0';
        } else if (counterZero < counterOne){
            ch1 = '0';
        } else if (counterZero > counterOne){
            ch1 = '1';
        }
        int count0 = 0;
        for (int l = 0; l < size; l++){
            if (valid[l] == 0){
                count0++;
            }
        }
        if (count0 == 1){
            break;
        }
        for (int k = 0; k < size; k++){
            if (data[k][i] != ch1){
                valid[k] = 1;
            }
        }


    }

    int co2 = 0;
    for (int i = 0; i < size; i++){
        if (valid[i] == 0){
            co2 = toInt(data[i]);
        }
    }

    printf("part 2: %d", co2*o2);

 


}

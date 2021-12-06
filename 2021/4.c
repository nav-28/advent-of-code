
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int main(int argc, char ** argv){
    FILE *inputFile = fopen("input/4_input.txt", "r");
	printf("hi");
    // read data
	int boards[200][5][5];
	int cards = 0;
	while(!feof(stdin)){
		for (int i = 0; i < 5; i++)
			for (int y = 0; y < 5; y++)
				if (scanf("%d", &boards[cards][i][y])!= 1)
					printf("not a full board");
		cards++;
	}

	printf("%d\n", boards[cards][0][0]);
	printf("%d\n", cards);

    fclose(inputFile);
    return 0;
}

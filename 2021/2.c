#include<stdlib.h>
#include<string.h>
#include<stdio.h>

int main(int argc, char ** argv) {

	if (argc < 1) {
		printf("incorrect usage! use 2.out < input file ");
		return -1;
	}
	// solution for part 2
	int x;
	char s[50];
	int depth = 0, horizontal = 0, aim = 0;
	while(scanf("%s %d", s, &x) > 0 ) {
		if (strcmp(s, "forward") == 0) {
			horizontal = horizontal + x;
			depth = depth + aim * x;
		}
		else if (strcmp(s, "down") == 0) {
			aim = aim + x;
		}
		else if (strcmp(s,"up") == 0) {
			aim = aim - x;
		}
	}
	int final = horizontal * depth;
	printf("%d", final);
	return 0;

}

#include <stdio.h>

// object like macro
#define PI 3.1415

#define SQUARE(x) ((x) * (x))

int main() {

    int num = 5;
    int squared = SQUARE(num);

    printf("%d\n", squared);

}
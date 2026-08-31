#include <stdio.h>

void printNumber(int num){
    printf("Number was: %d\n", num);
}

int main(){

    int a = 5;
    // ascii representation
    char b = 'A';
    float c = 3.1415;

    float result = a + b + c; 

    printf("%f\n", result);

    printNumber(c);

    return 0;

}
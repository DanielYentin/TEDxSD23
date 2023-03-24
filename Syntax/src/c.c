#include <stdio.h>

int main() {
    int a = 0;
    int b = 10;
    int sum = sum_evens(a, b);
    printf("C: sum of even numbers between %d and %d is %d\n", a, b, sum);
}

int sum_evens(int a, int b) {
    if (a <= b) {
        int sum = 0;
        for (int i = a; i <= b; i++) {
            if (i % 2 == 0) {
                sum += i;
            }
        }
        return sum;
    } 
    else {
        return 0;
    }
}
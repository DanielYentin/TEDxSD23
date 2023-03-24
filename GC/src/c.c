#include <stdio.h> // for printing
#include <unistd.h> // for getting pid
#include <time.h> // for getting time
#include <stdbool.h> // for getting bools
#include <stdlib.h> // for malloc() and free()

#define SIZE 10##000##000
#define LOOPS 1##00
#define FREE false


typedef struct Person {
    int age;
} Person;

void fill_mem(Person* array) {
    for (int i = 0; i < SIZE; i++) {
        array[i] = (Person){0};
    }
}

void update_mem(Person* array) {
    for (int i = 0; i < SIZE; i++) {
        array[i].age += 1;
    }
}

int main() {
    int pid = getpid();
    printf("C pid: %d\n", pid);
    printf("Total iterations: %d\n", SIZE*LOOPS);

    struct timespec start, end;
    while (true) {
        clock_gettime(CLOCK_MONOTONIC_RAW, &start);

        //---------------------------------------------
        Person* array = malloc(sizeof(Person)*SIZE);
        fill_mem(array);
        for (int i = 0; i < LOOPS; i++) {
            update_mem(array);
        }
        if (FREE == true) {
            free(array); 
        }
        //---------------------------------------------

        clock_gettime(CLOCK_MONOTONIC_RAW, &end);

        long double duration = ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_nsec - start.tv_nsec) / 1000)/1000000.0;
        printf("%.5Lf\n", duration);
    }
}

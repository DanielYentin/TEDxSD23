#include <stdio.h> // for printing
#include <unistd.h> // for getting pid
#include <time.h> // for getting time
#include <stdbool.h> // for getting bools
#include <stdlib.h> // for malloc() and free()

#define tests 1##000##000##0

typedef struct Person {
    int age;
} Person;

void fill_mem(Person* array, int n) {
    for (int i = 0; i < n; i++) {
        Person person;
        person.age = i;
        array[i] = person;

    }
}

int main(int argc, char *argv[]) {
    int pid = getpid();
    printf("C pid: %d\n", pid);

	bool FREE = false;
	if (argc > 1) {
		FREE = true;
	}

    struct timespec start, end;
    while (true) {
        clock_gettime(CLOCK_MONOTONIC_RAW, &start);

        Person* array = malloc(sizeof(Person)*tests);
        fill_mem(array, tests);

        if (FREE) {
        	free(array);
        }

        clock_gettime(CLOCK_MONOTONIC_RAW, &end);

        long double duration = ((end.tv_sec - start.tv_sec) * 1000000 + (end.tv_nsec - start.tv_nsec) / 1000)/1000000.0;
        printf("%Lf\n", duration);
    }
}

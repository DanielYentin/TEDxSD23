import time, os
tests = 1_000_000

def main():
    pid = os.getpid()
    print(f"Python pid: {pid}")

    while True:
        start = time.time() 
        array = fill_mem(tests)
        end = time.time()
        duration = (end - start)
        print(f"{duration:.3f}")

def fill_mem(n):
    array = [None] * n
    for i in range(len(array)):
        array[i] = Person(i)
    return array

class Person:
    def __init__(self, age) -> None:
        self.age = age

if __name__ == "__main__":
    main()
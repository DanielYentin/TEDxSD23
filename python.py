import time, os
TESTS = 1_000_000

def main():
    pid = os.getpid()
    print(f"Python pid: {pid}")

    while True:
        start = time.time() 

        array = [None] * TESTS
        fill_mem(array)
        
        end = time.time()
        duration = (end - start)
        print(f"{duration:.5f}")

def fill_mem(array):
    for i in range(len(array)):
        array[i] = Person(i)

class Person:
    def __init__(self, age) -> None:
        self.age = age

if __name__ == "__main__":
    main()

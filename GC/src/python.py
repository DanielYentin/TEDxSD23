import time, os
SIZE = 1_000_000
LOOPS = 1
class Person:
    def __init__(self, age) -> None:
        self.age = age

def fill_mem(array):
    for i in range(len(array)):
        array[i] = Person(i)

def update_mem(array):
    for i in range(len(array)):
        array[i].age += 1

def main():
    pid = os.getpid()
    print(f"Python pid: {pid}")
    print(f"Total iterations: {SIZE*LOOPS}")

    while True:
        start = time.time() 
        
        #---------------------
        array = [None] * SIZE
        fill_mem(array)
        for i in range(LOOPS):
            update_mem(array)
        #---------------------

        end = time.time()
        duration = (end - start)
        print(f"{duration:.5f}")

if __name__ == "__main__":
    main()

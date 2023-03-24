def main():
    a: int = 0
    b: int = 10
    sum: int = sum_evens(a, b)
    print(f"Python: sum of even numbers between {a} and {b} is {sum}")

def sum_evens(a: int, b: int):
    if (a <= b):
        sum: int = 0
        for i in range(a, b+1):
            if i % 2 == 0:
                sum += i
        return sum
    else:
        return 0 
    
if __name__ == "__main__":
    main()
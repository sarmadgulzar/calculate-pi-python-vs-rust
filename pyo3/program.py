from time import time

from calculate_pi import calculate

N = 100_000_000

if __name__ == "__main__":
    start = time()
    pi = calculate(N)
    end = time()
    print(f"π = {pi}")
    print(f"Time elapsed: {round((end - start), 2)} seconds!")


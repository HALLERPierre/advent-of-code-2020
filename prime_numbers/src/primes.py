import math
import time

start = time.time()

primes = 0

x = 1

while (True):
    now = time.time()
    if now - start >= 10:
        print("Found", primes, "primes numbers on", x, "in", now - start, "s")
        break


    is_prime = True
    y = 3
    while (is_prime and y <= math.sqrt(x)):
        if (x % y == 0):
            is_prime = False
        y += 2
    if (is_prime):
        primes += 1
    x += 2

import time
import math

def is_prime(x):
    if x < 2:
        return False
    if x == 2:
        return True
    if x % 2 == 0:
        return False

    prime = 3
    while prime <= math.sqrt(x):
        if x % prime == 0:
            return False
        prime += 2

    return True


if __name__ == '__main__':
    time_sta = time.time()
    cnt=0
    for i in range(1_000_000):
        if is_prime(i):
            cnt+=1
    print(f'cnt:{cnt}')
    time_end = time.time()
    print(f'time:{time_end-time_sta}')

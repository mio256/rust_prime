def if_prime(n):
    if n==0 or n==1:
        return False
    elif n==2:
        return True
    else:
        i=2
        while i<n:
            if n%i==0:
                return False
            i+=1
        return True


import time

time_sta = time.time()

cnt=0
for i in range(100000):
    if if_prime(i):
        # print(f'{i}')
        cnt+=1
print(f'cnt={cnt}')

time_end = time.time()

tim = time_end- time_sta

print(tim)
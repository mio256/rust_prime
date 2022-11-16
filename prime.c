#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include <math.h> 

int if_prime(int num);

int main(void){
	clock_t cpu_time;
    int i;
    int cnt=0;
    for(i=1;i<1000000;i++){
        if(if_prime(i)){
            cnt++;
        }
    }
    printf("cnt:%d\n",cnt);
	cpu_time = clock();
	printf("time:%f\n", (double)cpu_time/CLOCKS_PER_SEC);
}

int if_prime(int num){
    if (num < 2) return 0;
    else if (num == 2) return 1;
    else if (num % 2 == 0) return 0; // 偶数はあらかじめ除く

    double sqrtNum = sqrt(num);
    for (int i = 3; i <= sqrtNum; i += 2)
    {
        if (num % i == 0)
        {
            // 素数ではない
            return 0;
        }
    }

    // 素数である
    return 1;
}
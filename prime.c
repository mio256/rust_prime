#include <stdio.h>
#include <time.h>
#include <unistd.h>

int if_prime(int n){
    if(n==0||n==1){
        return 0;
    }else if(n==2){
        return 1;
    }else{
        int i=2;
        while(i<n){
            if(n%i==0){
                return 0;
            }
            i++;
        }
        return 1;
    }
}

int main(void){
	clock_t cpu_time;
    int i;
    int cnt=0;
    for(i=1;i<100000;i++){
        if(if_prime(i)){
            cnt++;
        }
    }
    printf("cnt=%d\n",cnt);
	cpu_time = clock();
	printf("clock=%f\n", (double)cpu_time/CLOCKS_PER_SEC);
}
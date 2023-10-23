#include <stdio.h>
#include <stdbool.h>
#define print(x) _Generic((x), \
    char: printf("%c\n", (x)), \
    int: printf("%d\n", (x)), \
    long: printf("%ld\n", (x)), \
    float: printf("%f\n", (x)), \
    double: printf("%lf\n", (x)), \
    default: printf("Unknown type\n") \
)
void main(void){int ident_i;{ident_i = 0 ;while(ident_i != 10 ){{print(-ident_i );}ident_i = ident_i + 2 ;}}}
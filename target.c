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
void ident_DoStuff(void){int ident_Age;ident_Age = 18 ;if(ident_Age >= 18 ){print(9 );}else{print(69 );}}void main(void){ident_DoStuff();}
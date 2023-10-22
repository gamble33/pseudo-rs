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
void ident_DoStuff(void){int ident_i;ident_i = 1 ;while(ident_i < 10 ){print(ident_i );ident_i = ident_i + 1 ;}ident_i = 1 ;do {print(ident_i );ident_i = ident_i + 1 ;}while(!(ident_i == 10 ));}void main(void){ident_DoStuff();}
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
void ident_Add(int ident_A,int ident_B,int ident_Result){int ident_Test1;char ident_Test2;bool ident_Test3;ident_Result = ident_A + ident_B ;}void main(void){int ident_Result;ident_Add(35 ,34 ,ident_Result );print(ident_Result );}
#include <stdio.h>
#include <stdbool.h>
#define print(x) _Generic((x), \
    char: printf("%c\n", (x)), \
    bool: printf("%s\n", (x) ? "TRUE" : "FALSE"), \
    int: printf("%d\n", (x)), \
    long: printf("%ld\n", (x)), \
    float: printf("%f\n", (x)), \
    double: printf("%lf\n", (x)), \
    default: printf("Unknown type\n") \
)
void main(void){print((bool)false);}
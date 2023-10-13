#include <stdio.h>
#include <stdbool.h>
#define print(x) _Generic(x)
void ident_Add(int ident_A,int ident_B,int ident_Result){int ident_Test1;char ident_Test2;bool ident_Test3;ident_Result = ident_A + ident_B ;}void ident_Main(void){int ident_Result;ident_Add(35 ,34 ,ident_Result );print(ident_Result );}
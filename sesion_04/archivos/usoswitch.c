#include <stdio.h>
#include <math.h>






int main() {
float T;
int OP;
float RES;
printf("Ingresa la opción de calculo y el valor entero: ");
T=13;
OP=1;
switch(OP){
  case 1:
    RES=T/5;
    break;
  case 2:
    RES= pow(T,T);
    break;
  case 3:
  case 4:
    RES = 6 * T / 2;
    break;;
  default:
    RES = 1;
    break;
 }
printf("Resultado : %7.2f\n", RES);
return 0;
}

#include <stdio.h>
#include <math.h>






void main(void){
float SAL;
int NIV;
printf("Ingrese nivel académico del profesor: \n");
scanf("%d", &NIV);
printf("Ingrese el salario: \n");
scanf("%f", &SAL)
switch(NIV){
case 1: SAL = SAL * 1.035; break;
case 2: SAL = SAL * 1.041; break;
case 3: SAL = SAL * 1.048; break;
case 4: SAL = SAL * 1.053; break;    
 }
printf("Nivel :%d, salario: %8.2f\n", NIV,SAL);
 }

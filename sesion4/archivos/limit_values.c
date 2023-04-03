#include <stdio.h>
#include <limits.h>
#include <float.h>






int main(){
  printf("Min char %d\n", CHAR_MIN);
  printf("Max char %d\n", CHAR_MAX);

  printf("Min signed char %d\n", SCHAR_MIN);
  printf("Max signed char %d\n", SCHAR_MAX);

  printf("Min unsigned char %d\n",0 );
  printf("Max unsigned char %d\n",UCHAR_MAX);

  printf("Min int %d\n", INT_MIN);
  printf("Max int %d\n", INT_MAX );

  printf("Min short int %d\n", SHRT_MIN);
  printf("Max short int %d\n", SHRT_MAX);

  printf("Min long int %ld\n", LONG_MIN);
  printf("Max long int %ld\n", LONG_MAX);

  printf("Max unsigned long int %lu\n", ULONG_MAX);

  printf("Min float %f\n", FLT_MIN);
  printf("Max float %f\n", FLT_MAX);

  printf("Min double %f\n", DBL_MIN );
  printf("Max double %f\n", DBL_MAX );

  printf("Min long double %lf\n", 0);
  printf("Max long double %Lf\n", LDBL_MAX);

 return 0;
}

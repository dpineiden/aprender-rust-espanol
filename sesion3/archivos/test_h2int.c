#include <stdio.h>
#include <string.h>






#include  "./e2_3/deps/hexchar2int.h"

int main(){
  char line[] = "Hola mundo: 3249889934 abcdefg";
  size_t len = strlen(line);
  int i;
  for (i=0;i<len;i++){
    printf("Char: %c, value: %d\n", line[i], hexchar2int(line[i]));  
  }
  return 0;
}

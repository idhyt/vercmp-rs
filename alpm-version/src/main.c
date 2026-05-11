/* gcc main.c version.c -o main
 * > ./main 1.5.1 1.5
 */
#include "version.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
  if (argc != 3) {
    printf("用法: %s <版本号1> <版本号2>\n", argv[0]);
    printf("示例: %s 1.2.3a 1.2.3_beta1-r0\n", argv[0]);
    return 1;
  }

  const char *a = argv[1];
  const char *b = argv[2];

  int cmp = alpm_version_compare(a, b);

  const char *symbol;
  switch (cmp) {
  case ALPM_VERSION_EQUAL:
    symbol = "==";
    break;
  case ALPM_VERSION_LESS:
    symbol = "<";
    break;
  case ALPM_VERSION_GREATER:
    symbol = ">";
    break;
  default:
    symbol = "?";
    break;
  }

  printf("'%s' %s '%s'\n", a, symbol, b);

  return 0;
}

/* gcc main.c version.c -I./version.h -o main
 * > ./main 1.2.1_alpha 1.2.1_beta
 */
#include "version.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  if (argc != 3) {
    printf("用法: %s <版本号1> <版本号2>\n", argv[0]);
    printf("示例: %s 1.2.3_alpha1-r0 1.2.3_beta1-r0\n", argv[0]);
    return 1;
  }

  const char *a = argv[1];
  const char *b = argv[2];

  int cmp = apk_version_compare(a, b);

  const char *symbol;
  switch (cmp) {
  case APK_VERSION_EQUAL:
    symbol = "==";
    break;
  case APK_VERSION_LESS:
    symbol = "<";
    break;
  case APK_VERSION_GREATER:
    symbol = ">";
    break;
  default:
    symbol = "?";
    break;
  }

  printf("'%s' %s '%s'\n", a, symbol, b);

  return 0;
}

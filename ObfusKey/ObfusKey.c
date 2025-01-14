#include "obsfusc_cwd.h"
int main(int argc, char *argv[]) {
  if (argc > 1) {
    for (int i = 1; i < argc; i++) {
      switch (gen_encrypted_files(argv[i])) {
      case 0:
        printf("[+] %s\n", argv[i]);
        break;
      case 1:
        printf("[Error] %s\n", argv[i]);
        return 1;
      case 2:
        printf("[Dir] %s\n", argv[i]);
        break;

      default:
        printf("[?] %s\n", argv[i]);
        return 1;
      }
    }
    return 0;
  } else {
    printf("Usage: %s <filename>\n", argv[0]);
    return 1;
  }
}

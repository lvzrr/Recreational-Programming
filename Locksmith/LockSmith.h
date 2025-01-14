#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

const char lower[] = "abcdefghijklmnopqrstuvwxyz";
const char upper[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const char digits[] = "0123456789";
const char special[] = "!@#$%^&*()-_=+[]{}<>?/|\\~";

#define PASSWORD_LENGTH 500

char *gen_pass() {
  FILE *rand_src = fopen("/dev/urandom", "r");
  if (!rand_src) {
    perror("Unable to open /dev/urandom");
    exit(EXIT_FAILURE);
  }

  size_t total_chars =
      sizeof(lower) + sizeof(upper) + sizeof(digits) + sizeof(special) - 4;

  char *char_pool = (char *)malloc(total_chars + 1);
  if (!char_pool) {
    perror("Memory allocation failed");
    fclose(rand_src);
    exit(EXIT_FAILURE);
  }

  strcpy(char_pool, lower);
  strcat(char_pool, upper);
  strcat(char_pool, digits);
  strcat(char_pool, special);

  char *password = (char *)malloc(PASSWORD_LENGTH + 1);
  if (!password) {
    perror("Memory allocation failed");
    fclose(rand_src);
    free((void *)char_pool);
    exit(EXIT_FAILURE);
  }

  for (int i = 0; i < PASSWORD_LENGTH; i++) {
    unsigned char random_index;
    fread(&random_index, sizeof(random_index), 1, rand_src);
    password[i] = char_pool[random_index % total_chars];
  }

  password[PASSWORD_LENGTH] = '\0';

  fclose(rand_src);
  free((void *)char_pool);
  return password;
}

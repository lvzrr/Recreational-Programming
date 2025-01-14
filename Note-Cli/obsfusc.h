/* obfusc.h */

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

int gen_encrypted_files(const char *filename, const char *keyfilename,
                        const char *encryptedfile) {
  FILE *fr;
  FILE *fw;
  FILE *fw2;

  struct stat path_stat;
  stat(filename, &path_stat);
  if (S_ISDIR(path_stat.st_mode)) {
    return 2;
  }

  FILE *src = fopen("/dev/urandom", "r");
  assert(src != NULL);

  /*
   * strip filename extension and add .key
   */

  assert(keyfilename != NULL);
  assert(encryptedfile != NULL);

  /*
   * open files
   */

  fr = fopen(filename, "rb");
  if (fr)
    if (fr == NULL) {
      fprintf(stderr, "file %s not found\n", filename);
      return 1;
    }
  fw = fopen(keyfilename, "wb");
  fw2 = fopen(encryptedfile, "wb");

  if (fw == NULL || fw2 == NULL) {
    fprintf(stderr, "failed to open file %s or %s\n", keyfilename,
            encryptedfile);
    return 1;
  }
  /*
   * encrypt with XOR gate and random chars
   */

  while (1) {
    int to_encrypt = fgetc(fr);
    if (to_encrypt == EOF) {
      break;
    }
    int rand_byte;
    fread(&rand_byte, sizeof(int), 1, src);
    fputc(rand_byte, fw);
    fputc(rand_byte ^ to_encrypt, fw2);
  }
  fflush(fw);
  fflush(fw2);
  fflush(src);
  fclose(fr);
  fclose(fw);
  fclose(src);
  remove(filename);
  return 0;
}
int decrypt_file(char *encryptedfilename, char *keyfilename,
                 char *outfilename) {

  FILE *keyfile;
  FILE *encryptedfile;
  FILE *decryptedfile;

  /*
   * strip filename extensions .key and .crypt to get (output) filename
   */

  encryptedfile = fopen(encryptedfilename, "rb");
  if (encryptedfile == NULL) {
    fprintf(stderr, "encrypted file %s not found\n", keyfilename);
    return 1;
  }
  keyfile = fopen(keyfilename, "rb");
  if (keyfile == NULL) {
    fprintf(stderr, "key file %s not found\n", keyfilename);
    return 1;
  }
  decryptedfile = fopen(outfilename, "wb");
  if (decryptedfile == NULL) {
    fprintf(stderr, "failed to open file %s\n", outfilename);
    return 1;
  }
  /*
   *    Decrypt using XOR ant the key
   */

  while (1) {
    int encrypted_byte = fgetc(encryptedfile);
    int key_byte = fgetc(keyfile);
    if (encrypted_byte == EOF || key_byte == EOF) {
      break;
    }
    fputc(encrypted_byte ^ key_byte, decryptedfile);
  }

  /*
   *  close files
   */

  fclose(keyfile);
  fflush(encryptedfile);
  fclose(encryptedfile);
  fclose(decryptedfile);

  /*
   *  delete key and encrypted file
   */

  remove(keyfilename);
  remove(encryptedfilename);
  return 0;
}

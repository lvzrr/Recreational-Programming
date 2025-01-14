#include "Note-Cli.h"

int main(int argc, char **argv) {
  for (int i = 0; i < argc; i++) {
    switch (argv[i][1]) {
    case 'a':
      add_todo();
      return 0;
    case 'l':
      list_tasks();
      return 0;
    case 'r':
      remove_task_files();
      return 0;
    case 'p':
      printf("PATH: %s\nENC: %s\nKEY: %s\n", PATH, ENC_PATH, KEY_PATH);
      return 0;
    case 'd':
      delete_task();
      return 0;
    }
  }

  run_menu();
  return 0;
}

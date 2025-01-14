#include <fcntl.h>
#include <linux/types.h>
#include <pthread.h>
#include <raylib.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <unistd.h>

#define WINDOW_WIDTH 100
#define WINDOW_HEIGHT 65
#define FPS 144
#define FONT "./resources/LigaSFMonoNerdFont-Regular.otf"
#define FONT_SIZE 50
#define BUFFER_SIZE 6
#define EV_KEY 0x01

struct input_event {
  struct timeval time;
  unsigned short type;
  unsigned short code;
  unsigned int value;
};

typedef struct {
  char *display_buffer;
  char *keyboard_buffer;
} ThreadArgs;

// SOCKET SCRIPT RESULT:
#define SOCKET "/dev/input/event6"

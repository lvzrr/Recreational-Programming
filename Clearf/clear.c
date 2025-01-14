#include "clear.h"

char *UpdateBuffer(char *buffer, char newChar) {
  size_t b_len = strlen(buffer);

  for (size_t i = 0; i < b_len - 1; i++) {
    buffer[i] = buffer[i + 1];
  }

  buffer[b_len - 1] = newChar;
  buffer[b_len] = '\0';
  return buffer;
}
int clearchecker(int c, char *buffer, char *display_buffer) {

  if (strcmp(buffer, "clear") == 0) {
    c++;
    sprintf(display_buffer, "%03d", c);
    fflush(stdout);
  }
  return c;
}

void *startupdater(void *args) {

  ThreadArgs *argc = (ThreadArgs *)args;

  char *map = "..1234567890'¡#qwertyuiop.+.@asdfghjklñ._.zxcvbnm,.-_..\\";
  char *buffer = argc->keyboard_buffer;
  int fd = open(SOCKET, O_RDONLY);
  struct input_event event;
  int clearc = 0;
  while (1) {
    ssize_t n = read(fd, &event, sizeof(event));
    if ((event.type == EV_KEY) && (event.value == 1 || event.value == 2) &&
        (event.code <= strlen(map + 1))) {
      char c = map[event.code];
      UpdateBuffer(buffer, c);
      clearc = clearchecker(clearc, buffer, argc->display_buffer);
    }
  }

  return NULL;
}

pthread_mutex_t buffer_mutex = PTHREAD_MUTEX_INITIALIZER;
void Window_Manager(char *buffer) {
  InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Clearf is coming");
  SetTargetFPS(FPS);
  int glyphCount = 95;
  int *codepoints = (int *)malloc(glyphCount * sizeof(int));

  for (int i = 0; i < glyphCount; i++) {
    codepoints[i] = 32 + i;
  }
  Font font = LoadFontEx(FONT, FONT_SIZE, codepoints, glyphCount);
  free(codepoints);

  Vector2 text_position = {10, 10};

  char *display_buffer = calloc(sizeof(char), 4);
  strcpy(display_buffer, "000");

  ThreadArgs args;
  args.display_buffer = display_buffer;
  args.keyboard_buffer = buffer;

  pthread_t input_thread;
  pthread_create(&input_thread, NULL, startupdater, &args);

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(BLACK);
    DrawTextEx(font, display_buffer, text_position, FONT_SIZE, 0, WHITE);
    EndDrawing();
  }

  pthread_cancel(input_thread);
  pthread_join(input_thread, NULL);
  UnloadFont(font);
  CloseWindow();
}
int main() {
  char *buffer = calloc(sizeof(char), BUFFER_SIZE);
  strcpy(buffer, "     ");
  Window_Manager(buffer);
  free(buffer);
}

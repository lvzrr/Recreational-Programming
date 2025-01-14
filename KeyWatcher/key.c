#include "keys.h"

char *UpdateBuffer(char *buffer, char newChar) {
  size_t b_len = strlen(buffer);

  for (size_t i = 0; i < b_len - 1; i++) {
    buffer[i] = buffer[i + 1];
  }

  buffer[b_len - 1] = newChar;
  buffer[b_len] = '\0';
  return buffer;
}

void *startupdater(void *arg) {
  char *map = "..1234567890'¡#qwertyuiop.+.@asdfghjklñ._.zxcvbnm,.-_..\\";

  char *buffer = (char *)arg;
  int fd = open(SOCKET, O_RDONLY);
  struct input_event event;
  while (1) {
    ssize_t n = read(fd, &event, sizeof(event));
    if ((event.type == EV_KEY) && (event.value == 1 || event.value == 2) &&
        (event.code <= strlen(map + 1))) {
      switch (event.code) {
      case 0:
        UpdateBuffer(buffer, 'e');
        UpdateBuffer(buffer, 's');
        UpdateBuffer(buffer, 'c');
        UpdateBuffer(buffer, ' ');
        break;
      case 14:
        UpdateBuffer(buffer, 'b');
        UpdateBuffer(buffer, 'c');
        UpdateBuffer(buffer, 'k');
        UpdateBuffer(buffer, ' ');
        break;
      case 15:
        UpdateBuffer(buffer, 't');
        UpdateBuffer(buffer, 'a');
        UpdateBuffer(buffer, 'b');
        UpdateBuffer(buffer, ' ');
        break;
      case 28:
        UpdateBuffer(buffer, '\\');
        UpdateBuffer(buffer, 'n');
        UpdateBuffer(buffer, ' ');
        break;
      case 29:
        UpdateBuffer(buffer, '^');
        break;
      case 42:
        UpdateBuffer(buffer, 's');
        UpdateBuffer(buffer, 'h');
        UpdateBuffer(buffer, 'f');
        UpdateBuffer(buffer, 't');
        UpdateBuffer(buffer, ' ');
        break;
      case 57:
        UpdateBuffer(buffer, ' ');
        break;
      default:
        UpdateBuffer(buffer, map[event.code]);
        break;
      }
    }
  }
  return NULL;
}

pthread_mutex_t buffer_mutex = PTHREAD_MUTEX_INITIALIZER;
void Window_Manager(char *buffer) {
  SetConfigFlags(FLAG_BORDERLESS_WINDOWED_MODE);
  InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Keys");
  SetTargetFPS(FPS);
  int glyphCount = 95;
  int *codepoints = (int *)malloc(glyphCount * sizeof(int));

  for (int i = 0; i < glyphCount; i++) {
    codepoints[i] = 32 + i;
  }
  Font font = LoadFontEx(FONT, FONT_SIZE, codepoints, glyphCount);
  free(codepoints);

  Vector2 text_position = {10, 10};
  int i = 1;

  pthread_t input_thread;
  pthread_create(&input_thread, NULL, startupdater, (void *)buffer);

  while (!WindowShouldClose()) {
    BeginDrawing();
    // startupdater() -- async method
    ClearBackground(BLACK);
    DrawTextEx(font, buffer, text_position, FONT_SIZE, 0, WHITE);
    EndDrawing();
  }
  pthread_cancel(input_thread);
  pthread_join(input_thread, NULL);
  UnloadFont(font);
  CloseWindow();
}
int main() {
  char *buffer = calloc(sizeof(char), BUFFER_SIZE);
  strcpy(buffer, "             ");
  Window_Manager(buffer);
  free(buffer);
}

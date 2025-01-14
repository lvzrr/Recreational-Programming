/* Implementation of a ML algorithm for the OR gate
 * Works as an AND too if you change the training data
 * */

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

float train[][3] = {
    {0, 0, 0},
    {0, 1, 1},
    {1, 0, 1},
    {1, 1, 1},
};

float sigmoidf(float x) { return 1.f / (1.f + expf(-x)); }

float random_float() {
  // return ((int)rand() % 100 + (rand() % 10) * 0.1);
  return (float)rand() / (float)RAND_MAX;
}

#define train_count (sizeof(train) / sizeof(train[0]))

float cost(float w1, float w2, float bias) {
  float result = 0.00f;

  for (size_t i = 0; i < train_count; i++) {

    float x1 = train[i][0];
    float x2 = train[i][1];

    float expected = train[i][2];
    float y = sigmoidf((x1 * w1) + (x2 * w2) + bias);
    float d = y - expected;
    result += d * d;
  }
  result /= train_count;
  return result;
}

int main() {
  srand(time(0));

  float w1 = random_float();
  float w2 = random_float();
  float bias = random_float();

  printf("Initial weights: w1 = %f, w2 = %f, b = %f, c = %f\n", w1, w2, bias,
         cost(w1, w2, bias));

  float epsilon = 1e-1;
  float rate = 1e-1;

  for (size_t i = 0; i < 100000; ++i) {
    float c = cost(w1, w2, bias);
    float dw1 = (cost(w1 + epsilon, w2, bias) - c) / epsilon;
    float dw2 = (cost(w1, w2 + epsilon, bias) - c) / epsilon;
    float db = (cost(w1, w2, bias + epsilon) - c) / epsilon;
    w1 -= rate * dw1;
    w2 -= rate * dw2;
    bias -= rate * db;
  }

  printf("Final weights: w1 = %f, w2 = %f,b = %f, c = %f\n", w1, w2, bias,
         cost(w1, w2, bias));

  printf("x | y\n------\n");

  for (size_t i = 0; i < train_count; i++) {
    printf("%1.0f | %1.0f -> %f\n", train[i][0], train[i][1],
           sigmoidf((train[i][0] * w1) + (train[i][1] * w2) + bias));
  }
  return 0;
}

/* neural network that learns to multiply by TARGET */

#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#define TARGET 2

float train[][2] = {
    {0, TARGET * 0}, {1, TARGET},     {2, 2 * TARGET}, {3, 3 * TARGET},
    {4, 4 * TARGET}, {5, 5 * TARGET}, {6, 6 * TARGET},
};

#define MARGIN 0.02
#define train_count (sizeof(train) / sizeof(train[0]))
#define NUMTESTS 1000

float rand_float() {
  return ((float)rand() / (float)RAND_MAX) * 100;
} // we will guess a number between 0 and 100

float cost(float w) {

  float result = 0.0f;
  for (size_t i = 0; i < train_count; i++) {

    float exp = train[i][1]; // Expected output value

    // Training params

    float y = w * train[i][0]; // our output
    float d = y - exp;         // distance between guessed and expected

    result += d * d; // absoulte value of the difference
  }
  return result /= train_count; // average of differences
}

int main() {

  printf("\nArtificial neuron learns multiplication by %3d (%-4.3f margin | "
         "%03d tests)\n\n",
         TARGET, MARGIN, NUMTESTS);

  srand(time(0)); // set seed

  float w = rand_float();
  float temp_w = w;

  // Derivate cost function (Optimization by definition)
  // Limit (h->0): {cost(w+h)-cost(w)}/{h} where h is epsilon

  float epsilon = 1e-3; // infinitesimal steps for optimizing
  float rate = 1e-3;    // learning rate (so the derivative isnt too big)

  // lets train our model on different epochs
  // (first loop: epochs, second loop: the training itself)
  for (size_t j = 0; j < 1300; j += 100) {
    w = temp_w;
    for (size_t i = 0; i < j; i++) {

      float dcost =
          (cost(w + epsilon) - cost(w)) / epsilon; // derivative of cost

      w -= rate * dcost; // decrease our guess by the decreaded derivative (it
                         // will guide our guess (argument) to the minimum of
                         // the function)
                         //
                         // if dcost > 0 -> goes towards the decreasing
                         // direction as w is not in the good direction
                         //
                         // if dcost < 0 -> goes towards the decreasing too as
                         // --x = +x and w is already in the good direction
    }
    printf("Epoch %ld: w = %f, cost = %f\n", j, w, cost(w));
  }
}

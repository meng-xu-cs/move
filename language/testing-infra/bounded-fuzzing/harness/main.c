#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

extern bool process(const uint8_t *data, size_t size);

int main(void) {
  uint8_t data[0];
  process(data, 0);
}
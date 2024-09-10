#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

enum class LogLevel {
  Debug,
  Info,
  Warn,
  Error,
};

extern "C" {

int32_t test_rust(int32_t a, int32_t b);

void init_logger(LogLevel level);

}  // extern "C"

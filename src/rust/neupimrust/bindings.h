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

struct Settings {
  bool fast_read;
  bool fast_icnt;
};

struct NoIcnt {
  uintptr_t total_packages;
};

extern "C" {

int32_t test_rust(int32_t a, int32_t b);

void init_logger(LogLevel level);

void init_settings_with_file(const char *file_path);

void init_settings();

const Settings *get_settings();

uintptr_t get_total_packages(const NoIcnt *self);

void push(NoIcnt *self, uint32_t src, uint32_t dest, const void *request);

NoIcnt *new_icnt();

void delete_icnt(NoIcnt *ptr);

}  // extern "C"

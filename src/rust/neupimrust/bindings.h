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

template<typename T = void>
struct Option;

struct NoIcnt {
  uintptr_t total_packages;
};

/// A struct representing the application settings.
struct Settings {
  bool fast_read;
  bool fast_icnt;
  bool no_conflict_act_to_gact;
  bool no_conflict_gact_to_act;
};

extern "C" {

extern uint32_t GLOBAL_LOADS;

extern uint32_t GLOBAL_STORES;

extern uint32_t GLOBAL_COMPUTES;

extern Option<uint32_t> LAST_START_LOAD_CYCLE;

extern Option<uint32_t> LAST_START_STORE_CYCLE;

extern Option<uint32_t> LAST_START_COMPUTE_CYCLE;

extern uint32_t ACCUMULATED_DURATION_LOAD_CYCLE;

extern uint32_t ACCUMULATED_DURATION_STORE_CYCLE;

extern uint32_t ACCUMULATED_DURATION_COMPUTE_CYCLE;

/// 初始化日志记录器
///
/// # 参数
///
/// * `level` - 日志级别
void init_logger(LogLevel level);

/// cycle方法
void update_global_on_cycle(uint32_t cycle);

/// 保存累计的数据到文件
void save_global_counts_to_file();

/// 增加加载操作的计数
///
/// # 参数
///
/// * `loads` - 要增加的加载操作数量
void add_loads(uint32_t loads);

/// 增加存储操作的计数
///
/// # 参数
///
/// * `stores` - 要增加的存储操作数量
void add_stores(uint32_t stores);

/// 增加计算操作的计数
///
/// # 参数
///
/// * `computes` - 要增加的计算操作数量
void add_computes(uint32_t computes);

/// 获取当前的加载操作计数
///
/// # 返回值
///
/// 返回当前的加载操作总数
uint32_t get_loads();

/// 获取当前的存储操作计数
///
/// # 返回值
///
/// 返回当前的存储操作总数
uint32_t get_stores();

/// 获取当前的计算操作计数
///
/// # 返回值
///
/// 返回当前的计算操作总数
uint32_t get_computes();

/// 获取所有操作的总计数
///
/// # 返回值
///
/// 返回加载、存储和计算操作的总和
uint32_t get_total();

/// 减少加载操作的计数
///
/// # 参数
///
/// * `loads` - 要减少的加载操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
bool reduce_loads(uint32_t loads);

/// 减少存储操作的计数
///
/// # 参数
///
/// * `stores` - 要减少的存储操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
bool reduce_stores(uint32_t stores);

/// 减少计算操作的计数
///
/// # 参数
///
/// * `computes` - 要减少的计算操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
bool reduce_computes(uint32_t computes);

uintptr_t get_total_packages(const NoIcnt *self);

void push(NoIcnt *self, uint32_t src, uint32_t dest, const void *request);

NoIcnt *new_icnt();

void delete_icnt(NoIcnt *ptr);

/// Initializes the settings from a file specified by a C-style string path.
///
/// # Safety
///
/// This function is unsafe because it dereferences a raw pointer.
void init_settings_with_file(const char *file_path);

/// Initializes the settings using a default file path ("sjq.toml").
void init_settings();

/// Retrieves the current settings as a pointer to a `Settings` instance.
///
/// Returns a null pointer if the settings have not been initialized.
const Settings *get_settings();

}  // extern "C"

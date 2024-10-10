use std::fs::File;
use tracing::error;
#[no_mangle]
pub static mut GLOBAL_LOADS: u32 = 0;
#[no_mangle]
pub static mut GLOBAL_STORES: u32 = 0;
#[no_mangle]
pub static mut GLOBAL_COMPUTES: u32 = 0;

// 记录上一次加载、存储、计算操作的开始时间
#[no_mangle]
pub static mut LAST_START_LOAD_CYCLE: Option<u32> = None;
#[no_mangle]
pub static mut LAST_START_STORE_CYCLE: Option<u32> = None;
#[no_mangle]
pub static mut LAST_START_COMPUTE_CYCLE: Option<u32> = None;

// 记录持续的时间
#[no_mangle]
pub static mut ACCUMULATED_DURATION_LOAD_CYCLE: u32 = 0;
#[no_mangle]
pub static mut ACCUMULATED_DURATION_STORE_CYCLE: u32 = 0;
#[no_mangle]
pub static mut ACCUMULATED_DURATION_COMPUTE_CYCLE: u32 = 0;

/// cycle方法
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn update_global_on_cycle(cycle: u32) {
    unsafe {
        // 如果上一次加载、存储、计算操作的开始时间为None,切当前统计的加载、存储、计算操作不为0,则将当前统计的加载、存储、计算操作的开始时间设置为当前cycle
        if LAST_START_LOAD_CYCLE.is_none() && GLOBAL_LOADS != 0 {
            LAST_START_LOAD_CYCLE = Some(cycle);
        }
        if LAST_START_STORE_CYCLE.is_none() && GLOBAL_STORES != 0 {
            LAST_START_STORE_CYCLE = Some(cycle);
        }
        if LAST_START_COMPUTE_CYCLE.is_none() && GLOBAL_COMPUTES != 0 {
            LAST_START_COMPUTE_CYCLE = Some(cycle);
        }

        // 如果已经开始了，且当前统计为0，则将当前统计的加载、存储、计算操作的结束时间设置为当前cycle
        if LAST_START_LOAD_CYCLE.is_some() && GLOBAL_LOADS == 0 {
            ACCUMULATED_DURATION_LOAD_CYCLE += cycle - LAST_START_LOAD_CYCLE.unwrap();
        }
        if LAST_START_STORE_CYCLE.is_some() && GLOBAL_STORES == 0 {
            ACCUMULATED_DURATION_STORE_CYCLE += cycle - LAST_START_STORE_CYCLE.unwrap();
        }
        if LAST_START_COMPUTE_CYCLE.is_some() && GLOBAL_COMPUTES == 0 {
            ACCUMULATED_DURATION_COMPUTE_CYCLE += cycle - LAST_START_COMPUTE_CYCLE.unwrap();
        }
    }
}
#[derive(Debug, serde::Serialize)]
struct GlobalCounts {
    loads_cycles: u32,
    stores_cycles: u32,
    computes_cycles: u32,
}

/// 保存累计的数据到文件
#[no_mangle]
pub extern "C" fn save_global_counts_to_file() {
    let file = File::create("counts.json").expect("无法创建文件");
    let counts = GlobalCounts {
        loads_cycles: unsafe { ACCUMULATED_DURATION_LOAD_CYCLE },
        stores_cycles: unsafe { ACCUMULATED_DURATION_STORE_CYCLE },
        computes_cycles: unsafe { ACCUMULATED_DURATION_COMPUTE_CYCLE },
    };
    serde_json::to_writer_pretty(file, &counts).expect("无法写入文件");
}
/// 增加加载操作的计数
///
/// # 参数
///
/// * `loads` - 要增加的加载操作数量
#[no_mangle]
pub extern "C" fn add_loads(loads: u32) {
    unsafe {
        GLOBAL_LOADS += loads;
    }
}

/// 增加存储操作的计数
///
/// # 参数
///
/// * `stores` - 要增加的存储操作数量
#[no_mangle]
pub extern "C" fn add_stores(stores: u32) {
    unsafe {
        GLOBAL_STORES += stores;
    }
}

/// 增加计算操作的计数
///
/// # 参数
///
/// * `computes` - 要增加的计算操作数量
#[no_mangle]
pub extern "C" fn add_computes(computes: u32) {
    unsafe {
        GLOBAL_COMPUTES += computes;
    }
}

/// 获取当前的加载操作计数
///
/// # 返回值
///
/// 返回当前的加载操作总数
#[no_mangle]
pub extern "C" fn get_loads() -> u32 {
    unsafe { GLOBAL_LOADS }
}

/// 获取当前的存储操作计数
///
/// # 返回值
///
/// 返回当前的存储操作总数
#[no_mangle]
pub extern "C" fn get_stores() -> u32 {
    unsafe { GLOBAL_STORES }
}

/// 获取当前的计算操作计数
///
/// # 返回值
///
/// 返回当前的计算操作总数
#[no_mangle]
pub extern "C" fn get_computes() -> u32 {
    unsafe { GLOBAL_COMPUTES }
}

/// 获取所有操作的总计数
///
/// # 返回值
///
/// 返回加载、存储和计算操作的总和
#[no_mangle]
pub extern "C" fn get_total() -> u32 {
    unsafe { GLOBAL_LOADS + GLOBAL_STORES + GLOBAL_COMPUTES }
}

/// 减少加载操作的计数
///
/// # 参数
///
/// * `loads` - 要减少的加载操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
pub extern "C" fn reduce_loads(loads: u32) -> bool {
    unsafe {
        if GLOBAL_LOADS < loads {
            error!("错误：尝试将GLOBAL_LOADS减少到负值");
            return false;
        }
        GLOBAL_LOADS -= loads;
        true
    }
}

/// 减少存储操作的计数
///
/// # 参数
///
/// * `stores` - 要减少的存储操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
pub extern "C" fn reduce_stores(stores: u32) -> bool {
    unsafe {
        if GLOBAL_STORES < stores {
            error!("错误：尝试将GLOBAL_STORES减少到负值");
            return false;
        }
        GLOBAL_STORES -= stores;
        true
    }
}

/// 减少计算操作的计数
///
/// # 参数
///
/// * `computes` - 要减少的计算操作数量
///
/// # 返回值
///
/// 如果减少操作成功，返回`true`；如果减少操作会导致计数变为负值，返回`false`
#[no_mangle]
pub extern "C" fn reduce_computes(computes: u32) -> bool {
    unsafe {
        if GLOBAL_COMPUTES < computes {
            error!("错误：尝试将GLOBAL_COMPUTES减少到负值");
            return false;
        }
        GLOBAL_COMPUTES -= computes;
        true
    }
}

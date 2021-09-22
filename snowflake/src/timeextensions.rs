use std::time::{SystemTime, UNIX_EPOCH};

// 防止产生的时间比之前的时间还要小（由于NTP回拨等问题）,保持增量的趋势.
pub(crate) fn til_next_millis(last_timestamp: u64) -> u64 {
    let mut timestamp = current_time_millis();
    while timestamp < last_timestamp {
        timestamp = current_time_millis();
    }
    timestamp
}

// 获取当前的时间戳
pub(crate) fn current_time_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("系统当前时间早于1970-01-01 00:00:00,无法计算时间戳");
    let ms = since_the_epoch.as_secs() as u64 * 1000u64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as u64;
    ms
}

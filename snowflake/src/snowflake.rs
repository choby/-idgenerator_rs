use crate::timeextensions::{current_time_millis, til_next_millis};

#[derive(Debug)]
pub struct Snowflake {
    twepoch: u64,
    worker_id_bits: u8,
    datacenter_id_bits: u8,
    sequence_bits: u8,
    worker_id: u64,
    datacenter_id: u64,
    sequence: u64,
    last_timestamp: u64,
}

impl Snowflake {
    pub fn new(worker_id: u64, datacenter_id: u64) -> Snowflake {
        let snowflake = Snowflake {
            twepoch: 1420041600000,
            worker_id_bits: 5,
            datacenter_id_bits: 5,
            sequence_bits: 12,
            //MaxWorkerId  : -1 ^ (-1 << WorkerIdBits)
            sequence: 0,
            worker_id: worker_id,
            datacenter_id: 0,
            last_timestamp: 0,
        };
        if worker_id > snowflake.max_worker_id() {
            panic!(
                "worker Id 不能大于max_worker_id： {}",
                snowflake.max_worker_id()
            );
        }
        if datacenter_id > snowflake.max_datacenter_id() {
            panic!(
                "datacenterId不能大于max_datacenter_id: {}",
                snowflake.max_datacenter_id()
            );
        }
        snowflake
    }

    /**
     * 支持的最大机器id，结果是31 (这个移位算法可以很快的计算出几位二进制数所能表示的最大十进制数)
     */
    fn max_worker_id(&self) -> u64 {
        (-1 ^ (-1 << self.worker_id_bits)) as u64
    }
    /**
     * 数据标志ID最大值
     */
    fn max_datacenter_id(&self) -> u64 {
        (-1 ^ (-1 << self.datacenter_id_bits)) as u64
    }
    /**
     * 序列号ID最大值,这里为4095 (0b111111111111=0xfff=4095)
     */
    fn sequence_mask(&self) -> u64 {
        (-1 ^ (-1 << self.sequence_bits)) as u64
    }
    /**
     * 机器ID偏左移12位
     */
    fn worker_id_shift(&self) -> u8 {
        self.sequence_bits
    }

    /**
     * 数据ID偏左移17位
     */
    fn datacenter_id_shift(&self) -> u16 {
        (self.sequence_bits + self.worker_id_bits) as u16
    }

    /**
     * 数据中心ID(0~31)
     */
    fn timestamp_leftshift(&self) -> u16 {
        (self.sequence_bits + self.worker_id_bits + self.datacenter_id_bits) as u16
    }

    pub fn next(&mut self) -> u64 {
        let mut timestamp = current_time_millis();
        if self.last_timestamp > timestamp {
            panic!(
                "时间戳必须大于上一次生成ID的时间戳.  拒绝为{}毫秒生成id",
                self.last_timestamp - timestamp
            );
        }
        //如果上次生成时间和当前时间相同,在同一毫秒内
        if self.last_timestamp == timestamp {
            //sequence自增，和sequenceMask相与一下，去掉高位
            self.sequence = (self.sequence + 1) & self.sequence_mask();
            //判断是否溢出,也就是每毫秒内超过1024，当为1024时，与sequenceMask相与，sequence就等于0
            if self.sequence == 0 {
                //等待到下一毫秒
                timestamp = til_next_millis(self.last_timestamp);
            }
        } else {
            //如果和上次生成时间不同,重置sequence，就是下一毫秒开始，sequence计数重新从0开始累加,
            //为了保证尾数随机性更大一些,最后一位可以设置一个随机数
            self.sequence = 0;
        }
        self.last_timestamp = timestamp;
        return ((timestamp - self.twepoch) << self.timestamp_leftshift())
            | (self.datacenter_id << self.datacenter_id_shift())
            | (self.worker_id << self.worker_id_shift())
            | self.sequence;
    }
}

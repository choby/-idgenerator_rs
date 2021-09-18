use std::io::{self, BufRead};

#[derive(Debug)]
struct IdWorker {
    Twepoch: u64,
    WorkerIdBits: u8,
    DatacenterIdBits: u8,
    SequenceBits: u8,
    // MaxWorkerId: u64,
    // MaxDatacenterId: u64,
    // SequenceMask: u64,
    // WorkerIdShift: u32,
    // DatacenterIdShift: u32,
    // TimestampLeftShift: u32,
    WorkerId: u64,
    DatacenterId: u64,
    Sequence: u64,
    LastTimestamp: u64,
}

impl IdWorker {
    pub fn new(workerId: u64, datacenterId: u64) -> IdWorker {
        let idworker = IdWorker {
            Twepoch: 1420041600000,
            WorkerIdBits: 5,
            DatacenterIdBits: 5,
            SequenceBits: 12,
            //MaxWorkerId  : -1 ^ (-1 << WorkerIdBits)
            Sequence: 0,
            WorkerId: workerId,
            DatacenterId: 0,
            LastTimestamp: 0,
        };
        if workerId > idworker.max_worker_id() {
            panic!(format!(
                "worker Id 不能大于max_worker_id： {}",
                idworker.max_worker_id()
            ));
        }
        if datacenterId > idworker.max_datacenter_id() {
            panic!(format!(
                "datacenterId不能大于max_datacenter_id: {}",
                idworker.max_datacenter_id()
            ));
        }
        idworker
    }

    /**
     * 支持的最大机器id，结果是31 (这个移位算法可以很快的计算出几位二进制数所能表示的最大十进制数)
     */
    fn max_worker_id(&self) -> u64 {
        (-1 ^ (-1 << self.WorkerIdBits)) as u64
    }
    /**
     * 数据标志ID最大值
     */
    fn max_datacenter_id(&self) -> u64 {
        (-1 ^ (-1 << self.DatacenterIdBits)) as u64
    }
    /**
     * 序列号ID最大值,这里为4095 (0b111111111111=0xfff=4095)
     */
    fn sequence_mask(&self) -> u64 {
        (-1 ^ (-1 << self.SequenceBits)) as u64
    }
    /**
     * 机器ID偏左移12位
     */
    fn worker_id_shift(&self) -> u8 {
        self.SequenceBits
    }

    /**
     * 数据ID偏左移17位
     */
    fn datacenter_id_shift(&self) -> u16 {
        (self.SequenceBits + self.WorkerIdBits) as u16
    }

    /**
     * 数据中心ID(0~31)
     */
    fn timestamp_leftshift(&self) -> u16 {
        (self.SequenceBits + self.WorkerIdBits + self.DatacenterIdBits) as u16
    }

    pub fn next_id(&self) {
        // if self.
    }
}

fn current_time_millis() {
    (long)(DateTime.UtcNow - Jan1st1970).TotalMilliseconds;
}

fn main() {
    let idworker = IdWorker::new(6, 10);
    //s1 = &String::from("asdf hgdfhdfg");
    idworker.max_worker_id();
    println!("s1的值是:{:?}", &idworker.max_worker_id());
}

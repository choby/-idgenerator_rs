SnowFlake is an algorithm adopted by Twitter, whose purpose is to generate globally unique and trend-increasing IDs in a distributed system.

How many globally unique IDs can the SnowFlake algorithm generate in the same millisecond? Number of IDs in the same millisecond = 1024 X 4096 = 4194304

let mut worker = Snowflake::new(1, 1);

let id = worker.next();

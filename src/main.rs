use snowflake::Snowflake;

fn main() {
    let mut idworker = Snowflake::new(6, 10);
    let mut index = 0;
    while index < 100 {
        let id = idworker.next();
        println!("当前id:{:?}", id);
        index = index + 1;
    }
}

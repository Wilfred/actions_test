fn main() {
    println!("Hello, world!");
    dbg!(
        option_env!("DFT_COMMIT_SHORT_HASH"),
        option_env!("DFT_COMMIT_HASH"),
        option_env!("DFT_COMMIT_DATE")
    );
}

fn main() {
    let mut count = 0u64;
    println!("count is {}", count);
    while count < 1_000_000_000{
        count += 1;
    }
    println!("count is {}", count)
}

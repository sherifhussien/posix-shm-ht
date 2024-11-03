mod args;

fn main() {
    let ht_size: i32 = args::parse_args();
    println!("Hash table size: {ht_size}");
}

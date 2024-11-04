mod args;
mod hash_table;

use hash_table::HashTable;

fn main() {
    let ht_size: i32 = args::parse_args();
    println!("Hash table size: {ht_size}");

    let ht: HashTable<String, String> = hash_table::HashTable::new(ht_size as usize);
    println!("{:?}", ht);

    let idx = ht.hash(&"test2".to_string());
    print!("{}", idx);
}

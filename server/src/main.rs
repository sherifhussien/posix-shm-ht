mod args;
mod hash_table;

use hash_table::HashTable;

fn main() {
    let ht_size: i32 = args::parse_args();
    println!(">> Hash table size: {ht_size}");

    let mut ht: HashTable<String, String> = hash_table::HashTable::new(ht_size as usize);
    println!("{:?}", ht);


    ht.insert("test1".to_string(), "value1".to_string());
    println!("{:?}", ht);

    ht.insert("test2".to_string(), "value2".to_string());
    println!("{:?}", ht);

    ht.insert("test3".to_string(), "value3".to_string());
    println!("{:?}", ht);

    let g1 = ht.get("test1");
    println!("{:?}", g1);

    ht.insert("test1".to_string(), "value1 updated".to_string());
    println!("{:?}", ht);

    let g1 = ht.get("test1");
    println!("{:?}", g1);

    let g1 = ht.remove("test4");
    println!("{:?}", g1);

    let g1 = ht.remove("test1");
    println!("{:?}", g1);

    ht.insert("test6".to_string(), "value6".to_string());
    println!("{:?}", ht);

    ht.insert("test7".to_string(), "value7".to_string());
    println!("{:?}", ht);

}
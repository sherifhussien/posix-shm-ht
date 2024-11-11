use clap::{Arg, Command, crate_version};


pub fn parse_args() -> usize {
    let matches = Command::new("ht")
        .version(crate_version!())
        .about("A hash table that supports concurrent operations")
        .arg(
            Arg::new("hash_table_size")
                .short('n')
                .long("ht_size")
                .default_value("10")
                .help("Set the hash table size")
        )
        .get_matches(); // parse env::args_os

    let ht_size = matches.get_one::<String>("hash_table_size").unwrap();
    let ht_size: usize = ht_size.parse().expect("provided hash table size is not a number!");

    ht_size
}
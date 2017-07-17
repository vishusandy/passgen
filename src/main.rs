
extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate sconcat;
extern crate time;

mod dictsort;
mod password;

use dictsort::*;
use password::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    // dict = getcfg("words-sorted.bin");
    dict = getdict(false, true, true);
    // getdict(true, true, false);
    // dict = getcfg("sorted.bin");
    wordlengths(&dict);
    // wordlengths(&dict);
    let end = start.elapsed();
    println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
}

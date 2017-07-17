
extern crate argparse;
#[macro_use] extern crate log;
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
    
    // getdict(savefile, plurals, output)
    dict = getdict(false, false, false);
    
    // wordlengths(&dict);
    
    // dict len caps nums punc special
    let pass = transform(&dict, 8, true, false, false, "");
    println!("Password: {}", pass);
    
    let end = start.elapsed();
    println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
}

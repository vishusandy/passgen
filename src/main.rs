
#[macro_use] 
extern crate log;

extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
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
    let pass = transform(&dict, 10, true, true, true, "");
    println!("Password: {}", pass);
    
    let end = start.elapsed();
    println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
}

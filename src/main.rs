
extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate time;

mod dictsort;

use dictsort::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    dict = getcfg("words-sorted.bin");
    let mut largest = 0u8;
    let mut lens: HashMap<u8, usize> = HashMap::new();
    for (key, v) in &dict {
        if v.len() > 0 {
            let wrd = match v.last() {
                Some(w) => w,
                None => "Cannot find word",
            };
            print!("Last word is {}", wrd);
        }
        if *key > largest {
            largest = *key;
        }
        lens.insert(*key, v.len());
        println!(" Length {} has {} words", key, v.len());
    }
    
    
    let end = start.elapsed();
    println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
}

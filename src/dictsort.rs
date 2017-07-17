// todo: change deserialization from reading file to include_str()

extern crate time;
// #[macro_use]
// extern crate serde_derive;

extern crate serde;
extern crate rmp_serde as rmps;

#[allow(unused_imports)]
use std::collections::{HashMap, BTreeSet};
// use std::time::Instant;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

// getcfg() reads the hashmap dictionary (grouped by word lengths) from file
#[allow(dead_code)]
pub fn getcfg(file: &str) -> HashMap<u8, Vec<String>> {
    // let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    let mut buf = Vec::new();
    let mut f = File::open(file).expect("Could not open dictionary");
    #[allow(unused_must_use)]
    match f.read_to_end(&mut buf) {
        Ok(_) => {
            let mut ds = Deserializer::new(&buf[..]);
            Deserialize::deserialize(&mut ds).expect("Could not deserialize dictionary data")
        },
        _ => {
            HashMap::new()
        },
    }
    // dict = Deserialize::deserialize(&mut ds).expect("Could not deserialize dictionary data");
    // dict
}

// reads words.txt and returns hashmap dictionary and optionally saves dictionary to file
#[allow(dead_code)]
pub fn getdict(savefile: bool, plurals: bool, output: bool) -> HashMap<u8, Vec<String>> {
    // let start = Instant::now();
    #[allow(non_snake_case)]
    let WORDSTR: &'static str = include_str!("words.txt");
    
    // let mut dict: HashMap<u8, BTreeSet<String>> = HashMap::new();
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    
    let mut count = 0u64;
    let mut cap = 0u64;
    for line in WORDSTR.lines() {
        
        let mut word = line.to_string();
        if word.ends_with("%") {
            if !plurals {
                continue;
            }
            word.pop();
        }
        cap += word.len() as u64;
        let len = word.len();
        let b = dict.entry(len as u8).or_insert(Vec::new());
        b.push(word);
        count += 1;

        // Find the BTreeSet that stores words of the same length as the current word
        // let b = dict.entry(len as u8).or_insert(BTreeSet::new());
        // b.insert(word);
    }
    let bufcap = count * 9;
    if savefile {
        let mut buf = Vec::with_capacity((bufcap) as usize);
        dict.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialize dictionary");
        let mut f = BufWriter::new(File::create("sorted.bin").expect("Could not create output file"));
        #[allow(unused_must_use)]
        f.write(&buf).expect("Could not write to file");
    }
    if output {
        // let mut largest = 0u8;
        // let mut lens: HashMap<u8, usize> = HashMap::new();
        // for (key, v) in &dict {
            // 
            // if *key > largest {
                // largest = *key;
            // }
            // lens.insert(*key, v.len());
            // println!("Length {} has {} words", key, v.len());
            // println!("Lengths:\n{:?}\n--------------\n", lens);
            // println!("Largest length: {}\nNumber words: {}\nCapacity: {}\nVec Cap: {}", largest, count, cap, bufcap);
        // }
        // dictinfo(&dict);
        println!("# Words: {}\nCapacity: {}\nBuffer Capacity: {}", count, cap, bufcap);
    }
    
    // let end = start.elapsed();
    // if output {
        // println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
    // }
    dict
}

#[allow(dead_code)]
pub fn dictinfo(dict: &HashMap<u8, Vec<String>>) {
    let mut largest = 0u8;
    let mut lens: HashMap<u8, usize> = HashMap::new();
    for (key, v) in dict {
        
        if *key > largest {
            largest = *key;
        }
        lens.insert(*key, v.len());
        println!("Length {} has {} words", key, v.len());
    }
    println!("Lengths:\n{:?}\n--------------\n", lens);
    // println!("Largest length: {}\nNumber words: {}\nCapacity: {}\nVec Cap: {}", largest);
}

pub fn wordlengths(dict: &HashMap<u8, Vec<String>>) {
    let mut largest = 0u8;
    let mut lens: HashMap<u8, usize> = HashMap::new();
    // for r in dict.get(1..30) {
    for key in 1..30 {
        match dict.get(&key) {
            Some(v) => {
                print!("Length {: >2} has {: >5} words ", key, v.len());
                if v.len() > 0 {
                    let wrd = match v.last() {
                        Some(w) => w,
                        None => "Cannot find word",
                    };
                    println!("\t{}", wrd);
                }
                if key > largest {
                    largest = key;
                }
                lens.insert(key, v.len());
            },
            None => {},
        }
    }
    // for (key, v) in dict {
    //     print!("Length {: >2} has {: >5} words ", key, v.len());
    //     if v.len() > 0 {
    //         let wrd = match v.last() {
    //             Some(w) => w,
    //             None => "Cannot find word",
    //         };
    //         println!("\t{}", wrd);
    //     }
    //     if *key > largest {
    //         largest = *key;
    //     }
    //     lens.insert(*key, v.len());
    // }
}

// todo: change deserialization from reading file to include_str()

extern crate time;
// #[macro_use]
// extern crate serde_derive;

extern crate serde;
extern crate rmp_serde as rmps;

#[allow(unused_imports)]
use std::collections::{HashMap, BTreeSet};
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

// use dictionary::WORDLIST;
use dictionary::{SERIALIZED_DICT, SERIALIZED_DICT_STR, WORDLIST};

// getcfg() reads the hashmap dictionary (grouped by word lengths) from file
#[allow(dead_code)]
pub fn deserialize_dict() -> HashMap<u8, Vec<String>> {
    // let mut buf = Vec::new();
    
    // let mut f = File::open(file).expect("Could not open dictionary");
    // #[allow(unused_must_use)]
    
    let mut ds = Deserializer::new(&SERIALIZED_DICT[..]);
    Deserialize::deserialize(&mut ds).expect("Could not deserialize dictionary data")
    
    /*
    match f.read_to_end(&mut buf) {
        Ok(_) => {
            // let mut ds = Deserializer::new(&buf[..]);
            let mut ds = Deserializer::new(&SERIALIZED_DICT[..]);
            Deserialize::deserialize(&mut ds).expect("Could not deserialize dictionary data")
        },
        _ => {
            println!("Could not deserialize dictionary properly.");
            HashMap::new()
        },
    }
    */
    // dict
}


#[allow(dead_code)]
pub fn deserialize_dict_original(file: &str) -> HashMap<u8, Vec<String>> {
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
    // dict
}

#[allow(dead_code)]
pub fn get_dict(plurals: bool, output: bool) -> HashMap<u8, Vec<String>> {
    // let start = Instant::now();
    // #[allow(non_snake_case)]
    // let WORDLIST: &'static str = include_str!("words.txt");
    
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    
    let mut count = 0u64;
    let mut cap = 0u64;
    for line in WORDLIST.lines() {
        
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
    }
    let bufcap = count * 9;
    if output {
        println!("# Words: {}\nCapacity: {}\nBuffer Capacity: {}", count, cap, bufcap);
    }
    
    dict
}


// reads words.txt and returns hashmap dictionary and optionally saves dictionary to file
#[allow(dead_code)]
pub fn save_dict(mut savefile: &str, plurals: bool, output: bool) -> HashMap<u8, Vec<String>> {
    // let start = Instant::now();
    // #[allow(non_snake_case)]
    // let WORDLIST: &'static str = include_str!("words.txt");
    
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    
    let mut count = 0u64;
    let mut cap = 0u64;
    for line in WORDLIST.lines() {
        
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
    }
    let bufcap = count * 9;
    
    // save serialized file
    if savefile == "" {
        savefile = "sorted.bin";
    }
    let mut buf = Vec::with_capacity((bufcap) as usize);
    dict.serialize(&mut Serializer::new(&mut buf)).expect("Could not serialize dictionary");
    let mut f = BufWriter::new(File::create(savefile).expect("Could not create output file"));
    
    #[allow(unused_must_use)]
    f.write(&buf).expect("Could not write to file");
    
    if output {
        println!("# Words: {}\nCapacity: {}\nBuffer Capacity: {}", count, cap, bufcap);
    }
    
    dict
}

#[allow(dead_code)]
pub fn dict_info(dict: &HashMap<u8, Vec<String>>) {
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




#[allow(dead_code)]
pub fn word_lengths(dict: &HashMap<u8, Vec<String>>) {
    let mut largest = 0u8;
    let mut lens: HashMap<u8, usize> = HashMap::new();
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
}



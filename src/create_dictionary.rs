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
// use dictionary::{SERIALIZED_DICT, SERIALIZED_DICT_STR, WORDLIST};
use dictionary::{SERIALIZED_DICT_STR, WORDLIST};


pub fn deserialize_dict() -> HashMap<u8, Vec<&'static str>> {
    // let mut buf = Vec::new();
    
    // let mut f = File::open(file).expect("Could not open dictionary");
    // #[allow(unused_must_use)]
    
    let mut ds = Deserializer::new(&SERIALIZED_DICT_STR[..]);
    Deserialize::deserialize(&mut ds).expect("Could not deserialize dictionary data")
    
}


#[allow(dead_code)]
pub fn get_dict(plurals: bool, output: bool) -> HashMap<u8, Vec<&'static str>> {
    // let start = Instant::now();
    // #[allow(non_snake_case)]
    // let WORDLIST: &'static str = include_str!("words.txt");
    
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<&'static str>> = HashMap::new();
    
    let mut count = 0u64;
    let mut cap = 0u64;
    for line in WORDLIST.lines() {
        
        let word: &str;
        if &line[line.len()-1..line.len()] == "%" {
            if !plurals {
                continue;
            }
            word = &line[..line.len()-1];
        } else {
            word = line;
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


pub fn save_dict(mut savefile: &str, plurals: bool, output: bool) -> HashMap<u8, Vec<&'static str>> {
    // let start = Instant::now();
    // #[allow(non_snake_case)]
    // let WORDLIST: &'static str = include_str!("words.txt");
    
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<&'static str>> = HashMap::new();
    
    let mut count = 0u64;
    let mut cap = 0u64;
    for line in WORDLIST.lines() {
        
        let word: &str;
        if &line[line.len()-1..line.len()] == "%" {
            if !plurals {
                continue;
            }
            word = &line[..line.len()-1];
        } else {
            word = line;
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

pub fn save_dict_code(var_name: &str, dict: &HashMap<u8, Vec<&'static str>>, savefile: &str) {
    // let start = Instant::now();
    // #[allow(non_snake_case)]
    // let WORDLIST: &'static str = include_str!("words.txt");
    
    let mut capacity: usize = 0;
    for (length, v) in dict {
        capacity += v.len() * (*length as usize);
    }
    
    let mut output = String::with_capacity(capacity+capacity);
    
    // output.push_str("let dictionary: HashMap<u8, Vec<&'static str>> = [\n");
//     pub fn add_dict_nop(dict: &mut HashMap<u8, Vec<&'static str>>) {
// dict.insert(2, vec![
    
    output.push_str("use std::collections::HashMap;\n\npub fn ");
    output.push_str(var_name);
    output.push_str("(dict: &mut HashMap<u8, Vec<&'static str>>) { \n");
    for i in 0u8..30 {
        if let Some(v) = dict.get(&i) {
            output.push_str("\tdict.insert(");
            output.push_str(&(i.to_string()));
            output.push_str(", vec![\n");
            for j in 0..v.len() {
                output.push_str("\t\t\"");
                output.push_str(v[j]);
                output.push_str("\",\n");
            }
            output.push_str("\t]);\n");
        }
    }
    output.push_str("}");
    // output.push_str("].iter().cloned().collect();");
    
    let mut f = BufWriter::new(File::create(savefile).expect("Could not save dictionary code file."));
    #[allow(unused_must_use)]
    f.write(output.as_bytes());
    
}


#[allow(dead_code)]
pub fn dict_info(dict: &HashMap<u8, Vec<&'static str>>) {
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
pub fn word_lengths(dict: &HashMap<u8, Vec<&'static str>>) {
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
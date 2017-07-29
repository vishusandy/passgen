use dict_list_all::*;
use dict_list_np::*;
use plurals_list::*;
use rand::{thread_rng, Rng};
use rand::distributions::range::SampleRange;
// use num_integer::*;
use num::{Num, Zero, One};
use std::ops::Add;
use std::collections::HashMap;

use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;


pub fn safe_range<N>(start: N, end: N) -> N where N: Num + PartialOrd + Copy + SampleRange  {
    let e = end + N::one();
    if start < end {
        let mut rg = thread_rng();
        rg.gen_range(start, e)
    } else {
        start
    }
}


#[allow(dead_code)]
pub fn init_dict() -> HashMap<u8, (usize, usize)> {
    let mut dict: HashMap<u8, (usize, usize)> = HashMap::new();
    let version = DICT_A_INDEXES;
    for i in 0..version.len() {
        // let tmp = version[i].1;
        // dict.insert(version[i].0, (tmp.0, tmp.1) );
        dict.insert(version[i].0, (version[i].1 .0, version[i].1 .1) );
    }
    dict
}

#[allow(dead_code)]
pub fn is_word2(dict: &HashMap<u8, &'static [&'static str]>, plurals: bool, word: &str) -> bool {
        if let Some(v) = dict.get(&(word.len() as u8)) {
            // for i in v.0 .. v.1+1 {
            for i in 0..v.len() {
                let w = v[i];
                let t = v[i].len();
                if &w[t-1..t] == "%" {
                    if !plurals {
                        continue;
                    } else {
                        if &w[..t-1] == word {
                            return true;
                        }
                    }
                } else {
                    if w == word {
                        return true;
                    }
                }
                // if DICT_A_LIST[i] == word {
                //     return true;
                // }
            }
        }
        false
}

// pub fn init_dict2<A, B>(version: &[&'static str], list: &[(u8, (usize, usize))]) -> HashMap<u8, &'static [&'static str]> {

#[allow(dead_code)]
// pub fn init_dict2<'a>(list: &'a[&'static str], version: &'a[(u8, (usize, usize))]) -> HashMap<u8, &'static [&'static str]> {
pub fn init_dict2(plurals: bool) -> HashMap<u8, &'static [&'static str]> {
    // let version = if plurals { DICT_A_INDEXES } else { DICT_NP_INDEXES };
    // let list = if plurals { DICT_A_LIST } else { DICT_NP_LIST };
    if plurals {
        let mut dict: HashMap<u8, &'static [&'static str]> = HashMap::new();
        
        // let DICT_A_INDEXES = DICT_A_INDEXES;
        
        for i in 0..DICT_A_INDEXES.len() {
            dict.insert(DICT_A_INDEXES[i].0, &DICT_A_LIST[DICT_A_INDEXES[i].1 .0 .. DICT_A_INDEXES[i].1 .1]);
            if &DICT_A_LIST[DICT_A_INDEXES[i].1 .0 .. DICT_A_INDEXES[i].1 .1].len() != &(DICT_A_INDEXES[i].1 .1 - DICT_A_INDEXES[i].1 .0) {
                println!("Length mismatch");
                // println!("Length slice mismatch.  S0={}, S1={}", DICT_A_INDEXES[i].1 .0, DICT_A_INDEXES[i].1 .1);
            }
            // let tmp = DICT_A_INDEXES[i].1;
            // dict.insert(DICT_A_INDEXES[i].0, (tmp.0, tmp.1) );
            // dict.insert(DICT_A_INDEXES[i].0, (DICT_A_INDEXES[i].1 .0, DICT_A_INDEXES[i].1 .1) );
        }
        dict
    } else {
        let mut dict: HashMap<u8, &'static [&'static str]> = HashMap::new();
        
        // let DICT_NP_INDEXES = DICT_A_INDEXES;
        
        for i in 0..DICT_NP_INDEXES.len() {
            dict.insert(DICT_NP_INDEXES[i].0, &DICT_NP_LIST[DICT_NP_INDEXES[i].1 .0 .. DICT_NP_INDEXES[i].1 .1]);
            if &DICT_NP_LIST[DICT_NP_INDEXES[i].1 .0 .. DICT_NP_INDEXES[i].1 .1].len() != &(DICT_NP_INDEXES[i].1 .1 - DICT_NP_INDEXES[i].1 .0) {
                println!("Length mismatch");
                // println!("Length slice mismatch.  S0={}, S1={}", DICT_NP_INDEXES[i].1 .0, DICT_NP_INDEXES[i].1 .1);
            }
            // let tmp = DICT_NP_INDEXES[i].1;
            // dict.insert(DICT_NP_INDEXES[i].0, (tmp.0, tmp.1) );
            // dict.insert(DICT_NP_INDEXES[i].0, (DICT_NP_INDEXES[i].1 .0, DICT_NP_INDEXES[i].1 .1) );
        }
        dict
    }
    
}

#[allow(dead_code)]
pub fn get_word2(dict: &HashMap<u8, &'static [&'static str]>, len: u8) -> &'static str {
    let mut rg = thread_rng();
    match dict.get(&len) {
        Some( r ) => {
            // let chosen = safe_range(r.0, r.1);
            // DICT_A_LIST[chosen]
            if let Some(c) = rg.choose(r) {
                c
            } else {
                "error1"
            }
            
        },
        None => {
            #[allow(unused_assignments)]
            let mut closest: u8 = 0;
            let mut lower: u8 = 0;
            let mut upper: u8 = 0;
            // get closest length
            for n in dict.keys() {
                if *n < len && *n > lower {
                    lower = *n;
                } else if *n > len && *n < upper {
                    upper = *n;
                }
            }
            if len-lower <= upper-len {
                closest = lower;
            } else {
                closest = upper;
            }
            match dict.get(&closest) {
                Some( r ) => {
                    if let Some(c) = rg.choose(r) {
                        c
                    } else {
                        "error2"
                    }
                    // let chosen = safe_range(r.0, r.1);
                    // let out = DICT_A_LIST[chosen];
                    // debug!("get_word(len={}) = {}", len, out);
                    // out
                },
                None => {
                    println!("Error getting closest word length to {}\nclosest: {}", len, closest);
                    "error0"
                },
            }
        },
    }
}

#[allow(dead_code)]
pub fn get_word(dict: &HashMap<u8, (usize, usize)>, len: u8) -> &'static str {
    match dict.get(&len) {
        Some( r ) => {
            let chosen = safe_range(r.0, r.1);
            DICT_A_LIST[chosen]
        },
        None => {
            #[allow(unused_assignments)]
            let mut closest: u8 = 0;
            let mut lower: u8 = 0;
            let mut upper: u8 = 0;
            // get closest length
            for n in dict.keys() {
                if *n < len && *n > lower {
                    lower = *n;
                } else if *n > len && *n < upper {
                    upper = *n;
                }
            }
            if len-lower <= upper-len {
                closest = lower;
            } else {
                closest = upper;
            }
            match dict.get(&closest) {
                Some( r ) => {
                    let chosen = safe_range(r.0, r.1);
                    let out = DICT_A_LIST[chosen];
                    debug!("get_word(len={}) = {}", len, out);
                    out
                },
                None => {
                    println!("Error getting closest word length to {}\nclosest: {}", len, closest);
                    "error"
                },
            }
        },
    }
}

pub fn rand_length(len: u8, min: u8) -> u8 {
    match len {
        n if n <= min => 0,
        n if n < 5 => 1,
        n if n < 7 => safe_range(1, 2),
        n if n < 11 => safe_range(2, 3),
        n if n >= 11 => safe_range(2, 4),
        _ => 1,
    }
}



#[allow(dead_code)]
pub fn is_word_plurals(dict: &HashMap<u8, &'static [&'static str]>, word: &str) -> bool {
        if let Some(v) = dict.get(&(word.len() as u8)) {
            // for i in v.0 .. v.1+1 {
            for i in 0..v.len() {
                if v[i] == word {
                    return true;
                }
            }
        }
        for i in 0..PLURALS.len() {
            if PLURALS[i] == word {
                return true;
            }
        }
        false
}

#[allow(dead_code)]
pub fn is_word(dict: &HashMap<u8, (usize, usize)>, word: &str) -> bool {
        if let Some(v) = dict.get(&(word.len() as u8)) {
            for i in v.0 .. v.1+1 {
                let w = DICT_A_LIST[i];
                let t = DICT_A_LIST[i].len();
                if &w[t-1..t] == "%" {
                    if &w[..t-1] == word {
                        return true;
                    }
                }
                else if DICT_A_LIST[i] == word {
                    return true;
                }
            }
        }
        false
}

pub fn find_plurals(savefile: &str) -> Vec<&'static str> {
    // let all = init_dict2(DICT_A_INDEXES[..], DICT_A_LIST[..]);
    // let nop = init_dict2(DICT_NP_INDEXES[..], DICT_NP_LIST[..]);
    
    let mut n = 0usize;
    let mut plurals = Vec::new();
    
    for a in 0..DICT_A_LIST.len() {
        if n < DICT_NP_LIST.len() && DICT_A_LIST[a] != DICT_NP_LIST[n] {
            plurals.push(DICT_A_LIST[a]);
        } else {
            n += 1;
        }
    }
    let mut f = BufWriter::new(File::create(savefile).expect("Could not create output file"));
    for i in 0..plurals.len() {
        
        #[allow(unused_must_use)]
        f.write(plurals[i].as_bytes());
        #[allow(unused_must_use)]
        f.write(b"\n");
    }
    plurals
}


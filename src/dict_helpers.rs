use dict_list_all::*;
// use dict_list_np::*;
use rand::{thread_rng, Rng};
use rand::distributions::range::SampleRange;
use std::collections::HashMap;


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

pub fn init_dict2() -> HashMap<u8, &'static [&'static str]> {
    let mut dict: HashMap<u8, &'static [&'static str]> = HashMap::new();
    let version = DICT_A_INDEXES;
    for i in 0..version.len() {
        dict.insert(version[i].0, &DICT_A_LIST[version[i].1 .0 .. version[i].1 .1]);
        if &DICT_A_LIST[version[i].1 .0 .. version[i].1 .1].len() != &(version[i].1 .1 - version[i].1 .0) {
            println!("Length slice mismatch.  S0={}, S1={}", version[i].1 .0, version[i].1 .1);
        }
        // let tmp = version[i].1;
        // dict.insert(version[i].0, (tmp.0, tmp.1) );
        // dict.insert(version[i].0, (version[i].1 .0, version[i].1 .1) );
    }
    dict
}

pub fn safe_range<T: PartialOrd + SampleRange>(start: T, end: T) -> T {
    if start < end {
        let mut rg = thread_rng();
        rg.gen_range(start, end)
    } else {
        start
    }    
}


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
            // #[allow(unused_assignments)]
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

pub fn is_word2(dict: &HashMap<u8, &'static [&'static str]>, word: &str) -> bool {
        if let Some(v) = dict.get(&(word.len() as u8)) {
            // for i in v.0 .. v.1+1 {
            for i in 0..v.len() {
                if DICT_A_LIST[i] == word {
                    return true;
                }
            }
        }
        false
}

pub fn is_word(dict: &HashMap<u8, (usize, usize)>, word: &str) -> bool {
        if let Some(v) = dict.get(&(word.len() as u8)) {
            for i in v.0 .. v.1+1 {
                if DICT_A_LIST[i] == word {
                    return true;
                }
            }
        }
        false
}

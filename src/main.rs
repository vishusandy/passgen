// TODO: save serialized file if it does not exist,
//       if it does exist read and deserialize the file
//         on error read the words.txt file

// Todo: make the list of words a static array
//        then make a HashMap that contains the length and an array slice
//        update functions accordingly

// Todo: fix error where requested word size is too large and get_word()
//         returns "error"

// Todo: the dict_list is for no plurals only, possibly change this?


// Todo: try to load the words as a static array of vectors.  Then either      
// recode the functions to take an array or change the hashmap to reference a  
// vector and do the inserts for each length with a reference to the vector    
// in the array index of the same length.                                      

#[macro_use] 
extern crate log;

extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate time;

// mod dictsort;
mod dict_code_all;
mod dict_code_noplurals;
mod create_dictionary;
mod dict_list;
mod dict_list_all;
// mod password;
mod passwords;
mod dictionary;

use argparse::{ArgumentParser, StoreTrue, Store};
// use dictsort::*;
use create_dictionary::*;
use dict_code_all::*;
use dict_code_noplurals::*;
use dict_list_all::*;
// use leet::*;
use passwords::*;
use std::collections::HashMap;
use std::time::Instant;
use std::path::Path;

fn main() {
    let start = Instant::now();
    #[allow(unused_assignments)]
    // let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    
    
    // dict_info(dict)
    // word_lengths(dict)
    // deserialize_dict(file: &str)
    // get_dict(plurals: bool, outout: bool)
    // save_dict(savefile: &str, plurals: bool, output: bool)
    
    
    // dict = get_dict(false, false);
    // dict = deserialize_dict();
    
    
    // dict = get_dict(true, false);
    
    // let mut dict2: HashMap<u8, Vec<&'static str>> = save_dict2("str_dict.bin", true, true);
    
    
    
    let mut dict: HashMap<u8, Vec<&'static str>> = HashMap::new();
    let mut dict_nop: HashMap<u8, Vec<&'static str>> = HashMap::new();
    if Path::new("passgen_dict.msgpack").exists() {
        add_dict_all(&mut dict);
    } else {
        dict = save_dict("passgen_dict.msgpack", true, false);
    }
    
    if Path::new("passgen_dict_noplurals.msgpack").exists() {
        add_dict_nop(&mut dict_nop);
    } else {
        dict_nop = save_dict("passgen_dict_noplurals.msgpack", false, false);
    }
    
    
/*    let mut dict: HashMap<u8, Vec<&'static str>> = if Path::new("passgen_dict.msgpack").exists() {
        // get_dict(true, true)
        DICT_PLURALS
    } else {
        println!("Generating passgen_dict.msgpack");
        save_dict("passgen_dict.msgpack", true, true)
    };
    
    let mut dict_nop: HashMap<u8, Vec<&'static str>> = if Path::new("passgen_dict_noplurals.msgpack").exists() {
        // get_dict(false, true)
        DICT_NOPLURALS
    } else {
        println!("Generating passgen_dict_noplurals.msgpack");
        save_dict("passgen_dict_noplurals.msgpack", false, true)
    };*/
    
    // let mut dict_nop: HashMap<u8, Vec<&'static str>> = dict = get_dict(false, true);
    
    let after_dict = start.elapsed();
    
    
    /*
    if !Path::new("dict_code_all.rs").exists() {
        save_dict_code("add_dict_all", &dict, "dict_code_all.rs");
    }
    if !Path::new("dict_code_noplurals.rs").exists() {
        save_dict_code("add_dict_nop", &dict_nop, "dict_code_noplurals.rs");
    }
    */
    
    
    // println!("Dictionary length slices: \n{:?}", find_dict_code(true));
    
    
    let after_codegen = start.elapsed();
    
    
    
    // Prints wordlength information
    // wordlengths(&dict);
    let mut passlen: u8 = 8;
    let mut passcaps = false;
    let mut passnums = false;
    let mut passpunc = false;
    let mut specpunc = "".to_string();
    let mut passleet = false;
    let mut numwords: u64 = 1;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Smart password generator.  Generates passwords based on dictionary words but does not use actual words so as to avoid dictionary attacks.");
        ap.refer(&mut passlen).add_option(&["-l", "--length"], Store, "Desired password length.");
        ap.refer(&mut passcaps).add_option(&["-c", "--capitalize"], StoreTrue, "Randomly capitalize letters in the password.");
        ap.refer(&mut passnums).add_option(&["-n", "--numbers"], StoreTrue, "Randomly add random numbers.");
        ap.refer(&mut passleet).add_option(&["-e", "--leet"], StoreTrue, "Leet speak, overrides numbers option.  If punctuation option is present advanced leet is used.");
        ap.refer(&mut passpunc).add_option(&["-p", "--punctuation"], StoreTrue, "Randomly add punctuation character.s");
        ap.refer(&mut specpunc).add_option(&["-s", "--custom"], Store, "Use a special list of punctuation characters.");
        ap.refer(&mut numwords).add_option(&["-r", "--repeat"], Store, "Repeat the program a number of times to generate multiple passwords.");
        ap.parse_args_or_exit();
    }
    if passleet && passnums {
        passnums = false;
    }
    
    if numwords == 0 {
        // numwords = 1;
        return;
    }
    if numwords > 20 {
        numwords = 20;
    }
    let after_args = start.elapsed();
    
    for _ in 0..numwords {
    
        // dict len caps nums punc special
        // let pass = transform(&dict, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
        let pass = transform(&dict, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
        
        let idic = init_dict();
        println!("\nWord2() = {}", get_word2(&idic, passlen));
        println!("Password: {}", pass);
    
    }
    // println!("Testing 98_222: {:?}", 98_222);
    
    // after_dict
    // after_codegen
    // after_args
    // end
    
    let end = start.elapsed();
    let dict_time = after_dict;
    let code_time = after_codegen - dict_time;
    let args_time = after_args - code_time - dict_time;
    let gen_time = end - args_time - code_time - dict_time;
    
    // let args_time = after_args - dict_time;
    
    println!("Dictionary creation time: {}.{:08}", dict_time.as_secs(), dict_time.subsec_nanos());
    println!("Dictionary code generation time: {}.{:08}", code_time.as_secs(), code_time.subsec_nanos());
    println!("Argument parsing time: {}.{:08}", args_time.as_secs(), args_time.subsec_nanos());
    println!("Password generation time: {}.{:08}", gen_time.as_secs(), gen_time.subsec_nanos());
    println!("\nExec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
    
}

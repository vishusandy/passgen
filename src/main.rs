
// Todo: the DICT_A_LIST is for no plurals only, possibly change this?

// Todo: maybe add option for advanced leet speak and make numbers be added if specified

#[macro_use] 
extern crate log;

extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate time;

mod dict_list_np;
mod dict_helpers;
mod dict_list_all;
mod plurals_list;
mod passwords;

use argparse::{ArgumentParser, StoreTrue, Store};
// use dict_list_all::*;
// use dict_list_np::*;
use passwords::*;
use dict_helpers::*;
// use std::collections::HashMap;
use std::time::Instant;
// use std::path::Path;

fn main() {
    let start = Instant::now();
    
    // let idic2 = init_dict2(&DICT_A_LIST[..], &DICT_A_INDEXES[..]);
    let idic2 = init_dict2(false);
    // let idic2 = init_dict2(false);
    // let idic2 = init_dict2(DICT_A_INDEXES[..], DICT_A_LIST[..]);
    
    let after_dict = start.elapsed();
    
    // find_plurals("plurals.txt");
    
    // let idic = init_dict();
    
    let after_codegen = start.elapsed();
    
    let mut passlen: u8 = 8;
    let mut passcaps = false;
    let mut passnums = false;
    let mut passpunc = false;
    let mut specpunc = "".to_string();
    let mut passleet = false;
    let mut numwords: u64 = 1;
    let mut timeinfo = false;
    let mut testword = false;
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
        ap.refer(&mut timeinfo).add_option(&["-i", "--info"], StoreTrue, "Print timing information (execution time).");
        ap.refer(&mut testword).add_option(&["-t", "--testisword"], StoreTrue, "When generating a password check if the mutated word is a dictionary word.");
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
        // let pass = transform(&idic, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
        // println!("Password: {}\n-----------------", pass);
        
        let pass2 = transform2(&idic2, testword, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
        println!("Password: {}", pass2);
    }
    
    let end = start.elapsed();
    let dict_time = after_dict;
    let code_time = after_codegen - dict_time;
    let args_time = after_args - code_time - dict_time;
    let gen_time = end - args_time - code_time - dict_time;
    
    // let args_time = after_args - dict_time;
    if timeinfo {
        println!("Dictionary creation time: {}.{:08}", dict_time.as_secs(), dict_time.subsec_nanos());
        // println!("Dictionary code generation time: {}.{:08}", code_time.as_secs(), code_time.subsec_nanos());
        println!("Time to measure timing info: {}.{:08}", code_time.as_secs(), code_time.subsec_nanos());
        println!("Argument parsing time: {}.{:08}", args_time.as_secs(), args_time.subsec_nanos());
        println!("Password generation time: {}.{:08}", gen_time.as_secs(), gen_time.subsec_nanos());
        println!("\nExec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
    }
    
}

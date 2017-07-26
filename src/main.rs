
// Todo: the DICT_A_LIST is for no plurals only, possibly change this?

// Todo: if a consonant is surrounded by vowels replace it with a vowel


#[macro_use] 
extern crate log;

extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate time;

// mod dictsort;
// mod dict_code_all;
// mod dict_code_noplurals;
// mod create_dictionary;
mod dict_list_np;
mod dict_helpers;
mod dict_list_all;
// mod password;
mod passwords;
// mod dictionary;

use argparse::{ArgumentParser, StoreTrue, Store};
// use dictsort::*;
// use create_dictionary::*;
// use dict_code_all::*;
// use dict_code_noplurals::*;
// use dict_list_all::*;
// use leet::*;
use passwords::*;
use dict_helpers::*;
// use std::collections::HashMap;
use std::time::Instant;
// use std::path::Path;

fn main() {
    let start = Instant::now();
    
    let idic = init_dict();
    
    let after_dict = start.elapsed();
    
    let idic2 = init_dict2();
    
    let after_codegen = start.elapsed();
    
    let mut passlen: u8 = 8;
    let mut passcaps = false;
    let mut passnums = false;
    let mut passpunc = false;
    let mut specpunc = "".to_string();
    let mut passleet = false;
    let mut numwords: u64 = 1;
    let mut timeinfo = false;
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
        let pass = transform(&idic, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
        
        // println!("Apologetic: {}\nApoljetic: {}", passwords::is_word2(&idic, "apologetic"), passwords::is_word2(&idic, "apolojetic"));
        // println!("\nWord2() = {}", get_word2(&idic, passlen));
        println!("Password: {}\n-----------------", pass);
        
        let pass2 = transform2(&idic2, passlen, passcaps, passnums, passleet, passpunc, &specpunc);
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

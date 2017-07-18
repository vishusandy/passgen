
#[macro_use] 
extern crate log;

extern crate argparse;
extern crate rand;
extern crate rmp_serde as rmps;
extern crate serde;
extern crate time;

mod dictsort;
mod password;

use argparse::{ArgumentParser, StoreTrue, Store};
use dictsort::*;
use password::*;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    #[allow(unused_assignments)]
    let mut dict: HashMap<u8, Vec<String>> = HashMap::new();
    
    
    // TODO: save serialized file if it does not exist,
    //       if it does exist read and deserialize the file
    //         on error read the words.txt file
    
    // Retrieves serialized HashMap from the specified file
    // dict = getcfg("words-sorted.bin");
    // getdict(savefile, plurals, output)

    dict = getdict(false, false, false);
    
    // Prints wordlength information
    // wordlengths(&dict);
    let mut passlen: u8 = 8;
    let mut passcaps = false;
    let mut passnums = false;
    let mut passpunc = false;
    let mut specpunc = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Smart password generator.  Generates passwords based on dictionary words but does not use actual words so as to avoid dictionary attacks.");
        ap.refer(&mut passlen).add_option(&["-l", "--length"], Store, "Desired password length");
        ap.refer(&mut passcaps).add_option(&["-c", "--capitalize"], StoreTrue, "Randomly capitalize letters in the password");
        ap.refer(&mut passnums).add_option(&["-n", "--numbers"], StoreTrue, "Randomly add randdom numbers");
        ap.refer(&mut passpunc).add_option(&["-p", "--punctuation"], StoreTrue, "Randomly add punctuation characters");
        ap.refer(&mut specpunc).add_option(&["-s", "--custom"], Store, "Use a special list of punctuation characters.");
        ap.parse_args_or_exit();
    }
    // ap.refer(&mut pass).add_option(&["-", "--"], StoreTrue, "");
    
    // dict len caps nums punc special
    let pass = transform(&dict, passlen, passcaps, passnums, passpunc, &specpunc);
    println!("Password: {}", pass);
    
    let end = start.elapsed();
    println!("Exec time: {}.{:08}", end.as_secs(), end.subsec_nanos());
}

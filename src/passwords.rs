//  Todo: cleanup warnings
    // make a substr function and have functions use that instead
    //   of iterating through the word
    // also make a function that either returns the letter at a 
    //   given position or modifies a letter at a given position
    //   by using a closure to determine the transformation

// use leet::*;
use dict_helpers::*;
// use dict_list_all::*;
// use dict_list_np::*;
use rand::{thread_rng, Rng};
// use rand::distributions::range::SampleRange;
use std::collections::HashMap;


/*
    len     u8      total length of password
    caps    bool    use capital letters
    nums    bool    use numbers
    leet    bool    use leet speak
    punc    bool    use punctuation
    special &str    use special set of punctuation
*/

#[allow(dead_code)]
pub fn transform2(dict: &HashMap<u8, &'static [&'static str]>, testword: bool, length: u8, caps: bool, nums: bool, leet: bool, punc: bool, special: &str) -> String {
    // leet_speak numbers punctuation 
    // let words = Vec::<String>::new();
    let minword = 2;
    let mut len: u8 = length;
    
    if nums && punc && len < minword+2 {
        len = minword+2;
    } else if nums && len < minword+1 {
        len = minword+1;
    } else if punc && len < minword+1 {
        len = minword+1;
    } else if len < minword {
        len = minword;
    }
    
    let numlen: u8;
    let punclen: u8;
    
    // choose either numbers or punctuation to allocate first to avoid bias
    //   then allocate a number of characters for each if applicable

    if safe_range(0, 1) == 0 {
        numlen = if nums && (!leet || punc) { rand_length(len, minword) } else { 0 };
        punclen = if punc { rand_length(len-numlen, minword) } else { 0 };
    } else {
        punclen = if punc { rand_length(len, minword) } else { 0 };
        numlen = if nums && (!leet || punc) { rand_length(len-punclen, minword) } else { 0 };
    }
    
    // allocate remaining characters to the word
    len = len-punclen-numlen;
    
    if leet {
        let step0 = get_word2(dict, len);
        let rst =  leet_speak(&step0, punc);
        match rst {
            Some(w) => {
                let step2 = match caps {
                    true => capitalize(&w),
                    false => w,
                };
                let step3 = match punclen > 0 {
                    true if nums && numlen > 0 => 
                        add_punctuation(&add_numbers(&step2, numlen), punclen, special), //add numbers to ensure at least one number
                    true => 
                        add_punctuation(&step2, punclen, special), // add just punctuation to ensure a punctuation character
                    false => step2, // nothing to add
                };
                step3
            },
            _ => transform2(dict, testword, length, caps, nums, false, punc, special),
        }
    } else {
        let mut checkword: bool = testword;
        if nums || punc {
            checkword = false;
        }
        let step0 = get_word2(dict, len);
        let mut step1 = mutate_word(step0);
        if checkword {
            // change middle parameter to true to check plurals
            while is_word2(dict, false, &step1) {
                step1 = mutate_word(get_word2(dict, len));
            }
        }
        let step2 = match caps {
            true => capitalize(&step1),
            false => step1,
        };
        let step3 = if punc && punclen > 0 { 
            add_punctuation(&step2, punclen, special) 
        } else { 
            step2 
        };
        let step4 = if nums && numlen > 0 {
            add_numbers(&step3, numlen)
        } else {
            step3
        };
        step4
    }
}

#[allow(dead_code)]
pub fn transform(dict: &HashMap<u8, (usize, usize)>, testword: bool, length: u8, caps: bool, nums: bool, leet: bool, punc: bool, special: &str) -> String {
    // leet_speak numbers punctuation 
    // let words = Vec::<String>::new();
    let minword = 2;
    let mut len: u8 = length;
    
    if nums && punc && len < minword+2 {
        len = minword+2;
    } else if nums && len < minword+1 {
        len = minword+1;
    } else if punc && len < minword+1 {
        len = minword+1;
    } else if len < minword {
        len = minword;
    }
    
    let numlen: u8;
    let punclen: u8;
    
    // choose either numbers or punctuation to allocate first to avoid bias
    //   then allocate a number of characters for each if applicable

    if safe_range(0, 1) == 0 {
        numlen = if nums && (!leet || punc) { rand_length(len, minword) } else { 0 };
        punclen = if punc { rand_length(len-numlen, minword) } else { 0 };
    } else {
        punclen = if punc { rand_length(len, minword) } else { 0 };
        numlen = if nums && (!leet || punc) { rand_length(len-punclen, minword) } else { 0 };
    }
    
    // allocate remaining characters to the word
    len = len-punclen-numlen;
    
    if leet {
        let step0 = get_word(dict, len);
        let rst =  leet_speak(&step0, punc);
        match rst {
            Some(w) => {
                let step2 = match caps {
                    true => capitalize(&w),
                    false => w,
                };
                let step3 = match punclen > 0 {
                    true if nums && numlen > 0 => 
                        add_punctuation(&add_numbers(&step2, numlen), punclen, special), //add numbers to ensure at least one number
                    true => 
                        add_punctuation(&step2, punclen, special), // add just punctuation to ensure a punctuation character
                    false => step2, // nothing to add
                };
                step3
            },
            _ => transform(dict, testword, length, caps, nums, false, punc, special),
        }
    } else {
        let mut checkword: bool = testword;
        if nums || punc {
            checkword = false;
        }
        let step0 = get_word(dict, len);
        let mut step1 = mutate_word(step0);
        if checkword {
            while is_word(dict, &step1) {
                step1 = mutate_word(get_word(dict, len));
            }
        }
        let step2 = match caps {
            true => capitalize(&step1),
            false => step1,
        };
        let step3 = if punc && punclen > 0 { 
            add_punctuation(&step2, punclen, special) 
        } else { 
            step2 
        };
        let step4 = if nums && numlen > 0 {
            add_numbers(&step3, numlen)
        } else {
            step3
        };
        step4
    }
}

fn mutate_word(word: &str) -> String {
    let mut rg = thread_rng();
    
    let mut new = String::with_capacity(word.len());
    let len = word.len();
    
    let max = match len {
        a if a > 16 => 3,
        a if a > 5 && a < 16 => 2,
        _ => 1,
    };
    
    let num = if max != 1 { safe_range(1, max) } else { 1 };
    let mut ns: [usize; 50] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49];
    
    
    let mut letters = Vec::new();
    for i in 0..word.len() {
        letters.push(i);
    }
    rg.shuffle(&mut letters);
    let mut changes = Vec::new();
    for i in 0..num {
        changes.push(letters[i]);
    }
    
    // let mut changes = letters[0..num];
    
    // let mut letters = &mut ns[0..word.len()];
    // rg.shuffle(&mut letters);
    // let changes = &ns[0..num];
    
    // let changes = &letters[0..num+1];
    
/*    let mut changes = Vec::new();
    for i in 0..num {
        changes.push(letters[i]);
    }*/
    
    debug!("{} mutation points: {:?} with a max of {} and num of {}", word, changes, max, num);
    
    for i in 0..word.len() {
        let letter = &word[i..i+1];
        
        let is_basic_vowel = |a: &str| -> bool {
            match a {
                "a" | "e" | "i" | "o" | "u" => true,
                _ => false,
            }
        };
        
        let is_vowel = |a: &str| -> bool {
            match a {
                "a" | "e" | "i" | "o" | "u" | "y" => true,
                _ => false,
            }
        };
        // a y is a vowel if it is at the end of the word or surrounded by consonants
        // if a consonant is surrounded by other consonants use change_vowel()
        if changes.contains(&i) {
            match letter {
                
                // treat y as a vowel if at end of word or surrounded by consonants
                "y" if 
                        
                        i == word.len()-1 // at end of a word
                        || ( // or surrounded by consonants
                            (i == 0 || (i > 0 && !is_basic_vowel(&word[i-1..i]))) // beginning of word or previous letter is a consonant
                            && ( (i < word.len()-1 && !is_basic_vowel(&word[i+1..i+2])) /* || i == word.len()-1 */ ) // next letter is a consonant, do not check if end of word because that was already checked
                           )
                     => new.push_str(&change_vowel(&letter)),
                
                // a basic vowel
                "a" | "e" | "i" | "o" | "u" => new.push_str(change_vowel(&letter)),
                
                // change consonant to vowel if surrounded by consonants
                _ if ((i > 0 && !is_vowel(&word[i-1..i])) // if not beginning of word and previous letter is consonant
                      && !is_vowel(&word[i..i+1]) // not necessary in most situations
                      && (i < word.len()-1 && !is_vowel(&word[i+1..i+2])) // not at end of word and next letter is consonant
                  ) => new.push_str(change_vowel(&letter)),
                 
                
                // a normal consonant
                _ => new.push_str(change_consonant(&letter)),
            }
        } else {
            new.push_str(letter);
        }
    }
    println!("Dictionary word: {}\tMutated word: {}", word, new);
    new
}

fn change_vowel(letter: &str) -> &str {
    let mut rg = thread_rng();
    // aeiouy
    let vowels = vec!["a", "e", "i", "o", "u", "y"];
    let mut choice = match rg.choose(&vowels){
        Some(a) => a,
        None => "~",
    };
    while choice == letter {
        choice = match rg.choose(&vowels) {
            Some(a) => a,
            None => "",
        };
    }
    debug!("\tchange_vowel(letter={}) -> {}", letter, choice);
    choice
}

fn change_consonant(letter: &str) -> &str {
    let mut rg = thread_rng();
    // bcdfghjklmnpqrstvwxz
    let cons = vec!["b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x", "z"];
    let mut choice = match rg.choose(&cons){
        Some(a) => a,
        None => "'",
    };
    while choice == letter {
        choice = match rg.choose(&cons) {
            Some(a) => a,
            None => "'",
        };
    }
    debug!("\tchange_consonant(letter={}) -> {}", letter, choice);
    choice
}

fn capitalize(word: &str) -> String {
    let num = safe_range(1, word.len()-1) as u8;
    
    // figure out which letters to capitalize
    let mut v = Vec::new();
    for _ in 0..num {
        let mut c = safe_range(0, word.len()/2);
        while v.contains(&c) {
            c = safe_range(0, word.len()-1);
        }
        v.push(c);
    }
    v.sort();
    
    let mut new = String::with_capacity(word.len());
    
    for l in 0..word.len() {
        if v.contains(&l) {
            new.push_str(&word[l..l+1].to_uppercase());
        } else {
            new.push_str(&word[l..l+1]);
        }
    }
    debug!("capitalize():\n\tOriginal = {}\n\tCapitalized = {}", word, new);
    new
    
}

fn leet_speak(word: &str, adv: bool) -> Option<String > {
    // iterate through word, adding positions 
    // of leet letters to a vector

    let mut idxs = Vec::new();
    for i in 0..word.len() {
        match &word[i..i+1] {
            "a" | "e" | "i" | "o" | "s" | "t" | "z" => idxs.push(i),
            _ => {}
        }
    }
    // shuffle vector
    let mut rg = thread_rng();
    rg.shuffle(&mut idxs);
    
    if idxs.len() > 0 {
        let num = safe_range(1, idxs.len()/2);
        let mut leets = Vec::new();
        // and then take a random number of leet idxs
        for i in 0..num {
            leets.push(idxs[i]);
        }
        
        let mut new = String::with_capacity(word.len());
        
        // iterate through the word
        // check if each letter needs to be transformed
        // then add original or transformed letter to output
        for i in 0..word.len() {
            if leets.contains(&i) {
                let c = match &word[i..i+1] {
                    "a" if adv => "@",
                    "a" => "4",
                    "e" => "3",
                    "i" if adv => "!",
                    "i" => "1",
                    "o" => "0",
                    "s" if adv => "$",
                    "s" => "5",
                    "t" if adv => "+",
                    "t" => "7",
                    "z" => "2",
                    e => e,
                };
                new.push_str(c);
            } else {
                new.push_str(&word[i..i+1]);
            }
        }
        println!("Dictionary word: {}\tLeet speak word: {}", word, new);
        Some(new)
    } else {
        None
    }
}

// random_insert() is used to insert a character/string at a
// random position within the string and return a new string
fn random_insert(word: &str, character: &str) -> String {
    let loc = safe_range(0, word.len());
    let mut new = String::with_capacity(word.len()+1);
    
    // 0123456789
    // parachute
    // rand=3
    // para0chute
    
    if loc == 0 {
        new.push_str(character);
        new.push_str(word);
    } else if loc == word.len() {
        new.push_str(word);
        new.push_str(character);
    } else {
        // 01234
        // test
        // character=9
        // loc=rand(0,4) = 
        /*  0 9test
            1 t9est [0..loc(1)] + 9 + [loc(1)..]
            2 te9st [0..loc(2)] + 9 + [loc(2)..]
            3 tes9t [0..loc(3)] + 9 + [loc(3)..]
            4 test9 
        */
        
        // let b = &word[..loc];
        // let e = &word[loc..];
        // new.push_str(b);
        new.push_str(&word[..loc]);
        new.push_str(character);
        // new.push_str(e);
        new.push_str(&word[loc..]);
    }
    new
}

fn add_numbers(word: &str, num: u8) -> String {
     #[allow(unused_assignments)]
    let mut new = String::with_capacity(word.len()+(num as usize));
    new = word.to_string();
    for _ in 0..num {
        new = random_insert(&new, &(safe_range(0, 9).to_string()));
    }
    new
}

fn add_punctuation(word: &str, num: u8, special: &str) -> String {
    #[allow(unused_assignments)]
    let mut new = String::with_capacity(word.len()+(num as usize));
    let pchars = if special == "" { ",.?-/+*=_@#$%^&()" } else { special };
    new = word.to_string();
    for _ in 0..num {
        // 012345
        // insert
        // 0 i [0..1]
        // ...
        // 5 t [5..6]
        let ins = safe_range(0, pchars.len()-1);
        new = random_insert(&new, &pchars[ins..ins+1])
        // new = random_insert(&new, &(safe_range(0, 9).to_string()));
    }
    new
}


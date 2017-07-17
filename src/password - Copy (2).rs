use rand::{thread_rng, Rng};
use sconcat::Cat;
use std::collections::HashMap;
    
pub fn get_word(dict: &HashMap<u8, Vec<String>>, len: u8) -> &str {
    let mut rg = thread_rng();
    // let mut word: &str;
    // let rand = rg.gen_range(0, 9);
    match dict.get(&len) {
        Some(v) => {
            &v[rg.gen_range(0, v.len()-1)]
        },
        None => {
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
            if len-lower >= upper-len {
                closest = lower;
            } else {
                closest = upper;
            }
            match dict.get(&closest) {
                Some(v) => {
                    &v[rg.gen_range(0, v.len()-1)]
                },
                None => {
                    "error"
                },
            }
        },
    }
}
                    
/*
    len     u8      total length of password
    caps    bool    use capital letters
    nums    bool    use numbers
    // minnums u8      min number of numbers
    punc    bool    use punctuation
    // minpunc u8      min number of punctuation characters
    special &str    use special set of punctuation
*/

pub fn transform(dict: &HashMap<u8, Vec<String>>, length: u8, caps: bool, nums: bool, punc: bool, special: &str) {
// pub fn transform(dict: &HashMap<u8, Vec<String>>, length: u8, caps: bool, nums: bool, minnums: u8, punc: bool, minpunc: u8, special: &str) {
    // get a random number of characters to use for numbers and add_punctuation
    // check to make sure a word of certain length exists, if not pad with 
    // num/punc if applicable, otherwise just return the closest match
    let mut len: u8 = length;
    
    // Set a minimum length depending on input arguments
    if nums && punc && len < 4 {
        len = 4;
    } else if nums && len < 3 {
        len = 3;
    } else if punc && len < 3 {
        len = 3;
    } else if len < 2 {
        len = 2;
    }
    
    // get a word of length: len-(num+punc)
    
    let mut minword = 2;
    let mut maxword = 23;
    
    let mut rg = thread_rng();
    
    // Figure out how many characters to use for nums, punc, and a word
    let numlen: u8 = match nums {
        true if len-minword-1 > 1 => rg.gen_range(1, len-minword-1),
        true => 1,
        false => 0,
    };
    let punclen: u8 = match punc {
        true if len-minword-numlen > 1 => rg.gen_range(1, len-minword-numlen),
        true => 1,
        false => 0,
    };
    len = len-numlen-punclen;
    
    // Get a word and start mutating
    let mut word: String = mutate_word(get_word(dict, len));
    word = capitalize(&word);
    if nums {
        word = add_numbers(&word, numlen);
    }
    if punc {
        word = add_punctuation(&word, punclen, special);
    }
    // if caps is true capitalize at least 1 letter
    
}

fn mutate_word(word: &str) -> String {
    let len = word.len();
    let mut rg = thread_rng();
    // let rand = rg.gen_range(0, 9);
    // rg.shuffle(&mut an_array[..]);
    
    
    
    String::new()
}

fn change_vowel(letter: &str) -> String {
    let mut rg = thread_rng();
    
    
    String::new()
}
fn change_consonant(letter: &str) -> String {
    let mut rg = thread_rng();
    
    
    String::new()
}

/*
fn cap_letter(word: &str) -> String {
    let mut rg = thread_rng();
    let w = word.as_bytes();
    let choice = rg.choose(w);
    match choice {
        Some(c) => {
            let first = w[0..*c];
            let last = w[*c+1..];
            
            String::new()
        },
        None => {
            String::new()
        },
    }
    // let new = Cat + first + choice + last;
}
*/

fn capitalize2(word: &str) -> String {
    let mut rg = thread_rng();
    let num = rg.gen_range(1, word.len()-1) as u8;
    
    // figure out which letters to capitalize
    let mut v = Vec::new();
    for _ in 0..num {
        let mut c = rg.gen_range(0, word.len()-1);
        while v.contains(&c) {
            c = rg.gen_range(0, word.len()-1);
        }
        v.push(c);
    }
    v.sort();
    
    let new = String::with_capacity(word.len());
    let loc = 0;
    
    for l in 0..word.len() {
        if v.contains(&l) {
            new.push_str(word[l].to_uppercase());
        } else {
            new.push_str(word[l]);
        }
    }
    new
    
}

/*
fn capitalize(word: &str) -> String {
    let mut rg = thread_rng();
    let mut new = String::from(word);
    let num = rg.gen_range(1, word.len()-1) as u8;
    for n in 0..num {
        new = cap_letter(&new);
    }
    String::new()
}
*/
/*
fn capitalize(w: &str) -> String {
    let mut rg = thread_rng();
    let mut word = w.to_string();
    // ensure at least one letter is capitalized 
    // and at least one letter is left uncapitalized
    let num = rg.gen_range(1, word.len()-1) as u8;
    let mut places = [0..w.len()-1];
    rg.shuffle(&mut places);
    let mut loc = 0;
    let mut p = 0;
    while p <= num {
        // table
        // 35142
        // num=2
        let take = places[p]
        
        places += 1;
        loc += take;
    }
    
    for _ in 0u8..num {
        
    }
    word
    
}
*/

fn replace_at(word: &str, loc: usize) -> String {
    String::new()
}

fn add_numbers(word: &str, num: u8) -> String {
    
    String::new()
}

fn add_punctuation(word: &str, num: u8, special: &str) -> String {
    
    String::new()
}

/*
fn capitalize_at(word: &str, loc: usize) -> String {
    // let mut beginning = if loc == 0 {
    let mut beginning = word[0..loc+1].to_string();
    let chosen = beginning.pop().to_uppercase();
    let ending = if loc == word.len()-1 {
        word[loc+1..]
    } else {
        ""
    };
    let new = String::with_capacity(word.len()-1);
    new.push_str(&beginning);
    new.push(chosen);
    new.push_str(ending);
    new
}
*/

/*
fn add_numbers(word: &str, num: u8) -> String {
    let mut rg = thread_rng();
    static NUMBERLIST: &'static str = "0123456789";
    
    let mut new = String::new();
    for _ in 0u8..num {
        new.insert(rg.gen_range(0, NUMBERLIST.len()), rg.choose(NUMBERLIST.chars()));
        // new.insert_str(rg.gen_range(0, word.len()), NUMBERLIST[rg.gen_range(0, NUMBERLIST.len())]);
        // new.insert(rg.gen_range(0, word.len()), NUMBERLIST.chars().nth(rg.gen_range(0, NUMBERLIST.len())).unwrap());
    }
    new
}
*/

/*
fn add_punctuation(word: &str, num: u8, special: &str) -> String {
    let mut rg = thread_rng();
    let mut new = String::new();
    let punclist = match special {
        "" => ",.?-/+*=_@#$%^&()",
        l => l,
    };

    for _ in 0u8..num {
        new.insert(rg.gen_range(0, punclist.len()), rg.choose(punclist.chars()));
        // new.insert_str(rg.gen_range(0, word.len()), punclist[rg.gen_range(0, punclist.len()-1)]);
        // new.insert(rg.gen_range(0, word.len()), punclist.chars().nth(rg.gen_range(0, punclist.len()-1)).unwrap());
    }
    new
}
*/

/*
fn capitalize_random(w: &str) -> String {
    //      |
    // 0123456789 len=10
    // itsastring
    
    let mut word = String::new();
    let mut rg = thread_rng();
    let loc = rg.gen_range(0, word.len()-1);
    // let first = w[0..loc];
    // let letter = w[loc..loc+1].to_uppercase();
    // let last = ww[0..loc];
    
    let chosen = w[loc..loc+1].to_string();
    let letter = chosen.pop();
    word.push_str(w[0..loc]);
    if letter.is_uppercase() {
        return capitalize_random();
    } else {
        let s = letter.to_uppercase().to_string();
        word.push_str(&s);
    }
    // word.push_str(&letter);
    word.push_str(w[loc+1..]);
    
    // word.push_str(w[0..loc]);
    // word.push_str(&letter);
    // word.push_str(w[loc+1..]);
    
    // let (first, last) = word.split_at_mut(loc);
    
    word
}
*/


// No longer needed
fn random_insert(word: &str, character: &str) -> String {
    let mut rg = thread_rng();
    let loc = rg.gen_range(0, word.len());
    let (first, last) = word.split_at(loc);
    let mut new = String::with_capacity(word.len()+character.len());
    new.push_str(first);
    new.push_str(character);
    new.push_str(last);
    new.to_string()
}


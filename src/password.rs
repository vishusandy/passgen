use rand::{thread_rng, Rng};
use rand::distributions::range::SampleRange;
use std::collections::HashMap;

fn safe_range<T: PartialOrd + SampleRange>(start: T, end: T) -> T {
    if start < end {
        let mut rg = thread_rng();
        rg.gen_range(start, end)
    } else {
        start
    }    
}

pub fn get_word(dict: &HashMap<u8, Vec<String>>, len: u8) -> &str {
    match dict.get(&len) {
        Some(v) => {
            &v[safe_range(0, v.len()-1)]
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
            if len-lower >= upper-len {
                closest = lower;
            } else {
                closest = upper;
            }
            match dict.get(&closest) {
                Some(v) => {
                    let out = &v[safe_range(0, v.len()-1)];
                    debug!("get_word(len={}) = {}", len, &v[safe_range(0, v.len()-1)]);
                    out
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

pub fn transform(dict: &HashMap<u8, Vec<String>>, length: u8, caps: bool, nums: bool, punc: bool, special: &str) -> String {
    // get a random number of characters to use for numbers and add_punctuation
    // check to make sure a word of certain length exists, otherwise get closest length match

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
    
    let minword = 2;
    // maxword not really needed because get_word() automatically gets nearest length
    // although if there are no words of the length specified by the argument passed
    // to get_word() then the output password will not be the exact desired length 
    // let maxword = 23;
    
    
    // Figure out how many characters to use for nums, punc, and a word
    
    let numlen: u8 = match nums {
        true if len-minword-1 > 1 => safe_range(1, len-minword-1),
        true => 1,
        false => 0,
    };
    
    let punclen: u8 = match punc {
        true if len-minword-numlen > 1 => safe_range(1, len-minword-numlen),
        true => 1,
        false => 0,
    };
    len = len-numlen-punclen;
    
    debug!("transform()\nlen={}\nnumlen={}\npunclen={}", len, numlen, punclen);
    
    // Get a word and start mutating
    let mut word_mutated = mutate_word(get_word(dict, len));
    
    // use the following line instead of the above to test if the 
    // mutated word exists in the dictionary and do a retry
    
    // let mut word_mutated = get_word(dict, len).to_string();
    
    if let Some(v) = dict.get(&(word_mutated.len() as u8)) {
        info!("Mutated word `{}` exists in dictionary, retrying", word_mutated);
        while v.contains(&word_mutated) {
            word_mutated = mutate_word(get_word(dict, len));
            info!("Retry result: {}", word_mutated);
        }
    }
    
    debug!("Mutated word: {}", &word_mutated);
    
    let word_capitalized: String = if caps{
        capitalize(&word_mutated)
    } else {
        word_mutated
    };
    
    debug!("Capitalized word: {}", word_capitalized);
    
    let output = match (nums, punc) {
        (true, true) => add_numbers(&add_punctuation(&word_capitalized, punclen, special), numlen),
        (true, false) => add_numbers(&word_capitalized, numlen),
        (false, false) => word_capitalized,
        (false, true) => add_punctuation(&word_capitalized, punclen, special),
    };
    // info!("Password: {}", output);
    output
    
}

fn mutate_word(word: &str) -> String {
    let mut rg = thread_rng();
    
    let mut new = String::with_capacity(word.len());
    let len = word.len();
    
    let max = match len {
        a if a > 16 => 3,
        a if a > 5 && a < 15 => 2,
        _ => 1,
    };
    
    let num = safe_range(1, max);
    
    let mut letters = Vec::new();
    for i in 0..word.len() {
        letters.push(i);
    }
    
    rg.shuffle(&mut letters);
    
    let changes = &letters[0..num];
    for i in 0..word.len() {
        let letter = &word[i..i+1];
        
        if changes.contains(&i) {
            match letter {
                "a" | "e" | "i" | "o" | "u" | "y" => {
                    new.push_str(&change_vowel(&letter));
                },
                _ => {
                    new.push_str(&change_consonant(&letter));
                },
            }
        } else {
            new.push_str(letter);
        }
    }
    // debug!("mutate_word():\n\tOriginal = {}\n\tMutated word = {}", word, new);
    println!("Dictionary word: {}\nMutated word: {}", word, new);
    new
}

fn change_vowel(letter: &str) -> String {
    let mut rg = thread_rng();
    // aeiouy
    let vowels = vec!["a", "e", "i", "o", "u", "y"];
    let mut choice = match rg.choose(&vowels){
        Some(a) => a,
        None => "'",
    };
    while choice == letter {
        choice = match rg.choose(&vowels) {
            Some(a) => a,
            None => "'",
        };
    }
    debug!("\tchange_vowel(letter={}) -> {}", letter, choice.to_string());
    choice.to_string()
}

fn change_consonant(letter: &str) -> String {
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
    debug!("\tchange_consonant(letter={}) -> {}", letter, choice.to_string());
    choice.to_string()
}

fn capitalize(word: &str) -> String {
    let num = safe_range(1, word.len()-1) as u8;
    
    // figure out which letters to capitalize
    let mut v = Vec::new();
    for _ in 0..num {
        
        // let mut c = safe_range(0, word.len()-1);
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

fn add_numbers(word: &str, num: u8) -> String {
    let mut rg = thread_rng();
    let mut p = Vec::new();
    for i in 0..word.len()+1 {
        p.push(i);
    }
    rg.shuffle(&mut p);
    let mut places = Vec::new();
    for i in 0..num {
        places.push(p[i as usize]);
    }
    
    let mut new = String::with_capacity(word.len()+(num as usize));
    let mut widx = 0usize;
    for i in 0..word.len()+(num as usize) {
        if places.contains(&i) {
            let rand = safe_range(0,9);
            new.push_str(&rand.to_string());
        } else {
            new.push_str(&word[widx..widx+1]);
            widx += 1;
        }
    }
    new
}

fn add_punctuation(word: &str, num: u8, special: &str) -> String {
    let mut rg = thread_rng();
    let mut p = Vec::new();
    for i in 0..word.len()+1 {
        p.push(i);
    }
    rg.shuffle(&mut p);
    let mut places = Vec::new();
    for i in 0..num {
        places.push(p[i as usize]);
    }
    
    let mut new = String::with_capacity(word.len()+(num as usize));
    let mut widx = 0usize;
    let pchars = ",.?-/+*=_@#$%^&()";
    debug!("Generating punctuated word");
    for i in 0..word.len()+(num as usize) {
        if places.contains(&i) {
            let rand: usize;
            if special == "" {
                rand = safe_range(0, pchars.len()-1);
                new.push_str(&pchars[rand..rand+1]);
            } else {
                rand = safe_range(0, special.len()-1);
                new.push_str(&special[rand..rand+1]);
            }
        } else {
            new.push_str(&word[widx..widx+1]);
            widx += 1;
        }
    }
    debug!("Punctuated word is {}", new);
    new
}

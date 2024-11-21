//! mcw is rust crate that stands for: "most common word"
//! mcw does what it says -> returns most common words 
//! 
//! # Using the Crate
//! ```rust,ignore
//! use mcw::*;
//! fn main(){
//!      let words : Vec<Word> = get_words("These are words. Lorem Ipsum.", false).expect("error");
//!      for word in words{
//!          println!("{}", word);
//!      }
//! }
//! ```
//! ***Additional Note*** : argument of type bool contains whether the argument of type String is
//! file directory or not. Error only may occur if bool is true.
//! 
//! If your project ships with `tokio` in dependencies, then you can use async version of mcw, using
//! `async` feature in specifications. Only change is that program uses `tokio::fs` instead of
//! `std::fs`
//!
//! **/Cargo.toml**
//! ```ignore
//! [package]
//!
//! name = "your_package"
//! version = "0.1.0"
//! edition = "2021"
//! [dependencies]
//! mcw = {version = "1.3.0", features = ["async"]}
//! ```
//! **/src/main.rs**
//! ```rust,ignore
//! #[tokio::main]
//! async fn main(){
//!      let words : Vec<Word> = async_get_words("These are words. Lorem Ipsum. Async Version of
//!      function", false).await.expect("error");
//!
//!      for word in words{
//!         println!("{}", word);
//!      }
//! }
//! ```
//! ---

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone)]
pub struct Word{
    pub value : String,
    pub amount : u64
}

impl std::fmt::Display for Word{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[Word: {}; Amount: {}]", self.value, self.amount)
    }
}

#[cfg(not(feature = "cli"))]
#[cfg(not(feature = "async"))]
pub fn get_words(text: &str, is_file: bool) -> Result<Vec<Word>, Box<dyn std::error::Error>>{
    let input_text : Vec<String> = if is_file == false{
        text.to_string().split_whitespace().map(|s| s.to_string()).collect()
    }
    else{
        std::fs::read_to_string(text)?.split_whitespace().map(|s| s.to_string()).collect()
    };
    let mut processed_text_as_chars : Vec<Vec<char>> = vec![];
    for string in input_text{
        let mut chars_in : Vec<char> = vec![];
        for c in &string.chars().collect::<Vec<char>>(){
            if *c != '?' && *c != '!' && *c != '.' && *c != ',' && *c != ';' && *c != ':'{
                chars_in.push(*c);
            }
            else{
                continue;
            }
        }
        processed_text_as_chars.push(chars_in);
    }
    let mut processed_text : Vec<String> = vec![];
    for chrs in processed_text_as_chars{
        processed_text.push(String::from_iter(chrs.iter()));
    }
    let mut word_array : Vec<Word> = vec![];
    let mut hashmap_with_words : std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for text in processed_text{
        for string in text.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>(){
            if hashmap_with_words.contains_key(&string){
                let value = hashmap_with_words.get(&string).unwrap();
                hashmap_with_words.insert(string.to_lowercase(), value +1);
            }
            else{
                hashmap_with_words.insert(string.to_lowercase(), 1);
            }
        }
    }

    for key in hashmap_with_words.keys(){
        let value = hashmap_with_words.get(key).unwrap();
        word_array.push(Word{value: key.to_string(), amount: *value});
    }

    Ok(word_array)
}

#[cfg(not(feature = "cli"))]
#[cfg(feature = "async")]
async fn async_get_words(text: &str, is_file: bool) -> Result<Vec<Word>, Box<dyn std::error::Error>>{    
    let input_text : Vec<String> = if is_file == false{
        text.to_string().split_whitespace().map(|s| s.to_string()).collect()
    }
    else{
    tokio::fs::read_to_string(text).await?.split_whitespace().map(|s| s.to_string()).collect()
    };
    let mut processed_text_as_chars : Vec<char> = vec![];
    for string in input_text{
        let input_text_as_chars : Vec<char> = string.chars().collect();
        for c in &input_text_as_chars{
            if *c != '?' && *c != '!' && *c != '.' && *c != ',' && *c != ';' && *c != ':'{
                processed_text_as_chars.push(*c);
            }
            else{
                continue;
            }
        }
    }
    let processed_text = String::from_iter(processed_text_as_chars.iter()).to_lowercase();
    let mut word_array : Vec<Word> = vec![];
    let mut hashmap_with_words : std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for string in processed_text.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>(){
        if hashmap_with_words.contains_key(&string){
            let value = hashmap_with_words.get(&string).unwrap();
            hashmap_with_words.insert(string, value +1);
        }
        else{
            hashmap_with_words.insert(string, 1);
        }
    }

    for key in hashmap_with_words.keys(){
        let value = hashmap_with_words.get(key).unwrap();
        word_array.push(Word{value: key.to_string(), amount: *value});
    }

    Ok(word_array)
}

impl Word{
    pub fn filter_with_range(range: Range, words: Vec<Word>) -> Vec<Word>{
        let mut processed_words : Vec<Word> = vec![];
        for word in words{
            if Range::in_range(range, word.amount){
                processed_words.push(word);
            } 
        }
        processed_words
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Range{
    pub min: u64,
    pub max: u64
}

impl Range{
    pub fn in_range(range: Range, value: u64) -> bool{
        return value >= range.min && value <= range.max;
    }
}

/// Returns string from `Range`
impl ToString for Range{
    fn to_string(&self) -> String {
        format!("{}.{}", self.min, self.max)
    }
}

/// To create `Range` struct from String, use `Range::from(string)` 
/// `Min` is separated from `Max` by comma (',') or dot ('.').
/// Example: "0.10" -> parses to Range with `Min` equal 0 and `Max` equal 10
/// Example: "10,20" -> parses to Range with `Min` equal 10 and `Max` equal 20
impl From<String> for Range{
    fn from(value: String) -> Self {
        let mut number_1_string : Vec<char> = vec![];
        let mut number_2_string : Vec<char> = vec![];
        let mut is_num1 : bool = true; // true -> num1 false -> num2
        for c in value.chars().collect::<Vec<char>>(){
               if is_num1 && c != '.' && c != ','{
                   number_1_string.push(c);
               } 
               else if !is_num1 && c != '.' && c != ','{
                   number_2_string.push(c);
               }
               else{
                   is_num1 = false;
               }
        }
        let number_1 = match String::from_iter(number_1_string.iter()).trim().parse(){
            Ok(v) => v,
            Err(e) =>{
                eprintln!("{}", e);
                0
            }
        };
        let number_2 = match String::from_iter(number_2_string.iter()).trim().parse(){
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                10
            }
        };
        Range{min: number_1, max: number_2}
    }
}

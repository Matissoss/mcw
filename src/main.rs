use std::collections::HashMap;
use tokio;
use colored::Colorize;

#[derive(PartialEq,Eq, PartialOrd, Ord)]
struct Word{
    value : String,
    amount : u64
}

impl std::fmt::Display for Word{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "===>\"{}\"<=== \n was repeated {} times", 
            self.value.bold().yellow(), 
            self.amount.to_string().bold().italic().cyan())
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1{
        println!("Not enough arguments: use --help")
    }
    else if args.len() == 2 && args[1] == "--help" {
        println!("
        mcw -> most common word 

        --help
        
        -formula-
        *executable **file.format ***number
        ---------
        *executable -> execute program

        **file.format -> file that program should read 

        ***number -> how much times does word needs to be repeated to be mentioned
        
        -Example-
        ./mcw text.txt 10
        means that program will read from file named `text.txt` 
        and a word needs to be repeated 10 or more times to be mentioned
        ");
    } else if args.len() >= 2 {
        let file_name = args[1].clone();

        let file_contents = match std::fs::read_to_string(file_name) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                std::process::abort();
            }
        };

        let array = process_text(&file_contents).await;
        let mut words : Vec<Word> = vec![];
        let minimum_word : u64 = if args.len() >= 3{
            args[2].parse().expect("error parsing additional number")
        }
        else{
            1
        };
        for key in array.keys() {
            let word = array.get(key).unwrap();
            if *word >= minimum_word{
                words.push(Word{
                    value: key.to_string(),
                    amount : *word
                });
            }
        }
        words.sort_by(|a,b| a.amount.cmp(&b.amount));
        for w in words{
            println!("{}", w);
        }
    } else {
        println!("Not enough args");
    }
}

async fn process_text(file_contents: &str) -> HashMap<String, u64> {
    let words_as_vec: Vec<String> = file_contents
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut word_array_without_special : Vec<String> = vec![];
    for word in &words_as_vec {
        let word_as_chars: Vec<char> = word.chars().collect();
        let mut chars : Vec<char> = vec![];
        for c in word_as_chars {
            if c != '?' && c != ',' && c != '.' && c != '!'{
                chars.push(c);
            }
        }
            word_array_without_special.push(String::from_iter(&chars));
    }

    let words_formatted : Vec<String> = word_array_without_special
        .iter()
        .map(|s| s.to_lowercase())
        .collect();
    let mut words: HashMap<String, u64> = HashMap::new();
    for word in words_formatted{
        let word_in_hashmap = words.get(&word);
        if let Some(v) = word_in_hashmap{
            words.insert(word, v + 1);
        }
        else{
            words.insert(word, 1);
        }
    }
    words
}

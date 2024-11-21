use std::{collections::HashMap, fs};
#[cfg(feature = "cli")]
use colored::Colorize;
#[cfg(feature = "cli")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "cli")]
use toml;

#[cfg(feature = "cli")]
use std::sync::Arc;

#[cfg(feature = "cli")]
#[derive(Deserialize, Serialize, Clone)]
struct Configuration{
    ignored_chars : Vec<char>
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
struct Word{
    value : String,
    amount : u64
}

#[derive(Clone, Copy)]
struct Range{
    min: u64,
    max: u64
}

impl Range{
    fn in_range(range: Range, value: u64) -> bool{
        return value >= range.min && value <= range.max;
    }
}

impl From<String> for Range{
    fn from(value: String) -> Self {
        let mut number_1_string : Vec<char> = vec![];
        let mut number_2_string : Vec<char> = vec![];
        let mut is_num1 : bool = true; // true -> num1 false -> num2
        for c in value.chars().collect::<Vec<char>>(){
               if is_num1 && c != '.'{
                   number_1_string.push(c);
               } 
               else if !is_num1 && c != '.'{
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


#[cfg(feature = "cli")]
impl std::fmt::Display for Word{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "===>\"{}\"<=== \n was repeated {} times",
            self.value.bold().yellow(), 
            self.amount.to_string().bold().italic().cyan())
    }
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut file_directory = std::env::home_dir().unwrap();
    let config : Arc<Configuration> =if args.len() >= 4{
        file_directory.push(format!("/.config/mcw/{}.toml", args[3])); 
        let file_contents = match fs::read_to_string(file_directory){
            Ok(v) => v,
            Err(_) => {
                "ignored_chars = [',', '.', '?', '!', ':', ';']".to_string()
            }
        };

        let final_config = match toml::from_str(&file_contents){
            Ok(v) => v,
            Err(_) => {
                Configuration{ignored_chars : vec![',', '.', '?', '!', ':', ';']}.into()
            }
        };
        Arc::new(final_config)
    }
    else
    {   
        Arc::new(Configuration{ignored_chars : vec![',', '.', '?', '!', ':', ';']})
    };

    if args.len() == 1{
        println!("Not enough arguments: use --help")
    }
    else if args.len() == 2 && args[1] == "--help" {
        println!("
        mcw -> most common word 

        --help
        
        -formula-
        *executable **file.format ***range
        ---------
        *executable -> execute program

        **file.format -> file that program should read 

        ***range -> range beetwen which word needs to exist to be mentioned
        
        -Example-
        ./mcw text.txt 0.10
        means that program will read from file named `text.txt` 
        and a word needs to be repeated atleast 1 time to 10 times
        ");
    } else if args.len() >= 2 {
        let file_names : Arc<Vec<String>> = Arc::new(args[1].clone()
            .split_whitespace()
            .map(|f| f.to_string())
            .collect());

        let mut handles = vec![];
        for i in 0..file_names.len(){
            let shared_file_name = Arc::new(file_names[i].clone());
            let shared_conf = Arc::clone(&config);
            handles.push(tokio::spawn(async move{
                process_text(&shared_file_name, &shared_conf)
                    .await
            }));
        }

        let mut threads : Vec<Result<(HashMap<String, u64>,), tokio::task::JoinError>> = vec![];
        for handle in handles{
            threads.push(tokio::try_join!(handle));
        }
        let mut words : Vec<Word> = vec![];
        let word_range : Range = Range::from(args[2].clone());
        // Remove `Result<> and JoinError` - tuple remained
        let processed_vec : Vec<(HashMap<String, u64>,)> = threads
        .into_iter()
        .filter_map(|result| match result {
            Ok(map) => Some(map),
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        })
        .collect();
        let mut final_hashmap : Vec<HashMap<String, u64>> = vec![HashMap::new(); processed_vec.len()];
        for index in 0..processed_vec.len(){
            (final_hashmap[index],) = processed_vec[index].clone();
        }

        let merged_hashmap = merge_hashmap_vector(final_hashmap).await;

        for key in merged_hashmap.keys(){
            let word = merged_hashmap.get(key).unwrap();
                if Range::in_range(word_range, *word){
                    words.push(Word{
                        value: key.to_string(),
                        amount: *word
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

#[cfg(feature = "cli")]
async fn process_text(file_path: &str, config : &Configuration) -> HashMap<String, u64> {
    let file_contents = match fs::read_to_string(file_path){
        Ok(v) => v.to_string(),
        Err(_) => "".to_string()
    };
    let words_as_vec: Vec<String> = file_contents
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let mut word_array_without_special : Vec<String> = vec![];
    for word in &words_as_vec {
        let word_as_chars: Vec<char> = word.chars().collect();
        let mut chars : Vec<char> = vec![];
        for c in word_as_chars {
            let mut can_add : bool = true;
            for ignored_c in &config.ignored_chars{
                if c == *ignored_c{
                    can_add = false;
                }
            }
            if can_add {
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

#[cfg(feature = "cli")]
async fn merge_hashmap_vector(vector : Vec<HashMap<String, u64>>) -> HashMap<String, u64>{
    let mut returned_hashmap : HashMap<String, u64> = HashMap::new();
    for hshm in vector{
        for key in hshm.keys(){
            let cont1 = hshm.get(key).unwrap();
            returned_hashmap
            .insert(key.to_string(), cont1 + 1);
        }
    }
    returned_hashmap
}

#[cfg(not(feature = "cli"))]
#[cfg(not(feature = "async"))]
fn main(){
    return;
}

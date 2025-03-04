mod lib;
use lib as tan;

fn main(){
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(arg1) = args.get(1){
        match arg1.as_str(){
            "--help" | "-h" => {
                println!("
tan - 2.0.0                   
FLAGS:
-F/--file [FILE]      : takes text from file
-h/--help             : displays this message
-v/--version          : shows version
-i/--ignored [CHARS]  : ignores chars in text. chars must be split with ',' and passed without space;

made by matissoss
"
                );
            }
            "-v" | "--version" => {
                println!("tan version 2.0.0")
            }
            "-F" | "--file" => {
                if let Some(arg2) = args.get(2){
                    let mut ignored_chars = std::collections::HashSet::new();
                    if args.contains(&"-I".to_string()) || args.contains(&"--ignored".to_string()){
                        for (pos,arg) in args.iter().enumerate(){
                            if arg.as_str() == "-i" || arg.as_str() == "--ignored"{
                                if let Some(arg1) = args.get(pos+1){
                                    for c in arg1.split(',').map(|s| s.trim().to_string().chars().collect::<Vec<char>>()[0]).collect::<Vec<char>>(){
                                        ignored_chars.insert(c);
                                    }
                                } 
                            }
                        }
                    }
                    if let Ok(true) = std::fs::exists(arg2){
                        if let Ok(str) = std::fs::read_to_string(arg2){
                            for word in tan::get_words(&str, ignored_chars){
                                println!("{} : {}", word.value, word.amount);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

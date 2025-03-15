#[deprecated(since="2.0.0", note="!!!THIS CRATE SHOULD NOT BE USED AS IT WON'T BE LONGER MAINTAINED!!! async_get_words() is deprecated! use get_words() instead!")]
pub async fn async_get_words (){
    println!("async_get_words() is deprecated! use get_words() instead");
    std::process::exit(-1);
}

#[deprecated(since="2.1.0", note="!!!THIS CRATE SHOULD NOT BE USED AS IT WON'T BE LONGER MAINTAINED!!!")]
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Default)]
pub struct Word{
    pub value: String,
    pub amount: u64
}

#[deprecated(since="2.1.0", note="!!!THIS CRATE SHOULD NOT BE USED AS IT WON'T BE LONGER MAINTAINED!!!")]
pub fn get_words(input: &str, ignored_chars: std::collections::HashSet<char>) -> Vec<Word>{
    let mut inside_closure = (false, ' ');
    let mut temp_buf = Vec::new();
    let mut final_vector = Vec::new();
    let mut temp_hashmap : std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for c in input.chars().collect::<Vec<char>>(){
        if inside_closure == (true, '\"') && c == '\"'{
            if let Some(element) = temp_hashmap.get(&String::from_iter(temp_buf.iter())) {
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), *element+1);
            }
            else{
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), 1);
            }
            temp_buf = Vec::new();
            inside_closure = (false, ' ');
            continue;
        }
        else if inside_closure == (true, '\'') && c == '\"'{
            if let Some(element) = temp_hashmap.get(&String::from_iter(temp_buf.iter()).trim().to_string()) {
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), *element+1);
            }
            else{
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), 1);
            }
            temp_buf = Vec::new();
            inside_closure = (false, ' ');
            continue;
        }
        else if inside_closure == (false, ' ') && c == '\"' || c == '\''{
            inside_closure = (true, c);
            continue;
        }
        else if c == ' ' && inside_closure == (false, ' '){
            if let Some(element) = temp_hashmap.get(&String::from_iter(temp_buf.iter()).trim().to_string()) {
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), *element+1);
            }
            else{
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), 1);
            }
            temp_buf = Vec::new();
        }
        else{
            if !ignored_chars.contains(&c){
                temp_buf.push(c);
            }
        }
    }

    if !temp_buf.is_empty(){
            if let Some(element) = temp_hashmap.get(&String::from_iter(temp_buf.iter()).trim().to_string()) {
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), *element+1);
            }
            else{
                temp_hashmap.insert(String::from_iter(temp_buf.iter()).trim().to_string(), 1);
            }
    }

    for key in temp_hashmap.keys(){
        if let Some(el) = temp_hashmap.get(key){
            final_vector.push(Word{value: key.clone(), amount: *el});
        }
    }

    return final_vector;
}

<div align=center>
    <h1>tan - text analysis</h1>
</div>

---

tan (previously mcw) is rust crate that stands for: "text analysis"
mcw does what it says -> returns most common words  

# Using the Crate

```rust 
use tan;
fn main(){
    let words = tan::get_words("Hello World", std::collections::HashSet::new());
    for word in words{
        println!("{} : {}", word.value, word.amount);
    }
}
```

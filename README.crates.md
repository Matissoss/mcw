<div align=center>
    <h1>mcw - most common word</h1>
</div>
---
mcw is rust crate that stands for: "most common word"
mcw does what it says -> returns most common words  
# Using the Crate
```rust 
use mcw::*;
fn main() -> Result<(), Box<dyn std::error::Error>>{
      let words : Vec<Word> = get_words("These are words. Lorem Ipsum.", false)?;
      for word in words{
          println!("{}", word);
      }
      Ok(())
}
```
***Additional Note*** : argument of type bool contains whether the argument of type String is
file directory or not. Error only may occur if bool is true.
 
If your project ships with `tokio` in dependencies, then you can use async version of mcw, using
`async` feature in specifications. Only change is that program uses `tokio::fs` instead of
`std::fs`

**/Cargo.toml**
```
[package]
name = "your_package"
version = "0.1.0"
edition = "2021"
[dependencies]
mcw = {version = "1.3.0", features = ["async"]}
```
 **/src/main.rs**
 ```rust 
 #[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>>{
      let words : Vec<Word> = get_words("These are words. Lorem Ipsum. Async Version of
      function", false).await?;

      for word in words{
         println!("{}", word);
      }

      Ok(())
}
```

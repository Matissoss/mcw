# What are presets?

Presets are option to customize behaviour of program. 
Presets were added in v1.1.0. 
Currently only preset parameter are `ignored_chars`, which as name suggests - 
contains characters that CLI will ignore in counting words.

-Example-:

text = "Litwo Ojczyzno moja!"[^1]

ignored_chars = ['!', ',', [...]]

Output = "litwo", "ojczyzno", "moja"

-EOE[^0]-

-Example 2-:

text = "Ślachetne zdrowie, Nikt się nie dowie, Jako smakujesz, Aż się zepsujesz."[^2]

ignored_chars = ['!', ',', '.', '?', [...]]

Output = "ślachetne", "zdrowie", "nikt", "się", "nie", "dowie", "jako", "smakujesz", "aż", "się", "zepsujesz"

-EOE-

# How to make one?

|Operating System|Compatible?|
|----------------|-----------|
|Windows|not tested* |
|MacOS|Should Work|
|Linux|Yes|

`*` -> rust_analyzer says that `std::env::home_dir()` may lead to unexpected behaviour on Windows (idk if that's true)

Presets are located in: `~/.config/mcw` or `/home_path/mcw/`
To create preset, create `~/.config/mcw` directory, then add your preset in `.toml` format.

Only field currently you'll need to write is: `ignored_chars` 
which takes Vector of characters as input

-Example-
```toml
ignored_chars = [',', '?', '.']
```
-EOE-

To use preset, pass it as third/fourth argument
-Example-
`./exec 'file1.txt file2.txt' 1 YOUR_PRESET_WITHOUT_TOML_EXTENSION`
-EOE-

---
[^0]: EOE - End of Example
[^1]: Adam Mickiewicz, Pan Tadeusz
[^2]: Jan Kochanowski, "Na Zdrowie"

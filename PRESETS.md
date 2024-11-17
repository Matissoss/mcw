# Presets and how to use them
---
|Operating System|Compatible|
|----------------|----------|
|Windows         | idk*      |
|MacOS           |Not Tested|
|Linux           |Yes       |
* Program uses `std::env::home_dir()` which can lead to unexpected behaviour of file according to rust warnings
---
In version `1.1.0` presets were added to allow for customized filters.

To use them, pass them as third/fourth argument
-Example Formula-
`./exec path_to_file 10 PRESET`

`PRESET` is path to custom preset located in `~/.config/mcw/PRESET.toml`

Every `PRESET` is stored in `.toml` file, ***DON'T PROVIDE EXTENSION AS PROGRAM AUTOMATICALLY ADDS .toml EXTENSION***

# Creating Custom Preset 

As mentioned before, presets are located `~/.config/mcw/`. 
To create file you must provide ignored_chars of type `Vec<char>`

-Default Config-
```
ignored_chars = [',', '.', '?', '!']
```

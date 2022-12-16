# "Count" Dracula

A parser + utils for counting lines and figuring out their meaningfulness for better line counting API.

Can identify lines based on criterion of,
- Blank, (contains only whitespace)
- Meaningless, (has no impact on the code, like only braces, only comments etc.)
- Comment, (number of lines with comments)
- String, (is language string, raw strings, literal strings, strings, format strings etc.)
- Source (everything other than above)

## How to run?

Tentative, planned to be made into a library + binary.

The APIs for use as libraries are WIP.

```sh
# to test
cargo test

# to run as a dumb util
cargo run -- <rust|python|cpp|c> /path/to/file
```

## Table of Contents
- ["Count" Dracula](#count-dracula)
  - [How to run?](#how-to-run)
  - [Table of Contents](#table-of-contents)
  - [Key Goals](#key-goals)
  - [Works with](#works-with)
  - [Dependencies](#dependencies)
  - [Development](#development)
  - [Credits](#credits)

## Key Goals

- General enough to work over any language.
- Performant enough to work with as many files as possible, and allow scanning huge files.
- Pull Parsing for lazy parsing of the code to allow flexible usage. 

## Works with

- C & C++
- Python
- Rust
- etc...

## Dependencies

Default Runtime Dependencies, None.

Rust Version Dependencies, `1.59.0`. (Could possibly work with older rust versions)

## Development
## Credits
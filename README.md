# "Count" Dracula

A parser + utils for counting lines and figuring out their meaningfulness for better line counting API.

Can identify lines based on the following criteria:
- Blank, (contains only whitespace)
- Meaningless (has no impact on the code, e.g. braces, comments, etc.)
- Comment (number of lines with comments)
- String (strings, language strings, raw strings, literal strings, format strings, etc.)
- Source (the actual code)

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

Default runtime dependencies: None.

Rust version dependencies:`1.59.0` (Could possibly work with older Rust versions)

## Development
## Credits
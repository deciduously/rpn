# rpn
Quick RPN calculator in Rust
## Dependencies
Stable Rust v1.25.0+
## Usage
`cargo test` to test the example found on [Wikipedia](https://en.wikipedia.org/wiki/Reverse_Polish_notation#Example).

`cargo run` with no arguments will prompt for a line of input to evaluate, or attempt to evaluate any args passed as input. Be careful - `*` and `-` should be in quotes to avoid your shell parsing it as a glob!  For example: `cargo run -- 15 7 1 1 + '-' / 3 '*' 2 1 1 + + '-'`.

Rinse, repeat.

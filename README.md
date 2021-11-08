# Web browser in Rust

## How to build a web browser in rust
### Part 1: DOM
- set DOM Tree node's data structure

### Part 2: HTML
- parse HTML text and create DOM Tree

:::info
Rust's string is UTF-8, so we should use char_indices() to advance self.pos to the next character
:::


### Part 3: CSS


### HTML BNF (this browser's original)
nodes -> (node S*)*
node -> element | text

text -> (alphabet | digit )+
element -> open_tag S* nodes S* close_tag

open_tag -> "<" tag_name (S+ attributes)* ">"
close_tag -> "</" tag_name ">"
tag_name -> alphabet+

attributes -> (attribute S*)*
attribute -> attribute_name S* "=" S* attribute_value

attribute_name -> alphabet+
attribute_value -> '"' attribute_inner_value '"'
attribute_inner_value -> (alphabet | digit)+

alphabet = 'a'..'z' | 'A'..'Z'
digit = '0'..'9'
S -> " " | "\n"



## ToDo
- [ ] add unit test

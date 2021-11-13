# Web browser in Rust

## How to build a web browser in rust
### Part 1: DOM
- set DOM Tree node's data structure

### Part 2: HTML
- parse HTML text and create DOM Tree

### HTML BNF (this browser's original)
```
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
```

:white_check_mark:
Rust's string is UTF-8, so we should use char_indices() to  advance self.pos to the next character

### Part 3: CSS
- parse CSS text and create List Structure Stylesheet

row css text
```css
h1,h2,h3{
    margin: auto;
    color: #cc0000;
}
div.note {
    margin-bottom: 20px;
    padding: 10px;
}

```

- Stylesheet
    - Rule1:
        - selectors:
            - tag: h1, id: null, class: null
            - tag: h2, id: null, class: null
            - tag: h3, id: null, class: null
        - declarations:
            - name: "margin", value: "auto"
            - name: "color", value: Color(r:204,g:0,b:0,a:255)
    - Rule2
        - selectors:
            - tag: div, id: null, class: "note"
        - declarations:
            - name: "margin-bottom", value:Length(20.0,"px")
            - name: "padding", value: Length(10.0,"px")


:white_check_mark:
RGBA, A is opacity(透明度) for a color
: implementation point
```rust:style.rs
/// Apply style to a single element, returning the specified styles
///
/// To do: Allow multiple UA/author/user stylesheets, and implement the cascade
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap{
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity
    rules.sort_by(|&(a,_), &(b,_)|a.cmp(&b));
    for (_,rule) in rules {
        for declaration in &rule.declarations {// if multiple rules is containd, hash map overwrite the most specified one.
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    values
}
```
### Part 4: Style
- create Style Tree which is extended dom tree which attached style rule

StyledNode
    - node: pointer to dom tree node
    - specified_values: PropertyMap, map from css property names to values
    - children: vector of StyledNode

### Part 5,6: Boxes and Block layout
- layout takes the style tree and translates it into a bunch of rectangles in a 2-demensional space
-
#### The Box Model
> Layout is all about boxes. A box is a rectangular section of a web page. It has a width, a height, and a position on the page. This rectangle is called the content area because it's where the box's content is drawn. The content may be text, image, video, or other boxes. <br><br>
A box may also have padding, borders, and margins surrounding its content area. The CSS spec has a diagram showing how all these layers fit together.
>> quote from [robinson](https://limpet.net/mbrubeck/2014/09/08/toy-layout-engine-5-boxes.html)


- LayoutBox
  - Dimensions
  - BoxType
    - BlockNode(StyledNode)
    - InlineNode(StyledNode)
    - Anonymous
  - children(Vec<LayoutBox>)

##### note
- parent can calculate width first, but can't calculate height before children.

### Part 7: Painting
- Painting takes the tree of boxes from the layout module and turns them into an array of pixels.
- this process is known as "rasterization"
- this module only use rectangles to paint.
- display list is a vector of DisplayCommands
- 
## ToDo
- HTML
  -
- CSS
  - add various values
- Style
  - cascade system
  - initial and/or computed values
  - inheritance
  - the style attribute



## Ref
[robinson source code](https://github.com/mbrubeck/robinson)
[dackdive's blog](https://dackdive.hateblo.jp/entry/2021/02/23/113522)
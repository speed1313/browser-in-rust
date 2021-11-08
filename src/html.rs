use crate::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

// parse an HTML document and return the root element.
pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    // If the document contains a root element, just return it. Otherwise, create one.
    if nodes.len() == 1 {
        nodes.swap_remove(0) //nodes.len()==1 means source contains only one element(<html> ~ </html>).
    } else {
        dom::elem("html".to_string(), HashMap::new(), nodes) //else, it does not use html tag, so create it.
    }
}

impl Parser {
    /// Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
    ///Parse a single node.
    fn parse_node(&mut self) -> dom::Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }

    /// Parse a text node
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    ///Parse a single element, including its open tag, contents, and closing tag( <p link="http//:~~" > <h1> hello </h1> </p>)
    fn parse_element(&mut self) -> dom::Node {
        //Opening tag
        assert!(self.consume_char() == '<'); //panic if this expression is false
        let tag_name = self.parse_tag_name();
        let attrs = self.parse_attributes();
        assert!(self.consume_char() == '>');

        //Contents
        let children = self.parse_nodes();

        //Closing tag
        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag_name() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::elem(tag_name, attrs, children);
    }

    // Parse a single name="value" pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_tag_name();
        assert!(self.consume_char() == '=');
        let value = self.parse_attr_value();
        return (name, value);
    }

    // Parse a quoted value
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        assert!(self.consume_char() == open_quote);
        return value;
    }

    //Parse a list of name="value" pairs, separated by whitespace
    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    /// Parse a tag or attribute name
    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    ///Consume and discard zero or more whitespace characters
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    ///Consume characters until `test` returns false.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    /// Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return cur_char;
    }

    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Return true if all input is cousumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::dom::{elem, text};
    use std::collections::HashMap;
    #[test]
    fn test0(){
        assert_eq!(1,1);
    }
    #[test]
    fn test1() {
        let source = String::from(
            r#"
<html>
    <h1>hello</h1>
</html>
"#,
        );
        let expected = elem(
            String::from("html"),
            HashMap::new(),
            vec![
                elem(
                    String::from("h1"),
                    HashMap::new(),
                    vec![text(String::from("hello"))],
                ),
            ],
        );

        assert_eq!(expected, parse(source));
    }
    #[test]
    fn test2() {
        let source = String::from(
            r#"
<html>
    <body>
        <h1>Title</h1>
        <div id="main" class="test">
            <p>Hello<em>world</em>!</p>
        </div>
    </body>
</html>
"#,
        );
        let mut div_attrs = HashMap::new();
        div_attrs.insert(String::from("id"), String::from("main"));
        div_attrs.insert(String::from("class"), String::from("test"));
        let expected = elem(
            String::from("html"),
            HashMap::new(),
            vec![elem(
                String::from("body"),
                HashMap::new(),
                vec![
                    elem(
                        String::from("h1"),
                        HashMap::new(),
                        vec![text(String::from("Title"))],
                    ),
                    elem(
                        String::from("div"),
                        div_attrs,
                        vec![elem(
                            String::from("p"),
                            HashMap::new(),
                            vec![
                                text(String::from("Hello")),
                                elem(
                                    String::from("em"),
                                    HashMap::new(),
                                    vec![text(String::from("world"))],
                                ),
                                text(String::from("!")),
                            ],
                        )],
                    ),
                ],
            )],
        );

        assert_eq!(expected, parse(source));
    }
}

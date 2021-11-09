use crate::dom::{Node, NodeType, ElementData};
use crate::css::{Stylesheet, Rule, Selector, SimpleSelector, Value, Specificity};
use std::collections::HashMap;

/// Map from CSS property names to values
type PropertyMap = HashMap<String, Value>;

/// A node with associated style data
#[derive(Debug, PartialEq)]
pub struct StyledNode<'a> {
    node: &'a Node,// pointer to a DOM node
    pub specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

#[derive(PartialEq)]
pub enum Display{
    Inline,
    Block,
    None,
}

impl<'a> StyledNode<'a> {
    /// Return the specified value of a property if it exists, otherwise `None`
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// Return the specified value of property `name, or property `fallback_name` if that
    /// doesn't exist, or value `defalut` if neither does
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value)->Value{
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }

    /// The value of the `display property (defaults to inline)
    pub fn display(&self)->Display{
        match self.value("display"){
            Some(Value::Keyword(s))=> match &*s{
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            _ => Display::Inline
        }
    }
}


/// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
///
/// This finds only the specified values at the moment. Eventually it should be extended to find the
/// computed values too, including inherited values.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode{
        node: root,
        specified_values: match root.node_type{
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

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

/// A single CSS rule and the specificity of its most specific matching selector
type MatchedRule<'a> = (Specificity, &'a Rule);

/// Find all CSS rules that match the given element
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    // For now, we just do a linear scan of all the rules.  For large
    // documents, it would be more efficient to store the rules in hash tables
    // based on tag name, id, class, etc.
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

/// If `rule` matches `elem`, return a `MatchedRule`, Otherwise return `Node`
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    //Find the first (most specific) matching selector
    rule.selectors.iter().find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

/// Selector matching
fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector{
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

// if css's selector doesnt match html's tag, it is unused
fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    //Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    //Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
        return false;
    }

    // We didn't find any non-matching selector components
    return true;

}

#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use super::{style_tree, StyledNode};
    use crate::css;
    use crate::css::{Color,Value};
    use crate::dom::{text};
    use crate::html;

    #[test]
    fn test_style_tree_overwrite() {
        let html_source = String::from(r#"<p class="name">Hello</p>"#);

        let css_source = String::from(
            r#"
        p {
            color: #cccccc;
        }

        p.name {
            color: #cc0000;
        }
        "#,
        );
        let root = html::parse(html_source);
        let css = css::parse(css_source);

        let mut specified_values = HashMap::new();
        specified_values.insert(
            String::from("color"),
            Value::ColorValue(Color {
                r: 204,
                g: 0,
                b: 0,
                a: 255,
            }),
        );
        let text = text(String::from("Hello"));
        let expected = StyledNode {
            node: &root,
            specified_values,
            children: vec![StyledNode {
                node: &text,
                specified_values: HashMap::new(),
                children: vec![],
            }],
        };
        assert_eq!(expected, style_tree(&root, &css));
    }
}
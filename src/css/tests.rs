use crate::{
    css::{
        parser::CssParser,
        selector::{DomElement, matches_selector_list, parse_selector_list},
    },
    html::types::{Document, Element, Node},
};

#[test]
fn parses_extended_properties() {
    let d=CssParser::parse_declarations("display:flex; width:50%; padding:2pt 4pt; color:rgb(10,20,30); justify-content:space-between; white-space:nowrap").unwrap();
    assert_eq!(d.len(), 6);
}
fn element(tag: &str, id: Option<&str>, classes: &[&str], children: Vec<Node>) -> Element {
    Element {
        tag_name: tag.to_owned(),
        id: id.map(str::to_owned),
        classes: classes.iter().map(|class| (*class).to_owned()).collect(),
        attributes: Vec::new(),
        children,
    }
}

#[test]
fn selectors_use_real_tree_relationships() {
    let document = Document {
        children: vec![Node::Element(element(
            "invoice",
            None,
            &[],
            vec![
                Node::Element(element("div", None, &["row"], vec![])),
                Node::Element(element("div", None, &["row"], vec![])),
            ],
        ))],
    };

    let second = DomElement::new(&document, vec![0, 1]);

    assert!(matches_selector_list(
        &parse_selector_list("invoice > .row + .row").unwrap(),
        &second,
    ));
    assert!(matches_selector_list(
        &parse_selector_list("invoice > .row:nth-child(2)").unwrap(),
        &second,
    ));
    assert!(matches_selector_list(
        &parse_selector_list("invoice > .row:not(:first-child)").unwrap(),
        &second,
    ));
}

#[test]
fn selectors_support_attribute_matching() {
    let document = Document {
        children: vec![Node::Element(element(
            "invoice",
            None,
            &[],
            vec![Node::Element({
                let mut value = element("div", Some("total"), &["row", "important"], vec![]);
                value.attributes.push(crate::html::types::Attribute {
                    name: "data-kind".to_owned(),
                    value: "total".to_owned(),
                });
                value
            })],
        ))],
    };

    let total = DomElement::new(&document, vec![0, 0]);

    assert!(matches_selector_list(
        &parse_selector_list("#total[data-kind=\"total\"].important").unwrap(),
        &total,
    ));
}

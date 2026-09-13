use crate::html::{
    HtmlParser,
    helper::text_content,
    types::{Attribute, Element, Node},
};

// SECTION: types.rs

#[test]
fn attribute_return_correct_elements() {
    let element = Element {
        tag_name: "img".to_string(),
        id: None,
        classes: vec![],
        attributes: vec![
            Attribute {
                name: "src".to_string(),
                value: "logo.png".to_string(),
            },
            Attribute {
                name: "alt".to_string(),
                value: "Company logo".to_string(),
            },
        ],
        children: vec![],
    };

    assert_eq!(element.attribute("src"), Some("logo.png"));
    assert_eq!(element.attribute("alt"), Some("Company logo"));
}

#[test]
fn returns_none_for_missing_attribute() {
    let element = Element {
        tag_name: "div".to_string(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![],
    };

    assert_eq!(element.attribute("class"), None);
}

#[test]
fn attribute_return_first_element() {
    let element = Element {
        tag_name: "div".to_string(),
        id: None,
        classes: vec![],
        attributes: vec![
            Attribute {
                name: "data-test".to_string(),
                value: "first".to_string(),
            },
            Attribute {
                name: "data-test".to_string(),
                value: "second".to_string(),
            },
        ],
        children: vec![],
    };

    assert_eq!(element.attribute("data-test"), Some("first"));
}
#[test]
fn attribute_returns_none_when_missing() {
    let element = Element {
        tag_name: "div".into(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![],
    };

    assert_eq!(element.attribute("data-test"), None);
}

// SECTION: helper.rs

#[test]
fn text_content_returns_direct_text() {
    let element = Element {
        tag_name: "p".into(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![Node::Text("Hello".into())],
    };

    assert_eq!(text_content(&element), "Hello");
}

#[test]
fn text_content_returns_nested_text() {
    let element = Element {
        tag_name: "p".into(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![Node::Element(Element {
            tag_name: "strong".into(),
            id: None,
            classes: vec![],
            attributes: vec![],
            children: vec![Node::Text("Hello".into())],
        })],
    };

    assert_eq!(text_content(&element), "Hello");
}

#[test]
fn text_content_returns_text_from_multiple_children() {
    let element = Element {
        tag_name: "p".into(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![
            Node::Text("Hello ".into()),
            Node::Element(Element {
                tag_name: "strong".into(),
                id: None,
                classes: vec![],
                attributes: vec![],
                children: vec![Node::Text("world".into())],
            }),
            Node::Text("!".into()),
        ],
    };

    assert_eq!(text_content(&element), "Hello world!");
}

#[test]
fn text_content_returns_empty_string_without_text() {
    let element = Element {
        tag_name: "div".into(),
        id: None,
        classes: vec![],
        attributes: vec![],
        children: vec![],
    };

    assert_eq!(text_content(&element), "");
}

//SECTION: parser.rs

#[test]
fn parse_simple_element() {
    let document = HtmlParser::parse("<div>Hello</div>").expect("failed to parse HTML");
    // NOTE: The document has <html> as its only child.
    assert_eq!(document.children.len(), 1);

    let Node::Element(element) = &document.children[0] else {
        panic!("expected <html>");
    };

    // NOTE: <html> has two children: <head> and <body>.
    assert_eq!(element.tag_name, "html");
    assert_eq!(element.children.len(), 2);

    let Node::Element(body) = element.children[1].clone() else {
        panic!("expected <body>");
    };

    // NOTE: <body> should have only <div> as its child.
    assert_eq!(body.children.len(), 1);
    assert_eq!(body.tag_name, "body");

    // NOTE: <div> should have only one child: Node::Text.
    let Node::Element(div) = body.children[0].clone() else {
        panic!("expected <div>");
    };

    assert_eq!(div.tag_name, "div");
    assert_eq!(div.children.len(), 1);

    // NOTE: The text should be a Node::Text containing "Hello".
    let Node::Text(text) = div.children[0].clone() else {
        panic!("expected text");
    };

    assert_eq!(text, "Hello");
}

#[test]
fn parse_text_content() {
    let document = HtmlParser::parse("<p>Hello world</p>").expect("failed to parse HTML");

    let Node::Element(html) = &document.children[0] else {
        panic!("expected <html>");
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>");
    };
    let Node::Element(p) = body.children[0].clone() else {
        panic!("expected <p>");
    };

    assert_eq!(p.children, vec![Node::Text("Hello world".to_string())]);
}

#[test]
fn parse_element_id() {
    let document = HtmlParser::parse(r#"<div id="invoice"></div>"#).expect("failed to parse HTML");

    let Node::Element(html) = &document.children[0] else {
        panic!("expected <html>");
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>");
    };
    let Node::Element(div) = body.children[0].clone() else {
        panic!("expected <div>");
    };

    assert_eq!(div.id, Some("invoice".to_string()));
}

#[test]
fn parse_element_classes() {
    let document = HtmlParser::parse(r#"<div class="invoice large rtl"></div>"#)
        .expect("failed to parse HTML");

    let Node::Element(html) = &document.children[0] else {
        panic!("expected <html>");
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>");
    };
    let Node::Element(div) = body.children[0].clone() else {
        panic!("expected <div>");
    };

    assert_eq!(
        div.classes,
        vec![
            "invoice".to_string(),
            "large".to_string(),
            "rtl".to_string(),
        ]
    );
}

#[test]
fn parse_element_attributes() {
    let document =
        HtmlParser::parse(r#"<img src="logo.png" width="200">"#).expect("failed to parse HTML");

    let Node::Element(html) = &document.children[0] else {
        panic!("expected <html>");
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>");
    };
    let Node::Element(img) = body.children[0].clone() else {
        panic!("expected <img>");
    };

    assert_eq!(
        img.attributes,
        vec![
            Attribute {
                name: "src".to_string(),
                value: "logo.png".to_string(),
            },
            Attribute {
                name: "width".to_string(),
                value: "200".to_string(),
            },
        ]
    );
}

#[test]
fn parse_nested_elements() {
    let document = HtmlParser::parse("<div><p>Hello <strong>world</strong></p></div>")
        .expect("failed to parse HTML");

    let Node::Element(html) = &document.children[0] else {
        panic!("expected <html>");
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>");
    };
    let Node::Element(div) = body.children[0].clone() else {
        panic!("expected <div>");
    };
    let Node::Element(p) = div.children[0].clone() else {
        panic!("expected <p>");
    };

    assert_eq!(p.tag_name, "p");

    // NOTE: <p> has two children Node::Text(Hello),Node::Element(Strong)
    assert_eq!(p.children.len(), 2);

    assert_eq!(p.children[0], Node::Text("Hello ".to_string()));

    let Node::Element(strong) = &p.children[1] else {
        panic!("expected <strong>");
    };

    assert_eq!(strong.tag_name, "strong");
    assert_eq!(strong.children, vec![Node::Text("world".to_string())]);
}

#[test]
fn parse_multiple_root_elements() {
    let document = HtmlParser::parse("<h1>Invoice</h1><p>Hello</p>").expect("failed to parse HTML");

    let Node::Element(html) = document.children[0].clone() else {
        panic!("expected <html>")
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>")
    };

    assert_eq!(body.children.len(), 2);
}

#[test]
fn parse_ignores_comments() {
    let document =
        HtmlParser::parse("<div>Hello<!-- comment --></div>").expect("failed to parse HTML");

    let Node::Element(html) = document.children[0].clone() else {
        panic!("expected <html>")
    };
    let Node::Element(body) = html.children[1].clone() else {
        panic!("expected <body>")
    };

    let Node::Element(div) = body.children[0].clone() else {
        panic!("expected <body>")
    };

    assert_eq!(div.children, vec![Node::Text("Hello".to_string())]);
}

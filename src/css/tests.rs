use crate::{
    css::{
        computed::compute_style,
        edges::Edges,
        parser::CssParser,
        rules::{Declaration, Property, StyleRule, Value},
        selector::{
            DomElement, matches_selector_list, parse_selector_list, selector_list_specificity,
        },
        types::{
            Border, Color, ComputedStyle, Display, FontWeight, JustifyContent, Length, Position,
            TextAlign,
        },
    },
    html::types::{Attribute, Document, Element, Node},
    units::{Direction, Pt},
};
// SECTION: edges.rs
#[test]
fn zero_contains_zero_on_all_sides() {
    assert_eq!(Edges::ZERO.top, Pt::ZERO);
    assert_eq!(Edges::ZERO.right, Pt::ZERO);
    assert_eq!(Edges::ZERO.bottom, Pt::ZERO);
    assert_eq!(Edges::ZERO.left, Pt::ZERO);
}

#[test]
fn same_value_for_all_sides() {
    let value = Pt::new(10.0);

    assert_eq!(
        Edges::all(value),
        Edges {
            top: value,
            right: value,
            bottom: value,
            left: value,
        }
    );
}

#[test]
fn vertical_horizontal_create_two_values() {
    let vertical = Pt::new(10.0);
    let horizontal = Pt::new(20.0);

    assert_eq!(
        Edges::vertical_horizontal(vertical, horizontal),
        Edges {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    );
}

#[test]
fn three_step_create_top_horizontal_bottom() {
    let top = Pt::new(10.0);
    let horizontal = Pt::new(20.0);
    let bottom = Pt::new(30.0);

    assert_eq!(
        Edges::three_step(top, horizontal, bottom),
        Edges {
            top,
            right: horizontal,
            bottom,
            left: horizontal,
        }
    );
}

#[test]
fn four_step_create_each_side_independently() {
    let top = Pt::new(1.0);
    let right = Pt::new(2.0);
    let bottom = Pt::new(3.0);
    let left = Pt::new(4.0);

    assert_eq!(
        Edges::four_step(top, right, bottom, left),
        Edges {
            top,
            right,
            bottom,
            left,
        }
    );
}
// SECTION: rules.rs
#[test]
fn declaration_stores_property_value_and_importance() {
    let declaration = Declaration {
        property: Property::FontSize,
        value: Value::FontSize(Pt::new(20.0)),
        important: true,
    };

    assert_eq!(declaration.property, Property::FontSize);
    assert_eq!(declaration.value, Value::FontSize(Pt::new(20.0)));
    assert!(declaration.important);
}

#[test]
fn style_rule_stores_selector_declarations_and_source_order() {
    let selector =
        crate::css::selector::parse_selector_list(".title").expect("selector should parse");

    let rule = StyleRule {
        selector,
        declarations: vec![Declaration {
            property: Property::Display,
            value: Value::Display(Display::Block),
            important: false,
        }],
        source_order: 7,
    };

    assert_eq!(rule.declarations.len(), 1);
    assert_eq!(rule.source_order, 7);

    assert_eq!(
        rule.declarations[0],
        Declaration {
            property: Property::Display,
            value: Value::Display(Display::Block),
            important: false,
        }
    );
}

#[test]
fn value_equality_works_for_colors() {
    let first = Value::Color(Color::rgb(255, 0, 0));
    let second = Value::Color(Color::rgb(255, 0, 0));

    assert_eq!(first, second);
}

#[test]
fn different_properties_are_not_equal() {
    assert_ne!(Property::Margin, Property::Padding);
    assert_ne!(Property::Color, Property::BackgroundColor);
}

// SECTION: types.rs
#[test]
fn length_in_points_resolves_directly() {
    let length = Length::Pt(Pt::new(25.0));

    assert_eq!(length.resolve(Pt::new(100.0), Pt::new(10.0)), Pt::new(25.0));
}

#[test]
fn percentage_resolves_against_containing_size() {
    let length = Length::Percent(50.0);

    assert_eq!(
        length.resolve(Pt::new(200.0), Pt::new(10.0)),
        Pt::new(100.0)
    );
}

#[test]
fn auto_uses_auto_value() {
    let length = Length::Auto;

    assert_eq!(length.resolve(Pt::new(200.0), Pt::new(30.0)), Pt::new(30.0));
}

#[test]
fn border_solid_stores_width_and_color() {
    let width = Pt::new(2.0);
    let color = Color::rgb(100, 110, 120);

    assert_eq!(Border::solid(width, color), Border { width, color });
}

#[test]
fn computed_style_has_expected_defaults() {
    let style = ComputedStyle::default();

    assert_eq!(style.display, Display::Inline);
    assert_eq!(style.direction, Direction::LTR);
    assert_eq!(style.position, Position::Static);

    assert_eq!(style.font_size, Pt::new(12.0));
    assert_eq!(style.font_weight, FontWeight::Normal);
    assert_eq!(style.line_height, 1.5);
    assert_eq!(style.text_align, TextAlign::Start);

    assert_eq!(style.color, Color::BLACK);
    assert_eq!(style.background_color, None);

    assert_eq!(style.margin, Edges::ZERO);
    assert_eq!(style.padding, Edges::ZERO);
    assert_eq!(style.border, Border::NONE);

    assert_eq!(style.opacity, 1.0);
}

//SECTION: parser.rs
#[test]
fn parses_simple_font_size() {
    let declarations = CssParser::parse_declarations("font-size: 20pt;").expect("should parse");
    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0].property, Property::FontSize);
    assert_eq!(declarations[0].value, Value::FontSize(Pt::new(20.0)));
    assert!(!declarations[0].important);
}

#[test]
fn parses_multiple_declarations() {
    let declarations =
        CssParser::parse_declarations("font-size: 20pt; color: red; text-align: center;")
            .expect("should parse");

    assert_eq!(declarations.len(), 3);

    assert_eq!(declarations[0].property, Property::FontSize);

    assert_eq!(declarations[1].property, Property::Color);

    assert_eq!(declarations[2].property, Property::TextAlign);
}

#[test]
fn parses_hex_color() {
    let declarations = CssParser::parse_declarations("color: #ff0000;").expect("should parse");

    assert_eq!(declarations[0].value, Value::Color(Color::rgb(255, 0, 0)));
}

#[test]
fn parses_rgb_color() {
    let declarations =
        CssParser::parse_declarations("color: rgb(10, 20, 30);").expect("should parse");

    assert_eq!(declarations[0].value, Value::Color(Color::rgb(10, 20, 30)));
}

#[test]
fn parses_named_color() {
    let declarations = CssParser::parse_declarations("color: blue;").expect("should parse");

    assert_eq!(declarations[0].value, Value::Color(Color::rgb(0, 0, 255)));
}

#[test]
fn parses_px_as_points() {
    let declarations = CssParser::parse_declarations("font-size: 100px;").expect("should parse");

    assert_eq!(declarations[0].value, Value::FontSize(Pt::new(75.0)));
}

#[test]
fn parses_percentage_length() {
    let declarations = CssParser::parse_declarations("width: 50%;").expect("should parse");
    println!("{:?}", declarations);

    assert_eq!(declarations[0].property, Property::Width);

    assert_eq!(declarations[0].value, Value::Length(Length::Percent(50.0)));
}

#[test]
fn parses_auto_width() {
    let declarations = CssParser::parse_declarations("width: auto;").expect("should parse");

    assert_eq!(declarations[0].value, Value::Length(Length::Auto));
}

#[test]
fn parses_margin_shorthand() {
    let declarations = CssParser::parse_declarations("margin: 10pt 20pt;").expect("should parse");

    assert_eq!(
        declarations[0].value,
        Value::Edges(Edges::vertical_horizontal(Pt::new(10.0), Pt::new(20.0),))
    );
}

#[test]
fn parses_four_value_padding() {
    let declarations =
        CssParser::parse_declarations("padding: 1pt 2pt 3pt 4pt;").expect("should parse");

    assert_eq!(
        declarations[0].value,
        Value::Edges(Edges::four_step(
            Pt::new(1.0),
            Pt::new(2.0),
            Pt::new(3.0),
            Pt::new(4.0),
        ))
    );
}

#[test]
fn parses_display_flex() {
    let declarations = CssParser::parse_declarations("display: flex;").expect("should parse");

    assert_eq!(declarations[0].value, Value::Display(Display::Flex));
}

#[test]
fn parses_text_align_center() {
    let declarations = CssParser::parse_declarations("text-align: center;").expect("should parse");

    assert_eq!(declarations[0].value, Value::TextAlign(TextAlign::Center));
}

#[test]
fn parses_font_weight_bold() {
    let declarations = CssParser::parse_declarations("font-weight: bold;").expect("should parse");

    assert_eq!(declarations[0].value, Value::FontWeight(FontWeight::Bold));
}

#[test]
fn parses_important() {
    let declarations =
        CssParser::parse_declarations("font-size: 20pt !important;").expect("should parse");

    assert!(declarations[0].important);
}

#[test]
fn ignores_unknown_property() {
    let declarations = CssParser::parse_declarations("unknown-property: hello; color: red;")
        .expect("should parse");

    assert_eq!(declarations.len(), 1);
    assert_eq!(declarations[0].property, Property::Color);
}

#[test]
fn parses_stylesheet_and_assigns_source_order() {
    let rules = CssParser::parse_stylesheet(
        r#"
                .first { color: red; }
                .second { color: blue; }
                .third { color: green; }
            "#,
    )
    .expect("stylesheet should parse");

    assert_eq!(rules.len(), 3);

    assert_eq!(rules[0].source_order, 0);
    assert_eq!(rules[1].source_order, 1);
    assert_eq!(rules[2].source_order, 2);
}

#[test]
fn parses_justify_content() {
    let declarations =
        CssParser::parse_declarations("justify-content: space-between;").expect("should parse");

    assert_eq!(
        declarations[0].value,
        Value::JustifyContent(JustifyContent::SpaceBetween)
    );
}

//SECTION: selector.rs

fn element(
    tag: &str,
    id: Option<&str>,
    classes: &[&str],
    attributes: Vec<Attribute>,
    children: Vec<Node>,
) -> Element {
    Element {
        tag_name: tag.to_string(),
        id: id.map(str::to_string),
        classes: classes.iter().map(|class| class.to_string()).collect(),
        attributes,
        children,
    }
}
//tests

#[test]
fn matches_tag_selector() {
    let document = Document {
        children: vec![Node::Element(element("div", None, &[], vec![], vec![]))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list("div").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_class_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &["title"],
            vec![],
            vec![],
        ))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list(".title").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_id_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            Some("invoice"),
            &[],
            vec![],
            vec![],
        ))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list("#invoice").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn does_not_match_wrong_class() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &["title"],
            vec![],
            vec![],
        ))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list(".footer").expect("selector should parse");

    assert!(!matches_selector_list(&selector, &dom));
}

#[test]
fn matches_tag_and_class_together() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &["title"],
            vec![],
            vec![],
        ))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list("div.title").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_attribute_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &[],
            vec![Attribute {
                name: "data-kind".to_string(),
                value: "total".to_string(),
            }],
            vec![],
        ))],
    };

    let dom = DomElement::new(&document, vec![0]);

    let selector = parse_selector_list("[data-kind=\"total\"]").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_child_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &["parent"],
            vec![],
            vec![Node::Element(element(
                "span",
                None,
                &["child"],
                vec![],
                vec![],
            ))],
        ))],
    };

    let dom = DomElement::new(&document, vec![0, 0]);

    let selector = parse_selector_list(".parent > .child").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_descendant_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &["parent"],
            vec![],
            vec![Node::Element(element(
                "section",
                None,
                &[],
                vec![],
                vec![Node::Element(element(
                    "span",
                    None,
                    &["child"],
                    vec![],
                    vec![],
                ))],
            ))],
        ))],
    };

    let dom = DomElement::new(&document, vec![0, 0, 0]);

    let selector = parse_selector_list(".parent .child").expect("selector should parse");

    assert!(matches_selector_list(&selector, &dom));
}

#[test]
fn matches_first_child_selector() {
    let document = Document {
        children: vec![Node::Element(element(
            "div",
            None,
            &[],
            vec![],
            vec![
                Node::Element(element("span", None, &["first"], vec![], vec![])),
                Node::Element(element("span", None, &["second"], vec![], vec![])),
            ],
        ))],
    };

    let first = DomElement::new(&document, vec![0, 0]);

    let selector = parse_selector_list(":first-child").expect("selector should parse");

    assert!(matches_selector_list(&selector, &first));
}

#[test]
fn id_has_higher_specificity_than_class() {
    let class_selector = parse_selector_list(".title").expect("selector should parse");

    let id_selector = parse_selector_list("#title").expect("selector should parse");

    assert!(selector_list_specificity(&id_selector) > selector_list_specificity(&class_selector));
}

#[test]
fn class_has_higher_specificity_than_tag() {
    let tag_selector = parse_selector_list("div").expect("selector should parse");

    let class_selector = parse_selector_list(".title").expect("selector should parse");

    assert!(selector_list_specificity(&class_selector) > selector_list_specificity(&tag_selector));
}
//SECTION: computed.rs

fn make_element(tag: &str, id: Option<&str>, classes: &[&str], style: Option<&str>) -> Element {
    let mut attributes = Vec::new();

    if let Some(style) = style {
        attributes.push(Attribute {
            name: "style".to_string(),
            value: style.to_string(),
        });
    }

    Element {
        tag_name: tag.to_string(),
        id: id.map(str::to_string),
        classes: classes.iter().map(|class| class.to_string()).collect(),
        attributes,
        children: vec![],
    }
}

fn document_with_element(element: Element) -> Document {
    Document {
        children: vec![Node::Element(element)],
    }
}

#[test]
fn applies_matching_rule() {
    let element = make_element("div", None, &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(".title { font-size: 20pt; }").unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(20.0));
}

#[test]
fn ignores_non_matching_rule() {
    let element = make_element("div", None, &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(".footer { font-size: 20pt; }").unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(12.0));
}

#[test]
fn higher_specificity_wins() {
    let element = make_element("div", None, &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(
        r#"
                    div { font-size: 10pt; }
                    .title { font-size: 20pt; }
                "#,
    )
    .unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(20.0));
}

#[test]
fn id_specificity_wins_over_class() {
    let element = make_element("div", Some("main"), &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(
        r#"
                    .title { font-size: 20pt; }
                    #main { font-size: 30pt; }
                "#,
    )
    .unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(30.0));
}

#[test]
fn later_rule_wins_when_specificity_is_equal() {
    let element = make_element("div", None, &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(
        r#"
                    .title { font-size: 20pt; }
                    .title { font-size: 30pt; }
                "#,
    )
    .unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(30.0));
}

#[test]
fn important_wins_over_normal_declaration() {
    let element = make_element("div", Some("main"), &["title"], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(
        r#"
            #main { font-size: 20pt; }
            .title { font-size: 30pt !important; }
        "#,
    )
    .unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(30.0));
}

#[test]
fn inline_style_wins_over_stylesheet_rule() {
    let element = make_element("div", None, &["title"], Some("font-size: 40pt;"));

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(".title { font-size: 20pt; }").unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(40.0));
}

#[test]
fn inherited_font_size_is_copied_from_parent() {
    let element = make_element("span", None, &[], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let mut parent = ComputedStyle::default();

    parent.font_size = Pt::new(24.0);

    let style = compute_style(&dom, &element, Some(&parent), &[], ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(24.0));
}

#[test]
fn inherited_color_is_copied_from_parent() {
    let element = make_element("span", None, &[], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let mut parent = ComputedStyle::default();

    parent.color = Color::rgb(255, 0, 0);

    let style = compute_style(&dom, &element, Some(&parent), &[], ComputedStyle::default());

    assert_eq!(style.color, Color::rgb(255, 0, 0));
}

#[test]
fn margin_is_not_inherited() {
    let element = make_element("div", None, &[], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let mut parent = ComputedStyle::default();

    parent.margin = crate::css::edges::Edges::all(Pt::new(20.0));

    let style = compute_style(&dom, &element, Some(&parent), &[], ComputedStyle::default());

    assert_eq!(style.margin, crate::css::edges::Edges::ZERO);
}

#[test]
fn css_can_override_default_style() {
    let element = make_element("div", None, &[], None);

    let document = document_with_element(element.clone());

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet("div { display: flex; }").unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.display, Display::Flex);
}

#[test]
fn computed_style_applies_color() {
    let element = Element {
        tag_name: "div".to_string(),
        id: None,
        classes: vec!["title".to_string()],
        attributes: Vec::new(),
        children: Vec::new(),
    };

    let document = Document {
        children: vec![Node::Element(element.clone())],
    };

    let dom = DomElement::new(&document, vec![0]);

    let rules = CssParser::parse_stylesheet(".title { color: #ff0000; }").unwrap();

    let style = compute_style(&dom, &element, None, &rules, ComputedStyle::default());

    assert_eq!(style.color, Color::rgb(255, 0, 0));
}

#[test]
fn computed_style_inherits_font_size() {
    let element = Element {
        tag_name: "span".to_string(),
        id: None,
        classes: Vec::new(),
        attributes: Vec::new(),
        children: Vec::new(),
    };

    let document = Document {
        children: vec![Node::Element(element.clone())],
    };

    let dom = DomElement::new(&document, vec![0]);

    let mut parent = ComputedStyle::default();
    parent.font_size = Pt::new(24.0);

    let style = compute_style(&dom, &element, Some(&parent), &[], ComputedStyle::default());

    assert_eq!(style.font_size, Pt::new(24.0));
}

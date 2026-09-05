use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;

use cssparser::{CowRcStr, Parser as CssParser, ParserInput, SourceLocation, ToCss};
use precomputed_hash::PrecomputedHash;
use selectors::{
    Element as SelectorElementTrait, OpaqueElement,
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    bloom::BloomFilter,
    context::{
        MatchingContext, MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, QuirksMode,
        SelectorCaches,
    },
    matching::ElementSelectorFlags,
    parser::{
        NonTSPseudoClass, ParseRelative, Parser as SelectorParserTrait, PseudoElement,
        Selector as NativeSelector, SelectorImpl, SelectorList as NativeSelectorList,
        SelectorParseErrorKind,
    },
};

use crate::html::types::{Document, Element, Node};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct CssAtom(String);

impl From<&str> for CssAtom {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl Borrow<str> for CssAtom {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CssAtom {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CssAtom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl ToCss for CssAtom {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl PrecomputedHash for CssAtom {
    fn precomputed_hash(&self) -> u32 {
        // FNV-1a is only used as the selector-cache/bloom-filter hash.
        let mut hash = 2166136261u32;
        for byte in self.0.as_bytes() {
            hash ^= u32::from(*byte);
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PdfPseudoClass {
    Link,
    AnyLink,
    Visited,
    Hover,
    Active,
    Focus,
    FocusWithin,
    FocusVisible,
    Checked,
    Disabled,
    Enabled,
    Required,
    Optional,
    ReadOnly,
    ReadWrite,
}

impl ToCss for PdfPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(match self {
            Self::Link => ":link",
            Self::AnyLink => ":any-link",
            Self::Visited => ":visited",
            Self::Hover => ":hover",
            Self::Active => ":active",
            Self::Focus => ":focus",
            Self::FocusWithin => ":focus-within",
            Self::FocusVisible => ":focus-visible",
            Self::Checked => ":checked",
            Self::Disabled => ":disabled",
            Self::Enabled => ":enabled",
            Self::Required => ":required",
            Self::Optional => ":optional",
            Self::ReadOnly => ":read-only",
            Self::ReadWrite => ":read-write",
        })
    }
}

impl NonTSPseudoClass for PdfPseudoClass {
    type Impl = PdfSelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum PdfPseudoElement {}

impl ToCss for PdfPseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        Ok(())
    }
}

impl PseudoElement for PdfPseudoElement {
    type Impl = PdfSelectorImpl;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PdfSelectorImpl;

impl SelectorImpl for PdfSelectorImpl {
    type ExtraMatchingData<'a> = ();
    type AttrValue = CssAtom;
    type Identifier = CssAtom;
    type LocalName = CssAtom;
    type NamespaceUrl = CssAtom;
    type NamespacePrefix = CssAtom;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;
    type NonTSPseudoClass = PdfPseudoClass;
    type PseudoElement = PdfPseudoElement;
}

#[derive(Clone, Debug)]
pub struct PdfSelectorParser;

impl<'i> SelectorParserTrait<'i> for PdfSelectorParser {
    type Impl = PdfSelectorImpl;
    type Error = SelectorParseErrorKind<'i>;

    fn parse_nth_child_of(&self) -> bool {
        true
    }

    fn parse_is_and_where(&self) -> bool {
        true
    }

    fn parse_has(&self) -> bool {
        true
    }

    fn parse_non_ts_pseudo_class(
        &self,
        location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PdfPseudoClass, cssparser::ParseError<'i, Self::Error>> {
        if name.eq_ignore_ascii_case("link") {
            return Ok(PdfPseudoClass::Link);
        }
        if name.eq_ignore_ascii_case("any-link") {
            return Ok(PdfPseudoClass::AnyLink);
        }
        if name.eq_ignore_ascii_case("visited") {
            return Ok(PdfPseudoClass::Visited);
        }
        if name.eq_ignore_ascii_case("hover") {
            return Ok(PdfPseudoClass::Hover);
        }
        if name.eq_ignore_ascii_case("active") {
            return Ok(PdfPseudoClass::Active);
        }
        if name.eq_ignore_ascii_case("focus") {
            return Ok(PdfPseudoClass::Focus);
        }
        if name.eq_ignore_ascii_case("focus-within") {
            return Ok(PdfPseudoClass::FocusWithin);
        }
        if name.eq_ignore_ascii_case("focus-visible") {
            return Ok(PdfPseudoClass::FocusVisible);
        }
        if name.eq_ignore_ascii_case("checked") {
            return Ok(PdfPseudoClass::Checked);
        }
        if name.eq_ignore_ascii_case("disabled") {
            return Ok(PdfPseudoClass::Disabled);
        }
        if name.eq_ignore_ascii_case("enabled") {
            return Ok(PdfPseudoClass::Enabled);
        }
        if name.eq_ignore_ascii_case("required") {
            return Ok(PdfPseudoClass::Required);
        }
        if name.eq_ignore_ascii_case("optional") {
            return Ok(PdfPseudoClass::Optional);
        }
        if name.eq_ignore_ascii_case("read-only") {
            return Ok(PdfPseudoClass::ReadOnly);
        }
        if name.eq_ignore_ascii_case("read-write") {
            return Ok(PdfPseudoClass::ReadWrite);
        }

        Err(
            location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                name,
            )),
        )
    }

    fn allow_forgiving_selectors(&self) -> bool {
        false
    }

    fn default_namespace(&self) -> Option<CssAtom> {
        None
    }

    fn namespace_for_prefix(&self, _prefix: &CssAtom) -> Option<CssAtom> {
        None
    }
}

pub type SelectorList = NativeSelectorList<PdfSelectorImpl>;
pub type Selector = NativeSelector<PdfSelectorImpl>;

pub fn parse_selector_list(input: &str) -> Result<SelectorList, String> {
    let mut input = ParserInput::new(input);
    let mut parser = CssParser::new(&mut input);
    let selector_parser = PdfSelectorParser;

    let selectors = SelectorList::parse(&selector_parser, &mut parser, ParseRelative::No)
        .map_err(|error| format!("{error:?}"))?;

    parser
        .expect_exhausted()
        .map_err(|error| format!("{error:?}"))?;

    Ok(selectors)
}

#[derive(Clone, Debug)]
pub struct DomElement<'a> {
    pub document: &'a Document,
    pub path: Vec<usize>,
}

impl<'a> DomElement<'a> {
    pub fn new(document: &'a Document, path: Vec<usize>) -> Self {
        Self { document, path }
    }

    pub fn element(&self) -> Option<&'a Element> {
        node_at_path(self.document, &self.path).and_then(|node| match node {
            Node::Element(element) => Some(element),
            Node::Text(_) => None,
        })
    }

    fn parent_path(&self) -> Option<Vec<usize>> {
        if self.path.len() <= 1 {
            None
        } else {
            Some(self.path[..self.path.len() - 1].to_vec())
        }
    }

    fn sibling_element_path(&self, direction: i32) -> Option<Vec<usize>> {
        let parent_path = self.parent_path()?;
        let parent = node_at_path(self.document, &parent_path)?;
        let Node::Element(parent) = parent else {
            return None;
        };

        let current_index = *self.path.last()?;
        let mut iter: Box<dyn Iterator<Item = (usize, &Node)> + '_> = if direction < 0 {
            Box::new(parent.children.iter().enumerate().rev())
        } else {
            Box::new(parent.children.iter().enumerate())
        };

        iter.find(|(index, child)| {
            let before_or_after = if direction < 0 {
                *index < current_index
            } else {
                *index > current_index
            };
            before_or_after && matches!(child, Node::Element(_))
        })
        .map(|(index, _)| {
            let mut path = parent_path;
            path.push(index);
            path
        })
    }
}

fn node_at_path<'a>(document: &'a Document, path: &[usize]) -> Option<&'a Node> {
    let (first, rest) = path.split_first()?;
    let mut node = document.children.get(*first)?;

    for index in rest {
        let Node::Element(element) = node else {
            return None;
        };
        node = element.children.get(*index)?;
    }

    Some(node)
}

impl<'a> SelectorElementTrait for DomElement<'a> {
    type Impl = PdfSelectorImpl;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(
            self.element()
                .expect("selector adapter must point to an element"),
        )
    }

    fn parent_element(&self) -> Option<Self> {
        let path = self.parent_path()?;
        matches!(node_at_path(self.document, &path), Some(Node::Element(_)))
            .then(|| Self::new(self.document, path))
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.sibling_element_path(-1)
            .map(|path| Self::new(self.document, path))
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.sibling_element_path(1)
            .map(|path| Self::new(self.document, path))
    }

    fn first_element_child(&self) -> Option<Self> {
        let element = self.element()?;
        element
            .children
            .iter()
            .enumerate()
            .find_map(|(index, child)| {
                matches!(child, Node::Element(_)).then(|| {
                    let mut path = self.path.clone();
                    path.push(index);
                    Self::new(self.document, path)
                })
            })
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.element()
            .map(|element| element.tag_name.eq_ignore_ascii_case(local_name))
            .unwrap_or(false)
    }

    fn has_namespace(&self, namespace: &str) -> bool {
        namespace.is_empty()
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.element()
            .zip(other.element())
            .map(|(a, b)| a.tag_name.eq_ignore_ascii_case(&b.tag_name))
            .unwrap_or(false)
    }

    fn attr_matches(
        &self,
        namespace: &NamespaceConstraint<&CssAtom>,
        local_name: &CssAtom,
        operation: &AttrSelectorOperation<&CssAtom>,
    ) -> bool {
        let namespace_matches = match namespace {
            NamespaceConstraint::Any => true,
            NamespaceConstraint::Specific(url) => url.as_ref().is_empty(),
        };
        if !namespace_matches {
            return false;
        }

        let Some(element) = self.element() else {
            return false;
        };

        let Some(attribute) = element
            .attributes
            .iter()
            .find(|attribute| attribute.name.eq_ignore_ascii_case(local_name.as_ref()))
        else {
            return matches!(operation, AttrSelectorOperation::Exists);
        };

        operation.eval_str(&attribute.value)
    }

    fn match_non_ts_pseudo_class(
        &self,
        pseudo_class: &PdfPseudoClass,
        _context: &mut MatchingContext<'_, PdfSelectorImpl>,
    ) -> bool {
        match pseudo_class {
            PdfPseudoClass::Link | PdfPseudoClass::AnyLink => self
                .element()
                .map(|e| e.attribute("href").is_some())
                .unwrap_or(false),
            PdfPseudoClass::Visited => false,
            PdfPseudoClass::Checked => self
                .element()
                .map(|e| e.attribute("checked").is_some())
                .unwrap_or(false),
            PdfPseudoClass::Disabled => self
                .element()
                .map(|e| e.attribute("disabled").is_some())
                .unwrap_or(false),
            PdfPseudoClass::Enabled => self
                .element()
                .map(|e| e.attribute("disabled").is_none())
                .unwrap_or(false),
            PdfPseudoClass::Required => self
                .element()
                .map(|e| e.attribute("required").is_some())
                .unwrap_or(false),
            PdfPseudoClass::Optional => self
                .element()
                .map(|e| e.attribute("required").is_none())
                .unwrap_or(false),
            PdfPseudoClass::ReadOnly => self
                .element()
                .map(|e| e.attribute("readonly").is_some())
                .unwrap_or(false),
            PdfPseudoClass::ReadWrite => self
                .element()
                .map(|e| e.attribute("readonly").is_none())
                .unwrap_or(false),
            PdfPseudoClass::Hover
            | PdfPseudoClass::Active
            | PdfPseudoClass::Focus
            | PdfPseudoClass::FocusWithin
            | PdfPseudoClass::FocusVisible => false,
        }
    }

    fn match_pseudo_element(
        &self,
        _pseudo_element: &PdfPseudoElement,
        _context: &mut MatchingContext<'_, PdfSelectorImpl>,
    ) -> bool {
        false
    }

    fn apply_selector_flags(&self, _flags: ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        self.element()
            .map(|element| element.attribute("href").is_some())
            .unwrap_or(false)
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn has_id(&self, id: &CssAtom, case_sensitivity: CaseSensitivity) -> bool {
        self.element()
            .and_then(|element| element.id.as_deref())
            .map(|value| case_sensitivity.eq(value.as_bytes(), id.as_ref().as_bytes()))
            .unwrap_or(false)
    }

    fn has_class(&self, class: &CssAtom, case_sensitivity: CaseSensitivity) -> bool {
        self.element()
            .map(|element| {
                element
                    .classes
                    .iter()
                    .any(|value| case_sensitivity.eq(value.as_bytes(), class.as_ref().as_bytes()))
            })
            .unwrap_or(false)
    }

    fn has_custom_state(&self, _name: &CssAtom) -> bool {
        false
    }

    fn imported_part(&self, _name: &CssAtom) -> Option<CssAtom> {
        None
    }

    fn is_part(&self, _name: &CssAtom) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        self.element()
            .map(|element| element.children.is_empty())
            .unwrap_or(true)
    }

    fn is_root(&self) -> bool {
        self.parent_element().is_none()
    }

    fn add_element_unique_hashes(&self, _filter: &mut BloomFilter) -> bool {
        false
    }
}

pub fn matches_selector_list(selectors: &SelectorList, element: &DomElement<'_>) -> bool {
    let mut caches = SelectorCaches::default();
    let mut context = MatchingContext::new(
        MatchingMode::Normal,
        None,
        &mut caches,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::No,
        MatchingForInvalidation::No,
    );

    selectors::matching::matches_selector_list(selectors, element, &mut context)
}

pub fn selector_list_specificity(selectors: &SelectorList) -> u32 {
    selectors
        .slice()
        .iter()
        .map(NativeSelector::specificity)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

use crate::html::types::Element;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
    pub parts: Vec<CompoundSelector>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundSelector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Specificity {
    pub id: u16,
    pub class: u16,
    pub tag: u16,
}

impl Specificity {
    pub const ZERO: Self = Self {
        id: 0,
        class: 0,
        tag: 0,
    };
}

impl Selector {
    pub fn matches(&self, element: &Element, ancestors: &[Element]) -> bool {
        let Some(last) = self.parts.last() else {
            return false;
        };

        if !last.matches(element) {
            return false;
        }

        if self.parts.len() == 1 {
            return true;
        }

        let mut ancestor_index = ancestors.len();

        for part in self.parts[..self.parts.len() - 1].iter().rev() {
            let mut found = false;

            while ancestor_index > 0 {
                ancestor_index -= 1;

                if part.matches(&ancestors[ancestor_index]) {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }

    pub fn specificity(&self) -> Specificity {
        self.parts
            .iter()
            .fold(Specificity::ZERO, |mut total, part| {
                if part.id.is_some() {
                    total.id += 1;
                }

                total.class += part.classes.len() as u16;

                if part.tag.is_some() {
                    total.tag += 1;
                }

                total
            })
    }
}

impl CompoundSelector {
    fn matches(&self, element: &Element) -> bool {
        if let Some(tag) = &self.tag {
            if element.tag_name != *tag {
                return false;
            }
        }

        if let Some(id) = &self.id {
            if element.id.as_deref() != Some(id.as_str()) {
                return false;
            }
        }

        self.classes
            .iter()
            .all(|class| element.classes.iter().any(|item| item == class))
    }
}

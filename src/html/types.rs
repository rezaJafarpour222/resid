#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub tag_name: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Element {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|attribute| attribute.name == name)
            .map(|attribute| attribute.value.as_str())
    }
}

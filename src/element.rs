use std::{collections::HashMap};
use std::str::FromStr;
use quick_xml::{
    Reader,
    Error as QuickXMLError,
    events::Event
};

pub struct AttributeValue {
    pub value: String
}

impl<T: AsRef<str>> From<T> for AttributeValue {
    fn from(str_ref: T) -> Self {
        Self { value: str_ref.as_ref().to_string() }
    }
}

pub struct Element {
    tag: String,
    attributes: HashMap<String, AttributeValue>,
    text: Option<String>,
    children: Vec<Element>
}

impl Element {
    pub fn new(tag: impl AsRef<str>) -> Self {
        Self { 
            tag: tag.as_ref().to_string(), 
            attributes: HashMap::new(), 
            text: None,
            children: Vec::new() 
        }
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn set_tag(&mut self, tag: impl AsRef<str>) {
        self.tag = tag.as_ref().to_string();
    }

    pub fn set_text(&mut self, text: Option<impl AsRef<str>>) {
        self.text = text.map(|v| v.as_ref().to_string());
    }

    pub fn text(&self) -> Option<&str> {
        match &self.text {
            Some(s) => Some(s.as_str()),
            None => None
        }
    }

    pub fn attributes(&self) -> &HashMap<String, AttributeValue>{
        &self.attributes
    }

    pub fn set_attribute(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        self.attributes.insert(key.as_ref().to_string(), AttributeValue::from(value));
    }

    pub fn append_child(&mut self, child: Element) {
        self.children.push(child);
    }

    pub fn pop_child(&mut self) -> Option<Self> {
        self.children.pop()
    }

    pub fn from_reader(mut reader: Reader<&[u8]>) -> Result<Self, QuickXMLError> {
        let mut current_el = Element::new("root");
        let mut element_stack = vec![];

        loop {
            match reader.read_event() {
                Err(e) => return Err(e),
                Ok(Event::Start(e)) => {
                    element_stack.push(current_el);
                    current_el = Element::new(String::from_utf8_lossy(e.local_name().as_ref()));
                    
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            current_el.set_attribute(
                                String::from_utf8_lossy(attr.key.as_ref()), 
                                String::from_utf8_lossy(attr.value.as_ref())
                            );
                        } else {continue};
                    };
                },
                Ok(Event::Text(t)) => {
                    current_el.set_text(Some(String::from_utf8_lossy(t.as_ref())));
                },
                Ok(Event::End(_)) => {
                    let mut parent = element_stack.pop().unwrap();
                    parent.append_child(current_el);
                    current_el = parent;
                },
                Ok(Event::Eof) => {
                    element_stack.clear();
                    return Ok(current_el.pop_child().unwrap())
                },
                _ => continue
            }
        }
    }

    // fn push_child<'a>(&'a mut self, tag: impl AsRef<str>, attrs: HashMap<String, AttributeValue>) -> &'a mut Self {
    //     self.children.push(Self {
    //         tag: tag.as_ref().to_string(), 
    //         attributes: attrs,
    //         text: None,
    //         children: Vec::new()
    //     });
    //     self.children.last_mut().unwrap()
    // }
}

// impl<T: AsRef<str>> TryFrom<T> for Element {
impl FromStr for Element {
    type Err = QuickXMLError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut reader = Reader::from_str(s);
        Self::from_reader(Reader::from_str(s))
    }
}
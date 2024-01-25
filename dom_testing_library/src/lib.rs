use std::ops::Deref; //
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlElement, Node};
/// DomQuery is a trait that is implement on a data structure that holds onto the document for the test.
/// The get_by_X series tries to get exactly one element given the input by method of X
/// They return an `Error::NotFound` result if 0, or a `MoreThanOne` error if more than one.
/// The get_all_by_X series returns a list of 0 or more items given the input by method of X
pub trait DomQuery {
    /// Get the element whose inner text matches this method's input, exactly.
    /// See get_by_text_contains for a non-exact matching method.
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error>;
    /// Get all elements whose inner text matches this method's input, exactly.
    /// Seeget_by_text_contains for a non-exact matching method.
    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get an element whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error>;
    /// Get a list of elements whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get an element by it's id, matches exactly. See get_by_id_contains for non-exact matching.
    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error>;
    /// Get all elements by their id, matches exactly.
    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement>;
    /// Get an element whose id contains the text string, see get_by_id for exact id matching.
    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error>;
    /// Get a list of elements whose id contains the text string, see get_by_id for exact id matching.
    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement>;
    /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, Error>;
    /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement>;
    /// Get an element matching ARIA role.
    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, Error>;
    /// Get a list of elements matching AIRA role.
    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Vec<TestElement>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_by_placeholder_text<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_all_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Vec<TestElement>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_all_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Vec<TestElement>;
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("JsValue:{js_value}")]
    JsValue { js_value: String },
    #[error("Error turning a {input} in a {output}")]
    DynInto {
        input: &'static str,
        output: &'static str,
    },
    #[error("Not Found:Attempting to find: {ident} by method {method}")]
    NotFound { ident: String, method: String },
    #[error("Found more than one element by method of get_{method} with input of {ident}, if you were expecting more than one match see the get_all_{method} version of this method instead.")]
    MoreThanOne { method: String, ident: String },
}

impl Error {
    pub fn is_js_value(&self) -> bool {
        matches!(self, Error::JsValue { .. })
    }
    pub fn is_dyn_into(&self) -> bool {
        matches!(self, Error::DynInto { .. })
    }
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound { .. })
    }
    pub fn is_more_than_one(&self) -> bool {
        matches!(self, Error::MoreThanOne { .. })
    }
    /// Panics if JsValue can't be cast to a string, or isn't a string? See JsValue::as_string() for details.
    fn _handle_js_val(js_val: JsValue) -> Self {
        Self::JsValue {
            js_value: js_val.as_string().unwrap(),
        }
    }
    fn dyn_into(input: &'static str, output: &'static str) -> Self {
        Self::DynInto { input, output }
    }
    fn node_to_html_element() -> Self {
        Self::dyn_into(stringify!(Node), stringify!(HtmlElment))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestElement(pub HtmlElement);

impl TestElement {
    /// Click outside of an element. This will try to turn the parent node of the element into an HtmlElement and click on it. If the parent
    /// node isn't an HtmlElement this will panic.
    pub fn click_outside(&self) {
        self.0
            .parent_element()
            .expect("click_outside to be called on an element with a parent.")
            .unchecked_into::<HtmlElement>()
            .click()
    }

    /// Gets the html string of the element.
    pub fn as_html_string(&self) -> String {
        self.0.outer_html()
    }
}

impl From<HtmlElement> for TestElement {
    fn from(value: HtmlElement) -> Self {
        Self(value)
    }
}

impl Deref for TestElement {
    type Target = web_sys::HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct DocumentWrapper<'a>(pub &'a Document);
impl DocumentWrapper<'_> {
    pub fn body_string(&self) -> String {
        self.0.body().unwrap().outer_html()
    }
}
impl Deref for DocumentWrapper<'_> {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl DomQuery for DocumentWrapper<'_> {
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_text(text.as_ref());
        if elements.len() > 1 {
            return Err(Error::MoreThanOne {
                method: "by_text".to_string(),
                ident: text.as_ref().to_string(),
            });
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: text.as_ref().to_string(),
                method: "by_text".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }
    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        get_all_text_nodes(self.0)
            .find_parents_of_matching_text(text)
            .into_iter()
            .map(TestElement)
            .collect()
    }

    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_text_contains(text.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_text_contains".to_string(),
                ident: text.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: text.as_ref().to_string(),
                method: "by_text_contains".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        get_all_text_nodes(self.0)
            .find_parents_of_containing_text(text)
            .into_iter()
            .map(TestElement)
            .collect()
    }

    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_id(id.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_id".to_string(),
                ident: id.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: id.as_ref().to_string(),
                method: "by_id".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }
    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if item.id() == id.as_ref() {
                if let Ok(item) = item
                    .dyn_into::<HtmlElement>()
                    .map_err(|_| Error::node_to_html_element())
                {
                    list.push(item.into())
                }
            }
        }
        list
    }

    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_id_contains(id.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_id_contains".to_string(),
                ident: id.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: id.as_ref().to_string(),
                method: "by_id_contains".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if item.id().contains(id.as_ref()) {
                if let Ok(item) = item
                    .dyn_into::<HtmlElement>()
                    .map_err(|_| Error::dyn_into(stringify!(Element), stringify!(HtmlElment)))
                {
                    list.push(item.into());
                }
            }
        }
        list
    }

    fn get_by_label_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_label_text(text.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_label_text".to_string(),
                ident: text.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: text.as_ref().to_string(),
                method: "by_label_text".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_label_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        let mut list = Vec::new();
        let html_fors = self
            .get_all_by_text(text)
            .into_iter()
            .map(|e| {
                e.0.dyn_into::<web_sys::HtmlLabelElement>()
                    .map_err(|_| {
                        Error::dyn_into(stringify!(HtmlElement), stringify!(HtmlLabelElement))
                    })
                    .map(|label| label.html_for())
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        for html_for in html_fors {
            list.push(self.get_by_id(&html_for).unwrap())
        }
        list
    }

    fn get_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_label_text_contains(text.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_label_text".to_string(),
                ident: text.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: text.as_ref().to_string(),
                method: "by_label_text".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        let mut list = Vec::new();
        let html_fors = self
            .get_all_by_text_contains(text)
            .into_iter()
            .map(|e| {
                e.0.dyn_into::<web_sys::HtmlLabelElement>()
                    .map_err(|_| {
                        Error::dyn_into(stringify!(HtmlElement), stringify!(HtmlLabelElement))
                    })
                    .map(|label| label.html_for())
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        for html_for in html_fors {
            list.push(self.get_by_id(&html_for).unwrap())
        }
        list
    }

    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_display_value(value.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_display_value".to_string(),
                ident: value.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: value.as_ref().to_string(),
                method: "by_display_value".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement> {
        let elements = self
            .0
            .query_selector_all("input, textarea, select")
            .unwrap();
        let mut list = Vec::new();
        for i in 0..elements.length() {
            let item = elements
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .map_err(|_| Error::dyn_into(stringify!(Element), stringify!(HtmlElment)))
                .unwrap();
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.value() == value.as_ref() {
                    list.push(TestElement(item));
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.value() == value.as_ref() {
                    list.push(TestElement(item));
                }
            } else if let Ok(item) = item.dyn_into::<web_sys::HtmlSelectElement>() {
                if item.value() == value.as_ref() {
                    list.push(TestElement(item.into()));
                }
            }
        }
        list
    }

    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, Error> {
        let elements = self.get_all_by_role(role.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_role".to_string(),
                ident: role.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: role.as_ref().to_string(),
                method: "by_role".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Vec<TestElement> {
        let selector = format!("[role='{}']", role.as_ref());
        let elements = self.0.query_selector_all(&selector).unwrap();
        let mut list = Vec::new();
        for i in 0..elements.length() {
            list.push(
                elements
                    .item(i)
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .map_err(|_| Error::dyn_into(stringify!(Element), stringify!(HtmlElment)))
                    .unwrap()
                    .into(),
            )
        }
        list
    }

    fn get_by_placeholder_text<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error> {
        let elements = self.get_all_by_placeholder_text(placeholder_text.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_role".to_string(),
                ident: placeholder_text.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: placeholder_text.as_ref().to_string(),
                method: "by_role".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Vec<TestElement> {
        let elements = self.0.get_elements_by_tag_name("input, textarea");
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .map_err(|_| Error::dyn_into(stringify!(Element), stringify!(HtmlElment)))
                .unwrap();
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.placeholder() == placeholder_text.as_ref() {
                    list.push(TestElement(item));
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.placeholder() == placeholder_text.as_ref() {
                    list.push(TestElement(item));
                }
            }
        }
        list
    }

    fn get_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error> {
        let elements = self.get_all_by_placeholder_text_contains(placeholder_text.as_ref());
        if elements.len() > 1 {
            Err(Error::MoreThanOne {
                method: "by_role".to_string(),
                ident: placeholder_text.as_ref().to_string(),
            })
        } else if elements.is_empty() {
            Err(Error::NotFound {
                ident: placeholder_text.as_ref().to_string(),
                method: "by_role".to_string(),
            })
        } else {
            Ok(elements.first().cloned().unwrap())
        }
    }

    fn get_all_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Vec<TestElement> {
        let elements = self.0.get_elements_by_tag_name("input, textarea");
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .map_err(|_| Error::dyn_into(stringify!(Element), stringify!(HtmlElment)))
                .unwrap();

            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.placeholder().contains(placeholder_text.as_ref()) {
                    list.push(TestElement(item));
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.placeholder().contains(placeholder_text.as_ref()) {
                    list.push(TestElement(item));
                }
            }
        }
        list
    }
}

fn find_all_text_nodes(node: &Node, text_nodes: &mut Vec<Node>) {
    if node.node_type() == Node::TEXT_NODE {
        text_nodes.push(node.clone());
    } else if node.node_type() == Node::ELEMENT_NODE {
        let list = node.child_nodes();
        (0..list.length()).for_each(|i| {
            let child = list.get(i).unwrap();
            find_all_text_nodes(&child, text_nodes);
        });
    }
}

fn get_all_text_nodes(document: &Document) -> TextNodes {
    let mut text_nodes = Vec::new();
    let body = document.body().expect("Document should have a body");

    find_all_text_nodes(&body.into(), &mut text_nodes);
    TextNodes(text_nodes)
}
struct TextNodes(Vec<Node>);
impl TextNodes {
    fn find_parents_of_matching_text<S: AsRef<str>>(&self, text: S) -> Vec<HtmlElement> {
        let mut list = vec![];
        for node in self.0.iter() {
            if let Some(element) = node.parent_element() {
                if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
                    if html_element.inner_text() == text.as_ref() {
                        list.push(html_element.clone());
                    }
                }
            }
        }
        list
    }
    fn find_parents_of_containing_text<S: AsRef<str>>(&self, text: S) -> Vec<HtmlElement> {
        let mut list = vec![];
        for node in self.0.iter() {
            if node
                .text_content()
                .expect("Text node to have text content,")
                .contains(text.as_ref())
            {
                if let Some(element) = node.parent_element() {
                    if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
                        if html_element.inner_text().contains(text.as_ref()) {
                            list.push(html_element.clone());
                        }
                    }
                }
            }
        }
        list
    }
}
pub trait HoldsDocument {
    fn document(&self) -> DocumentWrapper;
}

impl<T> DomQuery for T
where
    T: HoldsDocument,
{
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_text(text)
    }

    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.document().get_all_by_text(text)
    }

    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_text_contains(text)
    }

    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.document().get_all_by_text_contains(text)
    }

    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        self.document().get_by_id(id)
    }

    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        self.document().get_all_by_id(id)
    }

    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        self.document().get_by_id_contains(id)
    }

    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        self.document().get_all_by_id_contains(id)
    }

    fn get_by_label_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_label_text(text)
    }

    fn get_all_by_label_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.document().get_all_by_label_text(text)
    }

    fn get_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_label_text_contains(text)
    }

    fn get_all_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.document().get_all_by_label_text_contains(text)
    }

    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, Error> {
        self.document().get_by_display_value(value)
    }

    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement> {
        self.document().get_all_by_display_value(value)
    }

    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, Error> {
        self.document().get_by_role(role)
    }

    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Vec<TestElement> {
        self.document().get_all_by_role(role)
    }

    fn get_by_placeholder_text<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error> {
        self.document().get_by_placeholder_text(placeholder_text)
    }

    fn get_all_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Vec<TestElement> {
        self.document()
            .get_all_by_placeholder_text(placeholder_text)
    }

    fn get_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Result<TestElement, Error> {
        self.document()
            .get_by_placeholder_text_contains(placeholder_text)
    }

    fn get_all_by_placeholder_text_contains<S: AsRef<str>>(
        &self,
        placeholder_text: S,
    ) -> Vec<TestElement> {
        self.document()
            .get_all_by_placeholder_text_contains(placeholder_text)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    /*
    #[wasm_bindgen_test]
    pub fn text_nodes() {
        let document = web_sys::window().unwrap().document().unwrap();
        let document = DocumentWrapper(&document);
        /*  let div = document.create_element("div").unwrap();
        div.clone()
            .unchecked_into::<HtmlElement>()
            .set_inner_text("babo");
        document
            .body()
            .unwrap()
            .append_child(div.dyn_ref::<Node>().unwrap())
            .unwrap();*/
        let text_nodes = get_all_text_nodes(&document);
         panic!("{}",text_nodes.0.into_iter().filter_map(|n|if n.text_content().unwrap().contains("hello"){
            Some(n.text_content().unwrap())
        }else {None}).collect::<Vec<String>>().join("\nSEP\n"));
    }*/

    #[wasm_bindgen_test]
    pub fn find_parents_of_matching_text() {
        let document = web_sys::window().unwrap().document().unwrap();
        let document = DocumentWrapper(&document);
        // create test wrapper
        let wrapper: web_sys::Element = document.create_element("div").unwrap();
        let div = document.create_element("div").unwrap();
        div.clone()
            .unchecked_into::<HtmlElement>()
            .set_inner_text("hello");
        wrapper.append_child(&div.into()).unwrap();
        document
            .body()
            .unwrap()
            .append_child(&wrapper.clone().into())
            .unwrap();
        let text_nodes = get_all_text_nodes(&document);
        let results = text_nodes.find_parents_of_matching_text("hello");
        if results.len() != 1 {
            panic!(
                "{}",
                results
                    .into_iter()
                    .map(|n| n.inner_html())
                    .collect::<Vec<String>>()
                    .join("\nSEP\n")
            )
        }
    }
    #[wasm_bindgen_test]
    pub fn find_parents_of_containing_text() {
        let document = web_sys::window().unwrap().document().unwrap();
        let document = DocumentWrapper(&document);
        // create test wrapper
        let wrapper: web_sys::Element = document.create_element("div").unwrap();
        let div = document.create_element("div").unwrap();
        div.clone()
            .unchecked_into::<HtmlElement>()
            .set_inner_text("other");
        wrapper.append_child(&div.into()).unwrap();
        document
            .body()
            .unwrap()
            .append_child(&wrapper.into())
            .unwrap();
        let text_nodes = get_all_text_nodes(&document);
        let results = text_nodes.find_parents_of_containing_text("other");
        if results.len() != 1 {
            panic!(
                "{}",
                results
                    .into_iter()
                    .map(|n| n.inner_html())
                    .collect::<Vec<String>>()
                    .join("\nSEP\n")
            )
        }
    }
}

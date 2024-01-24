use thiserror::Error;
use std::ops::{Deref};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement,Document};
/// DomQuery is a trait that is implement on a data structure that holds onto the document for the test.
/// The get_by_X series tries to get exactly one element given the input by method of X
/// They return an `Error::NotFound` result if 0, or a `MoreThanOne` error if more than one.
/// The get_all_by_X series returns a list of 0 or more items given the input by method of X
pub trait DomQuery{
    /// Get the element whose inner text matches this method's input, exactly.
    /// See get_by_text_contains for a non-exact matching method.
    fn get_by_text<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error>;
    /// Get all elements whose inner text matches this method's input, exactly.
    /// Seeget_by_text_contains for a non-exact matching method.
    fn get_all_by_text<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error>;
    /// Get an element whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_by_text_contains<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error>;
    /// Get a list of elements whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_all_by_text_contains<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error>;
    /// Get an element by it's id, matches exactly. See get_by_id_contains for non-exact matching.
    fn get_by_id<S:AsRef<str>>(&self,id:S) -> Result<TestElement,Error>;
    /// Get all elements by their id, matches exactly.
    fn get_all_by_id<S:AsRef<str>>(&self,id:S) -> Result<Vec<TestElement>,Error>;
    /// Get an element whose id contains the text string, see get_by_id for exact id matching.
    fn get_by_id_contains<S:AsRef<str>>(&self,id:S) -> Result<TestElement,Error>;
    /// Get a list of elements whose id contains the text string, see get_by_id for exact id matching.
    fn get_all_by_id_contains<S:AsRef<str>>(&self,id:S) -> Result<Vec<TestElement>,Error>;
    /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/> 
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label_text<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/> 
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label_text<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error>;
     /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/> 
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label_text_contains<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/> 
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label_text_contains<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error>;
    /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_by_display_value<S:AsRef<str>>(&self,value:S) -> Result<TestElement,Error>;
     /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_all_by_display_value<S:AsRef<str>>(&self,value:S) -> Result<Vec<TestElement>,Error>;
    /// Get an element matching ARIA role.
    fn get_by_role<S:AsRef<str>>(&self,role:S) -> Result<TestElement,Error>;
    /// Get a list of elements matching AIRA role.
    fn get_all_by_role<S:AsRef<str>>(&self,role:S) -> Result<Vec<TestElement>,Error>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_by_placeholder_text<S:AsRef<str>>(&self,placeholder_text:S) -> Result<TestElement,Error>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_all_by_placeholder_text<S:AsRef<str>>(&self,placeholder_text:S) -> Result<Vec<TestElement>,Error>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_by_placeholder_text_contains<S:AsRef<str>>(&self,placeholder_text:S) -> Result<TestElement,Error>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_all_by_placeholder_text_contains<S:AsRef<str>>(&self,placeholder_text:S) -> Result<Vec<TestElement>,Error>;
}



#[derive(Error, Debug,PartialEq)]
pub enum Error{
    #[error("JsValue:{js_value}")]
    JsValue{js_value:String},
    #[error("Error turning a {input} in a {output}")]
    DynInto {
        input: String,
        output: String,
    },
    #[error("Not Found:Attempting to find: {ident} by method {method}")]
    NotFound{
        ident:String,
        method:String
    },
    #[error("Found more than one element by method of get_{method} with input of {ident}, if you were expecting more than one match see the get_all_{method} version of this method instead.")]
    MoreThanOne{method:String,ident:String},
}

#[derive(Clone,Debug,PartialEq)]
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
}

impl From<HtmlElement> for TestElement{
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
impl Deref for DocumentWrapper<'_> {
    type Target = Document;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl DomQuery for DocumentWrapper<'_>{

    fn get_by_text<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_text(text.as_ref())?;
        if elements.len() > 1 {
            return Err(Error::MoreThanOne { method: "by_text".to_string(), ident: text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:text.as_ref().to_string(),method:"by_text".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }

    }
    fn get_all_by_text<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut result = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .map_err(|_|Error::DynInto{input:stringify!(Node).to_string(),output:stringify!(HtmlElment).to_string()})?;
            if &item.inner_text() == text.as_ref() {
                result.push(
                    TestElement(item)
                );
            }
        }
        Ok(result)
    }

    fn get_by_text_contains<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_text_contains(text.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_text_contains".to_string(), ident: text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:text.as_ref().to_string(),method:"by_text_contains".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }

    fn get_all_by_text_contains<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut result = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names
                .item(i)
                .unwrap()
                .dyn_into::<HtmlElement>()
                .map_err(|_|Error::DynInto{input:stringify!(Node).to_string(),output:stringify!(HtmlElment).to_string()})?;
            if item.inner_text().contains(text.as_ref()) {
                result.push(
                    TestElement(item)
                );
            }
        }
        Ok(result)
    }

    fn get_by_id<S:AsRef<str>>(&self,id:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_id(id.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_id".to_string(), ident: id.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:id.as_ref().to_string(),method:"by_id".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }

    }
    fn get_all_by_id<S:AsRef<str>>(&self,id:S) -> Result<Vec<TestElement>,Error> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if &item.id() == id.as_ref() {
                list.push(
                    item
                        .dyn_into::<HtmlElement>()
                        .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?
                        .into()
                )
            }
        }
        Ok(list)
    }

    fn get_by_id_contains<S:AsRef<str>>(&self,id:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_id_contains(id.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_id_contains".to_string(), ident: id.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:id.as_ref().to_string(),method:"by_id_contains".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }


    fn get_all_by_id_contains<S:AsRef<str>>(&self,id:S) -> Result<Vec<TestElement>,Error> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if item.id().contains(id.as_ref()) {
                list.push(
                    item
                        .dyn_into::<HtmlElement>()
                        .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?
                        .into()
                )
            }
        }
        Ok(list)
    }

    
    fn get_by_label_text<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_label_text(text.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_label_text".to_string(), ident: text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:text.as_ref().to_string(),method:"by_label_text".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }

    fn get_all_by_label_text<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error> {
        let mut list = Vec::new();
        let html_fors = self.get_all_by_text(text)?
            .into_iter()
            .map(|e| {
                e.0.dyn_into::<web_sys::HtmlLabelElement>()
                    .map_err(|_| Error::DynInto { input: stringify!(HtmlElement).to_string(), output: stringify!(HtmlLabelElement).to_string() })
                    .and_then(|label| Ok(label.html_for()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        for html_for in html_fors {
            list.push(self.get_by_id(&html_for)?)
        }
        Ok(list)
    }

    fn get_by_label_text_contains<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_label_text_contains(text.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_label_text".to_string(), ident: text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:text.as_ref().to_string(),method:"by_label_text".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }

fn get_all_by_label_text_contains<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error> {
    let mut list = Vec::new();
    let html_fors = self.get_all_by_text_contains(text)?
           .into_iter()
           .map(|e| {
            e.0.dyn_into::<web_sys::HtmlLabelElement>()
                .map_err(|_| Error::DynInto { input: stringify!(HtmlElement).to_string(), output: stringify!(HtmlLabelElement).to_string() })
                .and_then(|label| Ok(label.html_for()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    for html_for in html_fors {
        list.push(self.get_by_id(&html_for)?)
    }
    Ok(list)
    }

    fn get_by_display_value<S:AsRef<str>>(&self,value:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_display_value(value.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_display_value".to_string(), ident: value.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:value.as_ref().to_string(),method:"by_display_value".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
     }   

    fn get_all_by_display_value<S:AsRef<str>>(&self,value:S) -> Result<Vec<TestElement>,Error> {
        let elements = self.0.query_selector_all("input, textarea, select").unwrap();
        let mut list = Vec::new();
        for i in 0..elements.length() {
            let item = elements.item(i)
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?;
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if &ref_item.value() == value.as_ref() {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if &ref_item.value() == value.as_ref() {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            } else if let Ok(item) = item.dyn_into::<web_sys::HtmlSelectElement>() {
                if &item.value() == value.as_ref() {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            }
        }
        Ok(list)
    }

    fn get_by_role<S:AsRef<str>>(&self,role:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_role(role.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_role".to_string(), ident: role.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:role.as_ref().to_string(),method:"by_role".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }


    fn get_all_by_role<S:AsRef<str>>(&self,role:S) -> Result<Vec<TestElement>,Error> {
        let selector = format!("[role='{}']", role.as_ref());
        let elements = self.0.query_selector_all(&selector).unwrap();
        let mut list = Vec::new();
        for i in 0..elements.length() {
            list.push(
                elements.item(i)
                    .unwrap()
                    .dyn_into::<HtmlElement>()
                    .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?
                    .into()
            )
          
        }
        Ok(list)
    }

    fn get_by_placeholder_text<S:AsRef<str>>(&self,placeholder_text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_placeholder_text(placeholder_text.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_role".to_string(), ident: placeholder_text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:placeholder_text.as_ref().to_string(),method:"by_role".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }

    fn get_all_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Result<Vec<TestElement>, Error> {
        let elements = self.0.get_elements_by_tag_name("input, textarea");
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements.item(i)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?;
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if &ref_item.placeholder() == placeholder_text.as_ref() {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if &ref_item.placeholder() == placeholder_text.as_ref() {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            }
        }
        Ok(list)
    }



    fn get_by_placeholder_text_contains<S:AsRef<str>>(&self,placeholder_text:S) -> Result<TestElement,Error> {
        let elements = self.get_all_by_placeholder_text_contains(placeholder_text.as_ref())?;
        if elements.len() > 1 {
            Err(Error::MoreThanOne { method: "by_role".to_string(), ident: placeholder_text.as_ref().to_string() })
        } else if elements.len() == 0 {
            Err(Error::NotFound{ident:placeholder_text.as_ref().to_string(),method:"by_role".to_string()})
        } else {
            Ok(elements.get(0).cloned().unwrap())
        }
    }

    fn get_all_by_placeholder_text_contains<S:AsRef<str>>(&self,placeholder_text:S) -> Result<Vec<TestElement>,Error> {
        let elements = self.0.get_elements_by_tag_name("input, textarea");
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements.item(i)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .map_err(|_|Error::DynInto{input:stringify!(Element).to_string(),output:stringify!(HtmlElment).to_string()})?;
         
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.placeholder().contains(placeholder_text.as_ref()) {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.placeholder().contains(placeholder_text.as_ref()) {
                    list.push(
                        TestElement(
                            item.into()
                        )
                    );
                }
            }
        }
        Ok(list)
        }
}

pub trait HoldsDocument{
    fn document(&self) -> DocumentWrapper;
}

impl<T> DomQuery for T where T:HoldsDocument {
    fn get_by_text<S:AsRef<str>>(&self,text:S) -> Result<TestElement,Error> {
        self.document().get_by_text(text)
    }

    fn get_all_by_text<S:AsRef<str>>(&self,text:S) -> Result<Vec<TestElement>,Error> {
        self.document().get_all_by_text(text)

    }

    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_text_contains(text)
    }

    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_text_contains(text)
    }

    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        self.document().get_by_id(id)
    }

    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_id(id)
    }

    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, Error> {
        self.document().get_by_id_contains(id)
    }

    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_id_contains(id)
    }

    fn get_by_label_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_label_text(text)
    }

    fn get_all_by_label_text<S: AsRef<str>>(&self, text: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_label_text(text)
    }

    fn get_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, Error> {
        self.document().get_by_label_text_contains(text)
    }

    fn get_all_by_label_text_contains<S: AsRef<str>>(&self, text: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_label_text_contains(text)
    }

    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, Error> {
        self.document().get_by_display_value(value)
    }

    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_display_value(value)
    }

    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, Error> {
        self.document().get_by_role(role)
    }

    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_role(role)
    }

    fn get_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Result<TestElement, Error> {
        self.document().get_by_placeholder_text(placeholder_text)
    }

    fn get_all_by_placeholder_text<S: AsRef<str>>(&self, placeholder_text: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_placeholder_text(placeholder_text)
    }

    fn get_by_placeholder_text_contains<S: AsRef<str>>(&self, placeholder_text: S) -> Result<TestElement, Error> {
        self.document().get_by_placeholder_text_contains(placeholder_text)
    }

    fn get_all_by_placeholder_text_contains<S: AsRef<str>>(&self, placeholder_text: S) -> Result<Vec<TestElement>, Error> {
        self.document().get_all_by_placeholder_text_contains(placeholder_text)
    }
}



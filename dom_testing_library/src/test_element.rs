use super::*;

/// TestElement is a wrapper around web_sys::HtmlElement, we supply some helper functions while implementing Deref<HtmlElement>
/// so you can use any web_sys::HtmlElement method on TestElement.
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
    /// Gets the text the user would see. There might be multiple nodes, this will display all text
    pub fn display_text(&self) -> String {
        get_all_text_nodes(self.dyn_ref::<Node>().unwrap()).join_text(String::new())
    }
    ///Tries to parse the text context of an element
    pub fn parse<F>(&self) -> Result<F, <F as std::str::FromStr>::Err>
    where
        F: std::str::FromStr,
    {
        self.display_text().parse::<F>()
    }
    /// If the element is a Input, TextArea or Select Element this will set the display value. Otherwise it will panic.
    pub fn set_display_value<S: AsRef<str>>(&self, value: S) {
        if let Some(item) = self.dyn_ref::<web_sys::HtmlTextAreaElement>() {
            item.set_value(value.as_ref());
        } else if let Some(item) = self.dyn_ref::<web_sys::HtmlInputElement>() {
            item.set_value(value.as_ref());
        } else if let Some(item) = self.dyn_ref::<web_sys::HtmlSelectElement>() {
            item.set_value(value.as_ref());
        } else {
            panic!(
                "Expecting an input, textarea, or select element to set display value.
             If you want to set a non-display value just use .set_value() transparently."
            )
        }
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

use super::*;

/// If a struct holds onto an element it can automatically implement DomQuery.
/// ElementWrapper is just the wrapper around web_sys::Element
/// So this could be implemented as follows
/// ```rust
/// pub struct ServerFrontEndStruct{
///     elem:web_sys::Element,
/// }
/// impl ServerFrontEndStruct{
///     fn new() -> Self {
///         let document = web_sys::window().unwrap().document().unwrap();
///         Self{elem:document.body().unwrap().unchecked_into::<HtmlElement>()}    
///     }
/// }
/// impl HoldsElement for ServerFrontEndStruct{
///     fn element(&self) -> ElementWrapper{
///         ElementWrapper(&self.document)
///     }
/// }
/// ```
/// And now you can use DomQuery on ServerFrontEndStruct.
pub trait HoldsElement {
    fn element(&self) -> ElementWrapper;
}

/// DomQuery is a trait that is implement on a data structure that holds onto the document for the test.
/// The get_by_X series tries to get exactly one element given the input by method of X
/// They return an `Error::NotFound` result if 0, or a `MoreThanOne` error if more than one.
/// The get_all_by_X series returns a list of 0 or more items given the input by method of ˙˙˙
pub trait DomQuery {
    /// Get the element whose inner text matches this method's input, exactly.
    /// See get_by_text_contains for a non-exact matching method.
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError>;
    /// Get all elements whose inner text matches this method's input, exactly.
    /// Seeget_by_text_contains for a non-exact matching method.
    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get an element whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError>;
    /// Get a list of elements whose inner text contains the text content, i.e "abc" contains "a".
    /// Seeget_by_text for an exact matcher.
    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get an element by it's id, matches exactly. See get_by_id_contains for non-exact matching.
    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError>;
    /// Get all elements by their id, matches exactly.
    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement>;
    /// Get an element whose id contains the text string, see get_by_id for exact id matching.
    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError>;
    /// Get a list of elements whose id contains the text string, see get_by_id for exact id matching.
    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement>;
    /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get the element that is pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_by_label_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError>;
    /// Get a list of  elements that are pointed to by a label whose text is the input of the method.
    /// i.e <label for="field">Btn</label><input id="field"/>
    /// With input of "field" would return the input whose id is field.
    /// If you want to find the label element itself, see get_by_text
    fn get_all_by_label_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement>;
    /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, GetOneError>;
    /// Get elements whose display value is the exact match of this methods input.
    /// The elements that this method will find are: input, textarea, and select.
    /// This method will not match against items with non-display value attributes, i.e option, progress, li etc.
    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement>;
    /// Get an element matching ARIA role.
    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, GetOneError>;
    /// Get a list of elements matching AIRA role.
    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Vec<TestElement>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_by_placeholder<S: AsRef<str>>(&self, placeholder: S)
        -> Result<TestElement, GetOneError>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Matches exactly.
    fn get_all_by_placeholder<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement>;
    /// Get by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_by_placeholder_contains<S: AsRef<str>>(
        &self,
        placeholder: S,
    ) -> Result<TestElement, GetOneError>;
    /// Get a list of elements by placeholder text, checks textarea and input only. As those are the only applicable elements with placeholders.
    /// Checks if placeholder text contains method input.
    fn get_all_by_placeholder_contains<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement>;
}

impl<T> DomQuery for T
where
    T: HoldsElement,
{
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_text(text)
    }

    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.element().get_all_by_text(text)
    }

    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_text_contains(text)
    }

    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.element().get_all_by_text_contains(text)
    }

    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_id(id)
    }

    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        self.element().get_all_by_id(id)
    }

    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_id_contains(id)
    }

    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        self.element().get_all_by_id_contains(id)
    }

    fn get_by_label<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_label(text)
    }

    fn get_all_by_label<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.element().get_all_by_label(text)
    }

    fn get_by_label_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_label_contains(text)
    }

    fn get_all_by_label_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        self.element().get_all_by_label_contains(text)
    }

    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_display_value(value)
    }

    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement> {
        self.element().get_all_by_display_value(value)
    }

    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, GetOneError> {
        self.element().get_by_role(role)
    }

    fn get_all_by_role<S: AsRef<str>>(&self, role: S) -> Vec<TestElement> {
        self.element().get_all_by_role(role)
    }

    fn get_by_placeholder<S: AsRef<str>>(
        &self,
        placeholder: S,
    ) -> Result<TestElement, GetOneError> {
        self.element().get_by_placeholder(placeholder)
    }

    fn get_all_by_placeholder<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement> {
        self.element().get_all_by_placeholder(placeholder)
    }

    fn get_by_placeholder_contains<S: AsRef<str>>(
        &self,
        placeholder: S,
    ) -> Result<TestElement, GetOneError> {
        self.element().get_by_placeholder_contains(placeholder)
    }

    fn get_all_by_placeholder_contains<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement> {
        self.element().get_all_by_placeholder_contains(placeholder)
    }
}

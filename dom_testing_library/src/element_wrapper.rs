use super::*;

/// DocumentWrapper is wrapper struct around web_sys::Document. We use to allow effortless implementation of DomQuery trait
/// on any struct that holds onto a web_sys::Document
pub struct ElementWrapper<'a>(pub &'a Element);

impl Deref for ElementWrapper<'_> {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DomQuery for ElementWrapper<'_> {
    fn get_by_text<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        get_one_inner(self.get_all_by_text(text.as_ref()), "by_text", text)
    }
    fn get_all_by_text<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        get_all_text_nodes(self.0)
            .find_parents_of_matching_text(text)
            .into_iter()
            .map(TestElement)
            .collect()
    }

    fn get_by_text_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_text_contains(text.as_ref()),
            "by_text_contains",
            text,
        )
    }

    fn get_all_by_text_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        get_all_text_nodes(self.0)
            .find_parents_of_containing_text(text)
            .into_iter()
            .map(TestElement)
            .collect()
    }

    fn get_by_id<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError> {
        get_one_inner(self.get_all_by_id(id.as_ref()), "by_id", id)
    }
    fn get_all_by_id<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if item.id() == id.as_ref() {
                if let Ok(item) = item.dyn_into::<HtmlElement>() {
                    list.push(item.into())
                }
            }
        }
        list
    }

    fn get_by_id_contains<S: AsRef<str>>(&self, id: S) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_id_contains(id.as_ref()),
            "by_id_contains",
            id,
        )
    }

    fn get_all_by_id_contains<S: AsRef<str>>(&self, id: S) -> Vec<TestElement> {
        let tag_names = self.0.get_elements_by_tag_name("*");
        let mut list = Vec::new();
        for i in 0..tag_names.length() {
            let item = tag_names.item(i).unwrap();
            if item.id().contains(id.as_ref()) {
                if let Ok(item) = item.dyn_into::<HtmlElement>() {
                    list.push(item.into());
                }
            }
        }
        list
    }

    fn get_by_label<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        get_one_inner(self.get_all_by_label(text.as_ref()), "by_label", text)
    }

    fn get_all_by_label<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        let mut list = Vec::new();
        let html_fors = self
            .get_all_by_text(text)
            .into_iter()
            .map(|e| {
                e.0.dyn_into::<web_sys::HtmlLabelElement>()
                    .map(|label| label.html_for())
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        for html_for in html_fors {
            list.push(self.get_by_id(&html_for).unwrap())
        }
        list
    }

    fn get_by_label_contains<S: AsRef<str>>(&self, text: S) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_label_contains(text.as_ref()),
            "by_label",
            text,
        )
    }

    fn get_all_by_label_contains<S: AsRef<str>>(&self, text: S) -> Vec<TestElement> {
        let mut list = Vec::new();
        let html_fors = self
            .get_all_by_text_contains(text)
            .into_iter()
            .map(|e| {
                e.0.dyn_into::<web_sys::HtmlLabelElement>()
                    .map(|label| label.html_for())
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        for html_for in html_fors {
            list.push(self.get_by_id(html_for.as_str()).unwrap())
        }
        list
    }

    fn get_by_display_value<S: AsRef<str>>(&self, value: S) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_display_value(value.as_ref()),
            "by_display_value",
            value,
        )
    }

    fn get_all_by_display_value<S: AsRef<str>>(&self, value: S) -> Vec<TestElement> {
        let elements = self
            .0
            .query_selector_all("input, textarea, select")
            .unwrap();
        let mut list = Vec::new();
        for i in 0..elements.length() {
            let item = elements.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
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

    fn get_by_role<S: AsRef<str>>(&self, role: S) -> Result<TestElement, GetOneError> {
        get_one_inner(self.get_all_by_role(role.as_ref()), "by_role", role)
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
                    .unwrap()
                    .into(),
            )
        }
        list
    }

    fn get_by_placeholder<S: AsRef<str>>(
        &self,
        placeholder: S,
    ) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_placeholder(placeholder.as_ref()),
            "by_placeholder",
            placeholder,
        )
    }

    fn get_all_by_placeholder<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement> {
        let elements = self.0.query_selector_all("input, textarea").unwrap();
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();
            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.placeholder() == placeholder.as_ref() {
                    list.push(TestElement(item));
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.placeholder() == placeholder.as_ref() {
                    list.push(TestElement(item));
                }
            }
        }
        list
    }

    fn get_by_placeholder_contains<S: AsRef<str>>(
        &self,
        placeholder: S,
    ) -> Result<TestElement, GetOneError> {
        get_one_inner(
            self.get_all_by_placeholder_contains(placeholder.as_ref()),
            "by_placeholder",
            placeholder,
        )
    }

    fn get_all_by_placeholder_contains<S: AsRef<str>>(&self, placeholder: S) -> Vec<TestElement> {
        let elements = self.0.query_selector_all("input, textarea").unwrap();
        let mut list = Vec::new();

        for i in 0..elements.length() {
            let item = elements.item(i).unwrap().dyn_into::<HtmlElement>().unwrap();

            if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlTextAreaElement>() {
                if ref_item.placeholder().contains(placeholder.as_ref()) {
                    list.push(TestElement(item));
                }
            } else if let Some(ref_item) = item.dyn_ref::<web_sys::HtmlInputElement>() {
                if ref_item.placeholder().contains(placeholder.as_ref()) {
                    list.push(TestElement(item));
                }
            }
        }
        list
    }
}

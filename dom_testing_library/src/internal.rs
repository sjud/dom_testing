use super::*;

pub(crate) fn get_one_inner<S: AsRef<str>>(
    list: Vec<TestElement>,
    method: &'static str,
    ident: S,
) -> Result<TestElement, GetOneError> {
    if list.len() > 1 {
        Err(GetOneError::more_than_one(
            method,
            ident.as_ref().to_string(),
        ))
    } else if list.is_empty() {
        Err(GetOneError::not_found(method, ident.as_ref().to_string()))
    } else {
        Ok(list.first().cloned().unwrap())
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

pub(crate) fn get_all_text_nodes(node: &Node) -> TextNodes {
    let mut text_nodes = Vec::new();
    find_all_text_nodes(node, &mut text_nodes);
    TextNodes(text_nodes)
}
pub(crate) struct TextNodes(Vec<Node>);
impl TextNodes {
    pub(crate) fn join_text(&self, join_on: String) -> String {
        self.0
            .iter()
            .map(|node| node.text_content().unwrap())
            .collect::<Vec<String>>()
            .join(&join_on)
    }
    pub(crate) fn find_parents_of_matching_text<S: AsRef<str>>(&self, text: S) -> Vec<HtmlElement> {
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
    pub(crate) fn find_parents_of_containing_text<S: AsRef<str>>(
        &self,
        text: S,
    ) -> Vec<HtmlElement> {
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

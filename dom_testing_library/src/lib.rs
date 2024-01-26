use std::ops::Deref;
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};

mod internal;
use internal::*;
mod dom_query;
pub use dom_query::*;
mod element_wrapper;
pub use element_wrapper::*;
mod error;
pub use error::*;
mod test_element;
pub use test_element::*;

//We need to use unit_tests feature because wasm_pack can only run either an integration test or unit_tests at once?
#[cfg(all(test, feature = "unit_tests"))]
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

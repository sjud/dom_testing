/*
Test get_by_text get_all_by_text get_by_text_contains get_all_by_text_contains
*/

use dom_testing_library::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, Node};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn by_text() {
    // console_error_panic_hook::set_once();//
    let document = web_sys::window().unwrap().document().unwrap();
    let document = DocumentWrapper(&document);
    let div = document.create_element("div").unwrap();
    div.clone()
        .unchecked_into::<HtmlElement>()
        .set_inner_text("hello");
    document
        .body()
        .unwrap()
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();

    _ = document
        .get_by_text_contains("hello")
        .expect(&document.body_string());
    let div_2 = document.create_element("div").unwrap();
    div_2
        .clone()
        .unchecked_into::<HtmlElement>()
        .set_inner_text("hello_2");
    document
        .body()
        .unwrap()
        .append_child(div_2.dyn_ref::<Node>().unwrap())
        .unwrap();
    let err = document.get_by_text_contains("hello");
    assert!(err.is_err_and(|e| e.is_more_than_one()));
    assert_eq!(
        div,
        document.get_by_text("hello").unwrap().0.unchecked_into()
    );
    assert_ne!(div_2, (div));
    assert_eq!(
        document
            .get_by_text("hello_2")
            .unwrap()
            .0
            .unchecked_into::<web_sys::Element>(),
        div_2
    );
    assert!(document
        .get_by_text("hello_3")
        .is_err_and(|e| e.is_not_found()));
    assert_eq!(document.get_all_by_text_contains("hello").len(), 2);
    assert_eq!(document.get_all_by_text("hello").len(), 1);
}

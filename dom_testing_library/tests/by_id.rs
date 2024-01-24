/*
Test get_by_id get_all_by_id get_by_id_contains get_all_by_id_contains
*/

use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use dom_testing_library::*;
use web_sys::Node;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn find_component_by_text() {
    let document = web_sys::window().unwrap().document().unwrap();
    let document = DocumentWrapper(&document);
    let div = document.create_element("div").unwrap();
    document.body().unwrap()
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div_2 = document.create_element("div").unwrap();
    document.body().unwrap()
        .append_child(div_2.dyn_ref::<Node>().unwrap())
        .unwrap();
    div.set_id("div_1");
    _ = document.get_by_id_contains("div").unwrap();
    assert_eq!(div,document.get_by_id_contains("div").unwrap().0.unchecked_into::<>());
    div_2.set_id("div_2");
    assert_ne!(div_2,(div));
    let err = document.get_by_id_contains("div");
    assert_eq!(err,Err(Error::MoreThanOne{method:"by_id_contains".to_string(),ident:"div".to_string()}));
    assert_eq!(document.get_by_id("div_1").unwrap().0.unchecked_into::<web_sys::Element>(),div);
    assert_eq!(document.get_by_id("div_3"),Err(Error::NotFound{ident:"div_3".to_string(),method:"by_id".to_string()}));
    assert_ne!(document.get_by_id("div_1").unwrap(),(document.get_by_id("div_2").unwrap()));
    assert_eq!(document.get_all_by_id_contains("div").unwrap().len(),2);
}
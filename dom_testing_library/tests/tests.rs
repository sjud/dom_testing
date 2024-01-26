use dom_testing_library::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlElement, HtmlInputElement, HtmlLabelElement, Node};
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn by_id() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let div = document.create_element("div").unwrap();
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    test_wrapper
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div_2 = document.create_element("div").unwrap();
    test_wrapper
        .append_child(div_2.dyn_ref::<Node>().unwrap())
        .unwrap();
    div.set_id("div_1");
    _ = renderer.get_by_id_contains("div").unwrap();
    assert_eq!(
        div,
        renderer
            .get_by_id_contains("div")
            .unwrap()
            .0
            .unchecked_into()
    );
    div_2.set_id("div_2");
    assert_ne!(div_2, div);
    let err = renderer.get_by_id_contains("div");
    assert!(err.is_more_than_one());
    assert_eq!(
        renderer
            .get_by_id("div_1")
            .unwrap()
            .0
            .unchecked_into::<web_sys::Element>(),
        div
    );
    assert!(renderer.get_by_id("div_3").is_not_found());
    assert_ne!(
        renderer.get_by_id("div_1").unwrap(),
        (renderer.get_by_id("div_2").unwrap())
    );
    assert_eq!(renderer.get_all_by_id_contains("div").len(), 2);
}

#[wasm_bindgen_test]
pub fn by_label() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div = document.create_element("div").unwrap();
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field = document.create_element("input").unwrap();
    label.set_html_for("field_1");
    label.set_inner_text("Field One");
    field.set_id("field_1");
    div.append_child(&label.into()).unwrap();
    div.append_child(&field.into()).unwrap();
    test_wrapper.append_child(&div.into()).unwrap();
    assert!(renderer.get_by_label("Field One").is_ok());
    assert!(renderer.get_by_label_contains("Field").is_ok());
    assert!(renderer.get_by_label("Fiexl").is_not_found());
    assert!(renderer.get_by_label_contains("xx").is_not_found());
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field = document.create_element("input").unwrap();
    label.set_html_for("field_2");
    label.set_inner_text("Field Two");
    field.set_id("field_2");
    test_wrapper.append_child(&label.into()).unwrap();
    test_wrapper.append_child(&field.into()).unwrap();
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field: web_sys::Element = document.create_element("input").unwrap();
    label.set_html_for("field_3");
    label.set_inner_text("Field One");
    field.set_id("field_3");
    test_wrapper.append_child(&label.into()).unwrap();
    test_wrapper.append_child(&field.into()).unwrap();
    assert!(renderer.get_by_label_contains("Field").is_more_than_one());
    assert_eq!(renderer.get_all_by_label("Field One").len(), 2);
    assert_eq!(renderer.get_all_by_label_contains("Field").len(), 3);
}

#[wasm_bindgen_test]
pub fn by_text() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div = document.create_element("div").unwrap();
    div.clone()
        .unchecked_into::<HtmlElement>()
        .set_inner_text("hello");
    test_wrapper
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();

    _ = renderer.get_by_text_contains("hello").unwrap();
    let div_2 = document.create_element("div").unwrap();
    div_2
        .clone()
        .unchecked_into::<HtmlElement>()
        .set_inner_text("hello_2");
    test_wrapper
        .append_child(div_2.dyn_ref::<Node>().unwrap())
        .unwrap();
    let err = renderer.get_by_text_contains("hello");
    assert!(err.is_more_than_one());
    assert_eq!(
        div,
        renderer.get_by_text("hello").unwrap().0.unchecked_into()
    );
    assert_ne!(div_2, (div));
    assert_eq!(
        renderer
            .get_by_text("hello_2")
            .unwrap()
            .0
            .unchecked_into::<web_sys::Element>(),
        div_2
    );
    assert!(renderer.get_by_text("hello_3").is_not_found());
    assert_eq!(renderer.get_all_by_text_contains("hello").len(), 2);
    assert_eq!(renderer.get_all_by_text("hello").len(), 1);
}

#[wasm_bindgen_test]
pub fn by_display_value() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div = document.create_element("div").unwrap();
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field = document.create_element("input").unwrap();
    label.set_html_for("field_1");
    label.set_inner_text("Password");
    field.set_id("field_1");
    div.append_child(&label.into()).unwrap();
    div.append_child(field.dyn_ref::<Node>().unwrap()).unwrap();
    test_wrapper
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();
    let field = TestElement(field.unchecked_into::<HtmlElement>());
    field.set_display_value("1234");
    let input = renderer.get_by_display_value("1234").unwrap();
    assert_eq!(input, field);
    let area = document.create_element("textarea").unwrap();
    div.append_child(area.dyn_ref::<Node>().unwrap()).unwrap();
    let area = TestElement(area.unchecked_into::<HtmlElement>());
    area.set_display_value("1234");
    assert_eq!(renderer.get_all_by_display_value("1234").len(), 2);
}

#[wasm_bindgen_test]
pub fn by_role() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div = document.create_element("div").unwrap();
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field = document
        .create_element("input")
        .unwrap()
        .unchecked_into::<HtmlInputElement>();
    label.set_html_for("field_1");
    field.set_inner_text("Password");
    field.set_attribute("role", "password").unwrap();
    field.set_id("field_1");
    div.append_child(&label.into()).unwrap();
    div.append_child(field.dyn_ref::<Node>().unwrap()).unwrap();
    test_wrapper
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();
    assert!(renderer.get_by_role("password").is_ok());
}

#[wasm_bindgen_test]
pub fn by_placeholder() {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    let renderer = ElementWrapper(&test_wrapper);
    body.append_child(test_wrapper.dyn_ref::<Node>().unwrap())
        .unwrap();
    let div = document.create_element("div").unwrap();
    let label = document
        .create_element("label")
        .unwrap()
        .unchecked_into::<HtmlLabelElement>();
    let field = document
        .create_element("input")
        .unwrap()
        .unchecked_into::<HtmlInputElement>();
    label.set_html_for("field_1");
    field.set_placeholder("Password");
    field.set_id("field_1");
    div.append_child(&label.into()).unwrap();
    div.append_child(field.dyn_ref::<Node>().unwrap()).unwrap();
    test_wrapper
        .append_child(div.dyn_ref::<Node>().unwrap())
        .unwrap();
    let input = renderer.get_by_placeholder("Password").unwrap();
    assert_eq!(input, TestElement(field.unchecked_into::<HtmlElement>()));
}

use dom_testing_library::*;
use leptos::{create_runtime, mount_to, IntoView, RuntimeId};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn render_for_test<F, N>(f: F) -> TestRender
where
    F: FnOnce() -> N + 'static,
    N: IntoView,
{
    let runtime_id = create_runtime();
    let document = leptos::document();
    let body = document.body().unwrap();
    let test_wrapper = document.create_element("div").unwrap();
    body.append_child(&test_wrapper).unwrap();
    mount_to(test_wrapper.clone().unchecked_into::<HtmlElement>(), f);
    TestRender {
        runtime_id,
        element: test_wrapper,
    }
}

pub struct TestRender {
    runtime_id: RuntimeId,
    element: web_sys::Element,
}
impl Drop for TestRender {
    fn drop(&mut self) {
        self.runtime_id.dispose();
    }
}
impl HoldsElement for TestRender {
    fn element(&self) -> ElementWrapper {
        ElementWrapper(&self.element)
    }
}

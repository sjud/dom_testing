use dom_testing_library::*;
use leptos::{create_runtime, IntoView, RuntimeId};

pub fn render_for_test<F, N>(f: F) -> TestRender
    where
        F: FnOnce() -> N + 'static,
        N: IntoView, { 
    let runtime_id = create_runtime();
    leptos::mount_to_body(f);
    let document = leptos::document();
    TestRender{runtime_id,document}
} 


pub struct TestRender{
    runtime_id:RuntimeId,
    document:web_sys::Document,
}
impl Drop for TestRender {
    fn drop(&mut self) {
        self.runtime_id.dispose();
        
    }
}
impl HoldsDocument for TestRender{
    fn document(&self) -> DocumentWrapper {
        DocumentWrapper(&self.document)
    }
}
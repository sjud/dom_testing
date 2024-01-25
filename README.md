<h1>DOM Testing Library</h1>
<h2>Introduction</h2>
The dom-testing-library family of traits helps you test UI components in a user-centric way.
Inspired by Javascript's <a href="https://testing-library.com">Testing Library</a>

<h3>
    The more your tests resemble the way your software is used, the more confidence they can give you.
</h3>

<h2>
The problem
</h2>
You want to write maintainable tests that give you high confidence that your components are working for your users. As a part of this goal, you want your tests to avoid including implementation details so refactors of your components (changes to implementation but not functionality) don't break your tests and slow you and your team down.<br>
<h2>The solution</h2>
The core library, DOM Testing Library, is a light-weight solution for testing web pages by querying and interacting with DOM nodes. The main utilities it provides involve querying the DOM for nodes in a way that's similar to how the user finds elements on the page. In this way, the library helps ensure your tests give you confidence that your application will work when a real user uses it.

<h2>What this library is not</h2>
<ol>
    <li>A test runner or framework</li>
    <li>Specific to a testing framework</li>
</ol>
<h2>What you should avoid with DOM Testing Library</h2>
Testing Library encourages you to avoid testing implementation details like the internals of a component you're testing (though it's still possible). The Guiding Principles of this library emphasize a focus on tests that closely resemble how your web pages are interacted by the users.

You may want to avoid the following implementation details:
<ol>
    <li>Internal state of a component</li>
    <li>Internal methods of a component</li>
    <li>Lifecycle methods of a component</li>
    <li>Child components</li>
</ol>

<h2>Traits</h2>
The dom testing library has a family of traits that help with dom testing.


<h2>Examples</h2>

<h3>Leptos</h3>

```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn find_component_by_text() {
    let render = render_for_test(||{
        let count = create_rw_signal(0);
        view!{
        <button on:click=count.update(|c|c+=1)>Increment The Output</button>
        <div id="output">{move||count.get()}</div>
        }
    });
    render
        // The get_by_X method series return a struct that derefs into web_sys::HtmlElement
        .get_by_text("Increment The Output")
        .unwrap()
        // So we can just click it!
        .click();
    // value is a helper method that helps us parse our expected type!
    assert_eq!(render.find_by_id("output").unwrap().value::<usize>().unwrap(),1);
}
```
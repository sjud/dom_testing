<h1>DOM Testing Library</h1>
<h2>Introduction</h2>
The dom-testing-library family of traits helps you test UI components in a user-centric way.
Heavily inspired by Javascript's <a href="https://testing-library.com">Testing Library.</a>

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
DOM Testing Library encourages you to avoid testing implementation details like the internals of a component you're testing (though it's still possible). The Guiding Principles of this library emphasize a focus on tests that closely resemble how your web pages are interacted by the users.

You may want to avoid the following implementation details:
<ol>
    <li>Internal state of a component</li>
    <li>Internal methods of a component</li>
    <li>Lifecycle methods of a component</li>
    <li>Child components</li>
</ol>

<h2>Traits</h2>
The dom testing library has a family of traits that help with dom testing.
<h4> DomQuery </h4>
DomQuery can be implemented for any frontend specific renderer, it houses the easy "test like your user uses" domain language and functionality.

<h2>Examples</h2>

<h3>Leptos</h3>
<h3>Query the DOM using method calls even your users would understand.</h3>

```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn find_by_id() {
    let render = render_for_test(||{
        let count = create_rw_signal(0);
        view!{
            <button on:click=move|_|count.update(|c|*c+=1)>"Increment"</button>
            <div id="output">{move||count.get()}</div>
        }
    });
     
    render
        // The get_by_X method series return a struct that derefs into web_sys::HtmlElement
        .get_by_text("Increment")
        .unwrap()
        // So we can just click it!
        .click();

    assert_eq!(render.get_by_id("output").unwrap().parse::<usize>().unwrap(),1);
}
```

<h3>Query related components and iterate easily.</h3>

```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn iterate_list() {
    let render = render_for_test(||{
        view!{
            <ul>
                <li id="list_item_1">Hi</li>
                <li id="list_item_2">how</li>
                <li id="list_item_3">are</li>
                <li id="list_item_4">you?</li>
            </ul>
        }
    });
    let questions = render
        //The get_all_by_X method series return a Vec<TestElement> which derefs into HtmlElement 
        //but has helper functions describing the behavior of your app in a way that describes the usage of your app.
        .get_all_by_id_contains("list_item")
        .into_iter()
        .map(|test_element|test_element.display_text())
        .collect::<Vec<String>>()
        .join(" ");

    assert_eq!(questions,String::from("Hi how are you?"));
}
```

<h3> Convenient error handling for understandable tests.</h3>


```rust
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn handle_errors() {
    let render = render_for_test(||{
        view!{
            <ul>
                <li id="ghost_noises_1">Boo</li>
                <li id="ghost_noises_2">Boo</li>
    
            </ul>
        }
    });
    assert!(render
        .get_by_id_contains("ghost_noises")
        .is_more_than_one());
    assert!(render
        .get_by_id("shark_noise")
        .is_not_found());

}
```

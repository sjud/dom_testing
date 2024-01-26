use dom_testing_library::*;
use leptos::*;
use leptos_test::*;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn find_by_id() {
    let render = render_for_test(|| {
        let count = create_rw_signal(0);
        view! {
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

    assert_eq!(
        render
            .get_by_id("output")
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        1
    );
}

#[wasm_bindgen_test]
pub fn handle_errors() {
    let render = render_for_test(|| {
        view! {
            <ul>
                <li id="ghost_noises_1">Boo</li>
                <li id="ghost_noises_2">Boo</li>

            </ul>
        }
    });
    assert!(render.get_by_id_contains("ghost_noises").is_more_than_one());
    assert!(render.get_by_id("shark_noise").is_not_found());
}

#[wasm_bindgen_test]
pub fn iterate_list() {
    let render = render_for_test(|| {
        view! {
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
        .map(|test_element| test_element.display_text())
        .collect::<Vec<String>>()
        .join(" ");

    assert_eq!(questions, String::from("Hi how are you?"));
}

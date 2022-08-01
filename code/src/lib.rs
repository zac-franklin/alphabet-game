use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document window");

    setup_gameboard("rdelbnu".to_string(), &document);

    setup_guessboard("blunder".to_string(), &document);

    setup_listeners("blunder".to_string(), &document);

    document
        .get_element_by_id("app")
        .expect("should have app element")
        .dyn_ref::<HtmlElement>()
        .expect("should be an HtmlElement")
        .style()
        .set_property("visibility", "visible")
        .expect("should have style help");

    Ok(())
}

fn setup_gameboard(word: String, document: &Document) {
    let mut word_with_spaces = "".to_string();
    let letters = document
        .get_element_by_id("gameboard")
        .expect("should have #green-square on the page");
    for letter in word.chars() {
        word_with_spaces = word_with_spaces + &letter.to_string() + " ";
    }
    let letter_element = document
        .create_element("p")
        .expect("could not create p tag");
    letter_element.set_text_content(Some(&word_with_spaces));

    letters
        .append_child(&letter_element)
        .expect("could not create element");
}

fn setup_guessboard(word: String, document: &Document) {
    let pattern = "[".to_string() + &word + "]";
    let guesses = document
        .get_element_by_id("guessboard")
        .expect("should have #green-square on the page");
    for letter in word.chars() {
        let guess_element = document
            .create_element("input")
            .expect("could not create p tag");
        guess_element.set_attribute("type", "text").expect("bad");
        guess_element
            .set_attribute("id", &letter.to_string())
            .expect("bad");
        guess_element
            .set_attribute("pattern", &pattern)
            .expect("bad");
        guess_element.set_attribute("maxlength", "1").expect("bad");
        guess_element
            .set_attribute("autocapitalize", "none")
            .expect("bad");

        guesses
            .append_child(&guess_element)
            .expect("could not create element");
    }
}

fn setup_listeners(word: String, document: &Document) {
    for letter in word.chars() {
        let guess_board = document
            .get_element_by_id("guessboard")
            .expect("should have guessboard element");

        let winnings_element = document
            .get_element_by_id("winnings")
            .expect("should have element winnings");

        let guess_element_keydown = document
            .get_element_by_id(&letter.to_string())
            .expect("should have #green-square on the page");

        let guess_element_input = document
            .get_element_by_id(&letter.to_string())
            .expect("should have #green-square on the page");

        let a = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
            move |event: web_sys::KeyboardEvent| {
                let key = event
                    .clone()
                    .dyn_into::<web_sys::KeyboardEvent>()
                    .expect("should be keyboard event")
                    .key();
                //TODO: move cursor backwards
                if key == "Backspace" {
                    let prev_sibling = guess_element_keydown.previous_element_sibling();
                    let input_field = guess_element_keydown
                        .dyn_ref::<HtmlInputElement>()
                        .expect("Should be input element");
                    if input_field.value() == "" {
                        move_cursor_backwards(input_field, prev_sibling);
                    }
                }
            },
        );

        let b = Closure::<dyn FnMut()>::new(move || {
            let next_sibling = guess_element_input.next_element_sibling();
            let input_field = guess_element_input
                .dyn_ref::<HtmlInputElement>()
                .expect("should be an input element");
            if input_field.check_validity() && input_field.value() != "" {
                move_cursor_forward(input_field, next_sibling);
                if check(&guess_board, "blunder".to_string()) {
                    winnings_element
                        .dyn_ref::<HtmlElement>()
                        .expect("should be an HtmlElement")
                        .style()
                        .set_property("visibility", "visible")
                        .expect("should have style help");
                    for idx in 0..guess_board.children().length() {
                        if let Some(value) = guess_board.children().item(idx) {
                            value.set_attribute("disabled", "disabled").expect("bad");
                        };
                    }
                }
            } else {
                input_field.set_value("");
            }
        });

        document
            .get_element_by_id(&letter.to_string())
            .expect("should have #green-square on the page")
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement")
            .set_onkeydown(Some(a.as_ref().unchecked_ref()));

        document
            .get_element_by_id(&letter.to_string())
            .expect("should have #green-square on the page")
            .dyn_ref::<HtmlElement>()
            .expect("should be HtmlElement")
            .set_oninput(Some(b.as_ref().unchecked_ref()));

        a.forget();
        b.forget();
    }
}

fn move_cursor_forward(_input_field: &HtmlInputElement, next_element: Option<Element>) {
    if let Some(next) = next_element {
        let next_input = next
            .dyn_ref::<HtmlInputElement>()
            .expect("should be an input element");
        next_input.set_value("");
        next_input.focus().expect("need to focus");
    }
}

fn move_cursor_backwards(_input_field: &HtmlInputElement, prev_element: Option<Element>) {
    if let Some(prev) = prev_element {
        let prev_input = prev
            .dyn_ref::<HtmlInputElement>()
            .expect("should be an input element");
        prev_input.set_value("");
        prev_input.focus().expect("need to focus");
    }
}

fn check(guess_board: &Element, word: String) -> bool {
    let mut score = 0;
    let guess_elements = guess_board.children();
    for (idx, c) in word.chars().enumerate() {
        if let Some(value) = guess_elements.item(idx as u32) {
            let input_val = value
                .dyn_ref::<HtmlInputElement>()
                .expect("should be an input element")
                .value();
            if input_val == c.to_string() {
                score += 1;
            }
        };
    }

    if score == word.chars().count() {
        true
    } else {
        false
    }
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlButtonElement, HtmlCollection, HtmlElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document window");

    setup_guess_board("blunder".to_string(), &document);

    setup_choice_board("rdelbnu".to_string(), &document);

    setup_listeners("blunder".to_string(), &document);

    document
        .get_element_by_id("app")
        .expect("should have app element")
        .dyn_ref::<HtmlElement>()
        .expect("app should be an HtmlElement")
        .style()
        .set_property("visibility", "visible")
        .expect("should be able to set app style to visible");

    Ok(())
}

fn setup_guess_board(correct_word: String, document: &Document) {
    let guess_board = document
        .get_element_by_id("guess_board")
        .expect("should have guess_board element on the page");

    for letter in correct_word.chars() {
        let guess_div = document
            .create_element("div")
            .expect("should be able to create guess div for letter");

        guess_div
            .set_attribute("id", &letter.to_string())
            .expect("should be able to set guess div id");

        guess_div
            .set_attribute("class","guess_div")
            .expect("should be able to set guess div class to guess_div");

        guess_board
            .append_child(&guess_div)
            .expect("could not add guess_div as child to guess_board");
    }
}

fn setup_choice_board(scrambled_word: String, document: &Document) {
    let choice_board = document
        .get_element_by_id("choice_board")
        .expect("should have choice_board element on the page");

    for letter in scrambled_word.chars() {
        let choice_id = generate_choice_id(&letter.to_string());
        let choice_button = document
            .create_element("button")
            .expect("should be able to create choice button for letter");

        choice_button
            .set_attribute("id", &choice_id)
            .expect("should be able to set choice button id");

        choice_button
            .set_attribute("class", "choice_button")
            .expect("should be able to set choice button class to choice_button");

        choice_button.set_text_content(Some(&letter.to_string()));

        choice_board
            .append_child(&choice_button)
            .expect("could not add choice_button as child to choice_board");
    }

    let backspace_choice_id = generate_choice_id(&"backspace".to_string());

    let backspace_button = document
        .create_element("button")
        .expect("could not create backspace button");

    backspace_button
        .set_attribute("id", &backspace_choice_id)
        .expect("should be able to set backspace button id");

    backspace_button
        .set_attribute("class", "backspace_button")
        .expect("should be able to set backspace button class to backspace_button");

    backspace_button.set_text_content(Some("←"));

    choice_board
        .append_child(&backspace_button)
        .expect("could not add backspace_button as child to choice_board");
}

fn setup_listeners(correct_word: String, document: &Document) {
    for letter in correct_word.chars() {
        let choice_id = generate_choice_id(&letter.to_string().to_owned());

        let guess_element_children = document
            .get_element_by_id("guess_board")
            .expect("should have guess_board element on the page")
            .children();

        let choice_board = document
            .get_element_by_id("choice_board")
            .expect("should have choice_board element on the page");

        let backspace_button = document
            .get_element_by_id("backspace_button")
            .expect("should have backspace_button element on the page");

        let winnings_element = document
            .get_element_by_id("winnings")
            .expect("should have winnings element on the page");

        let snack_element = document
            .get_element_by_id("snackbar")
            .expect("should have winnings element on the page");

        let correct_word = correct_word.to_owned();
        
        let handle_guess = Closure::<dyn FnMut(web_sys::Event)>::new(
            move |event: web_sys::Event| {
                let target_element = event
                    .target()
                    .expect("event should have a target element");

                let target_button = target_element
                    .dyn_ref::<HtmlButtonElement>()
                    .expect("target element should be a button");

                target_button.set_disabled(true);

                for child_idx in 0..guess_element_children.length(){
                    if let Some(curr_guess_div) = guess_element_children.item(child_idx) {
                        if curr_guess_div.inner_html() == "" {
                            curr_guess_div.set_inner_html(&letter.to_string());
                            if let None = curr_guess_div.next_element_sibling() {
                                if check_win(&guess_element_children, correct_word.to_string()) {
                                    backspace_button
                                        .dyn_ref::<HtmlButtonElement>()
                                        .expect("backspace_button should be a button element")
                                        .set_disabled(false);

                                    choice_board
                                        .dyn_ref::<HtmlElement>()
                                        .expect("choice_board should be an HtmlElement")
                                        .style()
                                        .set_property("display", "none")
                                        .expect("should be able to set choice_board display style to none");

                                    winnings_element
                                        .dyn_ref::<HtmlElement>()
                                        .expect("winnings_element should be an HtmlElement")
                                        .style()
                                        .set_property("visibility", "visible")
                                        .expect("should be able to set winnings_element style to visible");
                                } else {
                                    snack_element
                                        .dyn_ref::<HtmlElement>()
                                        .expect("snackbar should be an HtmlElement")
                                        .style()
                                        .set_property("visibility", "visible")
                                        .expect("should be able to set snack_element visibility to visible");
                                }
                            }
                            break;
                        }
                    }
                }
            },
        );

        document
            .get_element_by_id(&choice_id)
            .expect("should have choice_id on the page")
            .dyn_ref::<HtmlElement>()
            .expect("choice_id should be HtmlElement")
            .set_onclick(Some(handle_guess.as_ref().unchecked_ref()));

        handle_guess.forget();
    }

    let guess_element_children = document
        .get_element_by_id("guess_board")
        .expect("should have guess_board element on the page")
        .children();

    //move to setup listeners
    let handle_backspace = Closure::<dyn FnMut()>::new(
        move || {
            for child_idx in 0..guess_element_children.length(){
                if let Some(curr_guess_div) = guess_element_children.item(child_idx) {
                    if let Some(next_guess_div) = curr_guess_div.next_element_sibling() {
                        if curr_guess_div.inner_html() != "" && next_guess_div.inner_html() == "" {
                            reset_cleared_letter(&curr_guess_div);
                            break;
                        }
                    } else {
                        reset_cleared_letter(&curr_guess_div);
                        let window = web_sys::window().expect("no global window exists");
                        let document = window
                            .document()
                            .expect("window should have document");

                        document
                            .get_element_by_id("snackbar")
                            .expect("should have winnings element on the page")
                            .dyn_ref::<HtmlElement>()
                            .expect("snackbar should be an HtmlElement")
                            .style()
                            .set_property("visibility", "hidden")
                            .expect("should be able to set snack_element visibility to hidden");

                        break;
                    }
                }
            }
        },
    );

    document
        .get_element_by_id("backspace_button")
        .expect("should have backspace_button on the page")
        .dyn_ref::<HtmlElement>()
        .expect("backspace_button should be HtmlElement")
        .set_onclick(Some(handle_backspace.as_ref().unchecked_ref()));

    handle_backspace.forget();

    let correct_word = correct_word.to_owned();

    let redirect_key_to_click = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
        move |event: web_sys::KeyboardEvent| {
            let key = event
                .clone()
                .dyn_into::<web_sys::KeyboardEvent>()
                .expect("event should be keyboardevent")
                .key();

            let key_lower = key.to_lowercase();
            
            if correct_word.contains(&key_lower) {
                let letter_id = generate_choice_id(&key_lower);
                key_to_click(&letter_id);
            }

            if "backspace" == key_lower  {
                let letter_id = generate_choice_id(&key_lower);
                key_to_click(&letter_id);
            }
        },
    );

    document.set_onkeydown(Some(redirect_key_to_click.as_ref().unchecked_ref()));

    redirect_key_to_click.forget();
}

fn check_win(guess_elements: &HtmlCollection, word: String) -> bool {
    let mut score = 0;
    for (idx, letter) in word.chars().enumerate() {
        if let Some(guess) = guess_elements.item(idx as u32) {
            if guess.inner_html() == letter.to_string() {
                score += 1;
            }
        };
    }

    score == word.chars().count()
}

fn reset_cleared_letter(element: &Element)
{
    let choice_id = generate_choice_id(&element.inner_html().to_string());
    let window = web_sys::window().expect("no global window exists");

    let document = window
        .document()
        .expect("window should have document");

    document
        .get_element_by_id(&choice_id)
        .expect("should have choice_id element on the page")
        .dyn_ref::<HtmlButtonElement>()
        .expect("choice_id should be a button element")
        .set_disabled(false);

    element.set_inner_html("");
}

fn key_to_click(id: &str) {
    let window = web_sys::window().expect("no global window exists");

    let document = window
        .document()
        .expect("window should have document");

    document
        .get_element_by_id(id)
        .expect("should have key value id on the page")
        .dyn_ref::<HtmlElement>()
        .expect("key value id should be HtmlElement")
        .click();
}

fn generate_choice_id(letter: &String) -> String {
    letter.to_owned()+"_button"
}

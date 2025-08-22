use leptos::*;
use wasm_bindgen::prelude::*;

mod components;
mod tax_calculator;

use components::*;
use tax_calculator::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <TaxCalculator />
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    rsx!(cx,
         style { [include_str!("./assets/css/global.css")] }
         Router {}
    )
}

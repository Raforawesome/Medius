use dioxus::prelude::*;

pub fn default(cx: Scope) -> Element {
    let selected = use_state(&cx, || "".to_string());
    rsx!(cx,
        style { [include_str!("../assets/css/dir_select.css")] }
        p {
            class: "select-box",
            [if selected.get() != "" {
                selected.get()
            } else {
                "Select a directory"
            }]
        }
        button {
            class: "select-button",
            onclick: |_| {
                let res = rfd::FileDialog::new()
                    .set_directory(dirs::home_dir().unwrap())
                    .pick_folder();
                if let Some(p) = res {
                    selected.modify(|_| p.to_str().unwrap().to_string());
                }
            },
            "Select"
        }
        button {
            class: "go-button",
            onclick: |_| {},
            "Go!"
        }
    )
}

use dioxus::prelude::*;

pub fn default(cx: Scope) -> Element {
    let selected = use_state(&cx, || "".to_string());
    let folder_open_svg: &'static str = include_str!("../assets/svg/folder_open.svg");

    rsx!(cx,
        style { [include_str!("../assets/css/dir_select.css")] }
        table {
            class: "select-box",
            tr {
                td {
                    dangerous_inner_html: "{folder_open_svg}",
                }
                td {
                    p {
                        class: "select-text",
                        [if selected.get() != "" {
                            selected.get()
                        } else {
                            "Select a directory"
                        }]
                    }
                }
                td {
                    dangerous_inner_html: "{folder_open_svg}",
                }
            }
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

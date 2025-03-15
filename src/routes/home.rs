use axum::response::Html;
use maud::{html, PreEscaped};

use crate::ui::layout::layout;

pub async fn handler() -> Html<String> {
    Html(
        layout(
            "Easyshare",
            html! {
                div class="container" {
                    h1 { "Easyshare"}
                    form id="form" hx-encoding="multipart/form-data" hx-post="/upload" {
                        input type="file" multiple name="files" required;
                        button { "Upload" }
                        div id="load";
                    }
                }
                 (PreEscaped("<script>
                const form = document.getElementById('form')
                form.addEventListener('submit', () => {
                    const loader = document.getElementById('load')
                    loader.classList.add('loader')
                })
    </script>"))
            },
        )
        .into_string(),
    )
}

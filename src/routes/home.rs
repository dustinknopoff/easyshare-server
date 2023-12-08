use axum::response::Html;
use maud::{html, PreEscaped};

use crate::ui::layout::layout;

pub async fn handler() -> Html<String> {
    Html(layout("Easyshare", html! {
        div class="container" {
            h1 { "Easyshare"}
            form id="form" hx-encoding="multipart/form-data" hx-post="/upload" {
                input type="file" multiple name="files" required;
                button { "Upload" }
                progress id="progress" value="0" max="100";
            }
        }
         (PreEscaped("<script>
        htmx.on('#form', 'htmx:xhr:progress', function(evt) {
          htmx.find('#progress').setAttribute('value', evt.detail.loaded/evt.detail.total * 100)
          htmx.find('#progress').classList.add('loading')
        });
    </script>"))
    }).into_string())
}
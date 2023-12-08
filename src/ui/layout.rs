use maud::{Markup, html, DOCTYPE};

pub fn layout(page_title: &str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title {
                (page_title)
            }
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/styles.css" type="text/css";
            script src="https://unpkg.com/htmx.org@1.9.8" {}
        }
        body {
            (children)
        }
    }
}

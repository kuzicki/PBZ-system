use super::base_page::base_template;
use maud::{html, Markup};

pub fn page() -> Markup {
    let content = html! {
        h2 { "Welcome to the Home Page" }
        p { "This is the main content of the home page." }
    };
    base_template("Home", content)
}

use maud::{html, Markup, DOCTYPE};

pub fn base_template(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css";
            }
            body {
                header {
                    h1 { "My Website" }
                    nav {
                        a href="/" { "Home" }
                        a href="/tech" { "Tech" }
                        a href="/unit" { "Unit" }
                        a href="/employee" { "Employee" }
                        a href="/transfer" { "Transfer" }
                        a href="/unit-tech" { "Unit tech" }
                        a href="/employee-tech" { "Employee tech" }
                    }
                }
                main {
                    (content)
                }
                footer {
                    p { "Footer information here." }
                }
            }
        }
    }
}

pub fn method_not_allowed() -> Markup {
    base_error_template("Method not allowed", "Method not allowed")
}

pub fn base_error_template(title: &str, error_name: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css";
            }
            body {
                header {
                    h1 { "My Website" }
                    nav {
                        a href="/" { "Home" }
                        a href="/tech" { "Tech" }
                        a href="/unit" { "Unit" }
                        a href="/employee" { "Employee" }
                        a href="/transfer" { "Transfer" }
                        a href="/unit-tech" { "Unit tech" }
                        a href="/employee-tech" { "Employee tech" }
                    }
                }
                main {
                    h2 { "An error has occured: "}
                    p {(error_name)}
                }
                footer {
                    p { "Footer information here." }
                }
            }
        }
    }
}
